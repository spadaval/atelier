---
schema: atelier.review
schema_version: 2
id: atelier-obgr
title: finding on atelier-l2aj @1cc3679c
target: atelier-l2aj
identity: gpt-6.1-sol-l2aj-r1
commit: 1cc3679c97b24f2ac05f48910f35f0d229263f59
created_at: 2026-11-11T07:39:52.987348143Z
event:
  event: finding
  recommendation: fix_now
---

medium: desk/src/pages/Mission.tsx:151 R1: Reset the answer form when the question changes. Waiting questions use a key made from the item and the filtered list position, so after answering the first of two questions on one item, the next question reuses its AnswerForm, including the old answer, settlement and done message. A fixture-only Chromium reproduction on this head answered 'Should the cache key include the checkout HEAD?' with 'Yes, key it by HEAD too.' and defer; after reload the remaining question 'Should we delete the cache instead?' already contained that answer, displayed the first act's success and had Send the answer enabled. Inbox.tsx:86 has the same problem when a poll replaces an item's oldest question while its form is open; reproduced at phone width with 'Delete it.' remaining under a new retain-cache question. This can submit an answer or settle a finding intended for a different question. Give forms a stable question identity or explicitly reset them when the represented question changes, including open, answer, settlement and result state. Add regression coverage for advancing between two questions and for a question changing during a poll. The supplied suites pass (41 unit tests and 42 Playwright tests) but exercise only one open question per successful item.
