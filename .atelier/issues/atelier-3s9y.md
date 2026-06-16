---
created_at: "2026-06-16T15:59:31.663188790+00:00"
id: "atelier-3s9y"
issue_type: "feature"
labels:
- "cli"
- "dependencies"
- "issue"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-em15"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Sort issue list queues by blocker order"
updated_at: "2026-06-16T15:59:31.663188790+00:00"
---

## Description

Update issue queue output so agents scanning `atelier issue list` variants see prerequisite work before dependent work and can read each row's action state without decoding workflow internals. This covers the list family used for discovery and next-work selection.

## Outcome

- `atelier issue list` orders visible issue rows so open blockers in the displayed set appear before the work they unblock, while preserving the existing grouping and compact row format.
- `atelier issue list` rows show a single readable state such as ready, blocked, active, review, validation, or done instead of `category/status` strings like `todo/todo`.
- Blocked queue rows show blocked state with a compact count or drill-down cue, not a long inline `blocked by <id>` suffix as the primary signal.
- `atelier issue list --ready` keeps excluding blocked work, but applies the shared deterministic tie breakers so ready output stays consistent with the broader queue contract.
- `atelier issue list --blocked` orders blocked rows by the shared dependency rule where blocked items depend on each other, and points to `atelier issue blocked <id>` or equivalent detail when exact blocker IDs are needed.
- Quiet output emits IDs in the same order as the human queue view for the selected filter.
- Exact `--status` and `--category` filters continue to work, with hidden blockers shown as blocker cues rather than inserted rows.

## Evidence

- Failing-before/passing-after CLI test or transcript shows a visible blocker chain in `atelier issue list` with the blocker row before the blocked row.
- CLI transcript or focused test for `atelier issue list` proves default rows contain readable state labels and do not contain duplicate category/status tokens.
- Transcript or test proves `atelier issue list --ready` excludes blocked rows and uses deterministic ordering for multiple ready candidates.
- Transcript or test proves `atelier issue list --blocked` communicates blocked state and a drill-down path while ordering dependent blocked rows consistently.
- Quiet-mode transcript shows the emitted ID order matches the human queue order.
- `atelier lint atelier-3s9y`, `atelier export --check`, focused cargo tests, and `git diff --check` pass.

## Notes

This issue owns the queue family only. It should not change issue lifecycle semantics, blocker creation, or workflow gates.
