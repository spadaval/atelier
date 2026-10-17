---
schema: atelier.review
schema_version: 1
id: atelier-nl8o
title: 'crates/atelier-app/src/use_cases.rs:20 with workflow status/type validation during load — pre-existing: an issue whose status or issue_type the workflow rejects fails every query with a field block naming the absolute .atelier/workflow.yaml path and a reference, never the issue''s record path, yet the recovery says ''fix every named record file''. Reproduced with status ''zzz'' and issue_type ''zzz'' on base-b3ai: no .atelier/issues/base-b3ai.md anywhere in issue list or check output, and the path is absolute where other errors are repo-relative. Not worsened by this change.'
target: atelier-x68o
identity: Independent reviewer (Claude subagent)
commit: dfe2703061fdcb63795e63352bf4895baf2a08dd
created_at: 2026-10-17T08:52:11.698362032Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/use_cases.rs:20 with workflow status/type validation during load — pre-existing: an issue whose status or issue_type the workflow rejects fails every query with a field block naming the absolute .atelier/workflow.yaml path and a reference, never the issue's record path, yet the recovery says 'fix every named record file'. Reproduced with status 'zzz' and issue_type 'zzz' on base-b3ai: no .atelier/issues/base-b3ai.md anywhere in issue list or check output, and the path is absolute where other errors are repo-relative. Not worsened by this change.
