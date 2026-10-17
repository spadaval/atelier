---
schema: atelier.review
schema_version: 1
id: atelier-2tbo
title: 'crates/atelier-app/src/record_history.rs:65-72: the walk follows HEAD''s first parents, so when HEAD is a work branch that merged the base in, every base commit since the branch point collapses into that one merge version. Reproduced in a scratch repo: branch feature from main; on main start the mission, then link a late item; on feature merge main; ''debrief <mission>'' on main reports ''scope added <late> after the start'', on feature it reports Started at the merge commit and Deviations (none). The same collapse hides reopened items and moves the start, and for a work branch cut before the mission existed it hides the mission''s whole history. The output gives no cue, so a debrief run from the agent''s usual checkout (an open mission, or a done one before the branch merges) silently drops deviations. kgcr from master is unaffected. Fix: walk the first-parent line of the mission''s base branch (workflow branch_base, or the merge-base with it) and then HEAD''s own first parents, or at least print a cue when HEAD''s first-parent line merged the base after the mission started, and document the limit in docs/product/mission-reviews.md.'
target: atelier-rx22
identity: Independent reviewer 2 (Claude subagent)
commit: 518ba162bcf5a01d3faa37694526c38836d0474b
created_at: 2026-10-17T19:45:10.255381007Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_history.rs:65-72: the walk follows HEAD's first parents, so when HEAD is a work branch that merged the base in, every base commit since the branch point collapses into that one merge version. Reproduced in a scratch repo: branch feature from main; on main start the mission, then link a late item; on feature merge main; 'debrief <mission>' on main reports 'scope added <late> after the start', on feature it reports Started at the merge commit and Deviations (none). The same collapse hides reopened items and moves the start, and for a work branch cut before the mission existed it hides the mission's whole history. The output gives no cue, so a debrief run from the agent's usual checkout (an open mission, or a done one before the branch merges) silently drops deviations. kgcr from master is unaffected. Fix: walk the first-parent line of the mission's base branch (workflow branch_base, or the merge-base with it) and then HEAD's own first parents, or at least print a cue when HEAD's first-parent line merged the base after the mission started, and document the limit in docs/product/mission-reviews.md.
