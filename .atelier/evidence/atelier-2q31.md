---
created_at: "2026-06-14T00:17:07.191343174+00:00"
id: "atelier-2q31"
evidence_type: "validation"
captured_at: "2026-06-14T00:17:07.191237510+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-d7lw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Main command routing thinned by extracting storage access and projection freshness policy into src/command_storage.rs. src/main.rs now keeps Clap routing and command dispatch while command_storage owns access modes, runtime DB opening, projection freshness rebuild/degraded-orientation behavior, and related tests. Architecture doc records the boundary. Proof: cargo test command_storage --lib --bins passed command_storage::tests::access_modes_declare_projection_freshness_policy; cargo nextest run test_top_level_help_only_shows_core_commands test_root_status_summarizes_checkout_orientation test_init_creates_atelier_directory passed 3/3; cargo fmt -- --check passed; cargo build --quiet passed; target/debug/atelier lint atelier-d7lw passed; target/debug/atelier export --check passed; git diff --check passed."
updated_at: "2026-06-14T00:17:09.610463547+00:00"
---

Main command routing thinned by extracting storage access and projection freshness policy into src/command_storage.rs. src/main.rs now keeps Clap routing and command dispatch while command_storage owns access modes, runtime DB opening, projection freshness rebuild/degraded-orientation behavior, and related tests. Architecture doc records the boundary. Proof: cargo test command_storage --lib --bins passed command_storage::tests::access_modes_declare_projection_freshness_policy; cargo nextest run test_top_level_help_only_shows_core_commands test_root_status_summarizes_checkout_orientation test_init_creates_atelier_directory passed 3/3; cargo fmt -- --check passed; cargo build --quiet passed; target/debug/atelier lint atelier-d7lw passed; target/debug/atelier export --check passed; git diff --check passed.
