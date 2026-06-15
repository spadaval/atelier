---
created_at: "2026-06-15T18:48:37.401720776+00:00"
id: "atelier-keg4"
evidence_type: "validation"
captured_at: "2026-06-15T18:48:10.392909357+00:00"
command: "bash -lc 'find crates/atelier-app/src -maxdepth 1 -type f -printf \"%f\\n\" | sort; rg \"pub struct (InitRequest|InitView|LintRequest|LintView|DoctorRequest|DoctorView|ManSnapshotRequest|ManSnapshotView|CanonicalExportRequest|CanonicalExportView)|pub fn (initialize|lint|doctor|snapshot|canonical_export)\" crates/atelier-app/src -n; cargo test -p atelier-cli init -- --nocapture; cargo test -p atelier-cli doctor -- --nocapture; cargo test -p atelier-cli lint -- --nocapture; cargo test -p atelier-cli man -- --nocapture'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-14z2"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 41637
    summary: "command_storage.rs\nexport.rs\nhealth.rs\ninit.rs\nlib.rs\nlint.rs\nprojection.rs\nrebuild.rs\nstorage_layout.rs\nworkflow_policy.rs\ncrates/atelier-app/src/health.rs:7:pub struct DoctorRequest<'a> {\ncrates/atelier-app/src/health.rs:18:pub struct DoctorView {\ncrates/atelier-app/src/health.rs:35:pub fn doctor(\ncrates/atelier-app/src/lint.rs:9:pub struct LintRequest<'a> {\ncrates/atelier-app/src/lint.rs:15:pub struct LintView {\ncrates/atelier-app/src/lint.rs:75:pub fn lint(\ncrates/atelier-app/src/init.rs:31:pub struct InitRequest {\ncrates/atelier-app/src/init.rs:38:pub struct InitView {\ncrates/atelier-app/src/init.rs:54:pub fn initialize(\ncrates/atelier-app/src/command_storage.rs:195:pub fn lint_db() -> Result<Database> {\ncrates/atelier-app/src/export.rs:24:pub struct CanonicalExportRequest<'a> {\ncrates/atelier-app/src/export.rs:31:pub struct CanonicalExportView {\ncrates/atelier-app/src/export.rs:38:pub fn canonical_export(\n\nrunning 10 tests\ntest identity::tests::test_init_and_load_roundtrip ... ok\nCreated /tmp/.tmpRXtLnm/.atelier\nCreated /tmp/.tmpRXtLnm/.atelier/config.toml\nCreated /tmp/.tmpRXtLnm/.atelier/runtime/state.db\nCreated /tmp/.tmpRXtLnm/.atelier/workflow.yaml\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\ntest commands::init::tests::test_run_database_usable ... ok\nCreated /tmp/.tmpJyDWkr/.atelier\nCreated /tmp/.tmpJyDWkr/.atelier/config.toml\nCreated /tmp/.tmpJyDWkr/.atelier/runtime/state.db\nCreated /tmp/.tmpJyDWkr/.atelier/workflow.yaml\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\nCreated /tmp/.tmpUrRYRx/.atelier\nCreated /tmp/.tmpUrRYRx/.atelier/config.toml\nCreated /tmp/.tmpUrRYRx/.atelier/runtime/state.db\nCreated /tmp/.tmpUrRYRx/.atelier/workflow.yaml\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\nCreated /tmp/.tmpYh3maH/.atelier\nCreated /tmp/.tmpYh3maH/.atelier/config.toml\nCreated /tmp/.tmpYh3maH/.atelier/runtime/state.db\nCreated /tmp/.tmpYh3maH/.atelier/workflow.yaml\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\nCreated /tmp/.tmpqOSYI1/.atelier/config.toml\nCreated /tmp/.tmpqOSYI1/.atelier/runtime/state.db\nCreated /tmp/.tmpqOSYI1/.atelier/workflow.yaml\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\nCreated /tmp/.tmpiT4wMO/.atelier\nCreated /tmp/.tmpiT4wMO/.atelier/config.toml\nCreated /tmp/.tmpiT4wMO/.atelier/runtime/state.db\nCreated /tmp/.tmpiT4wMO/.atelier/workflow.yaml\nAtelier initialized successfully!\n\nNext steps:\n  atelier lint                     # Verify tracker records and workflow setup\n  atelier issue create \"Task\"     # Create the first tracked issue\n  atelier man admin                # Review setup and repair guidance\ntest commands::init::tests::test_run_fresh_init ... ok\ntest commands::init::tests::test_run_partial_init_atelier_only ... ok\ntest commands::init::tests::test_run_force_update .."
    truncated: true
  stderr:
    bytes: 1934
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.76s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.62s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\nSwitched to branch 'mission/atelier-owtk'\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-14z2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "App module inventory and focused tests prove migrated setup/health/storage handlers expose request/outcome/view APIs while CLI renders"
updated_at: "2026-06-15T18:48:40.726238023+00:00"
---

App module inventory and focused tests prove migrated setup/health/storage handlers expose request/outcome/view APIs while CLI renders

Command: bash -lc 'find crates/atelier-app/src -maxdepth 1 -type f -printf "%f\n" | sort; rg "pub struct (InitRequest|InitView|LintRequest|LintView|DoctorRequest|DoctorView|ManSnapshotRequest|ManSnapshotView|CanonicalExportRequest|CanonicalExportView)|pub fn (initialize|lint|doctor|snapshot|canonical_export)" crates/atelier-app/src -n; cargo test -p atelier-cli init -- --nocapture; cargo test -p atelier-cli doctor -- --nocapture; cargo test -p atelier-cli lint -- --nocapture; cargo test -p atelier-cli man -- --nocapture'
Exit status: 0

Stdout summary (truncated):
command_storage.rs
export.rs
health.rs
init.rs
lib.rs
lint.rs
projection.rs
rebuild.rs
storage_layout.rs
workflow_policy.rs
crates/atelier-app/src/health.rs:7:pub struct DoctorRequest<'a> {
crates/atelier-app/src/health.rs:18:pub struct DoctorView {
crates/atelier-app/src/health.rs:35:pub fn doctor(
crates/atelier-app/src/lint.rs:9:pub struct LintRequest<'a> {
crates/atelier-app/src/lint.rs:15:pub struct LintView {
crates/atelier-app/src/lint.rs:75:pub fn lint(
crates/atelier-app/src/init.rs:31:pub struct InitRequest {
crates/atelier-app/src/init.rs:38:pub struct InitView {
crates/atelier-app/src/init.rs:54:pub fn initialize(
crates/atelier-app/src/command_storage.rs:195:pub fn lint_db() -> Result<Database> {
crates/atelier-app/src/export.rs:24:pub struct CanonicalExportRequest<'a> {
crates/atelier-app/src/export.rs:31:pub struct CanonicalExportView {
crates/atelier-app/src/export.rs:38:pub fn canonical_export(

running 10 tests
test identity::tests::test_init_and_load_roundtrip ... ok
Created /tmp/.tmpRXtLnm/.atelier
Created /tmp/.tmpRXtLnm/.atelier/config.toml
Created /tmp/.tmpRXtLnm/.atelier/runtime/state.db
Created /tmp/.tmpRXtLnm/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
test commands::init::tests::test_run_database_usable ... ok
Created /tmp/.tmpJyDWkr/.atelier
Created /tmp/.tmpJyDWkr/.atelier/config.toml
Created /tmp/.tmpJyDWkr/.atelier/runtime/state.db
Created /tmp/.tmpJyDWkr/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Created /tmp/.tmpUrRYRx/.atelier
Created /tmp/.tmpUrRYRx/.atelier/config.toml
Created /tmp/.tmpUrRYRx/.atelier/runtime/state.db
Created /tmp/.tmpUrRYRx/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Created /tmp/.tmpYh3maH/.atelier
Created /tmp/.tmpYh3maH/.atelier/config.toml
Created /tmp/.tmpYh3maH/.atelier/runtime/state.db
Created /tmp/.tmpYh3maH/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Created /tmp/.tmpqOSYI1/.atelier/config.toml
Created /tmp/.tmpqOSYI1/.atelier/runtime/state.db
Created /tmp/.tmpqOSYI1/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Created /tmp/.tmpiT4wMO/.atelier
Created /tmp/.tmpiT4wMO/.atelier/config.toml
Created /tmp/.tmpiT4wMO/.atelier/runtime/state.db
Created /tmp/.tmpiT4wMO/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
test commands::init::tests::test_run_fresh_init ... ok
test commands::init::tests::test_run_partial_init_atelier_only ... ok
test commands::init::tests::test_run_force_update ..

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.76s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.62s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
Switched to branch 'mission/atelier-owtk'
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)

