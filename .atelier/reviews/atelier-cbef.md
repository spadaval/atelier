---
schema: atelier.review
schema_version: 2
id: atelier-cbef
title: 'crates/atelier-cli/src/commands/workflow.rs:315 — the transition refusal still says ''terminal ancestor X retires nonterminal descendant Y'', but this change''s human-cli-output.md:99 says work that is not done is ''open, never nonterminal'', and Done when asks for ''open'' instead of ''nonterminal''. Failure: a start under a done parent prints a word the new contract forbids. Fix: write ''open descendant'', or narrow the doc sentence to counts.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 23527f8026ade3ebd70c6740ddb995c23d25fe87
created_at: 2026-10-17T07:38:21.001833222Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 4e8e51630991ab8cf13516ff566284c5e05eb760
    recorded_at: 2026-10-17T07:48:32.737190144Z
    resolution_commit: 5927bc001a2354c3e2a5e1c54f0953843e78bc94
    reason: User-facing nonterminal/terminal now read open/closed; workflow-config terms kept.
    migrated_from: atelier-0xoo
---

crates/atelier-cli/src/commands/workflow.rs:315 — the transition refusal still says 'terminal ancestor X retires nonterminal descendant Y', but this change's human-cli-output.md:99 says work that is not done is 'open, never nonterminal', and Done when asks for 'open' instead of 'nonterminal'. Failure: a start under a done parent prints a word the new contract forbids. Fix: write 'open descendant', or narrow the doc sentence to counts.
