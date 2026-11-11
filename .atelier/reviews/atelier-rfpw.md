---
schema: atelier.review
schema_version: 2
id: atelier-rfpw
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: GPT-6 Astra independent planning reviewer A
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:23:27.195669964Z
event:
  event: finding
  recommendation: fix_now
---

R5 (medium) .atelier/issues/atelier-oci2.md:38: The blanket requirement that nothing in next/ builds or serves a site conflicts with placing atelier-desk in next/crates/atelier-desk (desk.md:80) and building its embedded web app and browser tests in CI (atelier-94pv.md:31). The same prohibition appears in redesign-architecture.md:1012-1015 and cutover step E. If the Desk lands first, the cutover cannot satisfy these instructions without deleting or disabling part of it; postponing the Desk until after the switch violates the parallel-build design. Scope deletion and the no-Node requirement to the old atelier-site and tracker binary/release, explicitly preserve the independent Desk build and its CI jobs, and state how the Desk crate/app build survives the next/ move.
