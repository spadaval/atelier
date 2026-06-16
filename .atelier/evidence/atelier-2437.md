---
created_at: "2026-06-16T17:43:39.830818019+00:00"
id: "atelier-2437"
evidence_type: "validation"
captured_at: "2026-06-16T17:43:39.825227551+00:00"
command: "git show --stat --oneline db5bd17"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-jezn"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 368
    summary: "db5bd17 Remove routine export check guidance\n AGENTFACTORY.md                                    | 13 ++++----\n docs/architecture/quality/validation.md            | 14 ++++----\n .../export-check-reference-classification.md       | 39 ++++++++++++++++++++++\n docs/product/command-audit/index.md                |  1 +\n 4 files changed, 53 insertions(+), 14 deletions(-)\n"
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
    id: "atelier-jezn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "repo guidance diff removes routine export checks and adds classification"
updated_at: "2026-06-16T17:43:43.415765506+00:00"
---

repo guidance diff removes routine export checks and adds classification

Command: git show --stat --oneline db5bd17
Exit status: 0

Stdout summary:
db5bd17 Remove routine export check guidance
 AGENTFACTORY.md                                    | 13 ++++----
 docs/architecture/quality/validation.md            | 14 ++++----
 .../export-check-reference-classification.md       | 39 ++++++++++++++++++++++
 docs/product/command-audit/index.md                |  1 +
 4 files changed, 53 insertions(+), 14 deletions(-)

Stderr summary:
(none)

