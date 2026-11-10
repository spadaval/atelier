---
created_at: "2026-11-10T20:50:06.211383188+00:00"
id: "atelier-roux"
evidence_type: "test"
commit: "cca92c2ab67b1408ac0b334c3fdeb9fbda08f7d1"
result: "pass"
captured_at: "2026-11-10T20:50:06.211267743+00:00"
target:
  kind: "issue"
  id: "atelier-jq24"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jq24"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 301d9a37: cargo fmt --check, cargo clippy --all-targets and cargo nextest run in next/ pass (158 tests); the CI job's next/ steps pass on a scratch clone (rm -rf ../crates, fmt, build --locked, nextest --locked); each new test seen failing under a mutation of the code it covers; git diff --stat master...HEAD -- crates/ is empty"
updated_at: "2026-11-10T20:50:06.430046907+00:00"
---

At 301d9a37: cargo fmt --check, cargo clippy --all-targets and cargo nextest run in next/ pass (158 tests); the CI job's next/ steps pass on a scratch clone (rm -rf ../crates, fmt, build --locked, nextest --locked); each new test seen failing under a mutation of the code it covers; git diff --stat master...HEAD -- crates/ is empty
