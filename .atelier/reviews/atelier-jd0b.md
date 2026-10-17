---
schema: atelier.review
schema_version: 1
id: atelier-jd0b
title: 'docs/product/human-cli-output.md:252-253 — the new rule prescribes ''<code>: <failure>. Next: <step>'', but only 6 of about 103 coded error strings in crates/*/src use ''Next:''. None of the seven codes this change introduces does: merge.rs:213 ''; unlink it before merging'', :229 ''; check out the work branch first'', :242, :245 '', so run ...'', :269, :287, and session.rs:160. So the contract the change writes contradicts the errors the same change writes, and output_terms.rs only asserts ''Next:'' count <= 1, so a missing step passes. Fix: make the errors this change touched follow the stated form, and say in the doc whether the form binds existing errors now or is a target for later items.'
target: atelier-91w1
identity: Independent reviewer (Claude subagent)
commit: b3a32046fd1d563706e20e9f7e672b687b8a11db
created_at: 2026-10-17T08:20:41.919086164Z
event:
  event: finding
  recommendation: fix_now
---

docs/product/human-cli-output.md:252-253 — the new rule prescribes '<code>: <failure>. Next: <step>', but only 6 of about 103 coded error strings in crates/*/src use 'Next:'. None of the seven codes this change introduces does: merge.rs:213 '; unlink it before merging', :229 '; check out the work branch first', :242, :245 ', so run ...', :269, :287, and session.rs:160. So the contract the change writes contradicts the errors the same change writes, and output_terms.rs only asserts 'Next:' count <= 1, so a missing step passes. Fix: make the errors this change touched follow the stated form, and say in the doc whether the form binds existing errors now or is a target for later items.
