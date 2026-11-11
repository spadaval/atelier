---
schema: atelier.item
schema_version: 3
id: atelier-pvo0
title: Name the command for the remaining silent or bare truncations
status: dropped
mission: atelier-4pzj
labels:
- cli-output
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T09:14:09.736875227Z
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

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-378y
- discovered_from review: atelier-hrfh
- discovered_from review: atelier-wslw

## Report

Dropped: Every command it names (`issue show`, `evidence list`, the old `status`) was removed in 0.5.
