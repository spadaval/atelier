---
created_at: "2026-06-14T07:31:23.239876161+00:00"
id: "atelier-qmrs"
evidence_type: "validation"
captured_at: "2026-06-14T07:31:23.239768792+00:00"
target:
  kind: "issue"
  id: "atelier-papa"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-papa"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Evidence relation role errors are corrective: invalid --role validation is rejected before writes, the accepted relation vocabulary names validates, and the error routes classification to --kind plus normal record/attach flows. Proof: cargo nextest run --test cli_integration -E 'test(test_evidence_relation_role_errors_are_corrective) or test(test_evidence_record_help_shows_issue_targeted_manual_and_command_flows) or test(test_evidence_capture_records_command_metadata_and_attaches_targets)'; cargo fmt -- --check; git diff --check; atelier lint; atelier lint atelier-papa; atelier export --check."
updated_at: "2026-06-14T07:31:25.801759565+00:00"
---

Evidence relation role errors are corrective: invalid --role validation is rejected before writes, the accepted relation vocabulary names validates, and the error routes classification to --kind plus normal record/attach flows. Proof: cargo nextest run --test cli_integration -E 'test(test_evidence_relation_role_errors_are_corrective) or test(test_evidence_record_help_shows_issue_targeted_manual_and_command_flows) or test(test_evidence_capture_records_command_metadata_and_attaches_targets)'; cargo fmt -- --check; git diff --check; atelier lint; atelier lint atelier-papa; atelier export --check.
