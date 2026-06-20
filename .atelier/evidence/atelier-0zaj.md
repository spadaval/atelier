---
created_at: "2026-06-19T04:38:35.825402538+00:00"
id: "atelier-0zaj"
evidence_type: "validation"
captured_at: "2026-06-19T04:38:35.825395380+00:00"
target:
  kind: "issue"
  id: "atelier-xuxl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xuxl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Legacy pull_request shape is rejected after migration. Issue parser rejects top-level pull_request, workflow validation rejects pull_request fields with workflow_issue_field_legacy, provider PR persistence now writes structured review.kind pull_request objects, and rebuild tests cover invalid legacy/schema paths. Proof: cargo test -p atelier-records --lib passed; cargo test -p atelier-workflow --lib passed; cargo test -p atelier-app pr::tests --lib passed; cargo test -p atelier-app rebuild --lib passed; atelier lint atelier-xuxl passed; git diff --check passed."
updated_at: "2026-06-19T04:38:38.554201291+00:00"
---

Legacy pull_request shape is rejected after migration. Issue parser rejects top-level pull_request, workflow validation rejects pull_request fields with workflow_issue_field_legacy, provider PR persistence now writes structured review.kind pull_request objects, and rebuild tests cover invalid legacy/schema paths. Proof: cargo test -p atelier-records --lib passed; cargo test -p atelier-workflow --lib passed; cargo test -p atelier-app pr::tests --lib passed; cargo test -p atelier-app rebuild --lib passed; atelier lint atelier-xuxl passed; git diff --check passed.
