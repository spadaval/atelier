---
schema: atelier.review
schema_version: 1
id: atelier-p28x
title: finding on atelier-phfo @8d567d7f
target: atelier-phfo
identity: Independent reviewer (Claude Sonnet subagent)
commit: 8d567d7f4504d1d83a0e159f55bef11bb7d5feeb
created_at: 2026-10-23T14:41:34.755152324Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-cli/src/commands/review.rs:113,283-337 Two gaps against the Done-when text, neither affecting gates. (1) Author-identity warning is computed from merge-base(base, HEAD); recording from a checkout of the base (or detached at base) finds no authors and never warns, and an approval of a mission from a feature branch is judged against that branch's authors. Documented in the doc comment; acceptable. (2) The uv5d examples show 'Findings: 22 · 21 resolved · 1 open' and 'Approval: ✓ id @pin (identity) current' placed before the findings, with 'Other events'; the implementation prints rows, then 'Approval: ✓ current — id @pin by identity' and 'Findings: N disposed, M open'. It follows the increment goal text (rows then state) and carries the same information; no change needed.
