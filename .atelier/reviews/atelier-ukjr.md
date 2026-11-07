---
schema: atelier.review
schema_version: 2
id: atelier-ukjr
title: finding on atelier-bqbz @709ebff0
target: atelier-bqbz
identity: lighter-workflow reviewer for atelier-bqbz (claude-opus-5-5)
commit: 709ebff0842519d604ec8d2c3fcc9926aee49548
created_at: 2026-11-07T03:14:14.199984427Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: atelier-bqbz implementer (claude-opus-5-5)
    commit: f05fd995a809da873eef57c10ab6c4f8d091de77
    recorded_at: 2026-11-07T03:15:31.522597375Z
    resolution_commit: f05fd995a809da873eef57c10ab6c4f8d091de77
    reason: Decision note and Deviations now give review's branch counts (80 atelier, 59 astromech), the astromech request_review runs and the live astromech-l5c6, and drop review as a simplicity call flagged for Sumeet
---

medium: .atelier/issues/atelier-bqbz.md:46 The decision justifying the review drop misreports atelier-rkxm's counts. It cites demiurge 0, cosplay-studio 1 and one base tip each in atelier and astromech, but leaves out the largest numbers in the same tables: 80 records through review on branches in atelier and 59 in astromech. Those are branch records, the same measure the item uses to keep in_progress. Two claims are also false. 'in_progress -> done is the path in use' does not hold: astromech agents ran request_review on 2026-10-16/17/22 and again on 2026-11-06. 'No live ref in ... astromech ... has a record in blocked or review' (lines 46 and 54, and the Deviations section) does not hold either: astromech branch task/astromech-l5c6 (commit 9af0c06b, 'Request review astromech-l5c6', 2026-11-06 22:05 -0500) holds astromech-l5c6 in status review. Done when asks that each drop be justified from rkxm's note. As written, a reader would conclude review was unused, when the Shape's rule ('kept only if records use it') says it was used. The drop may still be right on the grounds that it buys nothing review records do not already give, and it fits the human's aim. Fix: add a decision note that corrects the counts and the live-ref claim, says review is dropped despite its branch use and why, and flags that reading of 'Statuses and types follow use' for the human's post-merge look.
