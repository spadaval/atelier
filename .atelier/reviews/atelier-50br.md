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
---

medium atelier-yptz / docs/strategies/redesign-product.md:179: The no-view-lost promise is broader than the listed replacements: work missions --all includes completed missions, work decisions includes answered decisions across mission scope, and work active is a filtered view; list currently promises open work and no active or closed filter, while show promises open questions. Workers could implement the stated acceptance line and still remove these views. Smallest fix at step B planning: map all nine old views and their historical visibility to show/list filters or sections, retaining completed missions and answered decisions without restoring the old command tree.
