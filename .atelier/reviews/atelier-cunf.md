---
schema: atelier.review
schema_version: 1
id: atelier-cunf
title: 'crates/atelier-cli/src/commands/session.rs:219 and the bail message at :353: the filter path.starts_with(".atelier") also matches policy and config files (.atelier/workflow.yaml, .atelier/config.toml), but the error says the branch ''changes Atelier records''. Reproduced: a feature branch whose only change is a comment in .atelier/workflow.yaml fails with ''changes Atelier records since main: .atelier/workflow.yaml''. Refusing an untraceable policy change is defensible and matches the docs (''changes nothing under .atelier/''), but the wording disagrees with the record''s ''changes records'' and with is_record_path in git_freshness.rs. Suggested fix: pick one rule. Either word the error as ''Atelier tracker files'' to match the docs, or filter with git_freshness::is_record_path plus an explicit policy-file rule, and cover the policy-only case in the test.'
target: atelier-fxbg
identity: Independent reviewer (Claude subagent)
commit: e4e1b6e78c9291ecbd3923b465f2cfd7f2e4a7a9
created_at: 2026-10-15T22:59:09.249407368Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/session.rs:219 and the bail message at :353: the filter path.starts_with(".atelier") also matches policy and config files (.atelier/workflow.yaml, .atelier/config.toml), but the error says the branch 'changes Atelier records'. Reproduced: a feature branch whose only change is a comment in .atelier/workflow.yaml fails with 'changes Atelier records since main: .atelier/workflow.yaml'. Refusing an untraceable policy change is defensible and matches the docs ('changes nothing under .atelier/'), but the wording disagrees with the record's 'changes records' and with is_record_path in git_freshness.rs. Suggested fix: pick one rule. Either word the error as 'Atelier tracker files' to match the docs, or filter with git_freshness::is_record_path plus an explicit policy-file rule, and cover the policy-only case in the test.
