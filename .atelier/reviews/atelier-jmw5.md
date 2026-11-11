---
schema: atelier.review
schema_version: 2
id: atelier-jmw5
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: GPT-6 Astra independent planning reviewer A
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:23:25.920167709Z
event:
  event: finding
  recommendation: fix_now
---

R2 (medium) docs/strategies/desk.md:89-92: Reading only the base commit and caching by that commit loses active work. The amended redesign-architecture.md:616-628 explicitly requires the Desk to apply the same branch selection as the CLI; next/crates/atelier-cli/src/repo.rs:80-105,123-137 and atelier-app/src/view.rs:25-35 select an unmerged item's branch unless the base is terminal. When an item starts, submits, is reviewed or escalates without master moving, the proposed page stays ready, misses its holder/questions/reviews, and fails desk-matches-cli even when show runs on the base branch. Specify captured base and relevant branch tips, use the core's selection rule, and key/invalidate cached views on all these inputs. Make atelier-1ie1 map this data flow and require a comparison with an active committed item branch, including a branch-only change.
