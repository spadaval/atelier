---
schema: atelier.review
schema_version: 2
id: atelier-2ecb
title: crates/atelier-cli/src/commands/work.rs:128-160 and crates/atelier-app/src/objective_graph.rs:61 — apply_ready_execution_gates and dependency_readiness_batch are a second source of truth for blocker readiness. This change shows they are redundant with classify_issue/dependency_closure, but it keeps them. The rows they would move carry a synthetic 'readiness gate' blocker that can never appear. This complexity predates the change and is not made worse by it. File a follow-up to remove the gate and its batch walk, or record why it stays.
target: atelier-6wyv
identity: Independent reviewer (Claude subagent)
commit: 1380e97a1504d0cb1859aaaee518e7d616ea02ea
created_at: 2026-10-17T08:53:38.902103467Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 7f23f3420c9f83a4d40cb552a0319201fcaf68ce
    recorded_at: 2026-10-17T08:54:25.154930884Z
    follow_up: atelier-b17k
    reason: Redundant gate is pre-existing; filed atelier-b17k.
    migrated_from: atelier-9p4s
---

crates/atelier-cli/src/commands/work.rs:128-160 and crates/atelier-app/src/objective_graph.rs:61 — apply_ready_execution_gates and dependency_readiness_batch are a second source of truth for blocker readiness. This change shows they are redundant with classify_issue/dependency_closure, but it keeps them. The rows they would move carry a synthetic 'readiness gate' blocker that can never appear. This complexity predates the change and is not made worse by it. File a follow-up to remove the gate and its batch walk, or record why it stays.
