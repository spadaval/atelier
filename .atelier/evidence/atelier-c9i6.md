---
created_at: "2026-06-18T18:15:24.180683008+00:00"
id: "atelier-c9i6"
evidence_type: "validation"
captured_at: "2026-06-18T18:15:24.180681247+00:00"
target:
  kind: "issue"
  id: "atelier-jdvz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jdvz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PR session-as-issue-events epic validated by child evidence: atelier-73p1 proves linked-PR and branch-owner target inference plus one-active enforcement; atelier-4j0g proves PR open/comment/review issue-event attribution without standalone session truth; atelier-2u2k proves pr merge behavior records attribution and preserves Atelier workflow status. Integrated verification passed: cargo nextest run -p atelier-records issue_attempts_are_derived_by_issue_role_and_serial; cargo nextest run -p atelier-cli --test cli_integration sessions; cargo nextest run -p atelier-cli commands::pr::tests; atelier lint; git diff --check."
updated_at: "2026-06-18T18:15:26.810318780+00:00"
---

PR session-as-issue-events epic validated by child evidence: atelier-73p1 proves linked-PR and branch-owner target inference plus one-active enforcement; atelier-4j0g proves PR open/comment/review issue-event attribution without standalone session truth; atelier-2u2k proves pr merge behavior records attribution and preserves Atelier workflow status. Integrated verification passed: cargo nextest run -p atelier-records issue_attempts_are_derived_by_issue_role_and_serial; cargo nextest run -p atelier-cli --test cli_integration sessions; cargo nextest run -p atelier-cli commands::pr::tests; atelier lint; git diff --check.
