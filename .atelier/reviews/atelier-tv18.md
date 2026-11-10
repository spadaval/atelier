---
schema: atelier.review
schema_version: 2
id: atelier-tv18
title: finding on atelier-pz1y @e5ad73f8
target: atelier-pz1y
identity: sol-pz1y-r1
commit: e5ad73f85965ffb06f34863d8d3e8d66e86ca848
created_at: 2026-11-10T22:30:53.367384321Z
event:
  event: finding
  recommendation: fix_now
---

Low: next/crates/atelier-cli/src/check.rs:44: R2. The uncommitted-record list comes from repo::planning, which excludes .atelier/config.toml. Reproduced in a scratch repository: append a valid merge_check to tracked config.toml, confirm git status reports its modification, and run atelier check. It exits 0 with only Every record is valid and omits Record changes not committed yet entirely. Config is one of the records this command validates, so the first Done when line also requires listing its uncommitted changes. Gather the uncommitted record paths for check including config.toml, without changing its informational exit semantics; add a config-only modification regression test.
