---
schema: atelier.review
schema_version: 2
id: atelier-yul7
title: finding on atelier-1oel @4c531865
target: atelier-1oel
identity: GPT-6.1 Sol independent reviewer
commit: 4c531865a2b70e6b5f3c29cf19c8a8cd5e9a8143
created_at: 2026-11-11T03:26:00.854105385Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: next/crates/atelier-cli/src/output.rs:482 status on the base checkout omits both new fields for held items. Reproduced with the branch binary in ~/.cache/atelier-qr5s/1oel/review/status-xi0spqf7: after three changes verdicts, t1 on the item branch sees round 3 and the escalation notice; the same caller on main sees only demo-ab12 (changes requested): Say hello, with neither the round nor atelier escalate. Done when requires the implementer to see both and does not restrict status to the item branch. The Entry already carries round and unapproved; render them in the held-item overview too, and cover base-checkout status in the scenario test.
