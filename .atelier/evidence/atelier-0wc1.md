---
created_at: "2026-06-19T04:25:47.164313828+00:00"
id: "atelier-0wc1"
evidence_type: "validation"
captured_at: "2026-06-19T04:25:47.164306358+00:00"
target:
  kind: "issue"
  id: "atelier-13yy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-13yy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Forgejo settings moved under review provider config. .atelier/config.toml and init template now use [review] mode = provider, provider = forgejo, [review.providers.forgejo], and [review.providers.forgejo.role_authors]; atelier forgejo roles provision writes the new role-authors table. Proof: cargo test -p atelier-app project_config --lib passed; cargo test -p atelier-cli forgejo --lib passed; atelier lint atelier-13yy passed; git diff --check passed."
updated_at: "2026-06-19T04:25:49.831872855+00:00"
---

Forgejo settings moved under review provider config. .atelier/config.toml and init template now use [review] mode = provider, provider = forgejo, [review.providers.forgejo], and [review.providers.forgejo.role_authors]; atelier forgejo roles provision writes the new role-authors table. Proof: cargo test -p atelier-app project_config --lib passed; cargo test -p atelier-cli forgejo --lib passed; atelier lint atelier-13yy passed; git diff --check passed.
