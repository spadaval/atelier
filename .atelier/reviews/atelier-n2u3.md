---
schema: atelier.review
schema_version: 2
id: atelier-n2u3
title: finding on atelier-u1ko @4ef4c424
target: atelier-u1ko
identity: GPT-6.1 Sol independent reviewer
commit: 4ef4c424b2b8f90fa627d9119fe24eb83be7d4b1
created_at: 2026-11-11T09:28:06.459999550Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/cutover.sh:236 R1: Failed service-state queries are accepted as service absence and successful shutdown. service_state suppresses stderr and ignores every is-enabled/is-active failure, producing a blank state on a user-bus error. stop_service at line 527 treats that state as absent and marks the service step done; accept_host at line 520 accepts it as stopped and disabled. Reproduced entirely in the review-3 sandbox: stop after step 2, make both service queries fail with a user-bus error, then resume. The script exits 0, says atelier-serve is not on this host, passes step 7 and closes the cutover item, while an independent call to the underlying service double reports enabled and active. The same helper can also journal an unknown original state, leaving rollback without the required restoration facts. Capture and validate both query results, preserve diagnostics, distinguish the expected disabled/inactive or explicitly absent states from query errors, and fail before recording or accepting an unknown state. Make callers propagate that failure and add a trial for unavailable service queries after the snapshot. Approval withheld; this violates the explicit stopped-and-disabled acceptance requirement.
