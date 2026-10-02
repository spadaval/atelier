---
schema: atelier.review
schema_version: 1
id: atelier-9yms
title: 'G14: land accepts staged canonical record changes when working bytes match HEAD. In /tmp/session-review-canonical-index-yt8wr3id/repo, stage an issue change and restore its working file to HEAD; git status reports MM .atelier/issues/atelier-svma.md but land passes at 5b5dca62c10736c4a13b19f56d350f71e8cadd96. S6 requires committed canonical records and fresh index checks. Refuse canonical status paths, including rename sources, while permitting honest unfinished ordinary code. This also reproduces at aab00773 and is a preexisting invariant gap, not a snapshot regression.'
target: atelier-1ioo
identity: session-reviewer
commit: 5b5dca62c10736c4a13b19f56d350f71e8cadd96
created_at: 2026-10-02T01:31:27.343154858Z
event:
  event: finding
  recommendation: fix_now
---

G14: land accepts staged canonical record changes when working bytes match HEAD. In /tmp/session-review-canonical-index-yt8wr3id/repo, stage an issue change and restore its working file to HEAD; git status reports MM .atelier/issues/atelier-svma.md but land passes at 5b5dca62c10736c4a13b19f56d350f71e8cadd96. S6 requires committed canonical records and fresh index checks. Refuse canonical status paths, including rename sources, while permitting honest unfinished ordinary code. This also reproduces at aab00773 and is a preexisting invariant gap, not a snapshot regression.
