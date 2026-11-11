---
created_at: "2026-11-11T02:17:26.543855657+00:00"
id: "atelier-e05n"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-e05n
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uwxy"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Read T3 Code's threads in the Desk's server and tie them to missions and items by the identities in the records"
updated_at: "2026-11-11T06:27:30.330690046+00:00"
---

## Goal

Make the Desk's server a client of T3 Code, so a page can show which agents are working on a mission and its items: each thread's title, state, model and link, tied to an item, review or mission by the thread identities the records hold. This item is the server's half: the client, the routes and the generated types. The pages come in atelier-l2aj.

## Done when

- [x] `atelier-desk` gets a read-only token by running the configured `t3` command (`auth session issue --subject mcp-client --scope orchestration:read` with a time to live) and calls T3 Code's MCP server over HTTP with it. It gets a new token when the old one expires or is refused. The command's path and the server's address come from the configuration file; with neither set, the Desk makes no call and reports threads as unavailable. The token is never logged, never in a response and never written to disk.
- [x] `GET /api/threads` serves T3 Code's live, unsettled threads with the fields `docs/architecture/desk.md` gives, and for each, `holds[]`: what its identity holds in the records of every repository served (`implementer`, `reviewer`, `lead` or `decider`), found by comparing whole strings.
- [x] The mission and item responses gain `threads[]`. An item's has one entry per identity its record names: its implementers, the reviewers of its reviews and the deciders of its answers. A mission's has its lead, the reviewers of its plan reviews, and every entry of each of its items, each naming the item it is on, so a mission with no lead still shows the agents working on its items; a test covers a mission with no lead and two items held by different threads. Each entry has its role, and the thread's title, state, model and link when T3 Code knows it as a live thread. An identity T3 Code does not know is listed with the identity as text and no link. A test covers an identity holding `/`, `:` and `%`, stored and returned whole.
- [x] The inbox gains, per repository, the live threads that are waiting on a person and hold something in that repository's records.
- [x] Every response that carries threads says whether T3 Code could be reached. With T3 Code stopped or the token refused, every route still answers 200 with its records, threads marked unavailable, and a test shows it with a stand-in that refuses connections.
- [x] The thread list is polled on an interval and kept in memory, so a page request never waits on T3 Code, and one failed poll does not empty the pages: the last good list is served with its age.
- [x] Tests run against a stand-in MCP server inside the test, with responses in the shape the real server gives; the shapes are copied from real responses and the Report says from which calls.
- [x] Proved against the running T3 Code on this host: with the server started from a configuration that names the `t3` binary matching the running server (under `~/.t3/runtime/versions/`, not the older `t3` on PATH), `GET /api/threads` lists this item's own implementer thread as running, and an item in a scratch clone started from inside that thread shows it under `threads` with role `implementer`. The Report gives both responses with any token removed, and says the session was revoked afterwards.
- [x] How a live thread that no record names is tied to a repository (by its T3 project's workspace root, or not at all) is settled, said in the document and tested.
- [x] Nothing reads `~/.t3/userdata/` or any other file of T3 Code's. The generated types in `desk/src/api/` are regenerated, and `docs/architecture/desk.md` describes the routes as built and the configuration's new keys.

## Scope

`next/crates/atelier-desk`, `next/Cargo.toml` and `next/Cargo.lock`, the generated files in `desk/src/api/`, and `docs/architecture/desk.md`. One HTTP client dependency may be added to `atelier-desk` alone, named in the Report with why; the tracker's crates gain none, and the existing dependency test still passes. No page or component of the app, no act, no thread started or messaged: the token is read-only. Effort, context size and token counts are not shown, since the MCP server gives none.

Expected size: 500 to 700 lines of Rust with tests.

## Context

- `docs/architecture/desk.md`: `GET /api/threads` and "Tying a thread to an item". The 2026-11-10 note on atelier-aab1: how an outside process gets a token and what the MCP server gives. Without `--subject mcp-client` the server answers 401.
- T3 Code reports `branch: null` for a thread working on a branch, and a title proves nothing: only the identity in a record ties a thread to work. Under T3 Code an agent's identity is its thread ID.
- A converted clone holds no identities. To get one, clone `~/.cache/atelier-desk/clones/atelier` under `~/.cache/atelier-desk/scratch/` and run the new binary's `start` there from inside a T3 thread; delete the copy afterwards.
- The switch (atelier-u1ko) moves `next/crates/` to `crates/`. Once it is started nothing merges into `next/` until atelier-wd1i has run; work may go on in the branch, and the orchestrator checks before merging. If the cutover runs while the branch is open, stop and report.
- Sizes: complexity medium, value medium, risk low. The risk is T3 Code's response shapes changing between nightlies, which shows as threads marked unavailable, not as a broken page.

## Report

Every Done when line holds at commit f4f7ee50. `cargo nextest run -j 4` in `next/` passes 273 of 273 tests at that commit, 20 of them atelier-desk's; `cargo clippy --all-targets`, `cargo fmt -- --check` and `git diff --check` are clean. By line:

- Token and client: `atelier_desk::t3::Client` runs the configured command as `auth session issue --subject mcp-client --scope orchestration:read --ttl 1h --label atelier-desk --json`, keeps the token in a struct with no `Debug`, and sends it only as a bearer header. Tests `live_threads_are_listed_with_what_they_hold` (the exact arguments; stopping revokes and issues no more) and `a_refused_or_expired_token_is_replaced_and_revoked` (a 401 and an expiry each get a new token and revoke the old session; with every token refused the error says so). Every route test asserts no response holds the stand-in's token text. `without_a_t3_table_no_call_is_made`: no `[t3]` table, no command run and no request.
- `GET /api/threads` with `holds[]`: `live_threads_are_listed_with_what_they_hold`, which also checks the list's arguments and that it follows `nextCursor` page by page.
- `threads[]` on mission and item: `a_mission_and_its_items_show_their_agents` covers a mission with no lead and two items held by different threads, a reviewer and a decider T3 Code does not know (listed with `thread: null`), a mission with a lead, and the identity `thread:delegated-task:command%3Amcp%3Ad4040a34%3Adelegate-task%3A6b85/sub:1`, stored and returned whole.
- Inbox: `the_inbox_lists_waiting_threads_that_hold_work_in_the_repository` (a running holder and a waiting thread no record names are left out).
- T3 Code stopped: `with_t3_code_stopped_every_route_still_answers` uses a port that refuses connections; every route answers 200 with its records and `t3.reachable: false`.
- Polling: `a_failed_poll_keeps_the_last_list`; the first test also shows a request before any poll makes no call to T3 Code.
- Stand-in shapes: `tests/fixtures/t3_thread_list.json` is `structuredContent` of a real `t3_thread_list` call (allProjects, includeSubagents, settled false, the live statuses) on 2026-11-11, trimmed to two threads; the stand-in's `initialize` answer, its `mcp-session-id` header, its 400 for a missing session and the `content`/`structuredContent`/`isError` result are copied from real `initialize`, `tools/list` and `tools/call` responses; the token JSON is a real `t3 auth session issue --json` output with the token replaced.
- Unnamed threads: tied to no repository; said under "Tying a thread to an item" and tested in the threads and inbox tests above.
- No T3 Code file is read (the crate opens none; the only process run is the configured `t3`); `desk/src/api/` regenerated with `ATELIER_UPDATE_TYPES=1 cargo nextest run -p atelier-desk --test types`; `docs/architecture/desk.md` describes the routes, fields, client and `[t3]` keys as built.

Live proof, on this host against T3 Code 0.1.9-nightly.20261110.4478 (`ps` showed that version's `t3 serve` running). A copy of the converted clone at `~/.cache/atelier-desk/scratch/e05n/atelier` ran the new binary's `start atelier-psjw` from inside this item's implementer thread, which wrote the thread ID into `implementers`. `atelier-desk` served that copy on 127.0.0.1:3786 with `[t3] command = "~/.t3/runtime/versions/0.1.9-nightly.20261110.4478/t3"` and `url = "http://127.0.0.1:3773/mcp"`. `GET /api/threads` listed eight live threads, `t3: {reachable: true, age_seconds: 0, error: null}`, and this thread:

```json
{"id": "thread:delegated-task:command%3Amcp%3Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%3Adelegate-task%3A6b851a38-1d19-4a4b-9325-287b92cdc7eb",
 "title": "Implement atelier-e05n: T3 Code threads in the Desk's server", "status": "running",
 "provider": "claudeAgent", "model": "claude-opus-5-5",
 "parent": "mcp:launch:c8dbaaa9e06fab8fb420252b7e948e99f6e9e8c66b86c2ffcb62a7cf50fef889", "pull_requests": [],
 "link": "https://app.t3.codes/threads/f69ed1d3-873d-498a-9515-8ab96392223c/thread%3Adelegated-task%3Acommand%253Amcp%253Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%253Adelegate-task%253A6b851a38-1d19-4a4b-9325-287b92cdc7eb",
 "updated_at": "2026-11-11T06:37:40.348Z",
 "holds": [{"repo": "atelier", "kind": "item", "id": "atelier-psjw", "role": "implementer"}]}
```

`GET /api/repos/atelier/items/atelier-psjw` gave `t3: {reachable: true, age_seconds: 0, error: null}` and:

```json
"threads": [{"identity": "thread:delegated-task:command%3Amcp%3Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%3Adelegate-task%3A6b851a38-1d19-4a4b-9325-287b92cdc7eb",
  "role": "implementer", "item": null,
  "thread": {"title": "Implement atelier-e05n: T3 Code threads in the Desk's server", "status": "running",
   "provider": "claudeAgent", "model": "claude-opus-5-5",
   "link": "https://app.t3.codes/threads/f69ed1d3-873d-498a-9515-8ab96392223c/thread%3Adelegated-task%3Acommand%253Amcp%253Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%253Adelegate-task%253A6b851a38-1d19-4a4b-9325-287b92cdc7eb"}}]
```

The mission page of atelier-z4cr, which has no lead, listed the same thread with `item: "atelier-psjw"`. No response held a token (`grep -c secret` gave 0; the real token is not a fixed string, so the check was that the bodies hold only the fields above). Before the stop, `t3 auth session list --json` showed one `atelier-desk` session (`mcp-client`, `orchestration:read`); after SIGINT the server exited 0 and the list showed none. My three probe sessions (label `desk-probe`) were each revoked in the command that issued them. The scratch copy and the server's configuration were deleted.

### Deviations

- No `branch` field on `/api/threads`: `t3_thread_list` gives none (its output schema has no `branch`), and `t3_thread_read`'s is `null` for a thread on a branch. The document says so.
- A mission or item response that carries threads also has `t3 {reachable, age_seconds, error}`; age is in seconds rather than a time, so the crate needs no time library.
- Size: about 840 lines of crate code and 530 of tests, against 500 to 700 expected in all. The tests are the larger part.

### Decisions

- HTTP client: `ureq` 3 with no default features (no TLS, no compression), added to `atelier-desk` alone and to its `dependencies` test's server list. It is blocking, which fits a poll on its own thread, and it adds four crates to the lock: `ureq`, `ureq-proto`, `utf8-zero` and `base64`. Using `hyper`'s client, already in the lock through axum, would need `hyper-util`'s client features and more code.
- Polling is every 10 seconds (`threads::POLL`), not configurable.
- "Waiting on a person" is T3 Code's status `waiting`; the schema gives no other mark.
- Records-built responses stay cached by commit without threads; each request joins the latest poll to them.

### Open items

- No live thread was `waiting` while this ran, so the inbox's filter is proved only against the stand-in.
- The new response fields are required in the TypeScript types, so a fixture in atelier-94pv typed as `MissionPage`, `ItemPage`, `InboxResponse` or `InboxRepo` needs `threads` and `t3` added.
- Whether the `https://app.t3.codes/...` links open on the phone stays atelier-aab1's open question.
