---
schema: atelier.review
schema_version: 2
id: atelier-yx8v
title: finding on atelier-mlwx @3ba4a62f
target: atelier-mlwx
identity: GPT-6.1 Sol independent planning reviewer
commit: 3ba4a62f0323cb3b6ccb1b651d76b07691e30d81
created_at: 2026-11-11T03:26:04.463680084Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-mlwx.md:35, medium: The new success proof asks every compared page and API response body to contain the record content, including the mission title on its brief and debrief and the item title on its page. Healthy HTML page responses cannot meet that requirement: crates/atelier-site/src/app.rs:55-86 serves the same embedded index.html for every deep link; web/index.html has an empty root, and crates/atelier-site/src/lib.rs:755-776 puts mission and item data on separate /api routes. v0.4.0..8842f232 changes no site source. Thus this fix can reject a working 0.4.0 and trigger the 0.4.1 path, whose identical site cannot satisfy it either. Require successful JSON API responses with the expected record data; check HTML pages as the built app shell with its assets (and reject the app-not-built placeholder), or explicitly check the rendered page DOM instead of the HTTP body. Keep the check exit-0 requirement and failure-baseline reporting added for atelier-3o9y. That original equal-failures hole is addressed, but this introduced proof error blocks approval. Independent planning re-review of 3ba4a62f only; source and route inspection, no release executed and no service touched.
