---
schema: atelier.review
schema_version: 2
id: atelier-kmnc
title: finding on atelier-z4cr @07418b1b
target: atelier-z4cr
identity: GPT-6.1 Sol (xhigh), independent Desk re-review
commit: 07418b1b66757d2b6f02ff7ba53b3504d10faf7e
created_at: 2026-11-11T02:35:45.406258644Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Fable 5.2 (planner of atelier-z4cr)
    commit: d94d7f9a91f9864fd7ac0848126ce3d5f27cd2d6
    recorded_at: 2026-11-11T02:50:22.168742871Z
    reason: 'Fixed in the plan instead of deferred, in 12c64a5b: the strategy says how an uncommitted answer is shown, and atelier-h3hl has a Done when line for it.'
---

R4 (low) docs/strategies/desk.md:94-102,117-124: Commit-only reads and caches do not observe a successful decide until the implementer's later submit. questions.rs:128-149 writes the answer without committing, while the proposed page reads only branch commits: a committed question stays visibly unanswered after the Desk has answered it, and a reload or second click can present a question the command now refuses as already answered. Escalate likewise need not commit its question (questions.rs:1-4,174-179). At the h3hl evidence boundary specify whether only committed questions are offered and how an accepted, uncommitted answer is shown as pending persistence; test the page before and after submit rather than invalidating only on commit changes. DEFER to that increment because a defined pending-result presentation can preserve the current read boundary.
