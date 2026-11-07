---
schema: atelier.review
schema_version: 2
id: atelier-gf8h
title: finding on atelier-s66u @9603ad9e
target: atelier-s66u
identity: lighter-workflow reviewer for atelier-s66u (claude-opus-5-5)
commit: 9603ad9e630db3d0da6f7018d9abe5d8b3576b71
created_at: 2026-11-07T00:56:44.182065592Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Sumeet Padavala <sumeet.padavala@gmail.com>
    commit: f7a9ab5166dde310f18c7ad345f1c3cd359ce11d
    recorded_at: 2026-11-07T01:07:31.626403597Z
    resolution_commit: b388ad0bf3d3aa8cee5960e2de10f9a150a059fd
    reason: evidence list's verdict states a missing or stale pass without a warning unless the policy runs evidence.attached on the item's workflow; tested by evidence_list_marks_a_missing_pass_only_under_an_evidence_attached_gate.
    migrated_from: atelier-pyun
---

low: crates/atelier-cli/src/commands/evidence.rs:871-872 'evidence list <item>' still ends 'Proof: ⚠ no passing proof' for an item with no receipts and 'Proof: ⚠ no fresh pass; newest pass <id> is stale' after any base merge (the new merge_gates test itself reaches this state). The Done when only names status and work views, but this ⚠ is the same nudge the ADR removes elsewhere ('having no receipt is not a gap'), and an agent reading it after a base merge is invited to re-record a suite. Suggested follow-up: neutral wording (e.g. 'Proof: no receipts', 'newest pass <id> is stale') without ⚠ unless the base policy has an evidence.attached gate, keeping the per-row freshness marks.
