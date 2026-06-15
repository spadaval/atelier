---
created_at: "2026-06-15T18:51:55.468730434+00:00"
id: "atelier-jzsh"
evidence_type: "validation"
captured_at: "2026-06-15T18:51:24.194796039+00:00"
command: "bash -lc 'target/debug/atelier --help >/tmp/zwna-help.txt; target/debug/atelier status >/tmp/zwna-status.txt; target/debug/atelier issue list --ready >/tmp/zwna-ready.txt; target/debug/atelier mission status atelier-v5nb >/tmp/zwna-mission.txt; target/debug/atelier doctor >/tmp/zwna-doctor.txt; target/debug/atelier export --check >/tmp/zwna-export.txt; wc -l /tmp/zwna-help.txt /tmp/zwna-status.txt /tmp/zwna-ready.txt /tmp/zwna-mission.txt /tmp/zwna-doctor.txt /tmp/zwna-export.txt'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-zwna"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 165
    summary: "  69 /tmp/zwna-help.txt\n  29 /tmp/zwna-status.txt\n  10 /tmp/zwna-ready.txt\n  93 /tmp/zwna-mission.txt\n  28 /tmp/zwna-doctor.txt\n   2 /tmp/zwna-export.txt\n 231 total\n"
    truncated: false
  stderr:
    bytes: 122
    summary: "2026-06-15T18:51:30.629850Z  WARN Projection index was stale; rebuilt local SQLite projection from /root/atelier/.atelier\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zwna"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Representative CLI smoke commands for help status issue mission doctor and export remain stable"
updated_at: "2026-06-15T18:51:59.011073430+00:00"
---

Representative CLI smoke commands for help status issue mission doctor and export remain stable

Command: bash -lc 'target/debug/atelier --help >/tmp/zwna-help.txt; target/debug/atelier status >/tmp/zwna-status.txt; target/debug/atelier issue list --ready >/tmp/zwna-ready.txt; target/debug/atelier mission status atelier-v5nb >/tmp/zwna-mission.txt; target/debug/atelier doctor >/tmp/zwna-doctor.txt; target/debug/atelier export --check >/tmp/zwna-export.txt; wc -l /tmp/zwna-help.txt /tmp/zwna-status.txt /tmp/zwna-ready.txt /tmp/zwna-mission.txt /tmp/zwna-doctor.txt /tmp/zwna-export.txt'
Exit status: 0

Stdout summary:
  69 /tmp/zwna-help.txt
  29 /tmp/zwna-status.txt
  10 /tmp/zwna-ready.txt
  93 /tmp/zwna-mission.txt
  28 /tmp/zwna-doctor.txt
   2 /tmp/zwna-export.txt
 231 total

Stderr summary:
2026-06-15T18:51:30.629850Z  WARN Projection index was stale; rebuilt local SQLite projection from /root/atelier/.atelier

