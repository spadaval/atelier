---
created_at: "2026-10-23T14:13:37.370456462+00:00"
id: "atelier-h1nc"
issue_type: "feature"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-h1nc
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
status: "in_progress"
title: "Land and merge --check end in a verdict, failures first with a fix"
updated_at: "2026-10-23T14:41:01.688405825+00:00"
---

## Goal

`land` and `merge --check` print failing checks first, one line each naming the record, the cause and a runnable fix with real IDs, collapse passes, and end with a verdict line that counts every failed check (`land: refused — 3 checks (…)`, `merge: refused — …`, `merge: passed`). A passing land lists what it checked and the next step; a successful merge is short and points to `atelier land`, then push. One gate-line renderer serves both commands.

## Done when

CLI tests over land and merge refusal and pass paths assert the exact lines: failures before passes, one fix command per failure using the fixture's real IDs, a verdict naming N checks when N > 1, a passing land's checked groups and Next line, and a short merge success. Includes a many-fix_now case. Delivers the renderer and verdict parts of S1; S1's precondition clause is atelier-rc8j's later increment. Author proof plus independent adversarial review.

## Scope

Children: atelier-9jr8, atelier-6rh7, atelier-bvl3, atelier-2tux. Gate reason text in `review_gate.rs` (`evaluate_scope`, `completion_claims`): dedupe transitive with direct blocker failures, drop empty `completion blockers:` reasons, fold `claims.reviewed` into `review.current` when they share a cause, dedupe authors by email. Do not change which checks pass or fail; this is output only. No `--json` changes.

## Context

Regroom 2026-10-23 (notes on each child) located the code: land renderer `crates/atelier-cli/src/commands/session.rs:426-457`, verdict `:595-622` (always counts 1); merge `merge.rs:615-642, 708-716, 787-800`; the 5-plus-count path lister exists three times in session.rs. Policy: `docs/product/human-cli-output.md` Verdict Line (update its "counts one check until…" sentence), ADR 0022.

## Notes

- 2026-10-23 [decision] The verdict names every failed gate check, the owner's bare and another record's as '<record> <check>', including checks folded into another line; land no longer prints land_invariants_unmet
  Reason: '| tail -1' then reads every failing check, as S1 asks; folding removes repeated causes from the lines without hiding a check
  Rejected: Keeping land_invariants_unmet as the verdict's one code; counting only the printed lines
- 2026-10-23 [decision] GateNode carries structured gaps and a fix, both serde-skipped; the renderer builds lines from them, and only the reason text changes in --json
  Reason: Lines need the records behind a failure (open items, blockers, findings, receipts) without parsing reason strings, and ADR 0022 keeps --json's shape
  Rejected: Parsing reason strings in the renderer; adding fields to --json
- 2026-10-23 [decision] merge and merge --check print gate lines and the verdict on stdout (stderr under --quiet), as merge --sync does; land's refusal stays on stderr
  Reason: Each command keeps the stream its refusal already used, so redirects and existing callers keep working
  Rejected: Moving land's refusal to stdout
