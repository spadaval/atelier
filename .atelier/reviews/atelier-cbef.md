---
schema: atelier.review
schema_version: 1
id: atelier-cbef
title: 'crates/atelier-cli/src/commands/workflow.rs:315 — the transition refusal still says ''terminal ancestor X retires nonterminal descendant Y'', but this change''s human-cli-output.md:99 says work that is not done is ''open, never nonterminal'', and Done when asks for ''open'' instead of ''nonterminal''. Failure: a start under a done parent prints a word the new contract forbids. Fix: write ''open descendant'', or narrow the doc sentence to counts.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 23527f8026ade3ebd70c6740ddb995c23d25fe87
created_at: 2026-10-17T07:38:21.001833222Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/workflow.rs:315 — the transition refusal still says 'terminal ancestor X retires nonterminal descendant Y', but this change's human-cli-output.md:99 says work that is not done is 'open, never nonterminal', and Done when asks for 'open' instead of 'nonterminal'. Failure: a start under a done parent prints a word the new contract forbids. Fix: write 'open descendant', or narrow the doc sentence to counts.
