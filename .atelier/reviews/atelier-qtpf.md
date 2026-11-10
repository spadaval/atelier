---
schema: atelier.review
schema_version: 2
id: atelier-qtpf
title: approval on atelier-xtvh @e4fe00ed
target: atelier-xtvh
identity: gpt-6.1-sol
commit: e4fe00ed27b10c60221428eb73c253bc39b9e1e4
created_at: 2026-11-10T19:22:52.003672408Z
event:
  event: approval
---

Verdict: merge. Starting CI at submit and reusing its result removes the required approval-to-merge delay; a smaller change would miss the bounded wait or safety revalidation. Sizes: complexity medium (one bounded subprocess polling loop and validation before and after the wait), value high (the required core-loop latency reduction), risk medium (forge and merge behavior affects real repositories, with explicit refusal and unchanged-state tests). Contract: pass. Fitness: pass, with the pre-existing unbounded locked CI lookup recommended for follow-up as R1, atelier-qu4a, defer. No fix_now findings remain recommended. The code fixes atelier-0lkh by printing before the first query, atelier-bywd by preserving the original item identity across the wait, and atelier-fvc4 by collecting gh output in temporary files instead of joining blocked pipe readers. Their regression tests pass. Reviewed the nine branch-owned commits in a1e38011..e4fe00ed excluding master, final code and both amended specifications. Every added type, cache, branch and helper traces to submit sharing, CI eligibility, bounded polling or fresh validation. Fresh offline proof at the pinned code: requested HEAD^ whitespace check and the full implementation whitespace check passed; cargo fmt -- --check and cargo metadata --locked --format-version 1 passed in both root and next; in next, cargo nextest run --locked --build-jobs 4 -j 4 passed all 123 tests, and cargo clippy --locked --all-targets -j 4 -- -D warnings passed. Cargo network access was disabled and all tests used TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp. Read the full test output and acceptance scenarios, covering submit/reuse/failure recovery, eligible and ineligible runs, latest failure, retry and deadline behavior, stalled gh and inherited output handles, interruption, concurrent merges, base/config/head changes and checkout switching during the wait, and no-workflow operation. An independent offline probe confirmed the deferred locked-query stall using the supplied gh double and a local bare origin; its scratch repository and processes were removed. Open questions: none. Residual risk: live GitHub and release-profile behavior were not exercised; the release exclusion of the timer hook was inspected. Forge preparation, submit operations and the locked CI recheck retain their existing unbounded calls. Only review records were written to the branch. Follow-up disposition of atelier-qu4a remains with the implementer before merge.
