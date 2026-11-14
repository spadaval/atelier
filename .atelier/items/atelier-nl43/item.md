---
schema: atelier.item
schema_version: 3
id: atelier-nl43
title: Call a state-changing command an action, not an act, everywhere
status: ready
mission: atelier-dy62
labels: []
priority: low
blocked_by: []
implementers: []
ready:
  by: sumeet, in conversation on 2026-11-14
  at: 2026-11-14T06:21:12.860651075Z
trivial: null
caused_by: null
created_at: 2026-11-14T05:56:52.602587494Z
---

## Goal

Say "action" for a command that changes a record's state, where the code,
the help, the documents and the skill say "act". "Act" is a word of this
tool's own, and Sumeet had to ask what it meant. This is a rename and
changes nothing else.

## Done when

- [ ] In `crates/`, `Act` is `Action`, the `acts` module is `actions`, and the other identifiers built on the noun follow (`ActRequest`, `ActResponse` and the like). The compiler finds the rest.
- [ ] The Desk's app follows: its generated API types, `Acts.tsx`, `lib/acts.ts` and their tests. If the server's routes name an act, they change with the app, which ships with it.
- [ ] `atelier --help` and each command's help say "action", as does every line a command prints. The Report lists each changed line of output, and the tests that expect them change by that word alone.
- [ ] `docs/product.md`, `docs/product/desk.md`, `docs/design.md`, `docs/architecture/desk.md` and `skills/agent-factory` say "action" for the noun. The verb stays where it is plain English ("three commands act on Git").
- [ ] No record format changes, and no record under `.atelier/` is edited. If a stored field or a file name holds the word, the item stops and the Report says where.
- [ ] `cargo nextest run`, `pnpm test` and `pnpm e2e` pass.

## Scope

In: `crates/`, `desk/`, the four documents, `skills/agent-factory`.

Out: any other rename; `CHANGELOG.md`'s past entries; the wording of old records and reviews; the live copy of the skill at `~/projects/atelier-skill`, which the host's move to the branch model (atelier-ql64) brings forward.

## Context

On 2026-11-14 `Act` and `acts::` occurred 262 times in 19 files under `crates/`, and the word about 76 times in `docs/design.md`, 27 in `docs/architecture/desk.md` and 6 in `docs/product.md`. It runs before atelier-x5al, which waits on it so that two wide edits do not overlap: atelier-x5al also waits on atelier-yclc, which has no go yet, and this item does not need it. The mission's Authority names this item as the one to drop if the mission has to shrink.
