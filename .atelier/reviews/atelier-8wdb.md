---
schema: atelier.review
schema_version: 2
id: atelier-8wdb
title: finding on atelier-7tul @6715d534
target: atelier-7tul
identity: codex-plan-rereviewer (gpt-6.1-sol)
commit: 6715d534869f0ae784e7e08b8aefd3c8280e24fb
created_at: 2026-11-09T02:12:19.422208168Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-7tul.md:44 Installing the new binary only after this item merges leaves the policy switch unable to merge with the installed binary. merge.rs:901-911 strictly parses the candidate policy and refuses unsupported validators even while running the committed base policy for trusted checks. In a disposable repository with a valid base policy and a candidate using ci.passed, the installed atelier merge --check exits 1 with workflow_config_invalid_validator for ci.passed. The new integration test runs the new binary and does not prove that the old installed binary can perform this transition. Build and use the reviewed branch binary for this item\u0027s merge (target/debug/atelier, or install before merge), retaining the old base policy and script as the fix correctly requires; install the resulting binary before land and subsequent merges. The original deletion hazard atelier-0908 and pre-merge acceptance defect atelier-8fki are corrected, but this newly introduced bootstrap instruction blocks Ready Work.
