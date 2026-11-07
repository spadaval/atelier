---
schema: atelier.review
schema_version: 2
id: atelier-tv24
title: approval on atelier-4nsl @c22e2999
target: atelier-4nsl
identity: independent reviewer subagent (claude-opus-5-5), atelier-4nsl
commit: c22e2999ee19148466c07cd9fde0a08c7bc85177
created_at: 2026-11-07T11:31:41.884266325Z
event:
  event: approval
---

Re-review of the fix diff 9d658287..c22e2999 resolving atelier-lezj, atelier-4t47, atelier-tzvy and atelier-a35g. R1: owned_when_merged now parses every work_branch record on the merged head and compares the parsed name; a failing git grep or unreadable blob refuses; an unparseable record mentioning the branch counts as an owner (conservative); test covers taken and 1.0 (asserts the quoted record). R2: status's branch list drops an owner done on the base when the branch holds commits the base lacks, so light_row lists it; merged branches keep their done row; test covers the reused-name case. R3: prune counts branch records' references and never makes them candidates; test prunes one retired issue and keeps the follow-up. R4: architecture and product docs state versions 2 and 3 and the older-binary consequence. Ran: full nextest with and without test-hooks (1009 and 1015 passed), git diff --check, cargo fmt --check, cargo metadata --locked. CI on c22e2999 was still pending when recorded; merge-ci-passed.sh gates the merge.
