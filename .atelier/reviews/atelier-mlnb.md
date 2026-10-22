---
schema: atelier.review
schema_version: 1
id: atelier-mlnb
title: 'low: docs/product/reviews-and-merge.md:205-207 and the approval_accepted doc comment (crates/atelier-app/src/review_gate.rs:196-202) say every finding pinned at or before the approval must be ''disposed by then''. The code enforces timing only for a FIX NOW resolution commit. A DEFER or NO ACTION disposition, or the FIX NOW disposition record itself, recorded after the approval still counts, as it does in merge''s findings.disposed. Scratch cases (a FIX NOW disposition, or a non-author DEFER, recorded after the approval) merge and land under both strategies. Behaviour agrees between the gates, so this is a wording error, but the product doc states a stricter rule than either gate applies. Fix: say that the finding needs an accepted disposition, with a FIX NOW resolved at or before the approval''s pin, and drop ''by then''. Fold this into the fix for the independence finding.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: 09b6fd28c588c3bc437be5dfb76b84c4763e0dc8
created_at: 2026-10-22T21:46:15.745570269Z
event:
  event: finding
  recommendation: defer
---

low: docs/product/reviews-and-merge.md:205-207 and the approval_accepted doc comment (crates/atelier-app/src/review_gate.rs:196-202) say every finding pinned at or before the approval must be 'disposed by then'. The code enforces timing only for a FIX NOW resolution commit. A DEFER or NO ACTION disposition, or the FIX NOW disposition record itself, recorded after the approval still counts, as it does in merge's findings.disposed. Scratch cases (a FIX NOW disposition, or a non-author DEFER, recorded after the approval) merge and land under both strategies. Behaviour agrees between the gates, so this is a wording error, but the product doc states a stricter rule than either gate applies. Fix: say that the finding needs an accepted disposition, with a FIX NOW resolved at or before the approval's pin, and drop 'by then'. Fold this into the fix for the independence finding.
