---
created_at: "2026-06-15T07:56:02.380925303+00:00"
id: "atelier-v4hd"
evidence_type: "test"
captured_at: "2026-06-15T07:55:59.891032147+00:00"
command: "cargo nextest run --test cli_integration cli_integration::issue_records::test_create_issue cli_integration::issue_records::test_create_issue_with_priority cli_integration::issue_records::test_create_issue_with_description cli_integration::issue_records::test_issue_show_human_shape_exposes_actionable_context"
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
    bytes: 0
    summary: ""
    truncated: false
  stderr:
    bytes: 10250
    summary: "   Compiling atelier-tracker v0.2.0 (/root/atelier/.atelier-worktrees/atelier-v5nb)\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/issue_records.rs:1656:19\n     |\n1656 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"issue\", \"ready\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n     |\n     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/issue_records.rs:1686:19\n     |\n1686 |     let (success, stdout, _stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/issue_workflow.rs:530:23\n    |\n530 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n    |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/issue_workflow.rs:654:19\n    |\n654 |     let (success, stdout, stderr) = run_atelier(\n    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/issue_workflow.rs:743:19\n    |\n743 |     let (success, stdout, stderr) =\n    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/issue_workflow.rs:766:19\n    |\n766 |     let (success, stdout, stderr) =\n    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/issue_workflow.rs:797:19\n    |\n797 |     let (success, stdout, stderr) =\n    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/issue_workflow.rs:823:23\n    |\n823 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n    |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/mission_projection.rs:1019:19\n     |\n1019 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"export\", \"--check\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/mission_projection.rs:1274:19\n     |\n1274 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"export\", \"--check\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/records_evidence.rs:1124:19\n     |\n1124 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/records_evidence.rs:1167:19\n     |\n1167 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration/records_evidence.rs:1275:19\n     |\n1275 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n  --> tests/cli_integration/workflow_policy.rs:51:19\n   |\n51 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"workflow\", \"check\"]);\n   |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n   --> tests/cli_integration/workflow_policy.rs:314:19\n    |\n314 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"workflow\", \"check\"]);\n    |  "
    truncated: true
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
title: "atelier-app now owns command job request/view boundaries for issue, mission, evidence, graph, workflow, maintenance, and status; issue CLI dispatch uses app IssueJob requests; focused app and representative CLI tests pass."
updated_at: "2026-06-15T07:56:05.661755789+00:00"
---

atelier-app now owns command job request/view boundaries for issue, mission, evidence, graph, workflow, maintenance, and status; issue CLI dispatch uses app IssueJob requests; focused app and representative CLI tests pass.

Command: cargo nextest run --test cli_integration cli_integration::issue_records::test_create_issue cli_integration::issue_records::test_create_issue_with_priority cli_integration::issue_records::test_create_issue_with_description cli_integration::issue_records::test_issue_show_human_shape_exposes_actionable_context
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier/.atelier-worktrees/atelier-v5nb)
warning: unused variable: `stdout`
    --> tests/cli_integration/issue_records.rs:1656:19
     |
1656 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `stdout`
    --> tests/cli_integration/issue_records.rs:1686:19
     |
1686 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/issue_workflow.rs:530:23
    |
530 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
    |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/issue_workflow.rs:654:19
    |
654 |     let (success, stdout, stderr) = run_atelier(
    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/issue_workflow.rs:743:19
    |
743 |     let (success, stdout, stderr) =
    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/issue_workflow.rs:766:19
    |
766 |     let (success, stdout, stderr) =
    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/issue_workflow.rs:797:19
    |
797 |     let (success, stdout, stderr) =
    |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/issue_workflow.rs:823:23
    |
823 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
    |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration/mission_projection.rs:1019:19
     |
1019 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration/mission_projection.rs:1274:19
     |
1274 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration/records_evidence.rs:1124:19
     |
1124 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration/records_evidence.rs:1167:19
     |
1167 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration/records_evidence.rs:1275:19
     |
1275 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
  --> tests/cli_integration/workflow_policy.rs:51:19
   |
51 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
   |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
   --> tests/cli_integration/workflow_policy.rs:314:19
    |
314 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
    |

