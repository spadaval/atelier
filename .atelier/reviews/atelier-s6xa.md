---
schema: atelier.review
schema_version: 2
id: atelier-s6xa
title: finding on atelier-1oel @c990bcc9
target: atelier-1oel
identity: GPT-6.1 Sol independent reviewer
commit: c990bcc92d1fdbc2261a99d3fb8dc96a4b4ee4c6
created_at: 2026-11-11T03:34:26.597219899Z
event:
  event: finding
  recommendation: fix_now
---

R3 low: next/crates/atelier-app/src/view.rs:120 The escalation notice is restricted to ChangesRequested, rather than the stated condition that three rounds have ended without an approve verdict. With the branch binary, started and submitted demo-ab12, ended rounds 1, 2 and 3 with changes verdicts, answered their carried finding, and submitted round 4. The implementer sees the required notice immediately after round 3, then only Status: in review, round 4 and the reviewer handoff, with no escalation notice, although no approval or escalation occurred. Reproduced in ~/.cache/atelier-qr5s/1oel/review/round4-uh8gmpq7 (its history holds the three changes verdicts and round-4 submit); the new scenario exercises round 4 but does not assert the notice. The sixth Done when line and the product Commands paragraph say once three rounds have ended, without restricting that promise to changes-requested status. Preserve that reminder for the implementer during subsequent unapproved review rounds and test both item/base checkouts, or have the accountable orchestrator explicitly resolve/narrow the status scope. Note that atelier escalate itself is currently refused while InReview, so any resolution must keep the printed instruction actionable. Keep submit permissive as required; this asks for the promised status guidance, not enforcement.
