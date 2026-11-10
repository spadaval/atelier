---
schema: atelier.review
schema_version: 2
id: atelier-zlm9
title: finding on atelier-fcb7 @378eb60a
target: atelier-fcb7
identity: grok-4.6-plan-r2
commit: 378eb60a7695bdea855a47332bda78b1218b3445
created_at: 2026-11-10T01:23:26.631526384Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:265-288 and atelier-oci2: the site crate is converted in place on master as mission 2's last step, while records stay old until cutover and wd1i still waits on skill evals. atelier serve in crates/atelier-cli/src/main.rs:991 calls atelier_site::serve, so the tree binary's dashboard cannot read this repo, which contradicts the claim that the tree's atelier can read the repository's records on every commit of master. Mission 2's criteria (fresh-agent-loop, rehearsal) do not need the site. Smallest fix: drop oci2 from mission 2 and convert the site on the cutover branch with the records.
