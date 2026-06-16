---
created_at: "2026-06-15T18:51:24.498068533+00:00"
id: "atelier-8vmg"
evidence_type: "validation"
captured_at: "2026-06-15T18:51:24.159326190+00:00"
command: "bash -lc 'rg \"derive\\(Parser\\)|derive\\(Subcommand\\)|fn init_tracing|record_command_event|try_parse|std::process::exit\" crates/atelier-cli/src/main.rs -n; rg \"atelier_app::(command_storage|init|lint|health|man|export|rebuild|projection|storage_layout|workflow_policy)::\" crates/atelier-cli/src -n; rg \"println!|eprintln!\" crates/atelier-app/src -n; test $? -eq 1'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zwna"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zwna"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "CLI shell owns Clap telemetry rendering and delegates migrated use cases through atelier-app APIs"
updated_at: "2026-06-15T18:51:27.922104822+00:00"
---

## Summary

CLI shell owns Clap telemetry rendering and delegates migrated use cases through atelier-app APIs

## Command

```console
bash -lc 'rg "derive\(Parser\)|derive\(Subcommand\)|fn init_tracing|record_command_event|try_parse|std::process::exit" crates/atelier-cli/src/main.rs -n; rg "atelier_app::(command_storage|init|lint|health|man|export|rebuild|projection|storage_layout|workflow_policy)::" crates/atelier-cli/src -n; rg "println!|eprintln!" crates/atelier-app/src -n; test $? -eq 1'
```

Exit status: 0

## Stdout

Bytes: 11549
Truncated: yes

```text
14:#[derive(Parser)]
98:#[derive(Subcommand)]
263:#[derive(Subcommand)]
404:#[derive(Subcommand)]
422:#[derive(Subcommand)]
434:#[derive(Subcommand)]
518:#[derive(Subcommand)]
560:#[derive(Subcommand)]
609:#[derive(Subcommand)]
615:#[derive(Subcommand)]
643:#[derive(Subcommand)]
653:#[derive(Subcommand)]
828:fn init_tracing(log_level: &str, log_format: &str) {
1556:    telemetry::record_command_event(
1567:    match Cli::try_parse() {
1574:            std::process::exit(exit_code);
crates/atelier-cli/src/main.rs:3:use atelier_app::command_storage::{
crates/atelier-cli/src/main.rs:1004:            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
crates/atelier-cli/src/main.rs:1013:            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
crates/atelier-cli/src/telemetry.rs:144:        let layout = atelier_app::storage_layout::StorageLayout::new(root);
crates/atelier-cli/src/telemetry.rs:152:                .unwrap_or(atelier_app::storage_layout::ATELIER_DIR)
crates/atelier-cli/src/telemetry.rs:297:            .join(atelier_app::storage_layout::ATELIER_DIR)
crates/atelier-cli/src/commands/plan.rs:126:    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)
crates/atelier-cli/src/commands/plan.rs:165:            atelier_app::rebuild::validate_canonical_state(state_dir)?;
crates/atelier-cli/src/commands/plan.rs:1109:    atelier_app::storage_layout::find_canonical_dir_from_cwd()
crates/atelier-cli/src/commands/activity_log.rs:156:    let state_dir = atelier_app::storage_layout::find_canonical_dir_from_cwd().ok()??;
crates/atelier-cli/src/commands/activity_log.rs:162:    let state_dir = atelier_app::storage_layout::find_canonical_dir_from_cwd().ok()??;
crates/atelier-cli/src/commands/man.rs:4:use atelier_app::command_storage::{command_storage, CommandStorageAccess};
crates/atelier-cli/src/commands/man.rs:90:    let stale_count = atelier_app::export::canonical_stale_entries(db, state_dir)?.len();
crates/atelier-cli/src/commands/mission.rs:593:    atelier_app::storage_layout::find_canonical_dir_from_cwd()
crates/atelier-cli/src/commands/mission.rs:1615:    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)
crates/atelier-cli/src/commands/mission.rs:2163:    let stale_entries = atelier_app::export::canonical_stale_entries(db, state_dir)
crates/atelier-cli/src/commands/evidence.rs:298:    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)
crates/atelier-cli/src/commands/evidence.rs:628:    atelier_app::storage_layout::find_canonical_dir_from_cwd()
crates/atelier-cli/src/commands/issue_workflow.rs:4:use atelier_app::workflow_policy::WorkflowPolicy;
crates/atelier-cli/src/commands/issue_workflow.rs:16:    let repo_root = atelier_app::storage_layout::find_repo_root()?;
crates/atelier-cli/src/commands/issue_workflow.rs:17:    let policy_path = repo_root.join(atelier_app::workflow_policy::WORKFLOW_POLICY_PATH);
crates/atelier-cli/src/commands/issue_workflow.rs:21:    atelier_app::workflow_policy::load(&repo_root).map(Some)
crates/atelier-cli/src/commands/workflow.rs:40:    let report = atelier_app::workflow_policy::check(db, &repo_root)?;
crates/atelier-cli/src/commands/workflow.rs:45:        atelier_app::workflow_policy::WORKFLOW_POLICY_PATH
crates/atelier-cli/src/commands/workflow.rs:72:    let policy = atelier_app::workflow_policy::load(&repo_root)?;
crates/atelier-cli/src/commands/workflow.rs:76:    let state_dir = atelier_app::storage_layout::StorageLayout::new(&repo_root).canonical_dir();
crates/atelier-cli/src/commands/workflow.rs:134:    let policy = atelier_app::workflow_policy::load(&repo_root)?;
crates/atelier-cli/src/commands/workflow.rs:165:    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)?;
crates/atelier-cli/src/commands/workflow.rs:178:    workflow: &'a atelier_app::workflow_policy::WorkflowDefinition,
crates/atelier-cli/src/commands/workflow.rs:181:) -> Result<&'a atelier_app::workflow_policy::TransitionDefinition> {
crates/atelier-cli/src/commands/workflow.rs:211
```

## Stderr

Bytes: 0
Truncated: no

```text
```
