---
created_at: "2026-06-16T17:41:38.832275674+00:00"
id: "atelier-993l"
evidence_type: "test"
captured_at: "2026-06-16T17:41:37.165962440+00:00"
command: "bash -lc 'target/debug/atelier export 2>&1 | tee /tmp/atelier-vuqb-export-refusal.txt; test ${PIPESTATUS[0]} -ne 0; grep -q \"Refusing to write canonical tracker records from the local projection\" /tmp/atelier-vuqb-export-refusal.txt; grep -q \"atelier doctor --fix\" /tmp/atelier-vuqb-export-refusal.txt'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-vuqb"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 41155
    summary: "Error: Refusing to write canonical tracker records from the local projection:\nwould remove tracked canonical record from local projection: .atelier/workflow.yaml\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-06rb.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0c13.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0i5d.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0lx7.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0ook.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-14nz.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-191m.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1bq9.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1k6a.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1ndg.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1oxj.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-201d.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2e83.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2f5d.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2o5r.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2q31.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2qrm.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2yxv.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-35xf.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-37ez.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-3aho.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-3b7u.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-3ud4.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-41xk.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-45b1.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4g4o.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4hec.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4itj.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4m3n.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4qwo.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4sst.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4yvb.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-51r9.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-53nl.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-545s.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-574b.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-5rat.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-5zt6.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-5zxg.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-630k.md\nwould rewrite tracked canonical record from local projection: .atelier/evidence/atelier-63dm.m"
    truncated: true
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vuqb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "export write path refuses to rewrite tracked canonical records"
updated_at: "2026-06-16T17:41:42.298165374+00:00"
---

export write path refuses to rewrite tracked canonical records

Command: bash -lc 'target/debug/atelier export 2>&1 | tee /tmp/atelier-vuqb-export-refusal.txt; test ${PIPESTATUS[0]} -ne 0; grep -q "Refusing to write canonical tracker records from the local projection" /tmp/atelier-vuqb-export-refusal.txt; grep -q "atelier doctor --fix" /tmp/atelier-vuqb-export-refusal.txt'
Exit status: 0

Stdout summary (truncated):
Error: Refusing to write canonical tracker records from the local projection:
would remove tracked canonical record from local projection: .atelier/workflow.yaml
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-06rb.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0c13.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0i5d.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0lx7.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-0ook.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-14nz.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-191m.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1bq9.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1k6a.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1ndg.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-1oxj.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-201d.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2e83.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2f5d.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2o5r.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2q31.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2qrm.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-2yxv.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-35xf.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-37ez.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-3aho.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-3b7u.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-3ud4.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-41xk.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-45b1.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4g4o.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4hec.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4itj.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4m3n.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4qwo.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4sst.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-4yvb.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-51r9.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-53nl.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-545s.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-574b.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-5rat.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-5zt6.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-5zxg.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-630k.md
would rewrite tracked canonical record from local projection: .atelier/evidence/atelier-63dm.m

Stderr summary:
(none)

