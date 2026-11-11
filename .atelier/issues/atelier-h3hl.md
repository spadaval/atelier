---
created_at: "2026-11-11T02:17:26.850729926+00:00"
id: "atelier-h3hl"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-h3hl
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
status: "in_progress"
title: "Perform ready and decide from the Desk's server through the core's functions, and start a planning session in T3 Code"
updated_at: "2026-11-11T06:49:59.596222414+00:00"
---

## Goal

Let the Desk act. The server performs `ready` and `decide` by calling `atelier-app`'s own functions, the ones the `atelier` command calls, under the human's name, and starts a planning session for a mission as a new T3 Code thread. This item is the server's half, with the proof of the strategy's act-is-a-record criterion; the buttons come in atelier-l2aj.

## Done when

- [x] `POST /api/act` runs `ready` or `decide` as `docs/architecture/desk.md` gives it under "Act functions": it finds the checkout with `atelier_app::git::checkout_of`, opens it with `Repo::open` under the configured caller and `Skip::new(None)`, calls `atelier_app::ready` or `atelier_app::decide`, and returns `done` with the text, paths and commit, or `refused` with the rules, text and next step as the function gave them. The Desk writes no refusal text of its own for a rule, runs no `atelier` binary and reads no command's output.
- [x] **act-is-a-record.** A test makes a scratch repository on the new records with a draft mission that may be made ready, clones it twice at the same commit, says go through `POST /api/act` in one and calls what the CLI's `ready <mission> --by <name>` calls in the other, and finds one new commit in each with the same message and the same changed paths, and the two mission records equal once the recorded time is set aside. `GET` of the mission then gives status ready.
- [x] Saying go leaves an unrelated file that was staged in that checkout staged and uncommitted, shown by a test.
- [x] `decide` from the Desk writes the answer in the checkout that has the item's branch out and commits nothing. Until a commit holds the answer, the item and its mission carry it in `pending_answers`, from the server's memory of the act, and the question is not listed as open in the inbox. Once a commit on the item's branch holds the answer, it leaves `pending_answers`. A test covers both states. The item response gives `can_decide` and `decide_at`.
- [x] Where no checkout has the branch an act needs, the response is the function's own refusal with its next step, shown by a test for `ready` and for `decide`.
- [x] An act is refused with 403 unless it carries the app's header and its `Origin`, when present, is one of the hosts the Desk answers to. The same rule covers starting a session. Tests cover a missing header and a foreign origin.
- [x] The caller and every identity are passed and stored whole. A test acts with a configured caller that holds a slash and finds it whole in the record.
- [x] `POST /api/sessions` with a repository and a mission starts a T3 Code thread with `t3_thread_launch`, under a second token that may operate, in the T3 project whose workspace root is that repository, with a first message that names the mission and asks for a planning session. It returns the thread's ID and link. Where no T3 project has that root, or T3 Code cannot be reached, it answers with a refusal that says so and starts nothing. The first message's text is in the configuration with a default, and the document gives it.
- [x] Starting a session is proved against the running T3 Code on this host, not a stand-in: one thread is started through `POST /api/sessions` in a scratch T3 project whose root is a scratch clone, its link opens, and the Report gives the thread's ID, the request and the response with any token removed. The scratch thread is stopped and the scratch project removed afterwards, and both sessions revoked. No thread is started in a project of Sumeet's.
- [x] The operating token is asked for only when a session is started, is never logged, returned or written to disk, and no other route uses it.
- [x] The generated types in `desk/src/api/` are regenerated, and `docs/architecture/desk.md` describes both routes, the header, and the acts as built.

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

## Report

Every Done when line holds at 6a1a3751, each shown by a named test in `next/crates/atelier-desk/tests/` or by the live run below. `cargo nextest run -j 4` in `next/` passes 282 of 282 (evidence atelier-ef1v); `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `git diff --check` pass (atelier-srzg).

- Acts through the core's functions, refusals as given: `acts.rs` `act()`; every test in `tests/acts.rs` compares against the function's own result. The Desk runs no binary and writes no refusal text.
- act-is-a-record: `acts::saying_go_from_the_desk_leaves_the_commit_the_cli_leaves`. Two clones of one commit; one new commit in each with the same message and paths; mission records equal but for `ready.at`; `GET` shows `ready`. A third clone made ready `--by someone-else` is checked to differ. Changing the Desk's `by` to `"sumeet"` (caller `sumeet/desk`) made the test fail on the text "Made ready, as sumeet said" against "as sumeet/desk said"; restored, it passes.
- Staged file kept: `acts::saying_go_leaves_an_unrelated_staged_file_staged`.
- decide and pending answers: `acts::an_answer_waits_in_memory_until_a_commit_on_the_branch_holds_it`. It checks no commit on `main` or the item's branch, the record modified in the item's worktree, `pending_answers` on the item and mission pages, the question gone from the inbox, `can_decide` and `decide_at`, and then, after a commit in that worktree, `pending_answers` empty.
- No checkout: `acts::where_no_checkout_has_the_branch_the_function_refuses`, for `decide` and `ready`, each equal to the function's own `Refused` with a `next`.
- 403: `acts::only_the_desks_own_app_may_act` covers no header, a wrong value, a foreign `Origin` and `Origin: null`, on `/api/act` and `/api/sessions`, writing nothing; it also checks that `http://localhost:5173` is let through.
- Caller with a slash: all of `tests/acts.rs` runs as `sumeet/desk`, and finds it whole in `ready.by` and `Answer by sumeet/desk:`.
- Sessions: `sessions::a_session_starts_a_thread_in_the_repositorys_project` (project found through a symlinked root, across pages; launch under the operate token; default message; token issued for 5m and revoked), `sessions::with_no_project_for_the_repository_nothing_starts`, `sessions::with_t3_code_unreachable_or_unconfigured_nothing_starts`. The stand-in refuses a launch under a read token with `capability_denied`, as T3 Code does. No response may contain a token, which each request checks.
- Types regenerated with `ATELIER_UPDATE_TYPES=1`; `types::the_committed_typescript_types_are_current` passes. `docs/architecture/desk.md` gives both routes, the header, the configuration key and the acts as built.

### Validation

Live, on this host, against T3 Code 0.1.9-nightly.20261110.4478 (evidence atelier-6t9m). Scratch clone `~/.cache/atelier-desk/scratch/h3hl/atelier` of the converted clone; scratch T3 project `mcp:fa54771c-2a3e-4efa-9dc2-225a8e8dae53` rooted there; Desk on `127.0.0.1:3871` with the configured `t3` under `~/.t3/runtime/versions/0.1.9-nightly.20261110.4478/`. A `session_message` asked for a one-line reply only.

```console
$ curl -X POST http://127.0.0.1:3871/api/sessions -H 'content-type: application/json' \
    -H 'x-atelier-desk: 1' -H 'origin: http://localhost:3871' \
    -d '{"repo":"atelier","mission":"atelier-295u"}'
{"result":"started","thread_id":"mcp:launch:7d5d9db9a4b1f0a477819ce2173fe75f7c3928e6dcff90ea971ca8acbc6b5470","link":"https://app.t3.codes/threads/f69ed1d3-873d-498a-9515-8ab96392223c/mcp%3Alaunch%3A7d5d9db9a4b1f0a477819ce2173fe75f7c3928e6dcff90ea971ca8acbc6b5470","project_id":"mcp:fa54771c-2a3e-4efa-9dc2-225a8e8dae53"}
```

The thread, titled "Plan atelier-295u: ...", replied in one line and ran nothing. The same request without the header, and with `Origin: http://attacker.example`, got 403. The link opened, in T3 Code's preview browser, T3 Code's web page for the thread, "Open this thread. It opens in T3 Code if you're connected to the machine it runs on", with "Open in desktop app" pointing at `t3code://threads/f69ed1d3-.../mcp%3Alaunch%3A7d5d...`. That browser is not connected to this machine, so it did not show the thread itself. `t3 auth session list` showed the read session only, right after the launch; after SIGINT to the Desk it showed none of the Desk's. The thread was stopped (`no_active_run`), the project deleted with `force` (the thread then read `thread_not_found`), and the scratch folder deleted. No thread was started in any other project. Saying go by hand on the scratch clone (`atelier-295u`) gave one commit, "Ready atelier-295u", changing only that mission's record, with `ready.by: sumeet`; the mission page then showed it ready.

### Decisions

- The project is found with the read token (`t3_project_list`), so the operating token is issued only once a project matches, for 5 minutes, and revoked as soon as the launch returns. A project matches when its `workspaceRoot` is the served directory, compared as canonical paths; another checkout of the same repository does not match.
- A session that cannot start is `200` with `result: refused`, as an act's refusal is.
- The header is `x-atelier-desk: 1`, after the spike's `x-atelier-deck`. The check runs before the method check, so a `GET` of either route is 403.
- An answer leaves memory once the committed records show that question answered, by anyone. A restart forgets the rest; the answer stays in the checkout.
- Each act gets its own request struct (`ReadyAct`, `DecideAct`), because ts-rs cannot read `deny_unknown_fields` on an enum, and an unknown field such as `by` has to be refused.

### Surprises

- T3 Code started the thread on `claude-fable-5-2` in `approval-required` mode, although I had set the scratch project's default model to Claude Haiku 5.5. The Desk names no model, so planning sessions run on whatever T3 Code picks for a caller outside a thread. The document says so.

### Review

Round 1 (GPT-6.1 Sol, xhigh) recorded two `fix_now` findings at 1f516cb2. atelier-h6zc (pending answers matched by heading, which a Decisions section may repeat) is fixed in 6aa9fc86; the new test `acts::an_answer_is_told_from_an_earlier_one_under_the_same_heading` fails without the fix and passes with it, and 283 of 283 tests pass. atelier-j8f8 (master's `desk/e2e/fixtures.ts` lacks the new fields) is the first open item below, escalated.

### Open items

- The merge with master will fail the desk CI job's `pnpm typecheck`. atelier-94pv has merged since this branch started, and `desk/e2e/fixtures.ts` builds typed `MissionPage` and `ItemPage` objects without the new `pending_answers`, `can_decide` and `decide_at`. The fix is about four lines in that file, which this item's Scope excludes. It is for the orchestrator to place.
- Pending answers live in the server's memory only (by design); a restart drops them from the pages until committed.
