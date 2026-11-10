---
schema: atelier.review
schema_version: 2
id: atelier-x7cu
title: finding on atelier-mal9 @016be038
target: atelier-mal9
identity: sol-mal9-r2
commit: 016be0387f56299f4fc3de5730bd960a50c99e61
created_at: 2026-11-10T15:28:30.218136773Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-records/src/document.rs:33 The fenced-example fix still treats code as record fields. unfenced_lines remembers only the first three fence characters and closes on starts_with(open), so a valid four-backtick markdown fence containing a triple-backtick text example exposes its contents. Reproduced at 016be038 with body lines ````markdown / ```text / Taken: atelier-bbbb by example-thread / ``` / ````: findings()[0].is_settled() is true and validate returns no problems. Putting Finding: 1-aaaa.1, Answer by example-thread: Example only and Outcome: no action in the same example closes an otherwise open fix-now finding; open_findings returns zero and validate accepts verdict approve. This leaves atelier-6a6v unresolved for ordinary fenced Markdown examples and violates the Records rule that fenced fields are never read. Track the opening marker character and full run length, accept only a matching closing run at least that long followed by whitespace, and add these regression cases for settlement and Decisions.
