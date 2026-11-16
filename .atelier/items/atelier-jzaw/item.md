---
schema: atelier.item
schema_version: 3
id: atelier-jzaw
title: Show the project, not the system inbox, beside a thread that holds nothing
status: changes_requested
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3Ac3ba7105-6070-4aa0-a13f-d497b3d166bb
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T05:40:15.731911227Z
---

## Goal

The Desk's embedded panel in T3 Code shows the system-wide inbox beside every
thread with no current tie, which is most of the threads Sumeet talks to
(Sumeet, 2026-11-16). A panel beside one thread should never show anything
wider than that thread's project. The host knows the project; the Desk cannot
work it out for a thread with no ties. So `host:thread` gains an optional
`repository`, and the embedded Desk opens that repository's page for a thread
that holds nothing.

## Done when

- [x] `host:thread` takes an optional `repository`: the name of the Desk repository the host's project maps to.
- [x] With no current tie and a `repository` the Desk serves, the embedded Desk opens `/r/<repo>`, and the line reads that the thread holds no open work in this project.
- [x] With no current tie and no `repository`, or one the Desk does not serve, it opens the inbox as before.
- [x] A thread with a current tie opens its tie's page, whatever `repository` says.
- [x] A repeated `host:thread` for the same thread leaves the page alone, even with another `repository`.
- [x] The host contract test in `desk/e2e/host.spec.ts` covers the three cases, and "The host contract" in `docs/architecture/desk.md` describes the field.
- [x] `pnpm test` and `pnpm e2e` pass in `desk/`.

## Scope

In: `desk/src/lib/host.ts`, `desk/src/lib/follow.ts`, the embedded shell in
`desk/src/components/Shell.tsx`, their tests, and `docs/architecture/desk.md`.

Out: any new page, and the T3 Code extension that sends `host:thread`, which
lives in another repository.

## Report

The embedded Desk now opens a thread's project page, not the system inbox,
when the thread holds nothing and the host names the project.

- `host.ts` keeps the thread and its `repository` together, taken from the
  first `host:thread` that names the thread; a later message for the same
  thread, with any `repository`, changes nothing. A `repository` that is not
  a non-empty string counts as none.
- `follow.ts` splits the old `place` into `tied` (the first current tie's
  page, or none) and `place(ties, repository)`, which falls back to
  `/r/<repo>` with "This thread holds no open work in this project." and
  then to the inbox. `serves` says whether `GET /api/repos` lists the name
  with no read error.
- The embedded shell reads `GET /api/repos` only for a thread with no
  current tie and a named repository. A repository the Desk lists but cannot
  read (the fixture's `broken`), or a failed read of the list, gives the
  inbox, since the repository's page would only show an error.
- `desk/e2e/host.spec.ts` adds five cases to the follow table (a tied thread
  with a repository, no tie and past-only ties in a served repository, an
  unknown repository, an unreadable one) and checks that a changed
  `repository` for the same thread leaves the page alone. `follow.test.ts`
  covers `place` with a repository and `serves`.
- `docs/architecture/desk.md` describes the field in the `host:thread` row
  and in "Following the thread".

Checks: in `desk/`, `pnpm test` (60 passed), `pnpm build`, and `pnpm e2e`
(118 passed, desktop and phone).

### Round 2

Fixed review 1-8m4g's finding: a `GET /api/repos` request that rejects,
such as a reset connection, now gives the inbox like a non-OK status does,
instead of falling into the ties read's catch and leaving the last thread's
page. `desk/e2e/host.spec.ts` adds a test that aborts the request after a
thread switch and expects the inbox with its line.
