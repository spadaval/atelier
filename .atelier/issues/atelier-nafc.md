---
created_at: "2026-11-10T00:56:08.858805140+00:00"
id: "atelier-nafc"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-nafc
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1sbw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-dxrj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ym3z"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Rehearse the converter and the new binary on all five repositories, with one item taken to done in two"
updated_at: "2026-11-11T00:17:39.804090382+00:00"
---

## Goal

Prove the converter and the new binary on real data before the cutover depends on them: every repository converts and checks clean, and a whole item goes from ready to done on converted records in two of them. This is the mission criterion `rehearsal`.

## Done when

- [x] On a fresh scratch clone of each of the five repositories (atelier, astromech, demiurge, life-on-rails, cosplay-studio), `scripts/cutover/convert_records.py` converts, the result is committed in the clone, `--verify` passes, and the new binary's `check` passes.
- [x] In each converted clone, `atelier setup` installs the Git hooks, a commit of a deliberately broken record is refused by the pre-commit hook, and `status` and `list` run. A fresh clone has none of the real repository's local branches, and a converted record on an old branch is not a started item, so the cost of reading started items is measured on purpose: in the converted clone of the repository with the most open items, the new binary starts 30 ready items, the count of local branches the largest repository has today, and `status` and `list` are timed with those 30 branches present. The Report gives the times and says whether that is fast enough.
- [x] One item goes from ready to done in the converted atelier clone and in one converted product clone, by the commands alone: `start`, a small real change, `submit`, `review` and `verdict approve` by a second thread identity, and `merge`, with that repository's converted merge checks running. In each clone the forge path runs and nothing reaches the network, set up the way `next/crates/atelier-cli/tests/merge.rs` does it: the remote keeps a GitHub-form URL, which the forge code requires, Git is configured to rewrite that URL to a local bare repository, Git's allowed transports are limited to local files for every command run in the clone, and `gh` is the test double.
- [x] The Report lists, for each repository, where its CI workflows and its `AGENTS.md` name Atelier or one of its commands, as facts mission 3's cutover needs. Whether a repository's forge requires a pull request to change the base cannot be read from a clone: it is asked of the forge once per repository, with a read-only `gh api` query of the base branch's protection and rules, run outside the scratch clones and sending nothing but the repository's name. Where the forge does not answer or refuses, the Report says it could not be read and does not guess. The Report gives counts, file names and process facts, and nothing of any private repository's content.
- [x] Each defect the rehearsal finds in the converter or the new binary is fixed on this item's branch when it is small and inside the specifications, or filed as an item of this mission and named in the Report. The rehearsal is run again from fresh clones after the last fix.

## Scope

The rehearsal, and small fixes to `scripts/cutover/convert_records.py` or `next/` that it shows are needed. The four product repositories under `/home/supadava/projects/` are read-only and some are private: work only on scratch clones under `/home/supadava/.cache/atelier-fcb7/`, whose remotes are pointed at local bare repositories, as the third line above says, before any command of the new binary runs in them; commit nothing from them here; send nothing from them to any outside service; and report process facts, never their content. The new binary is run from `next/target/` and installed nowhere, and no real repository's hooks, records or configuration are touched. Rolling a cutover back, the runbook and the replacement `AGENTS.md` texts are mission 3's. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Transition ("2. Rehearse") and Testing. The converter is atelier-1sbw; `check` and the hooks are atelier-pz1y, `setup` is atelier-ym3z. The strategy's Risks name the one performance question: reading started items from their branches on every `status` may be slow in the repository with the most branches, and if it is, the fix is a cache inside one command's run, not a stored index.

This is the second of step E's two items in `docs/strategies/redesign-2-core.md`, and the mission's last.

## Notes

- 2026-11-10 [decision] The rehearsal starts beside atelier-zrss and before atelier-t2jo, not after them
  Reason: It needs the converter, check and setup, which are merged, and nothing of the bypass or the second fresh-agent scenario. Its proof is rerun from fresh clones on the final master before the mission closes, so nothing built later escapes it. The orchestrator of atelier-fcb7 changed the order, which the strategy's Authority leaves to it.
- 2026-11-10 [note] Handoff: approved by gpt-6.1-sol (atelier-uwtm) at 7c339a85 in round 1 with no findings; the outcome, proof and open items are in the Report. Scratch clones removed; the five real repositories match the snapshot taken before the first run.

## Report

All five Done when lines hold. `scripts/cutover/rehearse.sh <new work directory>` (224 lines) does the whole rehearsal from fresh clones and stops at the first failure. Its last run, at 6ca28892, passed every step on all five repositories in 36 s. On each clone the converter converts, the result is committed, `--verify` passes and the new binary's `check` passes. `setup` installs the three hooks, the pre-commit hook refuses a commit of a record whose status is broken, and `status` and `list` run (line 1; line 2 below). In astromech, 30 ready items were started, and `status` and `list` were timed with those 30 branches (line 2). One item went from ready to done by the commands alone in the atelier clone and in the life-on-rails clone, through `new`, `ready`, `start`, a one-line change to `README.md`, `submit`, `review` and `verdict approve` by a second thread, and `merge`. The forge path ran there against the `gh` double (line 3). The rehearsal found one defect in the new binary, fixed in 682abe5f with a test that failed before the fix (`planning.rs`, `start_new_creates_an_item_building_on_its_own_branch_in_one_commit`), and was run again from fresh clones after it (line 5). The forge and CI facts are below (line 4). The `next/` suite (211 tests), fmt and clippy `-D warnings` pass, and so do the converter's 14 tests. `git diff --stat master...HEAD -- crates/` is empty.

The last run, with trailing spaces removed (it prints record IDs, never titles):

```text
new binary: 16M, first on PATH with the gh double; Git limited to local files
atelier         clone                  ok   1724 ms  pushes go to atelier.git; fetch transport refused
atelier         names Atelier          ok     11 ms  lines naming Atelier: AGENTS.md:24 .github/workflows/ci.yml:14 .github/workflows/release.yml:1
atelier         convert                ok    723 ms  items: 161, missions: 8
atelier         check                  ok     56 ms  Every record is valid: 161 items, 8 missions and config.toml.
atelier         push to bare           ok     87 ms
atelier         setup and pre-commit   ok    111 ms  post-checkout pre-commit pre-push installed; broken atelier-00hl refused
atelier         status and list        ok    479 ms  status 235 ms, list 224 ms, 1 local branches
atelier         item ready to done     ok   2177 ms  atelier-vii5 done; 3 merge checks ran; merge commit 8216977e pushed; 1 pull requests opened in all
astromech       clone                  ok   1650 ms  pushes go to astromech.git; fetch transport refused
astromech       names Atelier          ok     20 ms  lines naming Atelier: AGENTS.md:16 .github/workflows/atelier.yml:22 .github/workflows/ci.yml:2
astromech       convert                ok   1211 ms  items: 240, missions: 4
astromech       check                  ok     69 ms  Every record is valid: 240 items, 4 missions and config.toml.
astromech       push to bare           ok     83 ms
astromech       setup and pre-commit   ok    124 ms  post-checkout pre-commit pre-push installed; broken astromech-0132 refused
astromech       status and list        ok   1039 ms  status 503 ms, list 513 ms, 1 local branches
astromech       start 30 items         ok  19858 ms  30 items started
astromech       status and list        ok   1136 ms  status 563 ms, list 554 ms, 31 local branches
demiurge        clone                  ok    721 ms  pushes go to demiurge.git; fetch transport refused
demiurge        names Atelier          ok      6 ms  lines naming Atelier: AGENTS.md:4
demiurge        convert                ok    311 ms  items: 19, missions: 1
demiurge        check                  ok     31 ms  Every record is valid: 19 items, 1 mission and config.toml.
demiurge        push to bare           ok     39 ms
demiurge        setup and pre-commit   ok     85 ms  post-checkout pre-commit pre-push installed; broken worldcrafter-08kf refused
demiurge        status and list        ok     85 ms  status 38 ms, list 28 ms, 1 local branches
life-on-rails   clone                  ok    549 ms  pushes go to life-on-rails.git; fetch transport refused
life-on-rails   names Atelier          ok      7 ms  lines naming Atelier: AGENTS.md:8
life-on-rails   convert                ok    487 ms  items: 47, missions: 3
life-on-rails   check                  ok     63 ms  Every record is valid: 47 items, 3 missions and config.toml.
life-on-rails   forge, for rehearsal   ok     37 ms
life-on-rails   push to bare           ok     50 ms
life-on-rails   setup and pre-commit   ok     84 ms  post-checkout pre-commit pre-push installed; broken lor-0xhc refused
life-on-rails   status and list        ok    143 ms  status 66 ms, list 55 ms, 1 local branches
life-on-rails   item ready to done     ok    776 ms  lor-9qph done; 1 merge checks ran; merge commit f63ddcd pushed; 2 pull requests opened in all
cosplay-studio  clone                  ok    312 ms  pushes go to cosplay-studio.git; fetch transport refused
cosplay-studio  names Atelier          ok      7 ms  lines naming Atelier: AGENTS.md:9
cosplay-studio  convert                ok    285 ms  items: 37, missions: 7
cosplay-studio  check                  ok     44 ms  Every record is valid: 37 items, 7 missions and config.toml.
cosplay-studio  push to bare           ok     49 ms
cosplay-studio  setup and pre-commit   ok     81 ms  post-checkout pre-commit pre-push installed; broken atelier-16mu refused
cosplay-studio  status and list        ok    121 ms  status 54 ms, list 47 ms, 1 local branches
all steps passed in 36 s; 12 gh calls, all to the double
```

### How the script proves what it claims

- **The new binary ran, hooks included.** `atelier` first on `PATH` is a two-line script that logs each call and runs the copy of `next/target/debug/atelier` in the work directory. The pre-commit step passes only when the commit is refused, `HEAD` is unchanged, and the log gained a `check --hook pre-commit` call. `ATELIER_SKIP`, `ATELIER_TEST_CI_WAIT` and the harness thread variables are unset, and `T3CODE_THREAD_ID` is set to the script's own threads.
- **Nothing reached the network.** Git's global and system config are replaced by one in the work directory. Each clone's `origin` keeps its GitHub URL, `url.<bare>.pushInsteadOf` sends pushes to a local bare repository, and `GIT_ALLOW_PROTOCOL=file` plus `protocol.allow never` and `protocol.file.allow always` limit every Git command to local files. The clone step fails unless `origin` is the only remote, its push URL is the bare repository, the only `url.` key is that rewrite, and `git ls-remote origin` (the GitHub fetch URL) is refused with "not allowed". `CARGO_NET_OFFLINE=true` covers the cargo merge checks. `gh` is a double that answers from files: a pull request is a file, and CI passes on the commit the branch's push URL holds.
- **The merge checks ran.** `merge` runs each check with `sh -c` found on `PATH`, and the `sh` there logs each `-c` command before running `/bin/sh`. The item step fails unless the merge added one log line per `[[merge_check]]` in the base's `config.toml`: 3 in atelier (`git diff --check`, `cargo fmt -- --check`, `cargo metadata --locked`) and 1 in life-on-rails (`git diff --check`).
- **The item is done everywhere.** After `merge` the step checks that the record says `done` on the base and that the bare repository's base is the local base.

| Repository | Records converted (items, missions) | `--verify` | `check` | Hooks installed and refusing | `status` / `list` |
| --- | --- | --- | --- | --- | --- |
| atelier | 161, 8 | passes, 183 files | valid | yes | 235 / 224 ms |
| astromech | 240, 4 | passes, 252 files | valid | yes | 503 / 513 ms; 563 / 554 ms with 31 branches |
| demiurge | 19, 1 | passes, 23 files | valid | yes | 38 / 28 ms |
| life-on-rails | 47, 3 | passes, 57 files | valid | yes | 66 / 55 ms |
| cosplay-studio | 37, 7 | passes, 53 files | valid | yes | 54 / 47 ms |

### Started branches

Astromech has the most open items (240) and the most local branches today (30). With 30 started items `status` on the base took 563 ms against 503 ms with none, and `list` 554 ms against 513 ms. Across the four runs the difference was 60 to 200 ms, so each started branch costs about 2 to 7 ms. Three runs by hand after the script: `status` on the base 598 to 886 ms, `status` on a started branch 43 ms, `show` of a started item 34 ms. All of it is a debug build; the installed binary is a release build and faster. It is fast enough: no cache is needed, and the strategy's risk did not come true.

### Product clone

Life-on-rails took the full item. It is the only product repository with a converted merge check (`git diff --check`, which is light), and its open items are fewer than astromech's. Astromech, demiurge and cosplay-studio convert with no merge checks. No product repository names a forge, so the script appends a `[forge]` section to life-on-rails' converted `config.toml` and commits it after `--verify` and `check`, so the forge path runs there too. The real repository has none.

### Defects and fumbles

- **Fixed (682abe5f): `status` told an implementer to write a Goal the item already had.** An item whose Goal is written and whose Done when holds no `- [ ]` line got "Write its Goal and its Done when lines in <path> first". It now names only what is missing, with the form a line takes: "Write its Done when lines in <path> first, each as `- [ ] <result>`". Converted records hit this often: 28 of the 30 astromech items the rehearsal started. Ready items whose Done when has no checkbox line: atelier 1, astromech 59, life-on-rails 4, cosplay-studio 29, demiurge 0. Their Done when is prose or plain `- ` bullets, as the old tool allowed.
- **Not fixed; the behaviour is not in the specifications: `merge` says nothing about the merge checks it ran.** On success it prints the merge commit and the push. An agent cannot tell that three checks ran, or that any did. One line such as "Ran 3 merge checks on the merged tree; each passed." would show it. For the orchestrator to file or not.
- **For the specification: an item with no Done when checkbox lines counts as ticked,** so `submit` sends it to review with nothing ticked. The architecture specification defines it that way ("every checklist line ... is ticked"), and the line `status` prints is orientation, not a condition. With 93 ready items in that state after conversion, someone should decide whether `submit` should refuse an item with no lines.
- **"Ready to start" lists items that cannot start.** On the base, `status` lists ready items whose mission is draft or whose blocker is not done under "Ready to start", each with its unmet conditions ticked off below it. The `Then:` line picks one that can start. It is as the specification says, but cosplay-studio shows 29 items under that heading and only one can start.

No step needed `ATELIER_SKIP`. The script pushes each converted base to its bare repository before `setup`. The cutover's runbook installs the hooks first and then pushes. The pre-push hook refuses that push, which is not an Atelier merge (atelier-pz1y's Report), so the runbook's order needs atelier-zrss.

### For the cutover

- **Ready items the new `start` refuses.** Some ready items belong to a draft mission, which the old tool let start and the new one does not: atelier 9 of 24, astromech 14 of 85, demiurge 0 of 2, life-on-rails 7 of 23, cosplay-studio 28 of 30. Those missions were draft in the old records too. This is for the backlog sort (atelier-5zre).
- **Pushing the product bases.** No product repository's config names a forge, so after the cutover no command pushes their base or their branches. Under the old tool their `AGENTS.md` names `land`, which the new tool lacks. The new `AGENTS.md` texts must say how the base reaches `origin`, or the conversion must write a `[forge]` section.
- **Testing the forge offline.** A test of the forge path cannot rewrite the whole URL with `url.<bare>.insteadOf`. The forge code reads `git remote get-url`, which applies that rewrite, and refuses a URL that names no GitHub repository. Only `pushInsteadOf` works, with the transport limit guarding fetches.
- **Clone on the base.** Astromech's real checkout is on a feature branch, so a clone must name the base (`git clone -b main`). The converter must run on the base.
- **Tracked files that still name a moved strategy's old path** (the converter lists them): atelier 3, astromech 1 (`AGENTS.md`), demiurge 4, life-on-rails 0, cosplay-studio 3.
- **Where CI and `AGENTS.md` name Atelier** (lines naming it, and the commands named):
  - atelier: `AGENTS.md` (24 lines: `land`, `merge`, `review`, `setup`, `start`, `status`, `work`, and an install line), `.github/workflows/ci.yml` (14: `check`, `serve`), `.github/workflows/release.yml` (1, no command).
  - astromech: `AGENTS.md` (16: `issue`, `land`, `merge`, `review`, `start`, `status`, `work`), `.github/workflows/atelier.yml` (22: it installs Atelier and runs `doctor` and `land`), `.github/workflows/ci.yml` (2, no command).
  - demiurge: `AGENTS.md` (4: `land`, `merge`, `review`, `start`, `status`, `work`); no workflow names Atelier.
  - life-on-rails: `AGENTS.md` (8: `land`, `merge`, `review`, `start`, `status`); no workflow names Atelier.
  - cosplay-studio: `AGENTS.md` (9: `issue`, `land`, `merge`, `review`, `start`, `status`); no workflow names Atelier.
- **Whether the forge requires a pull request to change the base.** One read-only GraphQL query per repository, run with the real `gh` from outside the clones, asked for the base ref's branch protection rule and its ruleset rules. For each of the five bases the forge answered with no branch protection rule and no rules, so a direct push is not refused. Before those five queries, one schema query (no repository named) checked that the fields exist.

### Rerunning it

`next/target/debug/atelier` must be built (`cargo build -j 4 --manifest-path next/Cargo.toml`). Then `scripts/cutover/rehearse.sh <a new or empty directory>` takes about 35 s, 19 s of it the 30 starts. Each repository's command output is in `<dir>/<repo>.log`, the `gh` calls in `<dir>/gh/calls`, the `atelier` calls in `<dir>/atelier.calls` and the merge checks in `<dir>/sh.calls`. The logs and clones hold the private repositories' records, so read them there and delete the directory afterwards.

### Decisions

- The script logs `atelier`, `sh` and `gh` calls through small scripts on `PATH`, rather than trusting exit codes, so that each claim above is a counted observation.
- The real repositories' snapshot (`git status --short | wc -l`, branch count and names, remote and `url.` config, the hooks directory, `core.hooksPath`) was taken before the first run and after the last. The two match for all five: 0 changed files each, and 11, 30, 2, 5 and 2 branches, with sample hooks only.

### Open items

- The rehearsal ran with the debug binary. The release build's times are not measured.
- The orchestrator files the `merge` output gap and the Done when question above if it wants them. They are not items yet.
