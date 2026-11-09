---
created_at: "2026-11-09T18:40:21.895659322+00:00"
id: "atelier-n7yb"
evidence_type: "validation"
commit: "fbd16d2958cd358a0ef7eebfe3755050b3ee835b"
result: "pass"
captured_at: "2026-11-09T18:40:21.895598558+00:00"
target:
  kind: "issue"
  id: "atelier-yn69"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yn69"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "pnpm dev --port 5199 with ATELIER_SERVE=http://127.0.0.1:7398: /api/repos proxied to that server (5 repositories), and a deep link served the dev index.html; systemd-analyze --user verify accepts crates/atelier-site/systemd/atelier-serve.service."
updated_at: "2026-11-09T18:40:22.030756266+00:00"
---

pnpm dev --port 5199 with ATELIER_SERVE=http://127.0.0.1:7398: /api/repos proxied to that server (5 repositories), and a deep link served the dev index.html; systemd-analyze --user verify accepts crates/atelier-site/systemd/atelier-serve.service.
