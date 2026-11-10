---
schema: atelier.review
schema_version: 2
id: atelier-fvc4
title: finding on atelier-xtvh @7d3a0a0a
target: atelier-xtvh
identity: gpt-6.1-sol
commit: 7d3a0a0a47072197837aa59399efb11fd9fb94bc
created_at: 2026-11-10T19:14:31.166013592Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/forge.rs:257 R2: The CI poll deadline covers only child.wait, not draining stdout/stderr. When gh exits but a helper still owns either pipe, the unconditional reader-thread joins at lines 257-258 block past the deadline, so merge never returns not yet at its bound. Reproduced offline at the pinned head using the supplied FAKE_GH with this runs-lookup mode: sleep 30 &; emit an empty workflow_runs response; exit 0. With ATELIER_TEST_CI_WAIT=300,50, merge printed its waiting notice but was still blocked after 3 seconds and needed its process group killed. The existing stall test uses exec sleep, so it exercises child.wait timing but cannot catch this pipe-drain hang. Keep the same deadline effective through output collection, return Pending on timeout, and clean up the spawned processes; add a regression with an inherited pipe holder. Recommendation: fix_now. Fresh proof at the pinned head: all 121 next workspace tests passed offline using TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp and cargo nextest run --offline --locked --build-jobs 4 -j 4. Root and next cargo fmt -- --check and cargo metadata --offline --locked --format-version 1 passed; the requested HEAD^ whitespace check and whole-branch whitespace check passed; next cargo clippy --offline --locked --all-targets -j 4 -- -D warnings passed. Both independent probes used a gh double first on PATH and a local bare origin; scratch repositories were removed. Code stayed unchanged. No approval recorded because R1 and R2 remain. Open questions: none. Residual risk: live GitHub behavior and a release-profile build were not exercised; forge preparation before the wait and CI revalidation after the lock remain unbounded as the Report discloses. The previous atelier-0lkh notice problem is fixed by inspection and its passing regression test, but no later approval is recorded in this round.
