---
created_at: "2026-10-22T23:08:36.697484181+00:00"
id: "atelier-u6xx"
evidence_type: "benchmark"
commit: "c5833bba72a3097a6c409f32903b490b43ccbb49"
result: "pass"
captured_at: "2026-10-22T23:08:36.697384055+00:00"
agent_identity: "claude-implementer-7b6e"
target:
  kind: "issue"
  id: "atelier-7b6e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7b6e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier land, release builds of c7e408c6 (before) and 98f4ffb5 (after), alternated on the same checkout, host load average 9-10 on 12 cores. This repository (3,136 commits, 363 merges, 1,086 review records) at c5833bba: median of 9 runs 3,891 ms before, 1,375 ms after; GIT_TRACE_PERFORMANCE shows 126 git log walks (11.4 s of Git time) replaced by one rev-list and one diff-tree (0.29 s); output identical. Benchmark history fixture (scripts/bench_read_commands.py --history-owners 60, 1,745 commits), land <owner>: median of 7 runs 3,510 ms before, 1,012 ms after, 181 log walks to none, output identical. Guard fixture land atelier-s000 (one approval): median of 11 runs 148 ms before, 145 ms after. atelier doctor here: 22.6 s before, 11.9 s after, output identical. Remaining land time after the change: about 0.15 s before the first history query, 0.3 s graph load, 0.8 s across about 310 serial merge-base and rev-list calls from approval checks."
updated_at: "2026-10-22T23:08:36.698005592+00:00"
---

atelier land, release builds of c7e408c6 (before) and 98f4ffb5 (after), alternated on the same checkout, host load average 9-10 on 12 cores. This repository (3,136 commits, 363 merges, 1,086 review records) at c5833bba: median of 9 runs 3,891 ms before, 1,375 ms after; GIT_TRACE_PERFORMANCE shows 126 git log walks (11.4 s of Git time) replaced by one rev-list and one diff-tree (0.29 s); output identical. Benchmark history fixture (scripts/bench_read_commands.py --history-owners 60, 1,745 commits), land <owner>: median of 7 runs 3,510 ms before, 1,012 ms after, 181 log walks to none, output identical. Guard fixture land atelier-s000 (one approval): median of 11 runs 148 ms before, 145 ms after. atelier doctor here: 22.6 s before, 11.9 s after, output identical. Remaining land time after the change: about 0.15 s before the first history query, 0.3 s graph load, 0.8 s across about 310 serial merge-base and rev-list calls from approval checks.
