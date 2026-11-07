---
schema: atelier.review
schema_version: 2
id: atelier-k79z
title: 'crates/atelier-cli/src/commands/mission_review.rs:546 vs 959-990: the two forms order sections differently. The Markdown puts Mission Reviews before the decision log; the HTML puts Decisions before Mission reviews. The Markdown Open Work also mixes planned items and review follow-ups in ID order, while the HTML groups planned, From review and Dropped. Done when requires the Markdown to follow the same structure. docs/product/mission-reviews.md lists the sections but omits Mission reviews. Fix: use one order in both forms, group the Markdown Open Work as the HTML does, and name Mission reviews in the doc.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:49.940647589Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    recorded_at: 2026-10-17T22:02:49.158126044Z
    resolution_commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    reason: Fixed in ff8838b691bc4425883a80bc6bdc4128f53579dd with tests; see the commit message
    migrated_from: atelier-exvs
---

crates/atelier-cli/src/commands/mission_review.rs:546 vs 959-990: the two forms order sections differently. The Markdown puts Mission Reviews before the decision log; the HTML puts Decisions before Mission reviews. The Markdown Open Work also mixes planned items and review follow-ups in ID order, while the HTML groups planned, From review and Dropped. Done when requires the Markdown to follow the same structure. docs/product/mission-reviews.md lists the sections but omits Mission reviews. Fix: use one order in both forms, group the Markdown Open Work as the HTML does, and name Mission reviews in the doc.
