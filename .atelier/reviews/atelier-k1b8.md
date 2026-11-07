---
schema: atelier.review
schema_version: 2
id: atelier-k1b8
title: 'G16: land certifies global active traceability before arbitrary checks but does not refresh it at the final fence. In /tmp/session-review-global-ref-fence-px9gxxd2/repo a completed approved owner runs a trusted merge check deleting only unrelated active owner atelier-eib7 branch; exact 5b5dca62c10736c4a13b19f56d350f71e8cadd96 first land passes, immediate second land refuses land_work_untraceable. HEAD, canonical files, index and working status remain unchanged. R1/S6 require all active work traceable when land returns. Revalidate global active-owner refs after arbitrary checks, preserving current/source/base fences. This predates snapshot consolidation but violates the required fresh-ref boundary.'
target: atelier-1ioo
identity: session-reviewer
commit: 5b5dca62c10736c4a13b19f56d350f71e8cadd96
created_at: 2026-10-02T01:42:22.373607032Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: session-reviewer
    commit: 84395a9d3ba6d28550023c3234fb166cc6cd5211
    recorded_at: 2026-10-02T02:02:59.769338176Z
    resolution_commit: 84395a9d3ba6d28550023c3234fb166cc6cd5211
    reason: 'G16 independently verified at root composition 84395a9d: restored unrelated active owner ref before original arbitrary-check fixture; first land now refuses land_work_untraceable after the trusted check deletes it. Global active traceability runs again at final fence. Source and separate-base hidden mutations introduced by checks still refuse; valid historical approval and whole-terminal retirement remain accepted. This verifies land global traceability, while merge retains its documented owner-scoped readiness. Full-hooks proof 554/554, zero skipped; no S4 latency acceptance claimed.'
    migrated_from: atelier-p2y7
---

G16: land certifies global active traceability before arbitrary checks but does not refresh it at the final fence. In /tmp/session-review-global-ref-fence-px9gxxd2/repo a completed approved owner runs a trusted merge check deleting only unrelated active owner atelier-eib7 branch; exact 5b5dca62c10736c4a13b19f56d350f71e8cadd96 first land passes, immediate second land refuses land_work_untraceable. HEAD, canonical files, index and working status remain unchanged. R1/S6 require all active work traceable when land returns. Revalidate global active-owner refs after arbitrary checks, preserving current/source/base fences. This predates snapshot consolidation but violates the required fresh-ref boundary.
