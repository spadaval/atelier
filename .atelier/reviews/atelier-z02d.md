---
schema: atelier.review
schema_version: 2
id: atelier-z02d
title: finding on atelier-pz1y @19254e96
target: atelier-pz1y
identity: sol-pz1y-r2
commit: 19254e96331516cfca73c4aac5751c7b2a0f88da
created_at: 2026-11-10T22:36:51.619445663Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/check.rs:31: R3. check omits ignored untracked records because git::files uses ls-files --others --exclude-standard. Reproduced in a scratch repository with a tracked valid config: create .atelier/items/demo-ab12/item.md containing no front matter and exclude its directory in .git/info/exclude. atelier check exits 0 and prints Every record is valid: 0 items, even while listing that same item as an uncommitted record change; atelier new then fails reading it. This violates validation of every record in the checkout and gives a false clean result. Enumerate ignored records too for check (keep the staged-only hook behavior), and cover ignored malformed items, missions and reviews.
