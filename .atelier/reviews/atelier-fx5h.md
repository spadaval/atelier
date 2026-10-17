---
schema: atelier.review
schema_version: 1
id: atelier-fx5h
title: 'crates/atelier-cli/src/commands/work.rs:780: the metadata line renders ''Criteria: 8: 8 unproven'', a bare number with no noun. docs/product/human-cli-output.md Counts says a count is a plain phrase with a real plural and scope counts follow their size as ''Scope: 3 issues: 1 ready''. Fix: write it with count_noun, e.g. ''Criteria: 8 criteria: 8 unproven'' (or ''1 criterion''), and update issue-inventory-and-mission-overview.md and the CLI test assertions.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:23.208871383Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/work.rs:780: the metadata line renders 'Criteria: 8: 8 unproven', a bare number with no noun. docs/product/human-cli-output.md Counts says a count is a plain phrase with a real plural and scope counts follow their size as 'Scope: 3 issues: 1 ready'. Fix: write it with count_noun, e.g. 'Criteria: 8 criteria: 8 unproven' (or '1 criterion'), and update issue-inventory-and-mission-overview.md and the CLI test assertions.
