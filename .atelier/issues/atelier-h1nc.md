---
created_at: "2026-10-23T14:13:37.370456462+00:00"
id: "atelier-h1nc"
issue_type: "feature"
labels:
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2tux"
  - kind: "issue"
    id: "atelier-6rh7"
  - kind: "issue"
    id: "atelier-9jr8"
  - kind: "issue"
    id: "atelier-bvl3"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Land and merge --check end in a verdict, failures first with a fix"
updated_at: "2026-10-23T14:13:47.238229194+00:00"
---

## Goal

`land` and `merge --check` print failing checks first, one line each naming the record, the cause and a runnable fix with real IDs, collapse passes, and end with a verdict line that counts every failed check (`land: refused — 3 checks (…)`, `merge: refused — …`, `merge: passed`). A passing land lists what it checked and the next step; a successful merge is short and points to `atelier land`, then push. One gate-line renderer serves both commands.

## Done when

CLI tests over land and merge refusal and pass paths assert the exact lines: failures before passes, one fix command per failure using the fixture's real IDs, a verdict naming N checks when N > 1, a passing land's checked groups and Next line, and a short merge success. Includes a many-fix_now case. Delivers the renderer and verdict parts of S1; S1's precondition clause is atelier-rc8j's later increment. Author proof plus independent adversarial review.

## Scope

Children: atelier-9jr8, atelier-6rh7, atelier-bvl3, atelier-2tux. Gate reason text in `review_gate.rs` (`evaluate_scope`, `completion_claims`): dedupe transitive with direct blocker failures, drop empty `completion blockers:` reasons, fold `claims.reviewed` into `review.current` when they share a cause, dedupe authors by email. Do not change which checks pass or fail; this is output only. No `--json` changes.

## Context

Regroom 2026-10-23 (notes on each child) located the code: land renderer `crates/atelier-cli/src/commands/session.rs:426-457`, verdict `:595-622` (always counts 1); merge `merge.rs:615-642, 708-716, 787-800`; the 5-plus-count path lister exists three times in session.rs. Policy: `docs/product/human-cli-output.md` Verdict Line (update its "counts one check until…" sentence), ADR 0022.
