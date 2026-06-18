---
created_at: "2026-06-18T17:59:59.429598976+00:00"
id: "atelier-ts1o"
evidence_type: "validation"
captured_at: "2026-06-18T17:59:59.429597283+00:00"
target:
  kind: "issue"
  id: "atelier-f2c4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f2c4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "session list/show now render derived issue attempts with issue, role, serial, state, and recent activity; focused proof: cargo nextest run -p atelier-cli --test cli_integration sessions (4 passed), cargo nextest run -p atelier-records issue_attempts_are_derived_by_issue_role_and_serial (1 passed), cargo fmt -- --check, git diff --check, atelier lint atelier-f2c4. Transcript: ./target/debug/atelier session list --active printed atelier-f2c4/worker/1 active worker serial=1 issue/atelier-f2c4 recent=work_started started - Started work; session show printed Activity rows from transition_applied/work_started. git status before/after list/show was unchanged, proving inspection-only behavior."
updated_at: "2026-06-18T18:00:02.528178724+00:00"
---

session list/show now render derived issue attempts with issue, role, serial, state, and recent activity; focused proof: cargo nextest run -p atelier-cli --test cli_integration sessions (4 passed), cargo nextest run -p atelier-records issue_attempts_are_derived_by_issue_role_and_serial (1 passed), cargo fmt -- --check, git diff --check, atelier lint atelier-f2c4. Transcript: ./target/debug/atelier session list --active printed atelier-f2c4/worker/1 active worker serial=1 issue/atelier-f2c4 recent=work_started started - Started work; session show printed Activity rows from transition_applied/work_started. git status before/after list/show was unchanged, proving inspection-only behavior.
