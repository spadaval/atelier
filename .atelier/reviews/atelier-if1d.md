---
schema: atelier.review
schema_version: 1
id: atelier-if1d
title: 'crates/atelier-app/src/mission_review.rs:435-480: brief() copies debrief()''s scope walk (the ''linked'' filter at 122 and the child loop at 157) and crates/atelier-cli/src/commands/mission_review.rs:799-816 copies the debrief''s Mission Reviews Markdown line format verbatim. If either changes (scope ordering, nesting, event line shape), the brief and debrief silently diverge on the same mission. Fix: extract one scope_tree(index, mission_id) -> Vec<(Issue, Option<parent>)> used by both views, and one mission_event_line(event) for the Markdown row.'
target: atelier-acrt
identity: Independent reviewer (Claude subagent)
commit: 5c858f437d2b7227bde75d9eaa3c650240b510b7
created_at: 2026-10-17T19:52:25.007411282Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/mission_review.rs:435-480: brief() copies debrief()'s scope walk (the 'linked' filter at 122 and the child loop at 157) and crates/atelier-cli/src/commands/mission_review.rs:799-816 copies the debrief's Mission Reviews Markdown line format verbatim. If either changes (scope ordering, nesting, event line shape), the brief and debrief silently diverge on the same mission. Fix: extract one scope_tree(index, mission_id) -> Vec<(Issue, Option<parent>)> used by both views, and one mission_event_line(event) for the Markdown row.
