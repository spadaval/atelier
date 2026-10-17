---
schema: atelier.review
schema_version: 1
id: atelier-ze12
title: 'crates/atelier-app/src/lint.rs:290: scoped ''check <mission>'' only examines receipts whose target is the mission itself, so renaming S1 in the strategy left ''atelier check atelier-r6bt'' passing while unscoped ''check'' reported 31 orphaned receipts on items in its scope. Decision et6q says a rename is never silent; unscoped check (and land) still catch it, so this is a gap in the scoped view only. Fix: when the scoped id is a mission, include receipts whose target is in its scope.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:23.466503778Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/lint.rs:290: scoped 'check <mission>' only examines receipts whose target is the mission itself, so renaming S1 in the strategy left 'atelier check atelier-r6bt' passing while unscoped 'check' reported 31 orphaned receipts on items in its scope. Decision et6q says a rename is never silent; unscoped check (and land) still catch it, so this is a gap in the scoped view only. Fix: when the scoped id is a mission, include receipts whose target is in its scope.
