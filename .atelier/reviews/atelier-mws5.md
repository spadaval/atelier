---
schema: atelier.review
schema_version: 1
id: atelier-mws5
title: 'crates/atelier-app/src/decisions.rs:163: decided() requires a workflow policy, so in a repository without .atelier/workflow.yaml (load_issue_workflow_policy returns None) every decision record is ''not decided'': work decisions lists done records under ''Open Decision Records'', -q omits them, and a done newer record never marks the older one superseded (reproduced in a scratch repo by removing workflow.yaml). The fix range made is_current depend on decided(), extending this from supersession to currency. Low impact since init writes a policy and transitions need one. Fix when convenient: fall back to status == "done" when there is no policy, or treat a policy-less repo as unsupported here explicitly.'
target: atelier-fip8
identity: Independent reviewer 2 (Claude subagent)
commit: f7f42a1ebdacfa39dc585542543231ed95f72ecd
created_at: 2026-10-17T17:50:47.654993868Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/decisions.rs:163: decided() requires a workflow policy, so in a repository without .atelier/workflow.yaml (load_issue_workflow_policy returns None) every decision record is 'not decided': work decisions lists done records under 'Open Decision Records', -q omits them, and a done newer record never marks the older one superseded (reproduced in a scratch repo by removing workflow.yaml). The fix range made is_current depend on decided(), extending this from supersession to currency. Low impact since init writes a policy and transitions need one. Fix when convenient: fall back to status == "done" when there is no policy, or treat a policy-less repo as unsupported here explicitly.
