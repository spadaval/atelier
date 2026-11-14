---
schema: atelier.item
schema_version: 3
id: atelier-nl43
title: Call a state-changing command an action, not an act, everywhere
status: done
mission: atelier-dy62
labels: []
priority: low
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3A1828caa7-d9d0-48e3-8641-39d9f47ddb06
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

- [x] In `crates/`, `Act` is `Action`, the `acts` module is `actions`, and the other identifiers built on the noun follow (`ActRequest`, `ActResponse` and the like). The compiler finds the rest.
- [x] The Desk's app follows: its generated API types, `Acts.tsx`, `lib/acts.ts` and their tests. If the server's routes name an act, they change with the app, which ships with it.
- [x] `atelier --help` and each command's help say "action", as does every line a command prints. The Report lists each changed line of output, and the tests that expect them change by that word alone.
- [x] `docs/product.md`, `docs/product/desk.md`, `docs/design.md`, `docs/architecture/desk.md` and `skills/agent-factory` say "action" for the noun. The verb stays where it is plain English ("three commands act on Git").
- [x] No record format changes, and no record under `.atelier/` is edited. If a stored field or a file name holds the word, the item stops and the Report says where.
- [x] `cargo nextest run`, `pnpm test` and `pnpm e2e` pass.

## Scope

In: `crates/`, `desk/`, the four documents, `skills/agent-factory`.

Out: any other rename; `CHANGELOG.md`'s past entries; the wording of old records and reviews; the live copy of the skill at `~/projects/atelier-skill`, which the host's move to the branch model (atelier-ql64) brings forward.

## Context

On 2026-11-14 `Act` and `acts::` occurred 262 times in 19 files under `crates/`, and the word about 76 times in `docs/design.md`, 27 in `docs/architecture/desk.md` and 6 in `docs/product.md`. It runs before atelier-x5al, which waits on it so that two wide edits do not overlap: atelier-x5al also waits on atelier-yclc, which has no go yet, and this item does not need it. The mission's Authority names this item as the one to drop if the mission has to shrink.

## Report

The noun is "action" in the code, the Desk's app, the help, the output and
the four documents. The verb "act" stays where it is plain English: "three
commands act on Git", "the name the human acts under", "nothing to act on",
"Only the Desk's own app may act" and the test
`only_the_desks_own_app_may_act`. No stored field or record file name held
the word, and no record under `.atelier/` changed except this one's Report.

Checks on this branch, from base 0b973a7c0: `cargo nextest run` ran 362
tests, 362 passed and 1 skipped, as on the base. `cargo fmt -- --check`,
`cargo clippy --all-targets -- -D warnings` and `git diff --check` pass. In
`desk/`, `pnpm typecheck` passes, `pnpm test` passed 41 of 41 and `pnpm
e2e` 50 of 50, after `pnpm build`.

Code, by `git mv` and a token rename (`Act` to `Action`, `act` to `action`,
also inside identifiers such as `item_acts` and `StopAct`):

- `crates/atelier-core/src/acts.rs` is `actions.rs`, and `acts::` is
  `actions::` everywhere. `Act` is `Action`. Locals, fields and functions
  follow: `act`, `refused_act`, `item_acts`, `mission_acts`,
  `review_acts`, `log::History::acts` (now `actions`) and the type alias
  `Acts` in `log.rs`.
- Test files `crates/atelier-core/tests/acts.rs` and
  `crates/atelier-desk/tests/acts.rs` are `actions.rs`; test names that
  held the noun say "action". `rustfmt` rewrapped lines the longer names
  made too wide, in `check.rs`, `plan.rs`, `actions.rs` and tests.
- The Desk's server: the route `POST /api/act` is `POST /api/action`, and
  the request's tag field `act` is `action`
  (`{"action": "stop", ...}`). `ActRequest`, `ActResponse` and `StopAct`
  are `ActionRequest`, `ActionResponse` and `StopAction`, and their
  generated files in `desk/src/api/` were renamed to match, which
  `atelier-desk::types` confirms.
- The app: `components/Acts.tsx` is `Actions.tsx`, `lib/acts.ts` and its
  test are `actions.ts` and `actions.test.ts`, `e2e/acts.spec.ts` is
  `actions.spec.ts`, the e2e helper `expectOnlyTheActs` is
  `expectOnlyTheActions`, and the CSS classes `act-result`, `act-text`,
  `act-rules`, `act-next` and `act-bar` are `action-…`.
- The live check `desk/live/act-is-a-record.ts` and its `pnpm` script are
  `action-is-a-record`; `desk/README.md` and `docs/architecture/desk.md`
  name it so. The done item atelier-v958 and its reviews still say
  `act-is-a-record`, as old records.

Changed lines of output. Nothing else a command prints held the noun:

- `atelier --help`, the `log` line: "The history of acts, read from Git…"
  is "The history of actions, read from Git…".
- `atelier log --help`: the same first line; "the item or mission, the
  act, the thread that acted…" is "…the action, the thread that acted…";
  "The acts are start, take-over, …" and "two acts on one record" are
  "The actions are …" and "two actions on one record"; `--since`'s "Only
  acts committed at or after…" is "Only actions committed…".
- `atelier check` and the pre-commit hook, refusing a status change:
  "Rule: a status changes only as an act of the lifecycle changes it.
  These change it as no act does:" and "A status changes only as an act
  of the lifecycle changes it (…)" say "action" for "act" in each place.
  `crates/atelier-cli/tests/hooks.rs` expects them with that word alone
  changed.

Documents: `docs/product.md`, `docs/product/desk.md`, `docs/design.md`
and `docs/architecture/desk.md` say "action" for the noun, including the
action table's heading column, the `actions` module in the crate table and
the heading "Action functions". `skills/agent-factory` never used the word,
so it is unchanged.

Outside this item's Scope and left alone, still saying "act" as a noun:
`docs/architecture/quality/standards.md:23` ("what its acts write"), the
field `act` and the comment "one line per act" in `scripts/scorecard/`,
and `acts::next` in an eval result under `evals/agent-factory/results/`.