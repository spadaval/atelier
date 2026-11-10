---
schema: atelier.review
schema_version: 2
id: atelier-x7sm
title: finding on atelier-fcb7 @36099cca
target: atelier-fcb7
identity: gpt-6-astra-amend-r2
commit: 36099ccac65c13c9181a48e8a7c1fc7b7bcd4a39
created_at: 2026-11-10T02:51:14.559111122Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:411-423: atelier-xbnh remains open for rollback ordering. The pushed row reverts the conversion and pushes before the following paragraph removes the new Git hooks; the restored old layout cannot be read by the new binary, and reinstalling the fallback first instead leaves hooks calling unsupported check --hook modes. The rollback can therefore stop before publication and restoration finish; ATELIER_SKIP cannot cure an unreadable record layout. The unpushed reset row does not invoke pre-commit or pre-push, but still needs hook cleanup before subsequent Git actions. Smallest fix: explicitly save the prior hook installation at cutover setup and disable/remove the newly installed Atelier hooks before any rollback record restoration, revert, push or fallback installation, then restore the prior hooks and rehearse both rollback rows with the real hooks enabled, including partial step 3 installation.
