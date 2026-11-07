---
schema: atelier.review
schema_version: 2
id: atelier-2x99
title: finding on atelier-3jdm @e833abea
target: atelier-3jdm
identity: independent reviewer subagent (claude-opus-5-5), atelier-3jdm
commit: e833abeab005d32e67e80423c216405a08b080e6
created_at: 2026-11-07T05:51:12.100256708Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 1d4ab895bbf1a33c8b935a4af9d6a8ca810a00fc
    recorded_at: 2026-11-07T05:52:29.470747260Z
    resolution_commit: 1d4ab895bbf1a33c8b935a4af9d6a8ca810a00fc
    reason: 'Fixed in 1d4ab895: validation.md now says record edits never make an approval stale.'
---

low: docs/product/validation.md:157-158 still says '[Approval freshness](reviews-and-merge.md) has the stricter rule for existing proof and review edits.' After this change approval freshness has no rule for proof edits (an edited receipt keeps the approval current, as record_edits_after_approval_keep_it_current asserts), and review rewrites are refused by review_append_only, not by freshness. Done when 6 requires docs/product to describe approval freshness as stale only on code; this line contradicts it, and an agent reading validation.md would expect a receipt edit after approval to need re-review. Fix: replace the sentence, e.g. 'Record edits, receipts included, never make an approval stale (see Approval freshness); review records stay append-only by their own check.'
