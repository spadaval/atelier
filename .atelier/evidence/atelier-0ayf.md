---
created_at: "2026-11-09T18:29:05.150881956+00:00"
id: "atelier-0ayf"
evidence_type: "validation"
commit: "d131401b117aa7a378d6f2922cc6605bbb9c3859"
result: "pass"
captured_at: "2026-11-09T18:29:05.150837413+00:00"
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
title: "After the atelier-5pk7 fix: scratch clone of this repository, closing atelier-fl12 on master and master at atelier-v3nm's done commit 26186c06 both pass with this branch's build (the base build refuses both). The reviewer's unlink repro refuses and prune repro passes. cargo nextest -p atelier-app -p atelier-cli: 986 passed, 996 with test-hooks. scripts/smoke-repos.sh: no regression across 5 repositories."
updated_at: "2026-11-09T18:29:05.283761580+00:00"
---

After the atelier-5pk7 fix: scratch clone of this repository, closing atelier-fl12 on master and master at atelier-v3nm's done commit 26186c06 both pass with this branch's build (the base build refuses both). The reviewer's unlink repro refuses and prune repro passes. cargo nextest -p atelier-app -p atelier-cli: 986 passed, 996 with test-hooks. scripts/smoke-repos.sh: no regression across 5 repositories.
