---
schema: atelier.review
schema_version: 3
round: 1
commit: e105e27ed6f643a8f98dc49138c170b3e9d1a9e8
reviewers:
- thread:delegated-task:command%3Amcp%3A94ac7775-6f88-438a-bddb-632ce4c8501a%3Adelegate-task%3Ab1623880-d41c-48ac-8ba9-9d490cfec046
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-16T17:47:10.842153522Z
---

## Summary

Complexity: low. The implementation changes three test-support files and adds no product logic.
Value: medium. It removes a reproduced source of failures from the Desk's current CI suite.
Risk of merging it: low. The change affects tests only, and a regression is visible and cheap to undo.

The small clock change is worth its cost; no smaller design is needed. I found no defect in the clock implementation. Approval is blocked by the checked acceptance criteria for runs that still fail.

Reviewed e105e27ed6f643a8f98dc49138c170b3e9d1a9e8 against the item's Goal, Done when and Scope. I read the full diff, fixture/server clock uses, all five specs' setup paths, iframe setup, and the product's relative-age, quiet-mark and week-strip calculations. The fixture/server no longer reads wall time; every tested page goes through serve(), which installs the shared starting instant before navigation; browser timezone is UTC. No product code changes.

Independent checks in /home/supadava/projects/atelier-jwfs-review:

- pnpm install --frozen-lockfile, pnpm build and git diff --check passed.
- pnpm test: 57 passed. Normal pnpm e2e: 134 passed.
- pnpm e2e views.spec.ts pages.spec.ts host.spec.ts --repeat-each=20: 1,597 passed, 3 failed in 5.1 minutes with six workers. Every failure was the keyboard-navigation test at host.spec.ts:314, not an age assertion.
- Full-suite runs with libfaketime's running system clock for Node and Chrome, TZ=UTC, FAKETIME_DONT_FAKE_MONOTONIC=1 and FAKETIME_DONT_RESET=1: 134 passed starting at 2026-11-16 23:59:58 UTC; 134 passed at 2026-11-13 14:59:59 UTC; 133 passed and one keyboard-navigation failure at 2027-03-01 00:00:59 UTC. These used a separate server port and two workers while the repeat run used six workers. A separate smoke check verified that the shim changes both Node's and Chrome's clocks.
- PR #178's Desk CI passed on the reviewed head.

The first, second and fifth Done when lines are supported. The third and fourth are not fully met: the implementation's Report already discloses failures in its repeat run, and my runs reproduce the acceptance gap. No clock-dependent assertion failed in my runs. I did not independently repeat the author's baseline failure-rate experiment or run Rust builds/tests, which this test-only diff does not need.

## Finding 1: Resolve the failing acceptance runs before marking them complete
Recommendation: fix now

.atelier/items/atelier-jwfs/item.md:36 and :37 mark both the 20-repeat run and all three shifted-system-clock full-suite runs as complete. The former is contradicted by the Report's own 1,598/1,600 result and my independent 1,597/1,600 result; the latter also failed independently at the March 2027 instant (133/134).

All four independent failures came from desk/e2e/host.spec.ts:314's keyboard test. One repeat and the March run timed out at line 330 with selected item atelier-aab1 instead of atelier-uwxy after the quick j/k sequence. Two other repeats navigated at line 352 to /embed/r/atelier/item/atelier-aab1 instead of /embed/r/atelier/item/atelier-uwxy. This is consistent with the separate keyboard race disclosed in the Report, not evidence of a defect in the clock fix.

Before merging, satisfy the written acceptance checks with an in-scope correction, or have the task's owner explicitly resolve whether those checks should exclude the independently established keyboard race and defer that work. Then record the actual validation against that agreed scope. An unrelated pre-existing failure can justify changing a requirement deliberately, but it cannot make an unconditional passing-run criterion true. This finding does not request product-code changes or repeated reruns until one happens to pass.
