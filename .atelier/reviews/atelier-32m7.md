---
schema: atelier.review
schema_version: 1
id: atelier-32m7
title: approval on atelier-phfo @8d567d7f
target: atelier-phfo
identity: Independent reviewer (Claude Sonnet subagent)
commit: 8d567d7f4504d1d83a0e159f55bef11bb7d5feeb
created_at: 2026-10-23T14:41:35.163702873Z
event:
  event: approval
---

Ran full workspace nextest with and without test-hooks (900 and 894 pass), fmt and diff checks pass. Built a scratch repo and compared review list/show approval state with merge --check and land for: author-name and author-email approval (list missing, gate FAIL, warning printed), genuine approval (current both), code commit staling (stale with the commit and path, gate FAIL, land refused), clean base merge (current both), hand-resolved base merge (stale both), trivial exemption (current, then stale after code), done-claim variants stale/author/exempt compared with land, merged item viewed from main (accepted on main, land passes). approval_standing preserves approval_state's result (Current if any current, else Stale if any independent accepted, else Missing); independent() made pub with no logic change. New titles are generated; no reader uses a review record's header.title (readers use summary). Shared invalid-value form checked by running the commands (exit 2).
