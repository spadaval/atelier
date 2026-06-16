---
created_at: "2026-06-16T17:41:38.832275674+00:00"
id: "atelier-993l"
evidence_type: "test"
captured_at: "2026-06-16T17:41:37.165962440+00:00"
command: "bash -lc 'target/debug/atelier export 2>&1 | tee /tmp/atelier-vuqb-export-refusal.txt; test ${PIPESTATUS[0]} -ne 0; grep -q \"Refusing to write canonical tracker records from the local projection\" /tmp/atelier-vuqb-export-refusal.txt; grep -q \"atelier doctor --fix\" /tmp/atelier-vuqb-export-refusal.txt'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vuqb"
  role: "validates"
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
status: "recorded"
title: "export write path refuses to rewrite tracked canonical records"
updated_at: "2026-06-16T17:41:42.298165374+00:00"
---

## Summary

export write path refuses to rewrite tracked canonical records

## Command

```console
bash -lc 'target/debug/atelier export 2>&1 | tee /tmp/atelier-vuqb-export-refusal.txt; test ${PIPESTATUS[0]} -ne 0; grep -q "Refusing to write canonical tracker records from the local projection" /tmp/atelier-vuqb-export-refusal.txt; grep -q "atelier doctor --fix" /tmp/atelier-vuqb-export-refusal.txt'
```

Exit status: 0

## Stdout

Bytes: 41155
Truncated: yes

```text
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
```

## Stderr

Bytes: 0
Truncated: no

```text
```
