---
created_at: "2026-11-11T03:21:50.136807966+00:00"
id: "atelier-psjw"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1oel"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Move the acts and repository access into atelier-app as functions that return data, make ready commit its own write, and name the human caller"
updated_at: "2026-11-11T03:26:03.986574166+00:00"
---

## Goal

Let a program that links the tracker's core, such as the Desk, perform an act by calling a function and get the result as data. Today the new tool's acts and its repository access live in the CLI crate (`next/crates/atelier-cli/src/plan.rs`, `questions.rs`, `repo.rs`, `git.rs`), so anything but the command line has to run the binary and read its text. `ready` also writes its record and prints a `git commit` line for its caller to run, and `start` carries a refusal for the case where that commit was never made.

Sumeet decided on 2026-11-11 that the Desk (mission atelier-z4cr, `docs/strategies/desk.md`, Design, Acts) calls the core's functions and runs no binary. This item is the tracker's half of that. It is done before the switch (atelier-u1ko) so the new skill is written once, for a `ready` that commits.

## Done when

- [ ] **Repository access is in `atelier-app`.** The `Repo` type and the Git calls it and the acts below need move from `atelier-cli` to `atelier-app`, and every CLI command uses them from there. Each function of today's `git.rs` lives in one crate only, in `atelier-app` when `Repo`, `ready`, `drop`, `close`, `lead` or `decide` calls it. A repository is opened by path, with the caller's identity given by the caller: `atelier-app` reads no environment variable to find the directory, the caller or the skip reason. The CLI passes its working directory, the identity the identity rule gives, and `ATELIER_SKIP`. `atelier-app` gains only dependencies the `next/` workspace already has.
- [ ] **`ready` and `decide` are functions of `atelier-app` with a typed result.** Each returns either what it did (the paths written and, where it committed, the commit) or a refusal: the rules it breaks, one line each, as `skip::refuse` receives them today, its text, and its next step as a field of its own where today's text ends with one. Whether a refusal can be skipped stays decided in one place. The CLI's `ready` and `decide` call these functions and print what they printed before, shown by their existing tests passing unchanged except where a line below changes behaviour. One test in `atelier-app` calls each function on a scratch repository opened by path, from a working directory outside it, and reads the result as data, for a success and for a refusal.
- [ ] **One rule for who commits.** A planning command whose write other threads act on at the base's tip commits it: `ready`, `drop` and `close`, which change a stored status, and `lead`, which binds the role that `status`, a question's handoff and `log` read from the base's tip. `new`, `set`, and settling a finding (`new --from`, `drop` on a finding) still leave the commit to Git, because a draft is edited several times and a settlement on a gardener's branch is committed with its run. The four commands that commit share one piece of code for it.
- [ ] **The commit holds only the paths the command wrote**, each as the file stands in the checkout, with the message the command used to print, and leaves every other staged, changed or untracked file as it was. The commands no longer print a commit line; they say the commit they made. Tests: with an unrelated file staged and another changed, `ready` on a mission makes one commit holding only `mission.md`, the first file is still staged and the second still changed; `ready` on a draft item no commit holds yet commits its whole `item.md`; `drop` of an item that exists only on its branch commits the base record it writes; `close` and `lead` each make one commit of their one path. When the commit fails, as when the pre-commit hook refuses it, the files and the index are as they were before the command, as `submit` already guarantees, and a test shows it for `ready`.
- [ ] **`start`'s refusal for planning not committed** (`not_committed` in `commands.rs`) is removed with its text, or kept with a test that shows a case it still catches now that `ready` commits. The Report says which and why.
- [ ] **A caller can name the person acting.** The identity a caller passes when it opens a repository is the one records get: an answer `decide` writes for a caller named `sumeet` reads `Answer by sumeet:`, where a caller with no identity is `human` today, and `ready`'s `by` is a parameter of the function, which `ready --by` fills. No flag is added. A name the record format cannot read back unchanged is refused before anything is written, with a test.
- [ ] **The specifications say what was built.** In `docs/strategies/redesign-product.md`: the rows for `ready`, `drop`, `close` and `lead`, and "Where records change", say which planning commands commit and which leave it to Git. Nothing in Words, Lifecycle or Review changes. In `docs/strategies/redesign-architecture.md`: the crate table and the sentences that say `atelier-app` runs no Git; "Where records are read and written" (planning commands, `lead`, and `start`'s uncommitted-planning refusal); the sentence in Status that no act makes a commit only to change a status; and Threads and roles, for a caller that names a person. The `last_changed` lines are left alone; the orchestrator writes them once.
- [ ] **Nothing else changed behaviour.** `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass in `next/`, each with `-j 4` where it builds. The Report lists every existing test whose expected output changed, each with the line above that changes it. It gives the non-test line count of each `next/` crate before and after (`scripts/count_non_test_rust.py`); the total should move little, since code moves and is not added.
- [ ] **The cutover's footing holds.** `scripts/cutover/rehearse.sh`, changed only where it ran a commit after one of the four commands, passes on fresh clones of all five repositories with a binary built from this branch. The eval fixtures under `evals/agent-factory/scenarios/next-loop*` still set up: their `setup.sh` runs to its end with that binary, changed only in the same way.

## Scope

`next/crates/atelier-app`, `next/crates/atelier-cli`, their tests, the specification passages named above, `scripts/cutover/rehearse.sh` and the `next-loop` fixtures' `setup.sh` where a commit follows one of the four commands, and this item's record.

No new command, flag, record kind or config option, and no new crate. Not `merge`, `submit`, `review`, `verdict`, `escalate`, `start`'s own commit or the forge code: they stay in `atelier-cli` and behave as they do. `new` and `set` move only as far as sharing `Repo` requires. Not `crates/`, `skills/` or anything of the Desk (`docs/strategies/desk.md`, mission atelier-z4cr and its items), which another thread owns.

A reviewer's finding that asks for behaviour the specifications and this item do not name is escalated to the orchestrator, not built.

## Context

It advances the Desk's mission, atelier-z4cr, and is run by the orchestrator of the cutover mission, atelier-qr5s, because it touches code the cutover switches on. It starts after atelier-1oel merges, since both change how a caller's identity is found, and it merges before the switch branch (atelier-u1ko) is cut.

Specifications: `docs/strategies/redesign-product.md` (Commands, Where records change) and `docs/strategies/redesign-architecture.md` (Workspace and crates, Status, Threads and roles, Where records are read and written, Hooks and the bypass). Today `save` in `plan.rs` writes the records for `new`, `set`, `ready`, `drop` and `close` and ends with a `Then:` line giving the commit; `lead` does the same for `mission.md`. `submit`, the take-over and `verdict` already commit named paths and put the files and the index back when the commit is refused, which is the behaviour to reuse.

The reason for the commit rule: other threads read a status and a lead at a commit, never from the base checkout's working tree, so a `ready` that is written and not committed is invisible to `start`, and the refusal that explains it exists only because the command left its own work half done.

The new binary is run only from `next/target/` in scratch repositories and is installed nowhere.

Expected size: about 800 lines move between the two crates, and about 150 are new or changed outside tests.

## Unknowns

- Whether a planning act could commit straight onto the base branch through Git objects, from any directory, as `start` builds its commit. That would remove the base-checkout rule for `ready`. It is a separate question and not this item's to answer.
