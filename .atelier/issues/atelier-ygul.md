---
created_at: "2026-11-09T07:05:20.547021291+00:00"
id: "atelier-ygul"
issue_type: "task"
labels:
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-v3nm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Stop land refusing a done validation item whose planning approval predates unrelated code"
updated_at: "2026-11-09T17:52:27.791577367+00:00"
---

## Goal

Marking validation item atelier-v3nm done made `atelier land` refuse on master for every thread: findings.disposed said its three planning fix_now findings came after the current approval, though approval atelier-4102 at dc3a18e4 followed them. The likely cause is that for a done item land requires no code to change between the approval and the done commit (`approval_is_current_to`, crates/atelier-app/src/review_gate.rs), and the mission's own merges changed code after dc3a18e4 while the item waited on them. A validation or planning-only item that changes no code should not need a fresh approval because unrelated code merged first.

## Done when

- [ ] A done item whose own commits change no code is not refused by land because code elsewhere changed after its planning approval; a regression test reproduces the atelier-v3nm case (planning fix_now, approval after it, unrelated code merged, item marked done on the base).
- [ ] The refusal message, where it still applies, names the code change that made the approval stale rather than saying the findings came after the approval.

## Notes

- 2026-11-09 [note] 2026-11-09: Closing mission atelier-fl12 hit the same refusal: findings.disposed said 4 fix_now findings came after the current approval. The approval it meant was Sumeet's planning approval of the mission, atelier-op8x at 2a7a5f14; the findings were the items' own code-review findings (atelier-vjuy, atelier-lfkn, atelier-huh0 and others), each already resolved by the item's later approval. So land also judges a mission's rolled-up item findings against the mission's planning approval. The fix should cover both the done validation item and the closed mission.
