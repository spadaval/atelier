---
created_at: "2026-06-15T18:33:32.891085193+00:00"
id: "atelier-1yor"
evidence_type: "validation"
captured_at: "2026-06-15T18:33:32.544209298+00:00"
command: "bash -lc 'rg \"println!|eprintln!\" crates/atelier-app/src -n; test $? -eq 1; rg \"CanonicalExportRequest|CanonicalExportView|canonical_export|pub mod command_storage|pub mod storage_layout\" crates/atelier-app/src -n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-4ren"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 857
    summary: "crates/atelier-app/src/export.rs:24:pub struct CanonicalExportRequest<'a> {\ncrates/atelier-app/src/export.rs:31:pub struct CanonicalExportView {\ncrates/atelier-app/src/export.rs:38:pub fn canonical_export(\ncrates/atelier-app/src/export.rs:39:    request: Request<CanonicalExportRequest<'_>>,\ncrates/atelier-app/src/export.rs:40:) -> Result<Outcome<ViewModel<CanonicalExportView>>> {\ncrates/atelier-app/src/export.rs:47:                    data: CanonicalExportView {\ncrates/atelier-app/src/export.rs:63:            data: CanonicalExportView {\ncrates/atelier-app/src/export.rs:424:    fn test_canonical_export_preserves_issue_activity_sidecars() {\ncrates/atelier-app/src/export.rs:530:    fn test_canonical_export_removes_stale_record_file() {\ncrates/atelier-app/src/lib.rs:6:pub mod command_storage;\ncrates/atelier-app/src/lib.rs:10:pub mod storage_layout;\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4ren"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Search confirms atelier-app has no println/eprintln and exposes explicit CanonicalExportRequest/CanonicalExportView API"
updated_at: "2026-06-15T18:33:36.085402618+00:00"
---

Search confirms atelier-app has no println/eprintln and exposes explicit CanonicalExportRequest/CanonicalExportView API

Command: bash -lc 'rg "println!|eprintln!" crates/atelier-app/src -n; test $? -eq 1; rg "CanonicalExportRequest|CanonicalExportView|canonical_export|pub mod command_storage|pub mod storage_layout" crates/atelier-app/src -n'
Exit status: 0

Stdout summary:
crates/atelier-app/src/export.rs:24:pub struct CanonicalExportRequest<'a> {
crates/atelier-app/src/export.rs:31:pub struct CanonicalExportView {
crates/atelier-app/src/export.rs:38:pub fn canonical_export(
crates/atelier-app/src/export.rs:39:    request: Request<CanonicalExportRequest<'_>>,
crates/atelier-app/src/export.rs:40:) -> Result<Outcome<ViewModel<CanonicalExportView>>> {
crates/atelier-app/src/export.rs:47:                    data: CanonicalExportView {
crates/atelier-app/src/export.rs:63:            data: CanonicalExportView {
crates/atelier-app/src/export.rs:424:    fn test_canonical_export_preserves_issue_activity_sidecars() {
crates/atelier-app/src/export.rs:530:    fn test_canonical_export_removes_stale_record_file() {
crates/atelier-app/src/lib.rs:6:pub mod command_storage;
crates/atelier-app/src/lib.rs:10:pub mod storage_layout;

Stderr summary:
(none)

