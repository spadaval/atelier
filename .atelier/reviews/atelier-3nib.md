---
schema: atelier.review
schema_version: 1
id: atelier-3nib
title: 'low: crates/atelier-cli/src/commands/merge_sync.rs:381-406. current_on_branch is the aggregate approval_state (Current if any approval is current), but it gates each approval in the loop, so ''Approval kept'' can name an approval that review.current would not accept. Reproduced: approve, code commit, main takes the branch with --no-ff, sync (stale), record a second approval, sync again: ''Approval kept: atelier-gows, atelier-vjd7 still covers the branch.'' although the first predates the fork point. The verdict and review.current still agree (both pass), so impact is a misleading ID list only. Fix: judge the fork-point condition per approval (expose a per-record current_on_branch from review_gate, or filter kept by it).'
target: atelier-fngz
identity: Independent reviewer (Claude subagent)
commit: 97e785a471c9a95cea99441e15682e56a1c90826
created_at: 2026-10-22T23:43:27.225588483Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/merge_sync.rs:381-406. current_on_branch is the aggregate approval_state (Current if any approval is current), but it gates each approval in the loop, so 'Approval kept' can name an approval that review.current would not accept. Reproduced: approve, code commit, main takes the branch with --no-ff, sync (stale), record a second approval, sync again: 'Approval kept: atelier-gows, atelier-vjd7 still covers the branch.' although the first predates the fork point. The verdict and review.current still agree (both pass), so impact is a misleading ID list only. Fix: judge the fork-point condition per approval (expose a per-record current_on_branch from review_gate, or filter kept by it).
