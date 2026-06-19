---
created_at: "2026-06-18T00:12:13.594173684+00:00"
id: "atelier-wtc1"
evidence_type: "test"
captured_at: "2026-06-18T00:12:13.594163950+00:00"
target:
  kind: "issue"
  id: "atelier-vvs3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vvs3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli sessions; cargo check -p atelier-cli; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier lint atelier-vvs3; target/debug/atelier status shows Active sessions with atelier-f4bs -> issue/atelier-vvs3; target/debug/atelier man worker shows atelier session list --active and no atelier session start; target/debug/atelier history --issue atelier-vvs3 shows session_started; git diff --check"
updated_at: "2026-06-18T00:12:17.368064265+00:00"
---

cargo test -p atelier-cli sessions; cargo check -p atelier-cli; cargo fmt -- --check; cargo build -p atelier-cli; target/debug/atelier lint atelier-vvs3; target/debug/atelier status shows Active sessions with atelier-f4bs -> issue/atelier-vvs3; target/debug/atelier man worker shows atelier session list --active and no atelier session start; target/debug/atelier history --issue atelier-vvs3 shows session_started; git diff --check
