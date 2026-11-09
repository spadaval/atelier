---
created_at: "2026-11-09T18:40:31.687031476+00:00"
id: "atelier-fmq4"
evidence_type: "validation"
commit: "089ddbdf5602fecd693f8291113687ddffb2d84b"
result: "pass"
captured_at: "2026-11-09T18:40:31.686988336+00:00"
target:
  kind: "issue"
  id: "atelier-ygul"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ygul"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the atelier-ezxy fix: closing atelier-fl12 on master and master at atelier-v3nm's done commit 26186c06 pass with this branch's build. Reviewer repros: unlink refuses, prune passes, parent-approval refuses. cargo nextest -p atelier-app -p atelier-cli: 987 passed, 997 with test-hooks. scripts/smoke-repos.sh: no regression across 5 repositories."
updated_at: "2026-11-09T18:40:31.819681655+00:00"
---

After the atelier-ezxy fix: closing atelier-fl12 on master and master at atelier-v3nm's done commit 26186c06 pass with this branch's build. Reviewer repros: unlink refuses, prune passes, parent-approval refuses. cargo nextest -p atelier-app -p atelier-cli: 987 passed, 997 with test-hooks. scripts/smoke-repos.sh: no regression across 5 repositories.
