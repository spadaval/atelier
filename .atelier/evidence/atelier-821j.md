---
created_at: "2026-06-13T23:01:30.953870114+00:00"
id: "atelier-821j"
evidence_type: "test"
captured_at: "2026-06-13T23:01:30.606014532+00:00"
command: "bash -lc 'if rg -n \"TODO|FIXME\" src tests build.rs; then exit 1; else echo \"no TODO/FIXME markers in src tests build.rs\"; fi'"
exit_status: "0"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e723"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'if rg -n \"TODO|FIXME\" src tests build.rs; then exit 1; else echo \"no TODO/FIXME markers in src tests build.rs\"; fi'"
updated_at: "2026-06-13T23:01:33.145531145+00:00"
---

bash -lc 'if rg -n "TODO|FIXME" src tests build.rs; then exit 1; else echo "no TODO/FIXME markers in src tests build.rs"; fi'

Command: bash -lc 'if rg -n "TODO|FIXME" src tests build.rs; then exit 1; else echo "no TODO/FIXME markers in src tests build.rs"; fi'
Exit status: 0

Stdout summary:
no TODO/FIXME markers in src tests build.rs

Stderr summary:
(none)

