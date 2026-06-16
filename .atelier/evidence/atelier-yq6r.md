---
created_at: "2026-06-15T18:56:02.111246375+00:00"
id: "atelier-yq6r"
evidence_type: "validation"
captured_at: "2026-06-15T18:55:52.169890806+00:00"
command: "bash -lc 'rg \"atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance|was removed|fallback shim|compatibility alias\" crates fuzz tests src docs -n; test $? -eq 1; set +e; target/debug/atelier finish >/tmp/vu2b-finish.out 2>/tmp/vu2b-finish.err; status=$?; set -e; test $status -ne 0; rg \"unrecognized subcommand .finish.\" /tmp/vu2b-finish.err; ! rg \"was removed|atelier issue close\" /tmp/vu2b-finish.err; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets; cargo fmt -- --check; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vu2b"
  role: "validates"
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
status: "recorded"
title: "Final root compatibility/no-shim validation passes after app CLI and root closeout commits"
updated_at: "2026-06-15T18:56:05.794163142+00:00"
---

## Summary

Final root compatibility/no-shim validation passes after app CLI and root closeout commits

## Command

```console
bash -lc 'rg "atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance|was removed|fallback shim|compatibility alias" crates fuzz tests src docs -n; test $? -eq 1; set +e; target/debug/atelier finish >/tmp/vu2b-finish.out 2>/tmp/vu2b-finish.err; status=$?; set -e; test $status -ne 0; rg "unrecognized subcommand .finish." /tmp/vu2b-finish.err; ! rg "was removed|atelier issue close" /tmp/vu2b-finish.err; RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets; cargo fmt -- --check; target/debug/atelier lint; target/debug/atelier export --check; target/debug/atelier doctor; git diff --check'
```

Exit status: 0

## Stdout

Bytes: 2437
Truncated: no

```text
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
```

## Stderr

Bytes: 452
Truncated: no

```text
rg: tests: No such file or directory (os error 2)
rg: src: No such file or directory (os error 2)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Checking atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
    Checking atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
    Checking atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.81s
```
