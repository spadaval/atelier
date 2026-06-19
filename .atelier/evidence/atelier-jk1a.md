---
created_at: "2026-06-19T04:27:13.734548793+00:00"
id: "atelier-jk1a"
evidence_type: "validation"
captured_at: "2026-06-19T04:27:13.734547334+00:00"
target:
  kind: "issue"
  id: "atelier-7v02"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7v02"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Config epic completed: child issues atelier-9h5w, atelier-13yy, and atelier-tv53 are closed with evidence atelier-kq20, atelier-0wc1, and atelier-n71u. Project config now requires mutually exclusive review modes, moves Forgejo under [review.providers.forgejo], rejects legacy [forgejo], and documents starter room/provider choices. Proof: cargo test -p atelier-app project_config --lib passed; cargo test -p atelier-cli forgejo --lib passed; atelier lint atelier-7v02 passed; atelier lint passed; git diff --check passed."
updated_at: "2026-06-19T04:27:16.488251994+00:00"
---

Config epic completed: child issues atelier-9h5w, atelier-13yy, and atelier-tv53 are closed with evidence atelier-kq20, atelier-0wc1, and atelier-n71u. Project config now requires mutually exclusive review modes, moves Forgejo under [review.providers.forgejo], rejects legacy [forgejo], and documents starter room/provider choices. Proof: cargo test -p atelier-app project_config --lib passed; cargo test -p atelier-cli forgejo --lib passed; atelier lint atelier-7v02 passed; atelier lint passed; git diff --check passed.
