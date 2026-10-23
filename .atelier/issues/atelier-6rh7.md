---
created_at: "2026-10-17T01:10:57.976480424+00:00"
id: "atelier-6rh7"
issue_type: "feature"
labels:
- "cli-output"
- "merge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Show failing merge gates first, each with its fix, then a verdict"
updated_at: "2026-10-23T14:14:37.299258459+00:00"
---

## Goal

`atelier merge --check` prints every gate in alphabetical order, mixing passes in with failures, and gives no fixes:

- After the bw0b approval went stale, `findings.disposed` was a single line of about 1,500 characters: 21 copies of `atelier-xxxx: unresolved FIX NOW: resolution needs fresh independent approval`. All 21 had one cause, already reported under `review.current`.
- `atelier-cktl has no disposition` doesn't show the finding text or its recommendation.
- `work.terminal: open scoped work: atelier-nqgr` doesn't give `atelier issue transition atelier-nqgr close --reason "…"`.
- `evidence.no_newer_failure: newer failing evidence atelier-n3tn` doesn't give the command or commit, or say to re-record.
- Merged duplicate reasons (review_gate.rs:842): `pass blockers.none_open: no open transitive blockers; no open blockers`. Empty list (review_gate.rs:877): `pass blockers.transitive_none_open: completion blockers: `.
- `review.current` prints `EXPLICIT TRIVIAL EXEMPTION recorded with explanation`, in capitals, even when a fresh independent approval exists (review_gate.rs:725). With only an exemption, fix_now findings FAIL and nothing says why the exemption doesn't count for them.
- A pass exits 0 with no verdict. A failure ends with `Error: merge blocked: a, b, c`, which repeats the gate names.

## Done when

- Failing gates come first, each starting with ⚠ and ending with one concrete fix command.
- Findings are listed one per line with their recommendation and first summary line. fix_now findings that only wait on a fresh approval collapse to `21 fix_now findings resolved; they need a fresh approval (see review.current)`. With a trivial exemption the line says a fix_now resolution needs an independent approval.
- Passes collapse to `✓ 5 gates pass (blockers, findings, close_reason, review.current via atelier-hleo, work.terminal)`. No empty or duplicated reasons. `review.current` names the independent approval when one exists.
- The last line is `Ready: atelier merge <id>` or `Blocked: N gates`.
- `--json` is unchanged. Tests pin the bw0b-like case (stale approval with 20+ fix_now findings) and the passing case.

## Context

In the cross-cutting audit, `merge <id> --check | grep -v ^pass` is the most repeated pipe in the transcripts (92 runs), plus 23 runs that grep `FAIL|Error|Merged`. In this repository, 25 of 38 runs were piped. `land`'s `land_invariants_unmet` prints the same gate reasons, so it benefits from the same changes.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. merge.rs:615-642 prints all gates alphabetically; merge --check has no verdict line (ends 'Error: merge blocked: …', merge.rs:709). New since written: claims.reviewed FAIL (merge.rs:598-614) repeats review.current's cause in ~400 chars and reads 'If X is in X's scope'; fold it when causes match. Duplicate reason dedupe review_gate.rs:1072-1087, empty 'completion blockers: ' :1166, 'EXPLICIT TRIVIAL EXEMPTION' :945, per-finding reason :1019. Ignore this body's --json remark: leave --json untouched (ADR 0022).
