---
schema: atelier.review
schema_version: 1
id: atelier-i8bf
title: 'crates/atelier-cli/tests/cli_integration/debrief.rs:351-381 and crates/atelier-app/src/record_history.rs: the Markdown-to-HTML test derives only item IDs from the Markdown; criterion slugs and marks and decision choices are hard-coded fixture values, so a fact added to the Markdown later would not be checked against the HTML, contrary to Done when''s ''every ... in the Markdown''. record_history has no tests for the mission-side link decision (atelier-rx22''s third decision note), the empty-history path, aged-proof deviations, or merges, which is how the defects above escaped. Fix: parse criterion rows and decision rows out of the Markdown in the test, and add unit or fixture coverage for those history cases.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:18.067161391Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/tests/cli_integration/debrief.rs:351-381 and crates/atelier-app/src/record_history.rs: the Markdown-to-HTML test derives only item IDs from the Markdown; criterion slugs and marks and decision choices are hard-coded fixture values, so a fact added to the Markdown later would not be checked against the HTML, contrary to Done when's 'every ... in the Markdown'. record_history has no tests for the mission-side link decision (atelier-rx22's third decision note), the empty-history path, aged-proof deviations, or merges, which is how the defects above escaped. Fix: parse criterion rows and decision rows out of the Markdown in the test, and add unit or fixture coverage for those history cases.
