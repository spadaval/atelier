---
created_at: "2026-06-19T04:25:35.930940669+00:00"
id: "atelier-kq20"
evidence_type: "validation"
captured_at: "2026-06-19T04:25:35.930939208+00:00"
target:
  kind: "issue"
  id: "atelier-9h5w"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9h5w"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Review mode config implemented and validated. Project config now requires [review] mode = room or provider; provider mode requires provider = forgejo plus [review.providers.forgejo], room mode rejects provider settings, missing modes are rejected, and legacy top-level [forgejo] is rejected. Proof: cargo test -p atelier-app project_config --lib passed (5 tests); atelier lint atelier-9h5w passed; atelier lint passed; git diff --check passed."
updated_at: "2026-06-19T04:25:38.750706436+00:00"
---

Review mode config implemented and validated. Project config now requires [review] mode = room or provider; provider mode requires provider = forgejo plus [review.providers.forgejo], room mode rejects provider settings, missing modes are rejected, and legacy top-level [forgejo] is rejected. Proof: cargo test -p atelier-app project_config --lib passed (5 tests); atelier lint atelier-9h5w passed; atelier lint passed; git diff --check passed.
