---
schema: atelier.review
schema_version: 2
id: atelier-aodt
title: finding on atelier-zpmb @c2642b25
target: atelier-zpmb
identity: gpt-6.1-sol-r1
commit: c2642b25f3d6053b00dfea9b623a50e9362dfb71
created_at: 2026-11-09T16:56:21.496557588Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: crates/atelier-site/src/lib.rs:275-280 counts repetitions of the original name but does not reserve the final URL name. Reproduced with three valid repositories in common-directory order whose committed project_slug values are demo, demo, demo-2: GET /api/repos lists demo, demo-2, demo-2. Both latter missions get the same links, and GET /api/demo-2/mission/demo-m001 returns repository b even when following the link for repository c. Thus a discovered repository cannot be served and another repository can answer its page. Allocate every final name in one shared unique namespace, including suffixed names and directory-name fallbacks, and test that all three repositories and their missions remain individually reachable.
