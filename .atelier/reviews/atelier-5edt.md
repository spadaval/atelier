---
schema: atelier.review
schema_version: 2
id: atelier-5edt
title: finding on atelier-5cuc @13b69a03
target: atelier-5cuc
identity: gpt-6.1-sol
commit: 13b69a03ccaa9164f77a70f6c11c10a08ce51a99
created_at: 2026-11-10T16:54:32.044836873Z
event:
  event: finding
  recommendation: fix_now
---

low: next/crates/atelier-cli/src/commands.rs:170 Return the role refusal before parsing the reviewer’s draft. After review binds t2, append a Finding heading without its Recommendation line and run submit as t2. check correctly returns Refusal::Role(Reviewer), but refusal then evaluates entry(facts); the new acts::next parses the unfinished finding and returns its parse error first. The command exits 2 without the required one-line message naming the reviewer role and review/verdict commands. Reproduced with the built binary in /home/supadava/.cache/atelier-fcb7/review-5cuc-prq_aodi (current reviewer t5, round 3); HEAD and working files stayed unchanged. Render role refusals without constructing an Entry, and extend the wrong-role scenario to an unfinished review body. This blocks the explicit reviewer-submit Done when line, even though all 78 tests, formatting, metadata, clippy, all-targets build, and whitespace checks passed.
