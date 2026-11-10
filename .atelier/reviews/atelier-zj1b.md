---
schema: atelier.review
schema_version: 2
id: atelier-zj1b
title: finding on atelier-6sti @d3806629
target: atelier-6sti
identity: gpt-6.1-sol-r1
commit: d38066290345dce0e9c44b2dae7e462e2d37ebc7
created_at: 2026-11-10T17:20:31.957582179Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/architecture/quality/scorecard-baseline-2026-11.md:63 Equal-length windows do not give merged items the same spread of observation ages, contrary to lines 66-68. The report itself says all observed astromech causes merged in the first week. In an equally long after-window, merges can instead cluster near its end, so fewer defects have had time to be found or fixed when the count is taken. A fresh 27-day counterexample gives median observation ages of 23 days for merges on days 1-7 and 3 days for merges on days 21-27. The bug-fix filing-rate guard does not test these ages. This false comparability guarantee can make the central defects-must-not-rise constraint appear satisfied with less observation. Replace it with an explicit limitation: equal windows cap exposure but do not equalize it, and an after-run with materially different merge ages cannot establish that the constraint held from the raw rate alone. No new baseline measurement is needed to correct this claim.
