# Scorecard scripts

These scripts compute the redesign scorecard's counted measures, the ones that need no judgment,
for each Atelier repository on this host, from frozen inputs. The measures and their definitions
are in the baseline mission's strategy, `git show
96d02deb7:docs/strategies/redesign-1-baseline.md` (Design). The output is
`docs/architecture/quality/scorecard-baseline/counted.json` and `counted.md`.

These scripts take merged items and thread ties from each repository's `atelier log`, frozen in the
snapshot, and read the record layout of Atelier 0.5. The November baseline's snapshot has no log
and its repositories the old layout, so only the scripts at commit `df9c84a43` reproduce it.

## Rerun the counted measures

Needs Python 3.11 or later (standard library only) and Git. To reproduce the November baseline, run
these from a checkout of `df9c84a43` (`git worktree add --detach ~/.cache/scorecard-baseline df9c84a43`),
whose README they are; a later snapshot runs from a current checkout. From the checkout's root:

```sh
mkdir -p ~/.cache/scorecard-rerun
nice -n 19 ionice -c3 python3 scripts/scorecard/counted.py \
  --snapshot ~/.local/share/atelier-baseline/2026-11 \
  --out ~/.cache/scorecard-rerun
sha256sum ~/.cache/scorecard-rerun/counted.json docs/architecture/quality/scorecard-baseline/counted.json
```

It takes one to two minutes. Leave out `--out` to overwrite the committed files. The two hashes
match when the rerun reproduces the baseline. Then check the snapshot is unchanged:

```sh
nice -n 19 ionice -c3 python3 scripts/scorecard/freeze.py verify
```

It takes no arguments: it checks `~/.local/share/atelier-baseline/2026-11`, the path built into
`freeze.py`, against the committed manifest. It prints
`ok: 4197 files match the manifest; modes are owner-only` and exits 0 when nothing changed; otherwise it lists each changed, missing or unlisted file and each loose mode, and exits 1.

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
  `<session>/subagents/agent-<id>.jsonl`, `codex-sessions/YYYY/MM/DD/rollout-*-<session>.jsonl`,
  and `logs/<repository>.txt`, each repository's whole `atelier log` output. It holds private
  messages: never commit it, copy it into the repository, or send any of it to an outside service.
- **The log** is read as text, by the line layout in `docs/design.md`, The log. The scripts never
  run `atelier`, and compute none of the numbers the gardener takes from the log itself (time in
  each status, rounds, take-overs, escalations, skips).
- **Each repository at its manifest commit**, read through `git log`, `ls-tree`, `grep` and `show`
  on that commit and its history, and `diff` of a commit a `merge` line names against its first
  parent, never the working tree. Nothing after the manifest commit is read, and no record's
  history is walked, so a base that moved on, a deleted branch or an item moved to another mission
  after the freeze gives the same numbers.
- **The window**: 2026-10-14T00:00:00Z (`WINDOW_START` in `snapshot.py`) to the manifest's
  `cutoff`. Records time-stamped after the cutoff are dropped.

## The T3 schema these scripts were written against

T3's database format is internal and its `user_version` is 0. The scripts were written against
the copy taken on 2026-11-10, whose schema fingerprint is the manifest's
`parts.t3_database.schema_sha256` (`ebd75cd5...`). They read only these tables and columns, and
`threads.py` refuses a database that lacks any of them, exiting with a message that names the
expected schema:

- `projection_projects(project_id, workspace_root)`
- `orchestration_v2_projection_threads(thread_id, project_id, created_at)`
- `orchestration_v2_projection_turn_items(thread_id, turn_item_id, ordinal, type, status, payload_json)`,
  using items of type `command_execution` (`input`, `output`), `file_change` (`fileName`),
  `subagent` (`childThreadId`, `nativeItemRef.driver`, `nativeItemRef.nativeId`), and the other
  tool types, each with `startedAt`
- `orchestration_v2_projection_provider_threads(thread_id, payload_json)`, using
  `nativeThreadRef.driver` and `nativeThreadRef.nativeId`

The database is opened with `file:...?mode=ro&immutable=1`, so no journal, `-wal` or `-shm` file
appears beside it.

## The scripts

`counted.py` is the one to run. The others are modules it imports, one per reader in the
strategy's Design, and can be imported on their own (`sys.path` set to this directory).
`draw_samples.py` draws the classified measures' samples from the same inputs: a closed mission's
start and close from its `ready` and `close` lines, its items from the item records at the
manifest commit that name it, its threads from the thread IDs on its and its items' lines and the
T3 threads above them, and fix-now findings from the review files at the manifest commit, leaving
out a finding a later round carries.

| File | Reads | Gives |
| --- | --- | --- |
| `snapshot.py` | the manifest | the window, the live-store guard, a read-only `git` helper |
| `threads.py` | T3's database | `threads()`: each thread's repository, transcript keys and the thread whose identity it acts under; `calls()`: its shell commands with output, edits and other tool calls |
| `transcripts.py` | the transcripts | `index()`: transcript files by thread key; `requests()`: each model response's tokens, list price and shell commands; `PRICES` |
| `atelier_log.py` | the frozen log, and merge commits and missions at the manifest commit | `read()`, `ties()`, `merged()`, `done()`, `missions()` |
| `gitreader.py` | history up to the manifest commit | `rework()`, `record_only()`, `line_count()` |
| `commands.py` | a command, its output or an edited path | `simple_commands()`, `atelier_calls()`, `is_process()`, `is_help()`, `refused()`, `shell_writes()`, `is_product_edit()` |
| `counted.py` | the readers | the seven measures, `counted.json` and `counted.md` |
| `test_scorecard.py` | a small database, transcripts, log and Git repository it builds | each measure's arithmetic, each tie rule, and the samples' populations |

`freeze.py` made the snapshot and checks it (`freeze.py verify`); it is not part of a rerun.

## How each measure is taken

- **Merged item**: an item with a `merge` line in the window whose merge commit, the one the line
  names, changes a file outside `.atelier/` from its first parent. `counted.md` shows beside it
  the items, not missions, with a `merge` or `close` line in the window.
- **Thread tied to an item**: a T3 thread is tied by the first line that names it with `start` or
  an implementer's take-over, which make it an implementer of the line's item, or `review` or a
  reviewer's take-over, which make it a reviewer. A Codex subagent's identity, `<T3
  thread>/<child>`, names its T3 thread. A thread Claude's Agent tool started runs under its
  parent's identity, so it takes its parent's item and role (`tied_by` `parent`). A thread named
  on several items is tied to the first. Any other thread is untied. Every T3 thread counts.
- **Tie coverage differs from the baseline's**, which tied a thread by the first item ID in its
  branch, title or first message. A thread that never ran `start` or `review`, such as an
  orchestrator or a lead, was tied by its title then and is untied now. So the after-run should
  state, beside its numbers, threads tied and untied (`threads` in `counted.json`) and that the
  baseline tied by title, and compare process share and tokens per item with that in mind.
- **Process share**: over threads tied to a merged item, the share of shell commands that call
  atelier or read or write records (see [Command forms](#command-forms)). The token form gives each
  model response's list-price cost to its tool calls in equal parts and sums the parts that went to
  process commands.
- **Confusion**: over every thread of the repository, shell commands in the window that call
  atelier, and of those, the ones whose output has one of atelier's refusal or error lines
  (`<command>: refused`, `Error: ` other than a harness's `Error: Exit code N`, and clap's
  `error: unexpected argument`, `unrecognized subcommand`, `the argument`, `the following
  required arguments` and `invalid value`) or that look up help, per 100.
- **Tokens per merged item**: the list-price cost of every thread tied to the item, summed. An
  item counts only when every such thread has a transcript and every request in them is to a
  priced model. Coverage is threads with a transcript out of threads counted.
- **Calls before first edit**: in each thread tied to a merged item as its implementer that
  edited a file,
  the tool calls before its first edit: a T3 `file_change`, or a shell command's file write (see
  [Command forms](#command-forms)), to a file outside `.atelier/` and scratch, build and harness
  directories. Shell commands alone are reported beside it.
- **Rework**: the first-parent history replayed line by line, each line dated when its commit
  reached the base. Of lines written from the window start to 14 days before the cutoff, the
  share deleted within 14 days. `.atelier/` and lock files are left out.
- **Record-only share of commits**: non-merge commits in the window that change only `.atelier/`.
- **Line count**: for a Rust repository, `scripts/count_non_test_rust.py` at the manifest commit;
  otherwise the main language's non-test physical lines, the main language being the one with the
  most non-test bytes outside dot-directories. `counted.md` says what counted as a test.

## Command forms

`commands.py` reads shell commands by heuristic, not as a shell, and is judged on the frozen inputs:
it keeps only the recognition that moves a headline number in `counted.md` there. It reads a
command into words and operators as a shell does: single quotes, double quotes and backslashes
quote what they cover, a backslash before a newline joins lines, and a number written against a
redirection (`2>`) is part of it. Outside quotes, `;`, `;;`, `&&`, `||`, `|`, `|&`, `&`, `(`,
`)` and newlines end a simple command; `<`, `>`, `>>`, `>|`, `&>`, `&>>`, `>&` and `<<`
redirect. Each simple command is read with the heredoc it is fed and the directory the last `cd`
before it changed to. Before reading it, leading `NAME=value` assignments, the keywords `!`, `{`,
`do`, `elif`, `else`, `if`, `then`, `until` and `while`, and the wrappers `command`, `env`,
`exec`, `ionice`, `nice`, `stdbuf`, `time`, `timeout` and `xargs` with the words after them that
start with `-` or a digit are dropped. The script of `bash`, `sh` or `zsh` given with a `-c` flag
(`-c`, `-lc` and the like) is read as commands of its own.

Recognised:

- **A call of atelier**: a simple command whose first word is `atelier`, or a path ending
  `/.cargo/bin/atelier`, with any arguments. `command -v atelier` and `command -V atelier` are
  lookups, not calls.
- **A help lookup**: a call of atelier with `--help` or `-h` among its arguments, or whose first
  argument not starting with `-` is `help` or `man`.
- **A read or write of records**: any command whose text names `.atelier` (not after a letter,
  digit, `_` or `-`) followed by `/`, whitespace, a quote or backtick, `;`, `)`, `|`, `&` or the
  end. This one is textual and counts the name anywhere, quoted, in a heredoc or in a commit
  message.
- **A file write**: the file after `>`, `>>` or `&>` (not `>&`); the files `sed` or `perl` edit
  with `-i` (or `--in-place`), after the script or the `-e`/`-f` script; and, in code a `python`,
  `python3` or `python3.N` command runs with `-c` or from a heredoc (with no script argument or
  `-`), read with Python's own parser so strings and comments are not calls: `open(p, mode)` with
  a mode starting `w`, `a` or `x`, `p.write_text()`, `p.write_bytes()` and `p.open(mode)`
  likewise, where `p` is a string literal, `Path(...)` of one, or a name the code bound earlier to
  one (an assignment binds after its right-hand side runs). A relative target is joined to the
  directory of the last `cd` before it, with `.` and `..` reduced. A write counts as an edit only
  to a path that is not under `/dev/`, holds no `$`, has no component `.atelier`, `.cache`,
  `.claude`, `.codex`, `.local`, `target`, `node_modules`, a name starting `tmp` or a name
  containing `scratch`, and ends in `.rs`, `.ts`, `.tsx`, `.js`, `.jsx`, `.mjs`, `.cjs`,
  `.svelte`, `.kt`, `.kts`, `.py`, `.go`, `.java`, `.swift`, `.md`, `.toml`, `.json`, `.yaml`,
  `.yml`, `.sh`, `.css`, `.html` or `.sql`.

Not recognised, removed because recognising it moved no headline number on the frozen inputs
(each change is what removing that form alone did):

- `$(...)` as a command of its own: an unquoted one splits at its parentheses like any `(`, and a
  quoted one is text. Removing it changed 5 of 4,640 atelier calls and 4 of 4,140 astromech calls.
- `tee`, `cp`, `mv` and `apply_patch` as file writes. Removing `cp` and `mv` dropped 2 of 132
  astromech threads with a first edit; `tee` and `apply_patch` changed nothing.
- A record commit found by the hash `git commit` printed: one life-on-rails command, 0.02 points
  of its process share.
- Wrapper options that take a non-numeric argument (`env -u NAME`, `stdbuf -o L`, `xargs -I {}`):
  the argument is read as the command. 16 of 4,640 atelier calls, 0.06 points of its process share
  and 0.06 per 100 of its confusion.
- `#` comments, which are read as words: no change.

Removed together, these moved atelier by 21 of 4,640 calls (confusion +0.07 per 100, process share
−0.07 points), astromech by 4 calls and 2 threads with a first edit, and life-on-rails's process
share by 0.02 points; no median of calls before first edit moved.

Not recognised, and never measured as part of the recogniser:

- Backtick and process substitution, `eval`, shell functions and aliases, and variable values:
  `atelier $c` is a call, but `$c` is not expanded, and a write to a path holding `$` is not an
  edit.
- A heredoc whose delimiter line is missing (its body runs to the end of the command), and `<<`
  written inside quotes, which is still read as a heredoc.
- Python code that Python's parser rejects, which is not read at all.
- Scripts run from files (`bash x.sh`, `python3 x.py`) and code in any other interpreter.
- Any other way to write a file (`install`, `rsync`, `dd`, `touch`, `ln`, `git apply`, `git mv`,
  Python through `shutil` or `os.replace`, a path built at run time).
- Directory changes other than `cd <dir>`, and the end of a subshell's `cd` at `)`.
- A commit of records that names no `.atelier/` path in its own command.

## Privacy

`counted.json` and `counted.md` hold numbers, item IDs, commit hashes, counts and coverage only:
no message text, thread titles, command text or paths under the home directory. Keep it so when
changing the scripts.
