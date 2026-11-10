---
schema: atelier.review
schema_version: 2
id: atelier-4e2b
title: finding on atelier-dxrj @ff2b4a8a
target: atelier-dxrj
identity: sol-dxrj-r1
commit: ff2b4a8a5540c069cd8e1c18887fede7432a5e81
created_at: 2026-11-10T22:25:13.576126171Z
event:
  event: finding
  recommendation: fix_now
---

low: next/crates/atelier-cli/src/view_output.rs:171 show does not print a settlement committed on an implementer branch, contrary to Done when. Reproduced with a done item demo-d0ne and a building item demo-ab12 held by t9: on demo-ab12, t9 runs drop demo-d0ne/1-aaaa.1 --reason Obsolete and commits the review change. list --deferred correctly removes the finding, but show demo-d0ne still prints 1-aaaa.1 (defer): Defer this, with no settlement. views::show uses repo.locate, which selects the done item from the base tip, so even committing the write does not make it visible until the unrelated item merges. Read the local settlement when displaying a done finding in the checkout where it was settled, while preserving the normal item lookup rules. Test show after each settlement on both the base and an implementer branch; the current test checks show only after a base commit.
