---
created_at: "2026-06-15T18:56:02.111246375+00:00"
id: "atelier-yq6r"
evidence_type: "validation"
captured_at: "2026-06-15T18:55:52.169890806+00:00"
command: "bash -lc 'rg \"atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance|was removed|fallback shim|compatibility alias\" crates fuzz tests src docs -n; test $? -eq 1; set +e; target/debug/atelier finish >/tmp/vu2b-finish.out 2>/tmp/vu2b-finish.err; status=$?; set -e; test $status -ne 0; rg \"unrecognized subcommand .finish.\" /tmp/vu2b-finish.err; ! rg \"was removed|atelier issue close\" /tmp/vu2b-finish.err; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets; cargo fmt -- --check; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-vu2b"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 2437
    summary: "docs/adr/0009-virtual-workspace-root-and-cli-binary.md:14:Keeping a root package while adding crates would preserve old `atelier::...`\ndocs/adr/0009-virtual-workspace-root-and-cli-binary.md:42:preserve root package compatibility aliases or old `atelier::...` re-export\ndocs/adr/0005-repo-owned-issue-workflow-state.md:63:   The workflow rollout does not preserve compatibility aliases, fallback\ncrates/atelier-cli/tests/cli_integration/records_evidence.rs:2263:        !stderr.contains(\"was removed\") && !stderr.contains(\"atelier init\"),\ncrates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};\ndocs/architecture/source-layout.md:36:  They must not become compatibility aliases, old-path re-exports, or staged\ncrates/atelier-cli/tests/cli_integration/setup_guidance.rs:107:    assert!(!stderr.contains(\"was removed\"), \"{stderr}\");\ncrates/atelier-cli/tests/cli_integration/setup_guidance.rs:753:    assert!(!stderr.contains(\"was removed\"), \"{stderr}\");\ncrates/atelier-cli/tests/cli_integration/setup_guidance.rs:774:    assert!(!stderr.contains(\"was removed\"), \"{stderr}\");\ncrates/atelier-cli/tests/cli_integration/setup_guidance.rs:2344:            !stderr.contains(\"was removed\"),\ncrates/atelier-cli/tests/cli_integration/setup_guidance.rs:2365:    assert!(!stderr.contains(\"was removed\"), \"{stderr}\");\ncrates/atelier-cli/tests/cli_integration/setup_guidance.rs:2443:            !stderr.contains(\"was removed\"),\ndocs/product/workflow-configuration.md:190:version 1 does not permit compatibility aliases, partial parsing, or silent\ndocs/architecture/quality/codex-mission-log-insights-2026-06-14.md:112:  `workflow check was removed; use issue transition <id> --options or mission\nerror: unrecognized subcommand 'finish'\nLint passed.\nCanonical export is current\nState: /root/atelier/.atelier\nDatabase: /root/atelier/.atelier/runtime/state.db\nState: /root/atelier/.atelier\nInstall health:\n  config: ok\n  ignored_runtime_paths: ok\nProjection rebuild:\n  state_dir: ok\n  rebuild_ready: ok\n  projection_fresh: ok\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\nCache health:\n  cache_dir: missing (optional)\n  projection_metadata: ok\nRuntime state:\n  directory: ok\n  database: ok\n  local_tables: ok\n  diagnostics: enabled\nCompatibility:\n  tables: \nLegacy health:\nconfig: ok\ndatabase: ok\nignore_rules: ok\nprojection_fresh: ok\nrebuild_ready: ok\nruntime_state: ok\nruntime_tables: ok\n"
    truncated: false
  stderr:
    bytes: 452
    summary: "rg: tests: No such file or directory (os error 2)\nrg: src: No such file or directory (os error 2)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Checking atelier-records v0.2.0 (/root/atelier/crates/atelier-records)\n    Checking atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)\n    Checking atelier-app v0.2.0 (/root/atelier/crates/atelier-app)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.81s\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vu2b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Final root compatibility/no-shim validation passes after app CLI and root closeout commits"
updated_at: "2026-06-15T18:56:05.794163142+00:00"
---

Final root compatibility/no-shim validation passes after app CLI and root closeout commits

Command: bash -lc 'rg "atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance|was removed|fallback shim|compatibility alias" crates fuzz tests src docs -n; test $? -eq 1; set +e; target/debug/atelier finish >/tmp/vu2b-finish.out 2>/tmp/vu2b-finish.err; status=$?; set -e; test $status -ne 0; rg "unrecognized subcommand .finish." /tmp/vu2b-finish.err; ! rg "was removed|atelier issue close" /tmp/vu2b-finish.err; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets; cargo fmt -- --check; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check'
Exit status: 0

Stdout summary:
docs/adr/0009-virtual-workspace-root-and-cli-binary.md:14:Keeping a root package while adding crates would preserve old `atelier::...`
docs/adr/0009-virtual-workspace-root-and-cli-binary.md:42:preserve root package compatibility aliases or old `atelier::...` re-export
docs/adr/0005-repo-owned-issue-workflow-state.md:63:   The workflow rollout does not preserve compatibility aliases, fallback
crates/atelier-cli/tests/cli_integration/records_evidence.rs:2263:        !stderr.contains("was removed") && !stderr.contains("atelier init"),
crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};
docs/architecture/source-layout.md:36:  They must not become compatibility aliases, old-path re-exports, or staged
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:107:    assert!(!stderr.contains("was removed"), "{stderr}");
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:753:    assert!(!stderr.contains("was removed"), "{stderr}");
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:774:    assert!(!stderr.contains("was removed"), "{stderr}");
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:2344:            !stderr.contains("was removed"),
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:2365:    assert!(!stderr.contains("was removed"), "{stderr}");
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:2443:            !stderr.contains("was removed"),
docs/product/workflow-configuration.md:190:version 1 does not permit compatibility aliases, partial parsing, or silent
docs/architecture/quality/codex-mission-log-insights-2026-06-14.md:112:  `workflow check was removed; use issue transition <id> --options or mission
error: unrecognized subcommand 'finish'
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
Database: /root/atelier/.atelier/runtime/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: 
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok

Stderr summary:
rg: tests: No such file or directory (os error 2)
rg: src: No such file or directory (os error 2)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Checking atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
    Checking atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
    Checking atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.81s

