---
schema: atelier.review
schema_version: 2
id: atelier-qdot
title: 'crates/atelier-cli/src/main.rs:1052-1072 — rewording clap rejections drops clap''s typo suggestion. Master: ''evidence list --result pas'' printed ''tip: a similar value exists: pass''; HEAD prints only ''Invalid result pas. Valid values: pass, fail, blocked, not-applicable''. Same loss for evidence attach <TARGET_KIND> and review --recommendation. The contract allows one sentence of guidance, so the suggestion (ContextKind::SuggestedValue) could become the hint. Low impact: the full valid list is still shown.'
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:31.245380805Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 5f00943ce60e1c6259d43098c548683140d5b06a
    recorded_at: 2026-10-17T09:55:35.200520705Z
    resolution_commit: efee9e953d9329f07a6c463676efc8b47c0b7b79
    reason: 'Fixed now: clap''s suggestion is kept as the guidance sentence.'
    migrated_from: atelier-tmzo
---

crates/atelier-cli/src/main.rs:1052-1072 — rewording clap rejections drops clap's typo suggestion. Master: 'evidence list --result pas' printed 'tip: a similar value exists: pass'; HEAD prints only 'Invalid result pas. Valid values: pass, fail, blocked, not-applicable'. Same loss for evidence attach <TARGET_KIND> and review --recommendation. The contract allows one sentence of guidance, so the suggestion (ContextKind::SuggestedValue) could become the hint. Low impact: the full valid list is still shown.
