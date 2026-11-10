---
schema: atelier.review
schema_version: 2
id: atelier-o80t
title: finding on atelier-xtvh @9c77b784
target: atelier-xtvh
identity: gpt-6.1-sol
commit: 9c77b7846b86dd445fac6b429dcbe8f0f9eff552
created_at: 2026-11-10T17:43:01.198354386Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-xtvh.md:40-43 promises a fixed nine-minute CI wait under the foreground limit, but its error and shortened-timer scenarios do not prove that bound when a poll itself stalls. The prerequisite forge.rs at d30b1f26 runs gh with Command::output() and no timeout, so a network call or gh process that never exits can hold merge past the bound even if the outer polling loop checks elapsed time correctly. Require the wait deadline to include time inside each poll, limit each child call to the remaining budget and end that child at expiry, and add a fake-gh scenario that does not return before the shortened deadline. It must exit not yet within the budget and leave no poll child running. This needs no flag, config option or dependency.
