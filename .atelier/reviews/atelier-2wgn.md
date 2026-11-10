---
schema: atelier.review
schema_version: 2
id: atelier-2wgn
title: finding on atelier-1sbw @f317ac78
target: atelier-1sbw
identity: gpt-6.1-sol
commit: f317ac78fdffcef88ab75a2773d9ee5f59688361
created_at: 2026-11-10T21:55:31.161114616Z
event:
  event: finding
  recommendation: fix_now
---

R3 .atelier/issues/atelier-1sbw.md:35: medium. The open-decision mapping supplies Goal but leaves the required Done when section undefined. The actual open decisions atelier-8i86 and atelier-j86z have only Context, Question and Options, so the general rule to retain Done when cannot supply it. next/crates/atelier-records/src/validate.rs requires both Goal and Done when on every item. The early real-repository proof at line 41 runs only status/list, which read records but do not run this validation, so it does not prove the converted decisions are valid. Specify how an unanswered decision becomes a valid needs-human item, including its Done when, without inventing an answer or discarding its options/context; require a fixture assertion against the target record requirements. This must be settled for the converter item rather than left to the later rehearsal.
