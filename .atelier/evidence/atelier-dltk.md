---
created_at: "2026-06-13T23:02:02.731642680+00:00"
id: "atelier-dltk"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T23:01:55.628008044+00:00\",\"command\":\"bash -lc 'cargo clippy --all-targets -- -W clippy::too_many_lines 2>&1 | rg \\\"too many lines|-->\\\"'\",\"exit_code\":0,\"exit_status\":\"0\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"test\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false},\"stdout\":{\"bytes\":6209,\"summary\":\"  --> src/db/records.rs:21:59\\nwarning: this function has too many lines (187/100)\\n   --> src/db/mod.rs:226:5\\nwarning: this function has too many lines (210/100)\\n   --> src/db/mod.rs:457:5\\nwarning: this function has too many lines (119/100)\\n   --> src/workflow_policy.rs:883:1\\nwarning: this function has too many lines (124/100)\\n    --> src/workflow_policy.rs:1010:1\\n    --> tests/cli_integration.rs:1703:23\\n    --> tests/cli_integration.rs:1827:19\\n    --> tests/cli_integration.rs:1880:19\\n    --> tests/cli_integration.rs:1921:19\\n    --> tests/cli_integration.rs:1944:19\\n    --> tests/cli_integration.rs:1975:19\\n    --> tests/cli_integration.rs:2001:23\\n    --> tests/cli_integration.rs:3805:19\\n    --> tests/cli_integration.rs:3835:19\\n    --> tests/cli_integration.rs:7150:19\\n    --> tests/cli_integration.rs:7193:19\\n    --> tests/cli_integration.rs:7244:19\\n    --> tests/cli_integration.rs:7288:19\\n    --> tests/cli_integration.rs:7305:19\\n    --> tests/cli_integration.rs:7681:19\\n    --> tests/cli_integration.rs:7774:19\\n    --> tests/cli_integration.rs:9445:19\\n    --> tests/cli_integration.rs:9723:19\\nwarning: this function has too many lines (114/100)\\n   --> tests/cli_integration.rs:875:1\\nwarning: this function has too many lines (102/100)\\n    --> tests/cli_integration.rs:3417:1\\nwarning: this function has too many lines (115/100)\\n    --> tests/cli_integration.rs:6343:1\\nwarning: this function has too many lines (251/100)\\n    --> tests/cli_integration.rs:6474:1\\n    --> tests/cli_integration.rs:6685:31\\nwarning: this function has too many lines (294/100)\\n    --> tests/cli_integration.rs:6836:1\\nwarning: this function has too many lines (131/100)\\n    --> tests/cli_integration.rs:8477:1\\nwarning: this function has too many lines (210/100)\\n    --> tests/cli_integration.rs:8655:1\\n    --> tests/cli_integration.rs:8724:32\\n    --> tests/cli_integration.rs:8755:53\\n    --> tests/cli_integration.rs:8755:66\\n    --> tests/cli_integration.rs:8782:62\\n    --> tests/cli_integration.rs:8787:37\\n    --> tests/cli_integration.rs:8787:49\\n    --> tests/cli_integration.rs:8809:33\\n    --> tests/cli_integration.rs:8809:58\\nwarning: this function has too many lines (202/100)\\n    --> tests/cli_integration.rs:8888:1\\nwarning: this function has too many lines (173/100)\\n     --> tests/cli_integration.rs:10232:1\\n    --> src/commands/agent_factory.rs:1535:12\\n    --> src/commands/agent_factory.rs:1653:8\\n    --> src/commands/agent_factory.rs:1705:4\\n    --> src/commands/agent_factory.rs:1731:8\\n    --> src/commands/agent_factory.rs:2114:8\\n    --> src/commands/agent_factory.rs:2133:8\\n  --> src/commands/comment.rs:21:8\\n  --> src/commands/create.rs:76:12\\n  --> src/commands/create.rs:82:8\\n   --> src/commands/create.rs:168:8\\n   --> src/commands/create.rs:223:4\\n --> src/commands/delete.rs:9:8\\n --> src/commands/label.rs:7:8\\n  --> src/commands/label.rs:50:8\\n  --> src/commands/projection.rs:26:8\\n --> src/commands/relate.rs:7:8\\n  --> src/commands/relate.rs:66:8\\n  --> src/commands/workflow.rs:15:7\\n   --> src/commands/workflow.rs:379:8\\n   --> src/commands/workflow.rs:551:8\\n   --> src/commands/workflow.rs:582:4\\n --> src/db/comments.rs:9:12\\n  --> src/db/dependencies.rs:53:12\\n   --> src/db/issues.rs:50:12\\n  --> src/db/labels.rs:22:12\\n   --> src/db/records.rs:11:12\\n  --> src/db/relations.rs:38:12\\n --> src/identity.rs:8:12\\n  --> src/identity.rs:17:12\\n  --> src/identity.rs:72:4\\n  --> src/identity.rs:91:4\\n   --> src/record_store.rs:315:15\\n   --> src/record_store.rs:355:12\\n   --> src/record_store.rs:689:12\\n --> src/storage_layout.rs:7:11\\n  --> src/storage_layout.rs:44:12\\n  --> src/storage_layout.rs:95:8\\n  --> src/workflow_policy.rs:91:9\\n   --> src/workflow_policy.rs:113:9\\n   --> src/commands/agent_factory.rs:129:24\\n   --> src/commands/agent_factory.rs:249:32\\n   --> src/commands/agent_factory.rs:251:12\\n   --> src/db/issues.rs:79:12\\n   --> src/db/records.rs:51:12\\n    --> src/main.rs:1136:1\\n    --> src/commands/agent_factory.rs:1467:9\\nwarning: this function has too many lines (118/100)\\n    --> src/commands/agent_factory.rs:1731:1\\nwarning: this function has too many lines (141/100)\\n    --> src/command\",\"truncated\":true}},\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-e723\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e723"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "bash -lc 'cargo clippy --all-targets -- -W clippy::too_many_lines 2>&1 | rg \"too many lines|-->\"'"
updated_at: "2026-06-13T23:02:05.138976634+00:00"
---

bash -lc 'cargo clippy --all-targets -- -W clippy::too_many_lines 2>&1 | rg "too many lines|-->"'

Command: bash -lc 'cargo clippy --all-targets -- -W clippy::too_many_lines 2>&1 | rg "too many lines|-->"'
Exit status: 0

Stdout summary (truncated):
  --> src/db/records.rs:21:59
warning: this function has too many lines (187/100)
   --> src/db/mod.rs:226:5
warning: this function has too many lines (210/100)
   --> src/db/mod.rs:457:5
warning: this function has too many lines (119/100)
   --> src/workflow_policy.rs:883:1
warning: this function has too many lines (124/100)
    --> src/workflow_policy.rs:1010:1
    --> tests/cli_integration.rs:1703:23
    --> tests/cli_integration.rs:1827:19
    --> tests/cli_integration.rs:1880:19
    --> tests/cli_integration.rs:1921:19
    --> tests/cli_integration.rs:1944:19
    --> tests/cli_integration.rs:1975:19
    --> tests/cli_integration.rs:2001:23
    --> tests/cli_integration.rs:3805:19
    --> tests/cli_integration.rs:3835:19
    --> tests/cli_integration.rs:7150:19
    --> tests/cli_integration.rs:7193:19
    --> tests/cli_integration.rs:7244:19
    --> tests/cli_integration.rs:7288:19
    --> tests/cli_integration.rs:7305:19
    --> tests/cli_integration.rs:7681:19
    --> tests/cli_integration.rs:7774:19
    --> tests/cli_integration.rs:9445:19
    --> tests/cli_integration.rs:9723:19
warning: this function has too many lines (114/100)
   --> tests/cli_integration.rs:875:1
warning: this function has too many lines (102/100)
    --> tests/cli_integration.rs:3417:1
warning: this function has too many lines (115/100)
    --> tests/cli_integration.rs:6343:1
warning: this function has too many lines (251/100)
    --> tests/cli_integration.rs:6474:1
    --> tests/cli_integration.rs:6685:31
warning: this function has too many lines (294/100)
    --> tests/cli_integration.rs:6836:1
warning: this function has too many lines (131/100)
    --> tests/cli_integration.rs:8477:1
warning: this function has too many lines (210/100)
    --> tests/cli_integration.rs:8655:1
    --> tests/cli_integration.rs:8724:32
    --> tests/cli_integration.rs:8755:53
    --> tests/cli_integration.rs:8755:66
    --> tests/cli_integration.rs:8782:62
    --> tests/cli_integration.rs:8787:37
    --> tests/cli_integration.rs:8787:49
    --> tests/cli_integration.rs:8809:33
    --> tests/cli_integration.rs:8809:58
warning: this function has too many lines (202/100)
    --> tests/cli_integration.rs:8888:1
warning: this function has too many lines (173/100)
     --> tests/cli_integration.rs:10232:1
    --> src/commands/agent_factory.rs:1535:12
    --> src/commands/agent_factory.rs:1653:8
    --> src/commands/agent_factory.rs:1705:4
    --> src/commands/agent_factory.rs:1731:8
    --> src/commands/agent_factory.rs:2114:8
    --> src/commands/agent_factory.rs:2133:8
  --> src/commands/comment.rs:21:8
  --> src/commands/create.rs:76:12
  --> src/commands/create.rs:82:8
   --> src/commands/create.rs:168:8
   --> src/commands/create.rs:223:4
 --> src/commands/delete.rs:9:8
 --> src/commands/label.rs:7:8
  --> src/commands/label.rs:50:8
  --> src/commands/projection.rs:26:8
 --> src/commands/relate.rs:7:8
  --> src/commands/relate.rs:66:8
  --> src/commands/workflow.rs:15:7
   --> src/commands/workflow.rs:379:8
   --> src/commands/workflow.rs:551:8
   --> src/commands/workflow.rs:582:4
 --> src/db/comments.rs:9:12
  --> src/db/dependencies.rs:53:12
   --> src/db/issues.rs:50:12
  --> src/db/labels.rs:22:12
   --> src/db/records.rs:11:12
  --> src/db/relations.rs:38:12
 --> src/identity.rs:8:12
  --> src/identity.rs:17:12
  --> src/identity.rs:72:4
  --> src/identity.rs:91:4
   --> src/record_store.rs:315:15
   --> src/record_store.rs:355:12
   --> src/record_store.rs:689:12
 --> src/storage_layout.rs:7:11
  --> src/storage_layout.rs:44:12
  --> src/storage_layout.rs:95:8
  --> src/workflow_policy.rs:91:9
   --> src/workflow_policy.rs:113:9
   --> src/commands/agent_factory.rs:129:24
   --> src/commands/agent_factory.rs:249:32
   --> src/commands/agent_factory.rs:251:12
   --> src/db/issues.rs:79:12
   --> src/db/records.rs:51:12
    --> src/main.rs:1136:1
    --> src/commands/agent_factory.rs:1467:9
warning: this function has too many lines (118/100)
    --> src/commands/agent_factory.rs:1731:1
warning: this function has too many lines (141/100)
    --> src/command

Stderr summary:
(none)

