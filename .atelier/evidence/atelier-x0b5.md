---
created_at: "2026-06-30T19:21:32.516531255+00:00"
id: "atelier-x0b5"
evidence_type: "test"
captured_at: "2026-06-30T19:21:32.516522728+00:00"
target:
  kind: "issue"
  id: "atelier-j8ot"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-j8ot"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo fmt -- --check; cargo test -p atelier-workflow --lib; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli --test cli_integration workflow -- --nocapture; target/debug/atelier check; target/debug/atelier check atelier-j8ot; git diff --check passed; rg stale workflow/action vocabulary only reports intentional negative tests and ADR rejected-vocabulary text"
updated_at: "2026-06-30T19:21:38.274978607+00:00"
---

cargo fmt -- --check; cargo test -p atelier-workflow --lib; cargo test -p atelier-cli workflow --lib; cargo test -p atelier-cli --test cli_integration workflow -- --nocapture; target/debug/atelier check; target/debug/atelier check atelier-j8ot; git diff --check passed; rg stale workflow/action vocabulary only reports intentional negative tests and ADR rejected-vocabulary text
