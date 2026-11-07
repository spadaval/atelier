---
schema: atelier.review
schema_version: 2
id: atelier-9gac
title: 'Low: crates/atelier-app/src/mission_review.rs:871-886 and docs/product/mission-reviews.md (Validation) The doc says each validation run shows ''the code commits since, and the work delivered after it'', but code_commits_since is set only when one of the run''s receipts still decides a mark, and later_work only when that count is above zero. On atelier-r6bt the 15:06 claude-implementer run at e6dcd73b lists no later work although rx22, acrt and nexd were delivered after it. Either compute commits/work for every run or state the condition in the doc.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:27.344116988Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    recorded_at: 2026-10-18T07:07:55.847132202Z
    resolution_commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    reason: Fixed in 8151a566
    migrated_from: atelier-vpwg
---

Low: crates/atelier-app/src/mission_review.rs:871-886 and docs/product/mission-reviews.md (Validation) The doc says each validation run shows 'the code commits since, and the work delivered after it', but code_commits_since is set only when one of the run's receipts still decides a mark, and later_work only when that count is above zero. On atelier-r6bt the 15:06 claude-implementer run at e6dcd73b lists no later work although rx22, acrt and nexd were delivered after it. Either compute commits/work for every run or state the condition in the doc.
