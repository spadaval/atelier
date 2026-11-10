# Scorecard scripts

These scripts compute the redesign scorecard's counted measures, the ones that need no judgment,
for each Atelier repository on this host, from frozen inputs. The measures and their definitions
are in `docs/strategies/redesign-1-baseline.md` (Design). The output is
`docs/architecture/quality/scorecard-baseline/counted.json` and `counted.md`.

## Rerun the counted measures

Needs Python 3.11 or later (standard library only) and Git. From the root of an atelier checkout:

```sh
mkdir -p ~/.cache/scorecard-rerun
nice -n 19 ionice -c3 python3 scripts/scorecard/counted.py \
  --snapshot ~/.local/share/atelier-baseline/2026-11 \
  --out ~/.cache/scorecard-rerun
sha256sum ~/.cache/scorecard-rerun/counted.json docs/architecture/quality/scorecard-baseline/counted.json
```

It takes about a minute. Leave out `--out` to overwrite the committed files. The two hashes
match when the rerun reproduces the baseline. Then check the snapshot is unchanged:

```sh
nice -n 19 ionice -c3 python3 scripts/scorecard/freeze.py verify
```

It prints `ok: 4197 files match the manifest; modes are owner-only` and exits 0 when nothing
changed; otherwise it lists each changed, missing or unlisted file and each loose mode, and exits 1.

Options of `counted.py`:

- `--snapshot DIR` (required): the snapshot directory. It has no default. A path inside a live
  store (`~/.t3/userdata/`, `~/.claude/projects/`, `~/.codex/sessions/`) is refused.
- `--projects DIR` (default `~/projects`): the directory holding the five repositories `atelier`,
  `astromech`, `demiurge`, `life-on-rails` and `cosplay-studio`. Each must contain its manifest
  commit. They are only read.
- `--manifest FILE` (default `docs/architecture/quality/scorecard-baseline/manifest.json`): gives
  the cutoff and each repository's commit.
- `--out DIR` (default the manifest's directory): where `counted.json` and `counted.md` go.

Run the tests with a temporary directory outside any Git repository (`/tmp` is small here):

```sh
mkdir -p ~/.cache/scorecard-tmp
TMPDIR=~/.cache/scorecard-tmp python3 -m unittest discover scripts/scorecard
```

## Inputs

- **The snapshot** (`--snapshot`), made once by `freeze.py` and described by `manifest.json`:
  `t3/statev2.sqlite` (T3's database), `claude-projects/<dir>/<session>.jsonl` with
  `<session>/subagents/agent-<id>.jsonl`, and `codex-sessions/YYYY/MM/DD/rollout-*-<session>.jsonl`.
  It holds private messages: never commit it, copy it into the repository, or send any of it to
  an outside service.
- **Each repository at its manifest commit**, read through `git log`, `ls-tree`, `cat-file`,
  `grep` and `show` on that commit and its history, never the working tree. Nothing after the
  manifest commit is read, so a later checkout gives the same numbers.
- **The window**: 2026-10-14T00:00:00Z (`WINDOW_START` in `snapshot.py`) to the manifest's
  `cutoff`. Records time-stamped after the cutoff are dropped.

## The T3 schema these scripts were written against

T3's database format is internal and its `user_version` is 0. The scripts were written against
the copy taken on 2026-11-10, whose schema fingerprint is the manifest's
`parts.t3_database.schema_sha256` (`ebd75cd5...`). They read only these tables and columns, and
`threads.py` refuses a database that lacks any of them, exiting with a message that names the
expected schema:

- `projection_projects(project_id, workspace_root)`
- `orchestration_v2_projection_threads(thread_id, project_id, title, created_at, payload_json)`,
  using `payload_json.branch`
- `orchestration_v2_projection_turn_items(thread_id, turn_item_id, ordinal, type, status, payload_json)`,
  using items of type `command_execution` (`input`, `output`), `file_change` (`fileName`),
  `user_message` (`text`), `subagent` (`childThreadId`, `nativeItemRef.nativeId`), and the other
  tool types, each with `startedAt`
- `orchestration_v2_projection_provider_threads(thread_id, payload_json)`, using
  `nativeThreadRef.driver` and `nativeThreadRef.nativeId`

The database is opened with `file:...?mode=ro&immutable=1`, so no journal, `-wal` or `-shm` file
appears beside it.

## The scripts

`counted.py` is the one to run. The others are modules it imports, one per reader in the
strategy's Design, and can be imported on their own (`sys.path` set to this directory):

| File | Reads | Gives |
| --- | --- | --- |
| `snapshot.py` | the manifest | the window, the live-store guard, a read-only `git` helper |
| `threads.py` | T3's database | `threads()`: each thread's repository, title, branch, first message, role and transcript keys; `calls()`: its shell commands with output, edits and other tool calls; `tie()`: the item it worked on |
| `transcripts.py` | the transcripts | `index()`: transcript files by thread key; `requests()`: each model response's tokens, list price and shell commands; `PRICES` |
| `records.py` | `.atelier/issues/` and history at the manifest commit | `items()`, `merged()`, `id_pattern()` |
| `gitreader.py` | history up to the manifest commit | `commits()`, `rework()`, `record_only()`, `line_count()` |
| `commands.py` | a command, its output or an edited path | `simple_commands()`, `atelier_calls()`, `is_process()`, `is_help()`, `refused()`, `shell_writes()`, `is_product_edit()` |
| `counted.py` | the readers | the seven measures, `counted.json` and `counted.md` |
| `test_scorecard.py` | a small database, transcripts and Git repository it builds | each measure's arithmetic |

`freeze.py` made the snapshot and checks it (`freeze.py verify`); it is not part of a rerun.

## How each measure is taken

- **Merged item**: an issue record at the manifest commit with status `done`, closed in the
  window, not a mission, whose ID is named in the message of a commit on the manifest commit's
  history that changes a file outside `.atelier/`. `counted.md` shows the done count beside it.
- **Thread tied to an item**: the first known item ID in the thread's branch, else in its title,
  else in its first message. Every T3 thread counts, including the subagents Claude's Agent
  tool started, each tied by its own prompt.
- **Shell commands** are parsed the way a shell splits them (`commands.simple_commands`): quotes,
  heredoc bodies, `&&`, `;`, pipes, `$(...)`, keywords such as `do` and `then`, wrappers such as
  `env`, `nice` and `timeout`, and `bash -c` scripts. A call of atelier is a simple command whose
  first word is `atelier` (the installed binary, any subcommand); a mention in quoted text or a
  heredoc is not one.
- **Process share**: over threads tied to a merged item, the share of shell commands that call
  atelier, name a path under `.atelier/`, or ran `git commit` and printed the hash of a commit on
  the manifest commit's history that changes only `.atelier/`. A quiet commit (`-q`) prints no
  hash, so a bare one of records staged earlier is not recognised. The token form gives each model
  response's list-price cost to its tool calls in equal parts and sums the parts that went to
  process commands. A call T3 recorded is judged from T3's copy, which includes its output.
- **Confusion**: over every thread of the repository, shell commands in the window that call
  atelier, and of those, the ones whose output has one of atelier's refusal or error lines
  (`<command>: refused`, `Error: `, clap's `error: unexpected argument` and kin) or that ask for
  help (`--help`, `-h`, `atelier help`, `atelier man`), per 100.
- **Tokens per merged item**: the list-price cost of every thread tied to the item, summed. An
  item counts only when every such thread has a transcript and every request in them is to a
  priced model. Coverage is threads with a transcript out of threads counted.
- **Calls before first edit**: in each thread tied to a merged item whose role is
  `implementation` or unlabelled (no "Act as the ... sub-agent" opening) and that edited a file,
  the tool calls before its first edit outside `.atelier/` and scratch, build and harness
  directories. An edit is a T3 `file_change`, or a shell command that writes a source,
  configuration or documentation file by redirection, `tee`, `sed -i`, `perl -i`, `cp`, `mv`, a
  patch, or Python (`open(p, "w")`, `Path(p).write_text()`, `p.open("w")`, with `p` a literal path
  or a name bound to one). A relative target is taken from the last `cd` before it in the same
  command. Shell commands alone are reported beside it.
- **Rework**: the first-parent history replayed line by line, each line dated when its commit
  reached the base. Of lines written from the window start to 14 days before the cutoff, the
  share deleted within 14 days. `.atelier/` and lock files are left out.
- **Record-only share of commits**: non-merge commits in the window that change only `.atelier/`.
- **Line count**: for a Rust repository, `scripts/count_non_test_rust.py` at the manifest commit;
  otherwise the main language's non-test physical lines, the main language being the one with the
  most non-test bytes outside dot-directories. `counted.md` says what counted as a test.

## Privacy

`counted.json` and `counted.md` hold numbers, item IDs, commit hashes, counts and coverage only:
no message text, thread titles, command text or paths under the home directory. Keep it so when
changing the scripts.
