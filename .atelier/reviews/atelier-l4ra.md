---
schema: atelier.review
schema_version: 1
id: atelier-l4ra
title: 'crates/atelier-core/src/lib.rs:379-404 and 506-523 (change_type/for_type): retyping work or a mission to decision maps Done when (or Success criteria) into the Decision section. Reproduced: `issue update <task> --issue-type decision` produced ''## Decision\n\nDone when was not specified.'', and decision.recorded then passes, so decide succeeds without any decision being recorded. This is exactly the migration path for existing ''Decide ...'' tasks such as atelier-28mz, so the new gate is defeated on its main use. The reverse (decision -> task) silently drops Options and Context and turns the Decision into Done when. Suggested fix: when converting into a decision, leave Decision empty (carry Done when/Success criteria into Context or Options instead); when converting out, keep Context and preserve Options/Decision text somewhere (or refuse retyping that would drop them); test both directions.'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:08.972811367Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-core/src/lib.rs:379-404 and 506-523 (change_type/for_type): retyping work or a mission to decision maps Done when (or Success criteria) into the Decision section. Reproduced: `issue update <task> --issue-type decision` produced '## Decision\n\nDone when was not specified.', and decision.recorded then passes, so decide succeeds without any decision being recorded. This is exactly the migration path for existing 'Decide ...' tasks such as atelier-28mz, so the new gate is defeated on its main use. The reverse (decision -> task) silently drops Options and Context and turns the Decision into Done when. Suggested fix: when converting into a decision, leave Decision empty (carry Done when/Success criteria into Context or Options instead); when converting out, keep Context and preserve Options/Decision text somewhere (or refuse retyping that would drop them); test both directions.
