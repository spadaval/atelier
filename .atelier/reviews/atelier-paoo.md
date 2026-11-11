---
schema: atelier.review
schema_version: 2
id: atelier-paoo
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: GPT-6 Astra independent planning reviewer A
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:23:26.438815974Z
event:
  event: finding
  recommendation: fix_now
---

R3 (medium) .atelier/issues/atelier-wd1i.md:44: The changed cutover still requires that the site answers for each converted repository, even though redesign-architecture.md:1097-1099 now disables atelier-serve. The mission Outcome in atelier-qr5s.md:24 still promises a site reading new records, and atelier-u1ko.md:55 still asks how to restart it during cutover. These are live instructions for the same path, not historical mentions; satisfying them would bring the Desk back onto the cutover critical path (and va99 already waits on wd1i). Amend the affected mission and item records with the site-removal decision: require the old service stopped/disabled after conversion, retain enabling it only for rollback, and remove any requirement for Desk availability before cutover completes.
