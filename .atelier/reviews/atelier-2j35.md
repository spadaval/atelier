---
schema: atelier.review
schema_version: 2
id: atelier-2j35
title: finding on atelier-8bh4 @70b45d46
target: atelier-8bh4
identity: independent reviewer subagent (claude-opus-5-5), atelier-8bh4
commit: 70b45d464142a56eba268b9270dc764ea7e13de8
created_at: 2026-11-07T09:39:49.023096484Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/work_classification.rs:100 human_children_note always tells the agent to run `atelier issue update <child> --no-parent --issue-type loose_end`, but astromech, cosplay-studio, demiurge and life-on-rails declare no loose_end, so under the shared binary the advice that start and status print for a needs-human child is refused there (reproduced in a scratch repo with the old starter policy: 'Invalid issue type loose_end'), where the old note's --no-parent + blocked_by advice worked. A regression in the four repos the brief requires to keep working. Fix: add --issue-type loose_end and the loose-end wording only when the policy declares loose_end (both callers have the policy); otherwise keep the earlier advice.
