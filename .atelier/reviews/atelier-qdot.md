---
schema: atelier.review
schema_version: 1
id: atelier-qdot
title: 'crates/atelier-cli/src/main.rs:1052-1072 — rewording clap rejections drops clap''s typo suggestion. Master: ''evidence list --result pas'' printed ''tip: a similar value exists: pass''; HEAD prints only ''Invalid result pas. Valid values: pass, fail, blocked, not-applicable''. Same loss for evidence attach <TARGET_KIND> and review --recommendation. The contract allows one sentence of guidance, so the suggestion (ContextKind::SuggestedValue) could become the hint. Low impact: the full valid list is still shown.'
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:31.245380805Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/main.rs:1052-1072 — rewording clap rejections drops clap's typo suggestion. Master: 'evidence list --result pas' printed 'tip: a similar value exists: pass'; HEAD prints only 'Invalid result pas. Valid values: pass, fail, blocked, not-applicable'. Same loss for evidence attach <TARGET_KIND> and review --recommendation. The contract allows one sentence of guidance, so the suggestion (ContextKind::SuggestedValue) could become the hint. Low impact: the full valid list is still shown.
