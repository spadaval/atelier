---
schema: atelier.review
schema_version: 1
id: atelier-91va
title: 'crates/atelier-app/src/review_gate.rs:417: the error also fires outside the work-branch rebase case, e.g. from land''s Historical/CurrentClaims evaluation (crates/atelier-cli/src/commands/session.rs:384-399) for another issue whose review records came from a squash-merged or cherry-picked branch, or after a commit --amend; there ''reset the branch'' points at the wrong branch. The ''usually'' hedge keeps the message honest and those paths were already failing with this error before the change, so no action needed now.'
target: atelier-j48s
identity: Independent reviewer (Claude subagent)
commit: 06f62abff594387257edc29e1fd681eb9afacd15
created_at: 2026-10-16T18:47:16.076759071Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/review_gate.rs:417: the error also fires outside the work-branch rebase case, e.g. from land's Historical/CurrentClaims evaluation (crates/atelier-cli/src/commands/session.rs:384-399) for another issue whose review records came from a squash-merged or cherry-picked branch, or after a commit --amend; there 'reset the branch' points at the wrong branch. The 'usually' hedge keeps the message honest and those paths were already failing with this error before the change, so no action needed now.
