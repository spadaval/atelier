---
created_at: "2026-10-17T09:14:09.736875227+00:00"
id: "atelier-pvo0"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
  - kind: "review"
    id: "atelier-378y"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-hrfh"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-wslw"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:40.770159490+00:00"
status: "superseded"
title: "Name the command for the remaining silent or bare truncations"
updated_at: "2026-11-11T03:55:40.770159490+00:00"
---

## Goal

atelier-ytxg made the work-view lists name the command that shows the rest. Its review found more cuts outside that item:
- `issue show`'s Recently Updated rollup stops at 8 rows with no count or command (crates/atelier-cli/src/commands/issue.rs; finding atelier-hrfh).
- "Did you mean a, b, c (and N more)?" names no command (crates/atelier-app/src/record_ref.rs; finding atelier-378y).
- `status` on an owner's branch silently cuts the owner's scoped work at 8, and `evidence list`'s `Showing: N of M` names no command.

## Done when

- [ ] Each of these names the command that shows the rest, or is documented as a bounded window.
- [ ] Rendering tests cover each.

## Context

Dropped in decision atelier-mzpn: `land`'s path lists (finding atelier-wslw), which atelier-md0v reshaped when `land` became report-only on work branches.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The truncations it names are in `issue show`, the old `status` owner block, `evidence list` and the old not-found hint, none of which the new binary has; its not-found error names `atelier list` (run in a scratch clone).
