---
created_at: "2026-06-20T20:22:19.229305657+00:00"
id: "atelier-nhuk"
evidence_type: "validation"
captured_at: "2026-06-20T20:22:19.229304117+00:00"
target:
  kind: "issue"
  id: "atelier-bruu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bruu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Moved normal issue command implementation from commands::agent_factory to commands::issue and deleted the obsolete module file. Validated rg shows no commands::agent_factory dispatch under crates/atelier-cli/src/main.rs or crates/atelier-cli/src/commands, cargo fmt -- --check, cargo check -p atelier-cli, cargo build -p atelier-cli, cargo test -p atelier-cli commands::issue:: -- --nocapture, cargo test -p atelier-cli --test cli_integration issues:: -- --nocapture, focused legacy dependency/search tests, target/debug/atelier issue --help/list/show smoke, target/debug/atelier lint atelier-bruu, atelier lint atelier-bruu, git diff --check. Independent Agent Factory review by Tesla reported no blocking findings."
updated_at: "2026-06-20T20:22:22.161755650+00:00"
---

Moved normal issue command implementation from commands::agent_factory to commands::issue and deleted the obsolete module file. Validated rg shows no commands::agent_factory dispatch under crates/atelier-cli/src/main.rs or crates/atelier-cli/src/commands, cargo fmt -- --check, cargo check -p atelier-cli, cargo build -p atelier-cli, cargo test -p atelier-cli commands::issue:: -- --nocapture, cargo test -p atelier-cli --test cli_integration issues:: -- --nocapture, focused legacy dependency/search tests, target/debug/atelier issue --help/list/show smoke, target/debug/atelier lint atelier-bruu, atelier lint atelier-bruu, git diff --check. Independent Agent Factory review by Tesla reported no blocking findings.
