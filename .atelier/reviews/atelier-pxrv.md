---
schema: atelier.review
schema_version: 2
id: atelier-pxrv
title: finding on atelier-l2aj @374c9e55
target: atelier-l2aj
identity: gpt-6.1-sol-l2aj-r2
commit: 374c9e55e0c93b08ca3ac156f09fde887ebee09d
created_at: 2026-11-11T07:50:22.747284800Z
event:
  event: finding
  recommendation: fix_now
---

medium: desk/src/pages/Item.tsx:121 R3: Reset the answer form when navigating between cached items. ItemPageView reuses ItemDetail when the destination item is already in useData, and AnswerForm is keyed only by its decision index. Visit item B, follow its blocker link to item A, open the answer form and type an answer, then follow a caused-by link back to B. If both have their oldest open question at index 0, B displays its own question with the open form and answer from A. Send the answer posts that stale text with B as the decide target. Independently reproduced at desktop and phone widths on the built app with fixture-only responses; the captured request targeted atelier-u1ko with the cache-key answer typed on atelier-uwxy. Key the form by repo, item, question and finding so changing its target remounts it, and add a regression using client-side navigation to a cached item rather than page.goto. Reproducer: ~/.cache/atelier-desk/l2aj-review-r2/probe.spec.ts; run with pnpm exec playwright test --config ~/.cache/atelier-desk/l2aj-review-r2/playwright.config.ts from desk/. Both diagnostic tests confirm the stale payload. No real act was sent.
