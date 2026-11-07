---
schema: atelier.review
schema_version: 2
id: atelier-7eur
title: 'docs/product/human-cli-output.md:99 — the widened rule says ''never nonterminal or terminal'', but output deliberately keeps workflow-configuration vocabulary: the work.terminal validator name (an interface in workflow.yaml and migrate notes) and the atelier-workflow errors such as "workflow done_statuses must contain at least one terminal status" and "cannot leave terminal status". The doc''s absolute claim is false for those paths. Follow-up: scope the sentence to work-state wording and name the configuration exception.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 6b4ae1c4e2f97d5c341a19336a6496db5a49c521
created_at: 2026-10-17T07:50:22.690293104Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 3182baa02c08987563558eb8ddf6b10b17fc18ca
    recorded_at: 2026-10-17T07:57:29.307994171Z
    resolution_commit: 812acbd01a11b2bf1c99856a7159ae4815dc5376
    reason: 'Fixed now: the rule covers work-state wording and names the workflow-configuration exception.'
    migrated_from: atelier-eaor
---

docs/product/human-cli-output.md:99 — the widened rule says 'never nonterminal or terminal', but output deliberately keeps workflow-configuration vocabulary: the work.terminal validator name (an interface in workflow.yaml and migrate notes) and the atelier-workflow errors such as "workflow done_statuses must contain at least one terminal status" and "cannot leave terminal status". The doc's absolute claim is false for those paths. Follow-up: scope the sentence to work-state wording and name the configuration exception.
