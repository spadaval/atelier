---
schema: atelier.review
schema_version: 1
id: atelier-n90q
title: 'low: crates/atelier-cli/src/commands/git.rs:152-199 land''s approvals list takes every Approval/TrivialExemption event for a done owner, with no independence check and no open fix_now finding check, while merge''s review.current requires an independent identity and the finding gates separately. A self-approval pinned before a close commit therefore covers a post-pin done claim in land though merge would refuse the approval. Pre-existing for commits at or before the pin; now extended to claims after it. Also, an in-scope child can be closed by status-only commit directly on the base after the owner merged (while the owner''s done is still unpushed), which the rule permits by design. Residual risk, not a break of the stated rule; consider reusing merge''s candidate filter in a follow-up.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: f7e1cefa82d568b23f9b06cfc193ab350def07c1
created_at: 2026-10-22T21:12:53.321602124Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/git.rs:152-199 land's approvals list takes every Approval/TrivialExemption event for a done owner, with no independence check and no open fix_now finding check, while merge's review.current requires an independent identity and the finding gates separately. A self-approval pinned before a close commit therefore covers a post-pin done claim in land though merge would refuse the approval. Pre-existing for commits at or before the pin; now extended to claims after it. Also, an in-scope child can be closed by status-only commit directly on the base after the owner merged (while the owner's done is still unpushed), which the rule permits by design. Residual risk, not a break of the stated rule; consider reusing merge's candidate filter in a follow-up.
