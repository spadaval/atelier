---
created_at: "2026-06-15T18:50:07.144642691+00:00"
id: "atelier-afeb"
evidence_type: "validation"
captured_at: "2026-06-15T18:49:45.203213104+00:00"
command: "bash -lc 'printf \"app modules\\n\"; find crates/atelier-app/src -maxdepth 1 -type f -printf \"%f\\n\" | sort; printf \"cli command modules\\n\"; find crates/atelier-cli/src/commands -maxdepth 1 -type f -printf \"%f\\n\" | sort; printf \"root command files\\n\"; find src -maxdepth 2 -type f 2>/dev/null | sort || true; printf \"representative transcripts\\n\"; target/debug/atelier --help >/tmp/nyn0-help.txt; target/debug/atelier issue list --ready >/tmp/nyn0-ready.txt; target/debug/atelier mission status atelier-v5nb >/tmp/nyn0-mission.txt; target/debug/atelier doctor >/tmp/nyn0-doctor.txt; target/debug/atelier export --check >/tmp/nyn0-export.txt; wc -l /tmp/nyn0-help.txt /tmp/nyn0-ready.txt /tmp/nyn0-mission.txt /tmp/nyn0-doctor.txt /tmp/nyn0-export.txt; rg \"atelier_app::(init|lint|health|man|export|rebuild|projection|storage_layout|workflow_policy)|atelier_sqlite::|atelier_records::\" crates/atelier-cli/src crates/atelier-app/src -n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-nyn0"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 16409
    summary: "app modules\ncommand_storage.rs\nexport.rs\nhealth.rs\ninit.rs\nlib.rs\nlint.rs\nprojection.rs\nrebuild.rs\nstorage_layout.rs\nworkflow_policy.rs\ncli command modules\nactivity_log.rs\nagent_factory.rs\ncomment.rs\ncreate.rs\ndelete.rs\ndeps.rs\nevidence.rs\nhistory.rs\nimport.rs\ninit.rs\nissue_workflow.rs\nlabel.rs\nman.rs\nmission.rs\nmod.rs\nnext.rs\nplan.rs\nrelate.rs\nstatus.rs\ntested.rs\ntree.rs\nwork.rs\nworkflow.rs\nroot command files\nrepresentative transcripts\n  69 /tmp/nyn0-help.txt\n  11 /tmp/nyn0-ready.txt\n  94 /tmp/nyn0-mission.txt\n  28 /tmp/nyn0-doctor.txt\n   2 /tmp/nyn0-export.txt\n 204 total\ncrates/atelier-app/src/health.rs:5:use atelier_sqlite::Database;\ncrates/atelier-app/src/health.rs:65:    let projection_fresh = atelier_sqlite::projection_index::check(active_db, &input.state_dir)\ncrates/atelier-app/src/lint.rs:6:use atelier_records::{issue_record_path, issue_section_diagnostic, IssueSectionName, RecordStore};\ncrates/atelier-app/src/lint.rs:7:use atelier_sqlite::Database;\ncrates/atelier-app/src/lint.rs:143:        if !atelier_sqlite::VALID_ISSUE_TYPES.contains(&issue.issue_type.as_str()) {\ncrates/atelier-app/src/init.rs:5:use atelier_sqlite::Database;\ncrates/atelier-app/src/init.rs:67:    for dir in atelier_records::canonical_record_dirs() {\ncrates/atelier-app/src/export.rs:9:use atelier_records::{\ncrates/atelier-app/src/export.rs:13:use atelier_sqlite::projection_index;\ncrates/atelier-app/src/export.rs:14:use atelier_sqlite::Database;\ncrates/atelier-app/src/rebuild.rs:13:use atelier_records::activity::IssueActivity;\ncrates/atelier-app/src/rebuild.rs:14:use atelier_records::{\ncrates/atelier-app/src/rebuild.rs:17:use atelier_sqlite::projection_index;\ncrates/atelier-app/src/rebuild.rs:18:use atelier_sqlite::Database;\ncrates/atelier-app/src/rebuild.rs:898:    use atelier_records::issue_record_path;\ncrates/atelier-app/src/command_storage.rs:5:use atelier_sqlite::projection_index;\ncrates/atelier-app/src/command_storage.rs:6:use atelier_sqlite::Database;\ncrates/atelier-cli/src/main.rs:7:use atelier_records::RecordStore;\ncrates/atelier-cli/src/main.rs:8:use atelier_sqlite::Database;\ncrates/atelier-cli/src/main.rs:1004:            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)\ncrates/atelier-cli/src/main.rs:1013:            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)\ncrates/atelier-app/src/workflow_policy.rs:10:use atelier_sqlite::Database;\ncrates/atelier-cli/src/telemetry.rs:144:        let layout = atelier_app::storage_layout::StorageLayout::new(root);\ncrates/atelier-cli/src/telemetry.rs:152:                .unwrap_or(atelier_app::storage_layout::ATELIER_DIR)\ncrates/atelier-cli/src/telemetry.rs:297:            .join(atelier_app::storage_layout::ATELIER_DIR)\ncrates/atelier-cli/src/commands/plan.rs:11:use atelier_records::{\ncrates/atelier-cli/src/commands/plan.rs:14:use atelier_sqlite::{\ncrates/atelier-cli/src/commands/plan.rs:74:    let mut data = atelier_records::normalized_plan_data(&current.record.data_json)?;\ncrates/atelier-cli/src/commands/plan.rs:126:    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)\ncrates/atelier-cli/src/commands/plan.rs:165:            atelier_app::rebuild::validate_canonical_state(state_dir)?;\ncrates/atelier-cli/src/commands/plan.rs:217:    Ok(atelier_records::normalized_plan_data(&record.data_json)?.revision)\ncrates/atelier-cli/src/commands/plan.rs:1109:    atelier_app::storage_layout::find_canonical_dir_from_cwd()\ncrates/atelier-cli/src/commands/status.rs:10:use atelier_records::activity::list_all_issue_activities;\ncrates/atelier-cli/src/commands/status.rs:11:use atelier_sqlite::Database;\ncrates/atelier-cli/src/commands/status.rs:46:    let export_stale = atelier_app::export::canonical_stale_entries(db, state_dir)?;\ncrates/atelier-cli/src/commands/status.rs:354:    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,\ncrates/atelier-cli/src/commands/status.rs:374:    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,\ncrates/atelier-cli/src/commands/status.rs:392:    workflow_policy: Opt"
    truncated: true
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nyn0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Workflow migration map: setup/health/storage/workflow policy live in app modules, CLI renders, storage calls use records/sqlite crates, no old root files remain"
updated_at: "2026-06-15T18:50:10.651229562+00:00"
---

Workflow migration map: setup/health/storage/workflow policy live in app modules, CLI renders, storage calls use records/sqlite crates, no old root files remain

Command: bash -lc 'printf "app modules\n"; find crates/atelier-app/src -maxdepth 1 -type f -printf "%f\n" | sort; printf "cli command modules\n"; find crates/atelier-cli/src/commands -maxdepth 1 -type f -printf "%f\n" | sort; printf "root command files\n"; find src -maxdepth 2 -type f 2>/dev/null | sort || true; printf "representative transcripts\n"; target/debug/atelier --help >/tmp/nyn0-help.txt; target/debug/atelier issue list --ready >/tmp/nyn0-ready.txt; target/debug/atelier mission status atelier-v5nb >/tmp/nyn0-mission.txt; target/debug/atelier doctor >/tmp/nyn0-doctor.txt; target/debug/atelier export --check >/tmp/nyn0-export.txt; wc -l /tmp/nyn0-help.txt /tmp/nyn0-ready.txt /tmp/nyn0-mission.txt /tmp/nyn0-doctor.txt /tmp/nyn0-export.txt; rg "atelier_app::(init|lint|health|man|export|rebuild|projection|storage_layout|workflow_policy)|atelier_sqlite::|atelier_records::" crates/atelier-cli/src crates/atelier-app/src -n'
Exit status: 0

Stdout summary (truncated):
app modules
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
cli command modules
activity_log.rs
agent_factory.rs
comment.rs
create.rs
delete.rs
deps.rs
evidence.rs
history.rs
import.rs
init.rs
issue_workflow.rs
label.rs
man.rs
mission.rs
mod.rs
next.rs
plan.rs
relate.rs
status.rs
tested.rs
tree.rs
work.rs
workflow.rs
root command files
representative transcripts
  69 /tmp/nyn0-help.txt
  11 /tmp/nyn0-ready.txt
  94 /tmp/nyn0-mission.txt
  28 /tmp/nyn0-doctor.txt
   2 /tmp/nyn0-export.txt
 204 total
crates/atelier-app/src/health.rs:5:use atelier_sqlite::Database;
crates/atelier-app/src/health.rs:65:    let projection_fresh = atelier_sqlite::projection_index::check(active_db, &input.state_dir)
crates/atelier-app/src/lint.rs:6:use atelier_records::{issue_record_path, issue_section_diagnostic, IssueSectionName, RecordStore};
crates/atelier-app/src/lint.rs:7:use atelier_sqlite::Database;
crates/atelier-app/src/lint.rs:143:        if !atelier_sqlite::VALID_ISSUE_TYPES.contains(&issue.issue_type.as_str()) {
crates/atelier-app/src/init.rs:5:use atelier_sqlite::Database;
crates/atelier-app/src/init.rs:67:    for dir in atelier_records::canonical_record_dirs() {
crates/atelier-app/src/export.rs:9:use atelier_records::{
crates/atelier-app/src/export.rs:13:use atelier_sqlite::projection_index;
crates/atelier-app/src/export.rs:14:use atelier_sqlite::Database;
crates/atelier-app/src/rebuild.rs:13:use atelier_records::activity::IssueActivity;
crates/atelier-app/src/rebuild.rs:14:use atelier_records::{
crates/atelier-app/src/rebuild.rs:17:use atelier_sqlite::projection_index;
crates/atelier-app/src/rebuild.rs:18:use atelier_sqlite::Database;
crates/atelier-app/src/rebuild.rs:898:    use atelier_records::issue_record_path;
crates/atelier-app/src/command_storage.rs:5:use atelier_sqlite::projection_index;
crates/atelier-app/src/command_storage.rs:6:use atelier_sqlite::Database;
crates/atelier-cli/src/main.rs:7:use atelier_records::RecordStore;
crates/atelier-cli/src/main.rs:8:use atelier_sqlite::Database;
crates/atelier-cli/src/main.rs:1004:            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
crates/atelier-cli/src/main.rs:1013:            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
crates/atelier-app/src/workflow_policy.rs:10:use atelier_sqlite::Database;
crates/atelier-cli/src/telemetry.rs:144:        let layout = atelier_app::storage_layout::StorageLayout::new(root);
crates/atelier-cli/src/telemetry.rs:152:                .unwrap_or(atelier_app::storage_layout::ATELIER_DIR)
crates/atelier-cli/src/telemetry.rs:297:            .join(atelier_app::storage_layout::ATELIER_DIR)
crates/atelier-cli/src/commands/plan.rs:11:use atelier_records::{
crates/atelier-cli/src/commands/plan.rs:14:use atelier_sqlite::{
crates/atelier-cli/src/commands/plan.rs:74:    let mut data = atelier_records::normalized_plan_data(&current.record.data_json)?;
crates/atelier-cli/src/commands/plan.rs:126:    atelier_app::projection::refresh_after_canonical_write(state_dir, db_path)
crates/atelier-cli/src/commands/plan.rs:165:            atelier_app::rebuild::validate_canonical_state(state_dir)?;
crates/atelier-cli/src/commands/plan.rs:217:    Ok(atelier_records::normalized_plan_data(&record.data_json)?.revision)
crates/atelier-cli/src/commands/plan.rs:1109:    atelier_app::storage_layout::find_canonical_dir_from_cwd()
crates/atelier-cli/src/commands/status.rs:10:use atelier_records::activity::list_all_issue_activities;
crates/atelier-cli/src/commands/status.rs:11:use atelier_sqlite::Database;
crates/atelier-cli/src/commands/status.rs:46:    let export_stale = atelier_app::export::canonical_stale_entries(db, state_dir)?;
crates/atelier-cli/src/commands/status.rs:354:    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
crates/atelier-cli/src/commands/status.rs:374:    workflow_policy: Option<&atelier_app::workflow_policy::WorkflowPolicy>,
crates/atelier-cli/src/commands/status.rs:392:    workflow_policy: Opt

Stderr summary:
(none)

