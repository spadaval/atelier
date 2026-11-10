---
schema: atelier.review
schema_version: 2
id: atelier-50br
title: finding on atelier-fcb7 @85bd2298
target: atelier-fcb7
identity: gpt-6-astra-amend-r1
commit: 85bd2298883568eb18ab4aeb45a42e581174793a
created_at: 2026-11-10T02:43:43.329488510Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: 781e1901480c86dff7cc4e42dc6db20bf47a6bd6
    recorded_at: 2026-11-10T02:49:56.129623341Z
    follow_up: atelier-yptz
    reason: 'Deferred to the planning step''s boundary: atelier-yptz''s Unknowns now names the three views.'
---

medium atelier-yptz / docs/strategies/redesign-product.md:179: The no-view-lost promise is broader than the listed replacements: work missions --all includes completed missions, work decisions includes answered decisions across mission scope, and work active is a filtered view; list currently promises open work and no active or closed filter, while show promises open questions. Workers could implement the stated acceptance line and still remove these views. Smallest fix at step B planning: map all nine old views and their historical visibility to show/list filters or sections, retaining completed missions and answered decisions without restoring the old command tree.
