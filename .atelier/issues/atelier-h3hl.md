---
created_at: "2026-11-11T02:17:26.850729926+00:00"
id: "atelier-h3hl"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-e05n"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-psjw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Perform ready and decide from the Desk's server through the core's functions, and start a planning session in T3 Code"
updated_at: "2026-11-11T06:27:22.151590626+00:00"
---

## Goal

Let the Desk act. The server performs `ready` and `decide` by calling `atelier-app`'s own functions, the ones the `atelier` command calls, under the human's name, and starts a planning session for a mission as a new T3 Code thread. This item is the server's half, with the proof of the strategy's act-is-a-record criterion; the buttons come in atelier-l2aj.

## Done when

- [ ] `POST /api/act` runs `ready` or `decide` as `docs/architecture/desk.md` gives it under "Act functions": it finds the checkout with `atelier_app::git::checkout_of`, opens it with `Repo::open` under the configured caller and `Skip::new(None)`, calls `atelier_app::ready` or `atelier_app::decide`, and returns `done` with the text, paths and commit, or `refused` with the rules, text and next step as the function gave them. The Desk writes no refusal text of its own for a rule, runs no `atelier` binary and reads no command's output.
- [ ] **act-is-a-record.** A test makes a scratch repository on the new records with a draft mission that may be made ready, clones it twice at the same commit, says go through `POST /api/act` in one and calls what the CLI's `ready <mission> --by <name>` calls in the other, and finds one new commit in each with the same message and the same changed paths, and the two mission records equal once the recorded time is set aside. `GET` of the mission then gives status ready.
- [ ] Saying go leaves an unrelated file that was staged in that checkout staged and uncommitted, shown by a test.
- [ ] `decide` from the Desk writes the answer in the checkout that has the item's branch out and commits nothing. Until a commit holds the answer, the item and its mission carry it in `pending_answers`, from the server's memory of the act, and the question is not listed as open in the inbox. Once a commit on the item's branch holds the answer, it leaves `pending_answers`. A test covers both states. The item response gives `can_decide` and `decide_at`.
- [ ] Where no checkout has the branch an act needs, the response is the function's own refusal with its next step, shown by a test for `ready` and for `decide`.
- [ ] An act is refused with 403 unless it carries the app's header and its `Origin`, when present, is one of the hosts the Desk answers to. The same rule covers starting a session. Tests cover a missing header and a foreign origin.
- [ ] The caller and every identity are passed and stored whole. A test acts with a configured caller that holds a slash and finds it whole in the record.
- [ ] `POST /api/sessions` with a repository and a mission starts a T3 Code thread with `t3_thread_launch`, under a second token that may operate, in the T3 project whose workspace root is that repository, with a first message that names the mission and asks for a planning session. It returns the thread's ID and link. Where no T3 project has that root, or T3 Code cannot be reached, it answers with a refusal that says so and starts nothing. The first message's text is in the configuration with a default, and the document gives it.
- [ ] Starting a session is proved against the running T3 Code on this host, not a stand-in: one thread is started through `POST /api/sessions` in a scratch T3 project whose root is a scratch clone, its link opens, and the Report gives the thread's ID, the request and the response with any token removed. The scratch thread is stopped and the scratch project removed afterwards, and both sessions revoked. No thread is started in a project of Sumeet's.
- [ ] The operating token is asked for only when a session is started, is never logged, returned or written to disk, and no other route uses it.
- [ ] The generated types in `desk/src/api/` are regenerated, and `docs/architecture/desk.md` describes both routes, the header, and the acts as built.

## Scope

`next/crates/atelier-desk`, the generated files in `desk/src/api/`, and `docs/architecture/desk.md`. The only acts are `ready` and `decide`; a session starts a thread and writes no record. No change to `atelier-app`, `atelier-cli` or what any command does: if `ready` or `decide` as a function lacks something the Desk needs, stop and report it to the orchestrator, who takes it to the redesign's planner. No page or component of the app. The server never makes a worktree, moves a ref or commits anything itself.

Expected size: 500 to 700 lines of Rust with tests.

## Context

- `docs/strategies/desk.md`, Design, Data flow 3 (Acts) and 4 (Sessions), and Access. `docs/architecture/desk.md`, `POST /api/act` and "Act functions". atelier-psjw (merged) gave the functions.
- Any thread independent of an item may `decide` it (atelier-t6o6), so the human needs no special standing. An identity can hold a slash (atelier-1oel).
- Starting a T3 thread from outside was never exercised: the 2026-11-10 probe on atelier-aab1 only read, and a read-only token was refused on `t3_thread_launch` with `capability_denied`. Use the `t3` binary under `~/.t3/runtime/versions/` that matches the running server. atelier-e05n adds the MCP client this item extends.
- Tests of acts use scratch repositories made in the test. To try an act by hand, clone `~/.cache/atelier-desk/clones/atelier` under `~/.cache/atelier-desk/scratch/`; never act on a live repository or on the shared converted clone.
- The switch (atelier-u1ko) moves `next/crates/` to `crates/`. Once it is started nothing merges into `next/` until atelier-wd1i has run; work may go on in the branch, and the orchestrator checks before merging. If the cutover runs while the branch is open, stop and report.
- Sizes: complexity medium, value high, risk medium. This is the one path that writes: a wrong checkout or a wrong caller would put a record under the wrong name. The act-is-a-record test and the function's own refusals guard it, and saying go is one commit to revert.
