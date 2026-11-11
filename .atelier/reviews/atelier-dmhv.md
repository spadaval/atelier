---
schema: atelier.review
schema_version: 2
id: atelier-dmhv
title: finding on atelier-wd1i @8b1b75e0
target: atelier-wd1i
identity: GPT-6.1 Sol independent planning reviewer
commit: 8b1b75e02cfdd1a68e3a72c829ff9170f4fde8bc
created_at: 2026-11-11T05:04:59.916756744Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-wd1i.md:49 The mandatory failure exit relies on F's rollback producing local/remote agreement and the old tree in every real clone, but F's inverse-journal algorithm does not yet specify that exit for published conversions made in temporary clones. If the first product conversion is pushed and the next conversion fails before step 6, reverting in the temporary clone leaves the real base at its original A while the remote is at revert R; there was no real-base journal move to undo. A push accepted remotely but reported failed also leaves reset versus revert undetermined by the journal alone. These are stops before acceptance, so G must roll back rather than choose fix-forward on the day. F findings atelier-rvrk and atelier-qd4w describe the missing protocol. Make G's prerequisite explicitly depend on that reviewed rollback reconciliation and its trials, including an accepted-but-reported-failed push and a stop after a product push before updating real clones; retain G's required final agreement. Do not leave the operator to decide which refs to reset or advance.
