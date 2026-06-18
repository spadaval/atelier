---
created_at: "2026-06-18T23:42:23.370599991+00:00"
id: "atelier-qsnp"
evidence_type: "validation"
captured_at: "2026-06-18T23:42:23.370598643+00:00"
target:
  kind: "issue"
  id: "atelier-0wyy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0wyy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PR orchestration moved behind atelier-app use-case boundary. Proof: cargo fmt -- --check passed; cargo test -p atelier-app pr::tests --no-fail-fast passed with 19 tests; cargo test -p atelier-cli --lib commands::pr::tests --no-fail-fast passed with 2 tests; git diff --check passed."
updated_at: "2026-06-18T23:42:26.023860483+00:00"
---

PR orchestration moved behind atelier-app use-case boundary. Proof: cargo fmt -- --check passed; cargo test -p atelier-app pr::tests --no-fail-fast passed with 19 tests; cargo test -p atelier-cli --lib commands::pr::tests --no-fail-fast passed with 2 tests; git diff --check passed.
