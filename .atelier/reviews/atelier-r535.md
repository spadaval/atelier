---
schema: atelier.review
schema_version: 1
id: atelier-r535
title: 'Reviewed git diff master...task/atelier-kops @ b04debc4 (session.rs trusted-point computation, new test steps in land_checks_claims_that_linked_records_or_removed_proof_affect, sessions-and-hooks.md, issue record, evidence lv4h/zf2i) against the Done-when. Code: trusted = merge-base(base@{upstream}, branch_base or HEAD), None under --deep or without an upstream or unrelated history; branch_base is the base tip or the fork point when the base moved, so trust never exceeds the branch base and stops at pushed history - meets the requirement. Ran the test (pass), then reverted only the session.rs change locally: the test fails at tests/session.rs:3770 (owned branch from unpushed failing-proof base commit), so the test exercises the claim; restored, tree clean. Grepped docs for stale trusted-point wording. Limits: did not run the full suite (relied on lv4h/zf2i); did not probe branch bases other than the policy base (e.g. mission branches), where the merge base with the policy base''s upstream is conservative (more claims checked), not unsafe.'
target: atelier-kops
identity: Independent reviewer (Claude subagent)
commit: b04debc463d854a7c0a5bae96eaee5459b306f81
created_at: 2026-10-17T06:04:04.592486505Z
event:
  event: review
---

Reviewed git diff master...task/atelier-kops @ b04debc4 (session.rs trusted-point computation, new test steps in land_checks_claims_that_linked_records_or_removed_proof_affect, sessions-and-hooks.md, issue record, evidence lv4h/zf2i) against the Done-when. Code: trusted = merge-base(base@{upstream}, branch_base or HEAD), None under --deep or without an upstream or unrelated history; branch_base is the base tip or the fork point when the base moved, so trust never exceeds the branch base and stops at pushed history - meets the requirement. Ran the test (pass), then reverted only the session.rs change locally: the test fails at tests/session.rs:3770 (owned branch from unpushed failing-proof base commit), so the test exercises the claim; restored, tree clean. Grepped docs for stale trusted-point wording. Limits: did not run the full suite (relied on lv4h/zf2i); did not probe branch bases other than the policy base (e.g. mission branches), where the merge base with the policy base's upstream is conservative (more claims checked), not unsafe.
