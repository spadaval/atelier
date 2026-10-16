---
schema: atelier.review
schema_version: 1
id: atelier-1k1s
title: 'crates/atelier-records/src/lib.rs:1432 and work_classification.rs:200: the decision body shape and needs-human classification are keyed on the literal type name ''decision'', not on the policy. A repository that already registered its own ''decision'' issue type under its work workflow (plausible for adopters filing design questions, the very motivation of this record) now fails every command: scratch repo with decision in work.applies_to and a Goal/Done when record -> `atelier check` and `work ready` both error ''Missing required issue body section Question''. Suggested follow-up: document the migration (retype or add Question) in workflow-configuration.md with an actionable error hint, or derive the shape from the decision workflow.'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:09.074544689Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-records/src/lib.rs:1432 and work_classification.rs:200: the decision body shape and needs-human classification are keyed on the literal type name 'decision', not on the policy. A repository that already registered its own 'decision' issue type under its work workflow (plausible for adopters filing design questions, the very motivation of this record) now fails every command: scratch repo with decision in work.applies_to and a Goal/Done when record -> `atelier check` and `work ready` both error 'Missing required issue body section Question'. Suggested follow-up: document the migration (retype or add Question) in workflow-configuration.md with an actionable error hint, or derive the shape from the decision workflow.
