---
schema: atelier.review
schema_version: 1
id: atelier-tcuc
title: 'docs/product/human-cli-output.md:237-246 — Done when requires one rule that says whether an error carries a code and is applied everywhere. As written, ''a named check refuses the command'' and ''an error about the command''s own input'' overlap, so the rule cannot decide common cases, and observed output contradicts it. merge_mission (merge.rs:242) is coded, though the refusal is about the reference the user passed. ''Transition ''close'' is not available from status ''todo'' for issue X'', a workflow-policy check, has no code. ''Invalid record file X: ...'' (atelier-app/src/record_index.rs:89), a check of records, has no code. check''s own refusal ''Check failed with N findings'' has no code. An agent or gate cannot predict from the doc which refusals it can match by code. Fix: make the criterion decidable, for example code if and only if the refusal comes from an enumerated gate or check family, and bring the listed cases into line or name them as exceptions.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:42.244263452Z
event:
  event: finding
  recommendation: fix_now
---

docs/product/human-cli-output.md:237-246 — Done when requires one rule that says whether an error carries a code and is applied everywhere. As written, 'a named check refuses the command' and 'an error about the command's own input' overlap, so the rule cannot decide common cases, and observed output contradicts it. merge_mission (merge.rs:242) is coded, though the refusal is about the reference the user passed. 'Transition 'close' is not available from status 'todo' for issue X', a workflow-policy check, has no code. 'Invalid record file X: ...' (atelier-app/src/record_index.rs:89), a check of records, has no code. check's own refusal 'Check failed with N findings' has no code. An agent or gate cannot predict from the doc which refusals it can match by code. Fix: make the criterion decidable, for example code if and only if the refusal comes from an enumerated gate or check family, and bring the listed cases into line or name them as exceptions.
