---
schema: atelier.review
schema_version: 1
id: atelier-zjet
title: 'crates/atelier-cli/src/human_output.rs:518 (decision_full_rows, used by the debrief) and crates/atelier-cli/src/commands/mission_review.rs:521-563: the decision log orders one-way notes, then every decision record with superseded and open ones mixed among current ones, then reversible notes. Done when asks for ''one-way first, then decision records, reversible notes, superseded records''. The Markdown also merges everything into one Decisions section with no one-way marker, so an agent reading the debrief cannot tell one-way from reversible, while the HTML shows a one-way badge: the two forms disagree on a fact the test does not check. Malformed decision notes are dropped from both forms. docs/product/mission-reviews.md:22 repeats the order without superseded records. Fix: render sections like work decisions (One-way Decisions, Decision Records [current], Reversible Decisions, Open Decision Records, Superseded Decisions, Malformed) in both forms, update the doc, and assert the superseded-last order in a fixture.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:16.084677391Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/human_output.rs:518 (decision_full_rows, used by the debrief) and crates/atelier-cli/src/commands/mission_review.rs:521-563: the decision log orders one-way notes, then every decision record with superseded and open ones mixed among current ones, then reversible notes. Done when asks for 'one-way first, then decision records, reversible notes, superseded records'. The Markdown also merges everything into one Decisions section with no one-way marker, so an agent reading the debrief cannot tell one-way from reversible, while the HTML shows a one-way badge: the two forms disagree on a fact the test does not check. Malformed decision notes are dropped from both forms. docs/product/mission-reviews.md:22 repeats the order without superseded records. Fix: render sections like work decisions (One-way Decisions, Decision Records [current], Reversible Decisions, Open Decision Records, Superseded Decisions, Malformed) in both forms, update the doc, and assert the superseded-last order in a fixture.
