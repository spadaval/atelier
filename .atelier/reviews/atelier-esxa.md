---
schema: atelier.review
schema_version: 2
id: atelier-esxa
title: finding on atelier-zpmb @7a392500
target: atelier-zpmb
identity: gpt-6.1-sol-r2
commit: 7a3925008cafd4f69b701fc80b5934c1175f0d39
created_at: 2026-11-09T17:09:44.195336137Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: crates/atelier-site/src/lib.rs:220-225 caches an incomplete brief when a Git failure is swallowed by the view builder, so atelier-cuq9 is only partly fixed. Reproduced at the pinned head in a scratch repository: load the mission summary, make only the strategy's loose blob unreadable, then GET its brief. It returns 200 with the mission-record fallback outcome, zero criteria and a cannot-be-read problem. Restore the blob without changing refs: subsequent brief requests return the identical broken body, although the strategy route now returns the real Markdown and criterion. mission_review.rs:1443-1446 turns the read error into None and criteria.rs:69-75 turns it into a successful parse-problem value, so testing HTTP status alone cannot prove the view depends only on the commit. Propagate Git read failures from commit-only views, or mark those builds non-cacheable, while preserving intentional malformed-document reporting. Add a recovery test that makes only the strategy blob unreadable and verifies the restored outcome and criteria on the next brief request. Recommendation: fix_now.
