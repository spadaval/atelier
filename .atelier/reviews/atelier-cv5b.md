---
schema: atelier.review
schema_version: 2
id: atelier-cv5b
title: finding on atelier-fcb7 @36de8f1a
target: atelier-fcb7
identity: grok-4.6-amend2-r1
commit: 36de8f1abc4b2ec76b1eb30ca8b9f85588d82929
created_at: 2026-11-10T04:06:20.082256586Z
event:
  event: finding
  recommendation: fix_now
---

high, redesign-product.md:88-89 and redesign-architecture.md:191: gardener is listed as a role gotten from the command a thread enters with, but the only entry is start --new on a garden item, which binds implementer; status then refuses other roles, and a work branch writes only its own .atelier, while new --from and drop on a finding must write the base. Consequence: the 9dku trial, held as that implementer, cannot settle deferred findings, or it writes Taken/Dropped on the garden branch and the base can be settled differently. Smallest fix: the implementer of an item labelled garden may run new --from and drop on findings; those two commands always read and write the finding on the base and refuse if it is already settled there; drop gardener as a separate command-gated role.
