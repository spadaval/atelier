---
created_at: "2026-11-10T05:20:54.282962588+00:00"
id: "atelier-93tc"
evidence_type: "test"
commit: "2dfeef82d317b0bb97a63633bc74feb58157d392"
result: "pass"
captured_at: "2026-11-10T05:20:54.282885865+00:00"
target:
  kind: "issue"
  id: "atelier-scx2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-scx2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "verify fails when it should: a tampered manifest copy (one size changed, one entry dropped, one bogus entry) gave changed/unlisted/missing lines and exit 1; a scratch tree with modes 644/755/775 gave three mode lines and exit 1. Backup method tested first on a scratch WAL database with a concurrent writer: copy consistent, integrity_check ok."
updated_at: "2026-11-10T05:20:54.423283102+00:00"
---

verify fails when it should: a tampered manifest copy (one size changed, one entry dropped, one bogus entry) gave changed/unlisted/missing lines and exit 1; a scratch tree with modes 644/755/775 gave three mode lines and exit 1. Backup method tested first on a scratch WAL database with a concurrent writer: copy consistent, integrity_check ok.
