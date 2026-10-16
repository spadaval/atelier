---
schema: atelier.review
schema_version: 1
id: atelier-r3ca
title: 'crates/atelier-cli/src/commands/bundle.rs:747-761: updated records skip the per-record checks atelier issue update runs (lint::issue_findings and validate_issue_record_against_workflow after a --body change, and mission_strategy''s check that the file exists). Reproduced: (1) on a ready (todo) epic, ''issue update --body'' without Proof is refused (''Ready epic requires Proof''), but the same bundle update applies and atelier check then fails; (2) a mission update with strategy docs/nope.md applies and check reports ''Mission strategy docs/nope.md does not exist''. The staged RecordIndex::load does not catch either. Bundle create has the same gaps, but for updates of existing, already-valid records this weakens validation compared with the existing update path, which the constraints rule out. Fix: run the issue update record checks (lint::issue_findings scoped to the updated records, strategy file existence) on each updated record in the stage before installing it, and test the ready-epic case.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a9234343ba772ab7e86fa682752f752f73f4af7e
created_at: 2026-10-16T00:40:13.236572557Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/bundle.rs:747-761: updated records skip the per-record checks atelier issue update runs (lint::issue_findings and validate_issue_record_against_workflow after a --body change, and mission_strategy's check that the file exists). Reproduced: (1) on a ready (todo) epic, 'issue update --body' without Proof is refused ('Ready epic requires Proof'), but the same bundle update applies and atelier check then fails; (2) a mission update with strategy docs/nope.md applies and check reports 'Mission strategy docs/nope.md does not exist'. The staged RecordIndex::load does not catch either. Bundle create has the same gaps, but for updates of existing, already-valid records this weakens validation compared with the existing update path, which the constraints rule out. Fix: run the issue update record checks (lint::issue_findings scoped to the updated records, strategy file existence) on each updated record in the stage before installing it, and test the ready-epic case.
