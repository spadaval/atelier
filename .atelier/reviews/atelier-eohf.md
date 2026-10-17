---
schema: atelier.review
schema_version: 1
id: atelier-eohf
title: 'crates/atelier-cli/src/commands/session.rs:160 and crates/atelier-cli/src/commands/merge.rs:287 — the new land_policy_invalid / merge_policy_invalid codes are added as anyhow context over atelier_workflow::load, whose error already carries workflow_config_parse_error. Reproduced in a fixture (append ''bogus_key: ['' to .atelier/workflow.yaml, commit, run land and merge --check): ''Error: land_policy_invalid: candidate workflow policy is invalid; repair it before landing / Caused by: error: workflow_config_parse_error ...''. That is two codes on one error, which the rule this change adds at docs/product/human-cli-output.md:247-251 forbids; before this change the error carried only the inner code. The same run states the failure three times (two warnings, then the error), and land''s warning says ''fix .atelier/workflow.yaml before merging''. There is no Next: step. Fix: drop the outer code, since the inner code already names the check, or replace the inner one; then give one Next: step.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:41.603594056Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/session.rs:160 and crates/atelier-cli/src/commands/merge.rs:287 — the new land_policy_invalid / merge_policy_invalid codes are added as anyhow context over atelier_workflow::load, whose error already carries workflow_config_parse_error. Reproduced in a fixture (append 'bogus_key: [' to .atelier/workflow.yaml, commit, run land and merge --check): 'Error: land_policy_invalid: candidate workflow policy is invalid; repair it before landing / Caused by: error: workflow_config_parse_error ...'. That is two codes on one error, which the rule this change adds at docs/product/human-cli-output.md:247-251 forbids; before this change the error carried only the inner code. The same run states the failure three times (two warnings, then the error), and land's warning says 'fix .atelier/workflow.yaml before merging'. There is no Next: step. Fix: drop the outer code, since the inner code already names the check, or replace the inner one; then give one Next: step.
