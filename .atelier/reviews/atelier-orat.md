---
schema: atelier.review
schema_version: 2
id: atelier-orat
title: finding on atelier-fcb7 @693a2ed2
target: atelier-fcb7
identity: gpt-6-astra-amend-r3
commit: 693a2ed2a4caf900df673ade817c3c8f8f38b283
created_at: 2026-11-10T02:53:01.582751401Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-architecture.md:385-386,413-431: atelier-x7sm is only partly resolved. Removing the new hooks before restoring records fixes the ordering, but cutover setup still records only base commits and rollback only removes hooks; neither saves nor restores the prior hook installation. Once setup preserves or chains a project hook (already required by atelier-pz1y), deleting its installed wrapper does not restore the original hook. Thus stops after step 3, step 4, or step 5 can restore records, binary, skill and service while losing the prior project hook. The rehearsal also still lists only completed step 3, not the partial installation requested by atelier-x7sm. Smallest fix: require recording the prior hook installation before setup, removing only the new Atelier integration before Git rollback, restoring the prior installation, and rehearsing rollback with real hooks after partial step 3. This is the rollback contract, not a request to settle the deferred setup implementation now.
