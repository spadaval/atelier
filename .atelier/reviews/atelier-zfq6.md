---
schema: atelier.review
schema_version: 2
id: atelier-zfq6
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:03.891858463Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/adr/0028-simpler-lifecycle.md:226 The ADR 0019 decision 3 entry omits that decision's 'Atelier never creates commits; agents commit records with the code they describe', which decision 2's open list (lines 103-105) and the PRODUCT_INTENT.md entry (lines 283-284) treat as open. Under line 204-205 ('everything else in that ADR stands') the ADR 0019 sentence is declared standing, so ADR 0028 contradicts itself and the atelier-jnrh implementer may treat it as settled. Fix: add 'how Atelier never creates commits reads is open (decision 2 above)' to that bullet and to ADR 0019's Status line.
