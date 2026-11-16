---
schema: atelier.item
schema_version: 3
id: atelier-prx5
title: Tell an embedding host which repository, record and thread it is looking at
status: changes_requested
mission: atelier-61gh
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3A197fd59b-2928-4d71-aa4d-94382300d50d
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

- [x] `GET /api/repos` gives each repository an `identity`: its `origin` remote as `host/path`, lower case, with no scheme, user, port or `.git`, which is the form T3 Code uses for a project's repository (`github.com/spadaval/atelier`). It is `null` where there is no `origin`. A unit test covers HTTPS, `git@host:path` and `ssh://` remotes.
- [x] One read answers what an ID names, across every repository served: the repository's name, whether it is an item or a mission, and its title and status. An ID that validates and names nothing is `404`.
- [x] One read answers what a thread identity is tied to, whether or not T3 Code lists the thread as live. The identity travels in a query parameter, never in the path. A thread tied to nothing gets an empty list, not `404`.
- [x] Each tie in that read carries the record's status and `current`. `current` is true for the last implementer of an item that is started, in review or approved; for a reviewer of an item's or a plan's open round; and for the lead or a planner of an open mission. It is false for a replaced implementer, a reviewer of a closed round, and any tie to a done or dropped record. `/api/threads` and the pages that use its map answer as they do on the base.
- [x] A test covers one thread that implemented an item now done and reviews another item's open round (the first tie is not current, the second is), and a replaced implementer (not current).
- [x] The three are typed for the app with `ts-rs`, covered by tests in `crates/atelier-desk`, and documented field by field in `docs/architecture/desk.md`.
- [x] `cargo nextest run -p atelier-desk`, `cargo clippy --all-targets -- -D warnings` and `pnpm test` pass.

## Scope

In: `crates/atelier-desk`, the generated types in `desk/src/api/`, `docs/architecture/desk.md`.

Out: any page that uses these reads (atelier-knx8 does); CORS headers and the `Host` check, which stay as they are.

## Context

`docs/architecture/desk.md`, "Tying a thread to an item", describes the map from identity to what it holds. That map (`holds` in `crates/atelier-desk/src/read.rs`) collects every implementer an item ever had and every round's reviewers, so it cannot say what is current; `Entry::holder` and `Entry::reviewer` can. T3 Code derives its key in `repositoryKeyFromRemote`; match its output for the remotes above, do not import it.

## Report

A page that frames the Desk can now learn, in one read each, which
repository a T3 project is, what an ID names and what a thread is tied to.

- `GET /api/repos` gives each repository `identity`, its `origin` as
  `host/path` (`fetch::identity`, `fetch::origin_identity`). The unit test
  `a_remote_is_named_as_t3_code_names_a_repository` covers HTTPS (with a
  user, token and port), `git@host:path`, bare `host:path`, `ssh://` with
  and without a port, and `git+ssh://`. A local path or `file://` URL gives
  `null`, as no `origin` does. It ports T3 Code's
  `parseGitRemoteRepository` rule for rule, reading a URL with the `url`
  crate (WHATWG, as T3's `new URL()`), so an IPv6 host is compressed, an
  international host is Punycode and `.`/`..` are resolved. Every case in
  the test was checked against that function run from the local T3 Code
  checkout.
  A host whose source control definition respells its keys (Azure
  DevOps over SSH) is not respelled here.
- `GET /api/records/{id}` answers `{repo, kind, id, title, status}`, from
  the first repository served that holds the ID, in configuration order.
  `404` when none does, `400` for an ID that does not validate.
- `GET /api/ties?thread=<identity>` answers `{thread, ties[]}`, each tie
  `{repo, kind, id, title, status, role, current}`. It reads the records
  only, so it answers with no T3 Code configured. An unknown identity gets
  `ties: []`; a missing or empty `thread` is `400`.
- Both reads share one per-repository index (`read::index`), cached by
  commits like every response. `holds` and `/api/threads` are untouched.
- `status` is a new `RecordStatus`, an untagged union of the item and
  mission status types, so the app gets `Status | MissionStatus`.
- `tests/embedding.rs` covers the three reads over two repositories: a
  thread that implemented a done item and reviews another's open round
  (false, then true), a replaced implementer and its taker (false, true),
  closed and open rounds of item and plan review, a planner, and a lead of
  an open and a done mission.
- `desk/e2e/fixtures.ts` gained `identity` on its two repositories, since
  `tsc` refuses the fixture without it. That file is outside the stated
  scope, but the change only keeps the build typing.

One reading to check: for a plan's round, "a reviewer of an open round" is
taken literally. While any review of the latest round lacks a verdict, the
holder of every review in it is current, including one who already gave a
verdict. Items have one review per round, so the question does not arise
there.

Checks: `cargo nextest run -p atelier-desk` (47 passed), `cargo clippy
--all-targets -- -D warnings`, `cargo fmt -- --check`, `pnpm test` (41
passed) and `tsc --noEmit` in `desk/`.

### Responses

- 1-ctz4.1: fixed in cb7ff9fd2
- 2-mihq.2: fixed in 5c7122fd4
