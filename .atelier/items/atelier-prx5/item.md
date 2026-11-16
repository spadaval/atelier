---
schema: atelier.item
schema_version: 3
id: atelier-prx5
title: Tell an embedding host which repository, record and thread it is looking at
status: ready
mission: atelier-61gh
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: 'sumeet, in conversation on 2026-11-15: get that mission going'
  at: 2026-11-16T02:21:33.165230332Z
trivial: null
caused_by: null
created_at: 2026-11-16T02:20:38.137711377Z
---

## Goal

Let a page that frames the Desk learn, in one read each, what the Desk serves
and what a name refers to. Today the T3 extension keeps a hand-made table
from T3 project to Desk repository, and asks for an item and then a mission
to find out which one an ID names.

## Done when

- [ ] `GET /api/repos` gives each repository an `identity`: its `origin` remote as `host/path`, lower case, with no scheme, user, port or `.git`, which is the form T3 Code uses for a project's repository (`github.com/spadaval/atelier`). It is `null` where there is no `origin`. A unit test covers HTTPS, `git@host:path` and `ssh://` remotes.
- [ ] One read answers what an ID names, across every repository served: the repository's name, whether it is an item or a mission, and its title and status. An ID that validates and names nothing is `404`.
- [ ] One read answers what a thread identity holds, from the same map `/api/threads` uses, whether or not T3 Code lists the thread as live. The identity travels in a query parameter, never in the path. A thread that holds nothing gets an empty list, not `404`.
- [ ] The three are typed for the app with `ts-rs`, covered by tests in `crates/atelier-desk`, and documented field by field in `docs/architecture/desk.md`.
- [ ] `cargo nextest run -p atelier-desk`, `cargo clippy --all-targets -- -D warnings` and `pnpm test` pass.

## Scope

In: `crates/atelier-desk`, the generated types in `desk/src/api/`, `docs/architecture/desk.md`.

Out: any page that uses these reads (atelier-knx8 does); CORS headers and the `Host` check, which stay as they are.

## Context

`docs/architecture/desk.md`, "Tying a thread to an item", describes the map from identity to what it holds. T3 Code derives its key in `repositoryKeyFromRemote`; match its output for the remotes above, do not import it.
