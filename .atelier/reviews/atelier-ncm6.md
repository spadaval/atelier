---
schema: atelier.review
schema_version: 2
id: atelier-ncm6
title: finding on atelier-5fsj @73542065
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r4
commit: 735420651eb5fc7102b92cbc6b8714c0ea3aa68a
created_at: 2026-11-10T16:04:24.392890156Z
event:
  event: finding
  recommendation: fix_now
---

low: scripts/scorecard/commands.py:129 R11. Wrapper removal skips option flags and numeric words but not nonnumeric option arguments. Synthetic env -u SCORECARD_EXAMPLE atelier status and stdbuf -o L atelier status both return no atelier_calls (expected [['status']]) and are not process commands; env -u SCORECARD_EXAMPLE python3 -c with a supported file write is also hidden. The README lists env and stdbuf with their options as recognized, while the tests only cover numeric/attached options. Consume the arguments of supported wrapper options and test these forms, or get an explicit scope decision listing the narrower supported options. Both additional positive-call assertions fail at the pinned head while the existing suite passes. This undercounts process/confusion and can delay first-edit detection on documented forms. Frozen headline impact is not established.
