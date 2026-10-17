---
created_at: "2026-10-17T01:10:57.965486113+00:00"
id: "atelier-e6fl"
issue_type: "bug"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep or reject --reason on transitions that aren't done transitions"
updated_at: "2026-10-17T01:10:57.965486113+00:00"
---

## Goal

`atelier issue transition db6h block --reason "waiting on vendor"` succeeds and the reason is stored nowhere: `grep -c vendor .atelier/issues/atelier-db6h.md` returns 0. `apply_transition` (`crates/atelier-cli/src/commands/workflow.rs:266-281`) writes `close_reason` only when the target status is in the done category. Any other transition drops it silently. For `block`, the reason is the only information the transition carries.

## Done when

On a transition that doesn't end in a done status, `--reason` either:
- is appended as a typed note (`[blocker]` for transitions into the blocked category, `[note]` otherwise) and echoed in the confirmation, or
- is refused with `--reason applies to done transitions; use atelier issue note <id> --kind blocker "…"`.

A CLI test covers `block --reason`, and `close --reason` behaviour is unchanged.

## Unknowns

Which of the two. Recording is friendlier; refusing keeps `--reason` meaning Close Reason only.
