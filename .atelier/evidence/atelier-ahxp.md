---
created_at: "2026-06-16T17:57:59.228346614+00:00"
id: "atelier-ahxp"
evidence_type: "validation"
captured_at: "2026-06-16T17:57:58.245064712+00:00"
command: "bash -lc 'target/debug/atelier issue show atelier-c4b8 | sed -n \"/Subissues/,/Recent Activity/p\" && target/debug/atelier history --epic atelier-c4b8 --event-kind evidence_attached --limit 40'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-c4b8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c4b8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "all command-surface epic children closed with validation evidence"
updated_at: "2026-06-16T17:58:02.904351864+00:00"
---

## Summary

all command-surface epic children closed with validation evidence

## Command

```console
bash -lc 'target/debug/atelier issue show atelier-c4b8 | sed -n "/Subissues/,/Recent Activity/p" && target/debug/atelier history --epic atelier-c4b8 --event-kind evidence_attached --limit 40'
```

Exit status: 0

## Stdout

Bytes: 8210
Truncated: yes

```text
Subissues
---------
6 total | status: done=6 | priority: high=5, medium=1
  atelier-2sut [done] high - Define normal, admin, and debug command boundaries
  atelier-a7gd [done] high - Route projection repair through doctor
  atelier-jezn [done] high - Remove export checks from normal guidance
  atelier-m1r7 [done] high - Validate cleaned command surface and recovery paths
  atelier-vuqb [done] high - Demote or remove export as a public command
  atelier-1xmi [done] medium - Audit remaining low-level command surfaces

Recent Activity
History
=======
Scope:          epic atelier-c4b8 - Epic: Clean up low-level command surfaces (including descendants)
Source:         canonical .atelier issue activity, records, evidence, and record links; local runtime diagnostics excluded
Ordering:       newest first, timestamp then record/path
Limit:          40
Filters:        event_kind=evidence_attached
Showing:        40 of 44 matching events

Events
------
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-m1r7 | Validate cleaned command surface and recovery paths | Attached evidence atelier-y4fs to issue/atelier-m1r7 (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-m1r7 | Validate cleaned command surface and recovery paths | Attached evidence atelier-wbvg to issue/atelier-m1r7 (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-a7gd | Route projection repair through doctor | Attached evidence atelier-tr2b to issue/atelier-a7gd (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-jezn | Remove export checks from normal guidance | Attached evidence atelier-tixz to issue/atelier-jezn (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-m1r7 | Validate cleaned command surface and recovery paths | Attached evidence atelier-rsr8 to issue/atelier-m1r7 (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-2sut | Define normal, admin, and debug command boundaries | Attached evidence atelier-rp25 to issue/atelier-2sut (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-m1r7 | Validate cleaned command surface and recovery paths | Attached evidence atelier-pipz to issue/atelier-m1r7 (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-vuqb | Demote or remove export as a public command | Attached evidence atelier-oe8b to issue/atelier-vuqb (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-m1r7 | Validate cleaned command surface and recovery paths | Attached evidence atelier-o3b6 to issue/atelier-m1r7 (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-1xmi | Audit remaining low-level command surfaces | Attached evidence atelier-n4gp to issue/atelier-1xmi (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-1xmi | Audit remaining low-level command surfaces | Attached evidence atelier-mwnm to issue/atelier-1xmi (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-a7gd | Route projection repair through doctor | Attached evidence atelier-lvf3 to issue/atelier-a7gd (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-jezn | Remove export checks from normal guidance | Attached evidence atelier-gvsl to issue/atelier-jezn (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-a7gd | Route projection repair through doctor | Attached evidence atelier-cpen to issue/atelier-a7gd (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-vuqb | Demote or remove export as a public command | Attached evidence atelier-bsjp to issue/atelier-vuqb (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) | issue/atelier-vuqb | Demote or remove export as a public command | Attached evidence atelier-993l to issue/atelier-vuqb (validates)
  2026-06-16 13:57 -04:00 | evidence_attached | (system) |
```

## Stderr

Bytes: 0
Truncated: no

```text
```
