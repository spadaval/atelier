---
schema: atelier.review
schema_version: 1
id: atelier-63zh
title: 'low: crates/atelier-cli/src/commands/git.rs:305-311 BaseReviews::new takes every base exemption whose base matches and never checks the CLI''s ''recorded on the base, for a commit the base already holds'' rule (review.rs exempt_base). A hand-written record pinning an unmerged side commit, committed on main before a raw ''git merge side'', lets land pass (verified in scratch). No action because land only judges commits already on the base, and ''review trivial --base <side commit>'' after the raw merge is accepted anyway. The result is the same one exemption per commit with a self-declared identity, so no second standard. The docs sentence describes the CLI only.'
target: atelier-j8kg
identity: Independent reviewer (Claude subagent)
commit: fb339c94ffc10b07cc2befb139d0ac322200fbe1
created_at: 2026-10-22T23:27:33.297478445Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-cli/src/commands/git.rs:305-311 BaseReviews::new takes every base exemption whose base matches and never checks the CLI's 'recorded on the base, for a commit the base already holds' rule (review.rs exempt_base). A hand-written record pinning an unmerged side commit, committed on main before a raw 'git merge side', lets land pass (verified in scratch). No action because land only judges commits already on the base, and 'review trivial --base <side commit>' after the raw merge is accepted anyway. The result is the same one exemption per commit with a self-declared identity, so no second standard. The docs sentence describes the CLI only.
