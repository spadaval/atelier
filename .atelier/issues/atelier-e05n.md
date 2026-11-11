---
created_at: "2026-11-11T02:17:26.543855657+00:00"
id: "atelier-e05n"
issue_type: "task"
labels: []
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
status: "backlog"
title: "Read T3 Code's threads in the Desk's server and tie them to missions and items by the identities in the records"
updated_at: "2026-11-11T06:21:52.241084473+00:00"
---

## Goal

Make the Desk's server a client of T3 Code, so a page can show which agents are working on a mission and its items: each thread's title, state, model and link, tied to an item, review or mission by the thread identities the records hold. This item is the server's half: the client, the routes and the generated types. The pages come in atelier-l2aj.

## Done when

- [ ] `atelier-desk` gets a read-only token by running the configured `t3` command (`auth session issue --subject mcp-client --scope orchestration:read` with a time to live) and calls T3 Code's MCP server over HTTP with it. It gets a new token when the old one expires or is refused. The command's path and the server's address come from the configuration file; with neither set, the Desk makes no call and reports threads as unavailable. The token is never logged, never in a response and never written to disk.
- [ ] `GET /api/threads` serves T3 Code's live, unsettled threads with the fields `docs/architecture/desk.md` gives, and for each, `holds[]`: what its identity holds in the records of every repository served (`implementer`, `reviewer`, `lead` or `decider`), found by comparing whole strings.
- [ ] The mission and item responses gain `threads[]`: one entry per identity the records name on that mission or item, with its role, and the thread's title, state, model and link when T3 Code knows it as a live thread. An identity T3 Code does not know is listed with the identity as text and no link. A test covers an identity holding `/`, `:` and `%`, stored and returned whole.
- [ ] The inbox gains, per repository, the live threads that are waiting on a person and hold something in that repository's records.
- [ ] Every response that carries threads says whether T3 Code could be reached. With T3 Code stopped or the token refused, every route still answers 200 with its records, threads marked unavailable, and a test shows it with a stand-in that refuses connections.
- [ ] The thread list is polled on an interval and kept in memory, so a page request never waits on T3 Code, and one failed poll does not empty the pages: the last good list is served with its age.
- [ ] Tests run against a stand-in MCP server inside the test, with responses in the shape the real server gives; the shapes are copied from real responses and the Report says from which calls.
- [ ] Proved against the running T3 Code on this host: with the server started from a configuration that names the `t3` binary matching the running server (under `~/.t3/runtime/versions/`, not the older `t3` on PATH), `GET /api/threads` lists this item's own implementer thread as running, and an item in a scratch clone started from inside that thread shows it under `threads` with role `implementer`. The Report gives both responses with any token removed, and says the session was revoked afterwards.
- [ ] How a live thread that no record names is tied to a repository (by its T3 project's workspace root, or not at all) is settled, said in the document and tested.
- [ ] Nothing reads `~/.t3/userdata/` or any other file of T3 Code's. The generated types in `desk/src/api/` are regenerated, and `docs/architecture/desk.md` describes the routes as built and the configuration's new keys.

## Scope

`next/crates/atelier-desk`, `next/Cargo.toml` and `next/Cargo.lock`, the generated files in `desk/src/api/`, and `docs/architecture/desk.md`. One HTTP client dependency may be added to `atelier-desk` alone, named in the Report with why; the tracker's crates gain none, and the existing dependency test still passes. No page or component of the app, no act, no thread started or messaged: the token is read-only. Effort, context size and token counts are not shown, since the MCP server gives none.

Expected size: 500 to 700 lines of Rust with tests.

## Context

- `docs/architecture/desk.md`: `GET /api/threads` and "Tying a thread to an item". The 2026-11-10 note on atelier-aab1: how an outside process gets a token and what the MCP server gives. Without `--subject mcp-client` the server answers 401.
- T3 Code reports `branch: null` for a thread working on a branch, and a title proves nothing: only the identity in a record ties a thread to work. Under T3 Code an agent's identity is its thread ID.
- A converted clone holds no identities. To get one, clone `~/.cache/atelier-desk/clones/atelier` under `~/.cache/atelier-desk/scratch/` and run the new binary's `start` there from inside a T3 thread; delete the copy afterwards.
- The switch (atelier-u1ko) moves `next/crates/` to `crates/`. Once it is started nothing merges into `next/` until atelier-wd1i has run; work may go on in the branch, and the orchestrator checks before merging. If the cutover runs while the branch is open, stop and report.
- Sizes: complexity medium, value medium, risk low. The risk is T3 Code's response shapes changing between nightlies, which shows as threads marked unavailable, not as a broken page.
