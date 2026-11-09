---
schema: atelier.review
schema_version: 3
id: atelier-xt52
title: finding on branch release-0.4.0 @6dd196be
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 6dd196be861e871264a466b5892360a9f8218715
created_at: 2026-11-09T20:56:21.572050054Z
event:
  event: finding
  recommendation: fix_now
---

low: CHANGELOG.md:22 ci.passed does not require a passing run on the latest commit as stated. It accepts an earlier branch commit with identical code, including when later commits add review records or approvals; a record-only branch needs no CI run. The targeted test ci_passed_passes_on_the_head_or_an_earlier_commit_with_its_code passes freshly. Say latest code rather than latest commit and mention the record-only exception, so users do not rerun CI after every approval commit. This is core behavior of the advertised one-command merge, not a stylistic distinction.
