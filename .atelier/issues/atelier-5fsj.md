---
created_at: "2026-11-10T00:56:05.416973437+00:00"
id: "atelier-5fsj"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-5fsj
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jah8"
    type: "advances"
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Script the scorecard's counted measures, so the baseline can be rerun after the cutover"
updated_at: "2026-11-10T16:28:59.849448778+00:00"
---

## Goal

Write the scripts that compute the scorecard's counted measures for a repository and a window, and run them for the baseline. Mission 4 reruns them after the cutover, so they are kept in the repository and reviewed like code.

## Done when

- [x] `scripts/scorecard/README.md` says how to run each script, which inputs it reads, and which T3 database schema it was written against.
- [x] Every script reads its inputs from a snapshot directory given as an argument, and the manifest's commits for Git. No script opens `~/.t3/userdata/` or the live transcript directories; a test asserts that the default paths are not used.
- [x] Running them on the baseline snapshot for atelier, astromech, demiurge, life-on-rails and cosplay-studio writes `docs/architecture/quality/scorecard-baseline/counted.json` and `docs/architecture/quality/scorecard-baseline/counted.md`, giving per repository: process share, confusion, tokens per merged item, calls before first edit, rework, the record-only share of commits, and the line count. Each token number has its coverage beside it, and rework says which commits were old enough to count.
- [x] `python3 -m unittest discover scripts/scorecard` passes. Its tests build a small SQLite file and a scratch Git repository and check each measure's arithmetic, so a changed definition fails a test.
- [x] A script given a T3 database whose schema it does not know exits with a message naming the schema it expected.
- [x] A fresh agent given only the README reran the scripts on the same snapshot and got a byte-identical `counted.json`; the Report records who and when.

## Scope

Python with the standard library only. Nothing changes in the `atelier` binary. The classified measures and the defect count are other items.

The shell-command recogniser covers a closed list of command forms, which `scripts/scorecard/README.md` names in full: calls of atelier, reads and writes of records, record commits and file writes. It is right for each listed form, with a test for each. A form the README does not list as recognised is a stated limit, and recognising it is in scope only when that moves a headline number in `counted.md` on the frozen inputs.

## Context

Each measure's definition is in `docs/strategies/redesign-1-baseline.md`, Design; do not change a definition, and escalate if one cannot be computed as written. The snapshot is at `~/.local/share/atelier-baseline/2026-11/`, described by `docs/architecture/quality/scorecard-baseline/manifest.json`. Starting material, not to be committed as it is: `/var/tmp/atelier-assessment/transcripts/` and `/var/tmp/atelier-assessment/history/`. Tie a thread to an item by the branch it worked on and the item IDs in its title and first message.

## Notes

- 2026-11-10 [note] Escalated to the orchestrator after three review rounds: round 3 (gpt-6.1-sol-5fsj-r3) left fix_now findings atelier-vj7n (R5), atelier-tu8a (R6) and atelier-h5kn (R7), all in the shell-command recogniser in scripts/scorecard/commands.py. Rounds 1 and 2's findings (pu40, ss09, d6fh, fvq9) are fixed at 31d3c6a1 and await an approval. Work is committed at 8b36a366; see the Report.
- 2026-11-10 [decision] 2026-11-10, orchestrator of atelier-jah8: the shell-command recogniser is a heuristic over a closed list of command forms, not a shell. The README lists every form it recognises (calls of atelier, reads and writes of records, record commits, file writes), and the scripts are right for those forms, with a test for each. A form the README does not list is a stated limit, named in the README as not recognised. It becomes this item's work only if recognising it moves a headline number in counted.md, shown in aggregate on the frozen inputs. Calls before first edit keeps the strategy's definition (first write outside .atelier/); T3 file_change alone is not accepted as a proxy, because it moved atelier's median from 20 to 33. Fable 5.1, Fable 5.2 and GPT-6 Astra stay unpriced: the assessment's table is the source of list prices, and an item with an unpriced request stays out of tokens per item, with the coverage saying so.
  Reason: Each review round found a new shell form the recogniser misread, none of which moved a baseline number after round 1. Parsing shell by heuristic has no last edge case, so the contract is the listed forms plus a measured limit. Precision wins over coverage, and the owner asked for better code, not more.
- 2026-11-10 [note] Escalated to the orchestrator after round 5 under its standing decision: round 5 (gpt-6.1-sol-5fsj-r5) left fix_now findings atelier-o0la (R12), atelier-tys3 (R13) and atelier-jdos (R14) on listed forms. Fixing all three in a scratch copy (about 10 lines) changed no headline number on the frozen inputs; only atelier's implementer threads with a first edit went from 80 to 81. Rounds 1 to 4's findings are fixed at cbe5bcfb and await an approval. Work is committed at e05b779a; see the Report.

## Report

All six Done when lines hold. `scripts/scorecard/counted.py --snapshot ~/.local/share/atelier-baseline/2026-11` writes `counted.json` and `counted.md` with the seven measures for the five repositories in one to two minutes, each token number with its coverage and rework with its eligible commits. `python3 -m unittest discover scripts/scorecard` passes 16 tests (`TMPDIR` under `~/.cache`). They build a small T3 database, Claude and Codex transcripts and a scratch Git repository and check every measure's numbers exactly. Thirty-six one-line changes to definitions (among them quotes and comments ignored, substitutions read out of order, wrapper option arguments kept, Python read as text, the rework window, `.atelier/` paths, shell writes ignored, Python path variables or `cd` ignored, Python read from any command, heredocs not attached to their command, single-quoted or nested substitutions mishandled, record commits ignored, tokens judged by text alone, `bash -c` and heredocs and wrappers and keywords not parsed, first versus last usage copy, merged without code, tie order, tests counted, harness exit codes, partly covered and unpriced items, record edit as first edit, records past the cutoff in Codex and T3, one price) each fail a test, and the unchanged copy passes. `Inputs.test_live_stores_are_refused_and_never_opened` asserts `--snapshot` has no default, each live store is refused, and every file and database opened in a full measure lies under the fixture snapshot. Given a database missing the expected tables, `counted.py` exits 1 naming `T3 statev2 with orchestration_v2 projections, as copied on 2026-11-10 (manifest schema_sha256 ebd75cd5...)` and the missing columns (test and a CLI run). Two runs on the snapshot give byte-identical `counted.json` (`Output.test_two_runs_write_identical_files_without_home_paths`, and by hand). `freeze.py verify` still passes: `ok: 4197 files match the manifest; modes are owner-only`, and only `statev2.sqlite` is in `t3/`.

Fresh rerun: a GPT-6 Luna child (`codex`, `gpt-6-luna`), given only the README's text, its path and the checkout path, reran the scripts into a scratch directory on 2026-11-10 at 16:16 UTC, after the last code change (`cbe5bcfb`, after round 4's fixes). Its `counted.json` had SHA-256 `5768105f2c48c1c931d1b3f69a8a7e20922961ea62ffd879f071028e8cd1df68`, the committed file's; `freeze.py verify` and the 16 tests passed. Its one note, that the run took longer than the README's "about a minute", is answered in the README since. Five earlier Luna reruns (14:41, 14:44, 15:11, 15:33 and 15:54 UTC) each matched that code's output; the first noted an unclosed pipe and no expected output for `verify`, fixed in `2026de87`.

### Decisions

Where the strategy's definitions left a choice, the scripts read them as follows; the README states each.

- **Merged item**: `done`, closed in the window, not a mission, and named by a commit on the base's history that changes a file outside `.atelier/`. The strategy names merged items but no test for one; a done decision with no code is not merged. Done counts sit beside it in `counted.md`.
- **Tie**: the first known item ID in the branch, else the title, else the first message, over every T3 thread including subagents Claude's Agent tool started. 1,621 of T3's 2,194 threads are such subagents; tying only top-level threads tied 7 astromech threads to a merged item, against 476 this way. A first message naming several items ties to the first named.
- **Shell commands** are read by a heuristic over the closed list of forms in the README's Command forms section, not as a shell; the orchestrator's decision note of 2026-11-10 and Scope set that contract. A small tokenizer reads words and operators as a shell does, marking an operator only outside quotes (round 4, atelier-wujd). Each simple command is read with the heredoc it is fed and the directory of the `cd` before it; each `$(...)` outside single quotes runs just before the command holding it, with its own `cd` kept inside (rounds 3 and 4, atelier-h5kn and atelier-wyg1). Wrapper options that take an argument skip it (round 4, atelier-ncm6). Python code is read with `ast`, so strings and comments are never calls (round 4, atelier-86f4). A call of atelier is a simple command whose first word is `atelier`, with any subcommand (round 1, atelier-ss09). In atelier the calls go from 5,823 under the first pattern to 4,640; a sample of 25 of the dropped matches were all mentions in quoted code, docs or heredocs. Round 4's fixes move no headline number by more than 0.1 point.
- **Process share**: a commit of records counts when the command names `.atelier/` paths, or when `git commit` printed the hash of a record-only commit on the manifest history (round 1, atelier-d6fh). Most commits run with `-q` and print no hash (38 hash lines from 2,389 `git commit` commands in atelier), so a quiet bare commit of records staged earlier still goes uncounted; the hash rule found one such commit, in life-on-rails. The token form splits each response's list-price cost equally over its tool calls and judges a call T3 recorded from T3's copy, which includes its output.
- **Confusion**: per shell command that calls the installed `atelier` (not `target/debug/atelier`), over every thread of the repository, in the window. A refusal is one of atelier's own formats; a harness's `Error: Exit code N` is not.
- **Calls before first edit**: the strategy says commands, the scorecard says tool calls; the headline counts tool calls and shell commands are reported beside it. An implementer thread is one opened `Act as the implementation sub-agent` or with no role line. The first edit is a T3 `file_change`, or a file write the README lists (redirection, `tee`, `sed -i`, `perl -i`, `cp`, `mv`, `apply_patch`, and Python run by `python -c` or from a heredoc writing a literal path or a name bound to one), outside `.atelier/` and scratch, build and harness directories, with a relative target taken from the `cd` before it (rounds 1 to 3: atelier-pu40, atelier-fvq9, atelier-vj7n, atelier-tu8a). Shell-first and shell-only implementers raise atelier's counted threads from 47 to 80 and lower its median from 33 to 22 calls. Other write forms are listed in the README as not recognised.
- **Rework**: lines are dated when they reach the base (first-parent replay, merges diffed against their first parent), so churn inside a branch before merge is not counted. `.atelier/` and lock files are left out. Replaying to the manifest commit reproduces Git's own line count for every file of all five repositories.
- **Line count**: the main language has the most non-test bytes outside dot-directories (vendored skills live in `.agents/` and `.claude/`). life-on-rails is close: TypeScript 1.13 MB against Kotlin 0.96 MB, so it counts TypeScript. Rust repositories (atelier, astromech) run `scripts/count_non_test_rust.py`.
- **Prices**: the assessment's table. It has no price for Claude Fable 5.1 and 5.2 or GPT-6 Astra; their tokens are counted, not priced, and an item with such a request is left out of tokens per item (6 threads in all).

### Surprises

- Token coverage is near complete, not 13%: 977 of 981 threads tied to a merged item have a transcript, because subagent transcripts survive where their parents' do not.
- Median tokens per merged item ($3.06 to $13.81) sit below the assessment's $11.50 to $20, because orchestrator threads name a mission or several items and are not tied to one item.
- Confusion runs 16 to 25 per 100 atelier calls; about half are `--help` lookups.
- cosplay-studio's commits name items by suffix (`wip/a197-vzo9`), so 8 of its 23 done items count as merged. It has no commits between July and 2026-11-05, so its rework is n/a.
- T3's `provider_turns` carry a `tokenUsage` per turn. The scripts do not use it, since the Design takes tokens from transcripts.

### Open items

- Prices for Fable 5.1, Fable 5.2 and GPT-6 Astra are the orchestrator's call; adding them to `PRICES` changes the token numbers.
- Tie coverage: threads tied to a merged item out of all threads are atelier 346 of 721, astromech 476 of 602, demiurge 64 of 105, life-on-rails 89 of 296, cosplay-studio 6 of 49. Merged items with a tied thread: 147 of 243, 196 of 339, 27 of 83, 34 of 48, 3 of 8.
