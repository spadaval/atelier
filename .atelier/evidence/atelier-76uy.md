---
created_at: "2026-06-13T19:40:24.256753422+00:00"
id: "atelier-76uy"
evidence_type: "validation"
captured_at: "2026-06-13T19:40:24.256685067+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fyms"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "fail"
title: "Independent validation for atelier-fyms (mission atelier-z04a). Classification: starter policy = fail/partial: workflow init writes the starter policy and focused init tests pass, but after workflow init+migration, `atelier issue create` still writes legacy status open; transition then requires rerunning migrate-statuses. Follow-up: atelier-eovw. workflow check = pass: temp transcript and repo `target/debug/atelier workflow check` pass. status migration = pass with test-health risk: hand-shaped legacy records migrate open->todo, closed->done, archived->archived and check passes; focused test `test_workflow_migrate_statuses_rewrites_legacy_issue_statuses_and_preserves_close_metadata` fails because it tries legacy close before .atelier/workflow.yaml and current CLI reports workflow_config_missing. start transition = pass: migrated todo issue starts to in_progress and records active work. blocked transition = pass: request_validation from in_progress is blocked with review_ready guidance and no evidence creation. close with evidence = pass: validation issue close blocks without reason/proof/clean tree, then succeeds after attached passing evidence and clean commit. lightweight spike close = pass after migrating the newly created spike: review->done with close reason and no evidence requirement. archive = pass when policy defines archive transition: close --to archived applies archive and writes archived. missing .atelier/workflow.yaml = pass: transition options fail with workflow_config_missing. unmigrated-record failures = pass: transition options on open record fail with workflow migrate-statuses guidance. raw workflow validate reliance = pass: workflow help exposes init/migrate-statuses/check only, normal status output routes to mission status, issue list --blocked, and doctor; focused no-raw-validator tests pass.\nCommands/tests: focused cargo nextest run selected workflow tests = 14 passed, 1 failed as above; corrected temp transcripts exercised the named CLI paths; target/debug/atelier workflow check pass; target/debug/atelier lint pass; target/debug/atelier export --check pass after rebuild; target/debug/atelier doctor pass after rebuild; git diff --check pass. Residual risks: temporary transcript required rerunning migrate-statuses after each new issue because of atelier-eovw; status-migration integration test fixture may need repair or implementation adjustment depending intended legacy-close support."
updated_at: "2026-06-13T19:40:26.327680187+00:00"
---

Independent validation for atelier-fyms (mission atelier-z04a). Classification: starter policy = fail/partial: workflow init writes the starter policy and focused init tests pass, but after workflow init+migration, `atelier issue create` still writes legacy status open; transition then requires rerunning migrate-statuses. Follow-up: atelier-eovw. workflow check = pass: temp transcript and repo `target/debug/atelier workflow check` pass. status migration = pass with test-health risk: hand-shaped legacy records migrate open->todo, closed->done, archived->archived and check passes; focused test `test_workflow_migrate_statuses_rewrites_legacy_issue_statuses_and_preserves_close_metadata` fails because it tries legacy close before .atelier/workflow.yaml and current CLI reports workflow_config_missing. start transition = pass: migrated todo issue starts to in_progress and records active work. blocked transition = pass: request_validation from in_progress is blocked with review_ready guidance and no evidence creation. close with evidence = pass: validation issue close blocks without reason/proof/clean tree, then succeeds after attached passing evidence and clean commit. lightweight spike close = pass after migrating the newly created spike: review->done with close reason and no evidence requirement. archive = pass when policy defines archive transition: close --to archived applies archive and writes archived. missing .atelier/workflow.yaml = pass: transition options fail with workflow_config_missing. unmigrated-record failures = pass: transition options on open record fail with workflow migrate-statuses guidance. raw workflow validate reliance = pass: workflow help exposes init/migrate-statuses/check only, normal status output routes to mission status, issue list --blocked, and doctor; focused no-raw-validator tests pass.
Commands/tests: focused cargo nextest run selected workflow tests = 14 passed, 1 failed as above; corrected temp transcripts exercised the named CLI paths; target/debug/atelier workflow check pass; target/debug/atelier lint pass; target/debug/atelier export --check pass after rebuild; target/debug/atelier doctor pass after rebuild; git diff --check pass. Residual risks: temporary transcript required rerunning migrate-statuses after each new issue because of atelier-eovw; status-migration integration test fixture may need repair or implementation adjustment depending intended legacy-close support.
