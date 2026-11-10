---
schema: atelier.review
schema_version: 2
id: atelier-y1tc
title: finding on atelier-xtvh @9c77b784
target: atelier-xtvh
identity: gpt-6.1-sol
commit: 9c77b7846b86dd445fac6b429dcbe8f0f9eff552
created_at: 2026-11-10T17:43:08.908989041Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-xtvh.md:38,40,42 leaves the interrupted-wait proof ambiguous about when remote state is captured. Submit may fail to push but succeed locally, with a warning that merge will push; the prerequisite merge.rs at d30b1f26:122-133 pushes and opens the pull request before consulting CI. On that fallback path a merge killed during its first CI wait has already changed the remote. Even the ordinary path has the approval commit ahead of the submitted remote, while line 58 leaves skipping that push undecided. Thus a snapshot taken before invoking merge cannot satisfy the literal remote-as-it-was assertion or the nothing-changed timeout assertion. Define the preparation/wait boundary: permit and report the fallback push and pull-request creation before waiting, assert no further writes during the wait against a snapshot taken after preparation, and test interruption both after successful submit and after failed-submit fallback. Alternatively choose a concrete push order that meets the stronger invariant. Do not leave the expected test state dependent on the Unknown.
