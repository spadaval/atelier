---
created_at: "2026-06-21T02:34:22.047465087+00:00"
id: "atelier-4998"
evidence_type: "validation"
captured_at: "2026-06-21T02:34:22.047463635+00:00"
target:
  kind: "issue"
  id: "atelier-x8g1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x8g1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "AGENTS.md and docs/architecture/quality/standards.md now state cargo nextest run is the default Rust test runner, with cargo test limited to explicit exception cases. Verified with rg literal search, git diff --check, and atelier lint atelier-x8g1."
updated_at: "2026-06-21T02:34:25.101211999+00:00"
---

AGENTS.md and docs/architecture/quality/standards.md now state cargo nextest run is the default Rust test runner, with cargo test limited to explicit exception cases. Verified with rg literal search, git diff --check, and atelier lint atelier-x8g1.
