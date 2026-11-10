---
schema: atelier.review
schema_version: 2
id: atelier-0lkh
title: finding on atelier-xtvh @f8e2d1fc
target: atelier-xtvh
identity: gpt-6.1-sol
commit: f8e2d1fca3f265a5b3e6a60bfa7815b12eacad6d
created_at: 2026-11-10T19:09:19.277025720Z
event:
  event: finding
  recommendation: fix_now
---

Low: next/crates/atelier-cli/src/merge.rs:303 R1: The first CI poll can consume the entire nine-minute bound before merge prints its waiting/interruption notice. The notice is emitted only after ci returns Pending, and the deadline check at line 313 returns NotYet before it is printed when that first call stalls. Reproduced with the supplied FAKE_GH stall mode and ATELIER_TEST_CI_WAIT=1500,50: while the process was blocked its only output was the pull request line; it then exited 3 without any Waiting up to or safe-to-interrupt message. This misses Done when line 5 and leaves users uninformed during the longest wait. Emit the bounded-wait/interruption notice before the first potentially blocking CI poll, and extend a_gh_that_never_answers_is_ended_at_the_bound to check that the notice is available while the poll is stalled. Review proof at the pinned head: all 121 next workspace tests passed offline with --build-jobs 4 -j 4; root and next formatting and locked metadata checks passed; requested HEAD^ whitespace check and full implementation whitespace check passed; next clippy --all-targets -j 4 -- -D warnings passed. Independent local-bare-origin/FAKE_GH run verified submit publication, approval moving the local head, timeout preserving refs, and successful merge using submit CI without repushing the branch. Verdict: merge after R1 is fixed and freshly reviewed. Sizes: complexity medium (poll deadline, eligibility cache and two-phase validation), value high (explicit required latency improvement), risk medium (merge and forge paths, failures generally visible). Contract: fail on the waiting notice. Fitness: pass for the remaining inspected change; the added moving parts trace to the requirements. No open questions. Live GitHub behavior and a release-profile build were not exercised; post-wait CI revalidation and pre-wait forge preparation remain unbounded as the Report discloses. No approval recorded.
