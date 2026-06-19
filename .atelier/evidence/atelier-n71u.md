---
created_at: "2026-06-19T04:26:02.608258864+00:00"
id: "atelier-n71u"
evidence_type: "validation"
captured_at: "2026-06-19T04:26:02.608257617+00:00"
target:
  kind: "issue"
  id: "atelier-tv53"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tv53"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Review mode config guidance validated. The starter config template documents exactly one mode: native room mode or provider mode with provider = forgejo; parser tests cover room mode, provider mode, legacy [forgejo] rejection, mixed room/provider rejection, and missing provider guidance. Proof: cargo test -p atelier-app project_config --lib passed; cargo test -p atelier-cli forgejo --lib passed; atelier lint atelier-tv53 passed; atelier lint passed; git diff --check passed."
updated_at: "2026-06-19T04:26:05.420067793+00:00"
---

Review mode config guidance validated. The starter config template documents exactly one mode: native room mode or provider mode with provider = forgejo; parser tests cover room mode, provider mode, legacy [forgejo] rejection, mixed room/provider rejection, and missing provider guidance. Proof: cargo test -p atelier-app project_config --lib passed; cargo test -p atelier-cli forgejo --lib passed; atelier lint atelier-tv53 passed; atelier lint passed; git diff --check passed.
