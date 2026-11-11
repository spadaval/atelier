---
schema: atelier.review
schema_version: 2
id: atelier-w2rh
title: finding on atelier-t747 @753b046c
target: atelier-t747
identity: GPT-6.1 Sol independent planning reviewer
commit: 753b046cec958c26e0504daaf8cb680ba44f181c
created_at: 2026-11-11T04:33:40.720432227Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-t747.md:45,51 (753b046c): medium, FIX NOW. The plan authorizes a second batch of three skill-arm runs per model after rewording harmful text. The owner's limit for this increment is three runs per cell and no more; line 45 expressly makes an exception, while the expected 60+18+42 runs at line 70 omits it. Thus a worker following the plan can spend additional Opus/Sol runs beyond the authorized experiment. Remove the rewording rerun exception and settle the harm policy within the three-run budget: cut optional harmful text, preserve fixed rules and report any unresolved harm or need for a separately authorized experiment. Replacement of genuinely invalid attempts should remain explicit and distinct from additional valid samples.
