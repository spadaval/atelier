---
schema: atelier.review
schema_version: 2
id: atelier-864b
title: finding on atelier-5zre @24b518dd
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer, round 5
commit: 24b518ddfecc75feb6fa567cf6434695261964b2
created_at: 2026-11-11T05:15:21.039250529Z
event:
  event: finding
  recommendation: fix_now
---

R1 low: .atelier/issues/atelier-n831.md:30 The rewrite keeps the behind-base count and separate record changes, but removes two other surviving requirements. At fc0fe395 its first Done when line requires `master, 1 ahead of origin/master` and `detached at <sha8>`, and its test line explicitly covers ahead and detached states. Neither need depends on a removed command, flag or record path. In a clean converted scratch clone, a local origin/master tracking ref is one commit behind master (`git status --short --branch` prints `master...origin/master [ahead 1]` and `git rev-list --count origin/master..master` prints 1), but the supplied binary's status says only `on the base branch master`. At the same commit checked out detached, it says only `on a detached HEAD`, with no commit ID. Thus these requirements are neither already met nor retained for conversion, contrary to the sort's fourth Done when requirement to keep the need as stated. Restore the ahead-of-upstream count and detached commit ID requirements, with their test cases, in atelier-n831; update its rewrite explanation if needed. Change no binary code and no protected record. This is a trivial restoration of existing record requirements, so FIX NOW applies despite this being an additional finding on the fix of the earlier low-severity atelier-qptk. Full reproduction output: /home/supadava/.cache/atelier-qr5s/5zre-review5/n831-proof.txt.

Verdict: merge after R1's small record correction; no approval is recorded while a surviving requirement is lost. Sizes: complexity medium (181 separate record judgments), value high (a usable cutover inventory), risk high (wrong drops or narrowing can silently lose work at conversion). Contract: fail on preserving the two requirements above; the other structural obligations pass. Fitness: otherwise pass; this branch adds no code or unnecessary mechanism.

Independently checked the 181 unique fate lines against fc0fe395: 68 drops (25 superseded, 43 declined), 27 rewrites, 10 as is and 76 left for the human. Audited each baseline open record's own Git history and metadata: 72 protected records, all byte-identical except the permitted changes to atelier-5zre. The seven records from atelier-brmd are restored byte-for-byte. Read original Goal/Done when contracts for the drops, the rewritten contracts, specification removal and command tables, duplicate targets, mission proposals and all 36 commits' changed paths. All eight missions are unchanged. The cutover facts match: 11 of 28 ready items are in draft missions; 3 ready items lack checkbox lines; all 126 deferred findings on done records have follow-ups (73 done, 11 backlog, 42 dropped). The two blocked-by links and three children involving dropped records are explained in the Report. Drop reasons agree with the per-record Reports.

Fresh proof passes: installed atelier 0.4.0 check (existing no-checkbox warnings), whole-branch git diff --check and the brief's record-excluded diff check. Conversion of pinned head 24b518ddfecc75feb6fa567cf6434695261964b2, converter --verify (127 files), and the supplied new binary check pass: 105 items and 8 missions. Scratch probes verified combined parse/reference diagnostics, draft start refusing before writes, already-started output, blocked and needs-human rows, redirected output with COLUMNS=30, removed flags, and the retained multi-label, blocker-chain, cycle, refusal-title and hidden-done-footer needs. Scratch data and probe.py are under /home/supadava/.cache/atelier-qr5s/5zre-review5/. The new binary ran only there. No Cargo build/install, push, pull request, merge or other-repository read ran; code-only Cargo checks were not run for this records-only change.

Open questions: none needed to establish R1. Residual risk: later master changes require reconciliation, and protected and mission proposals remain the human's judgment. Only this finding record is written by this review.
