---
created_at: "2026-10-01T22:17:19.659358204+00:00"
id: "atelier-3w39"
evidence_type: "benchmark"
commit: "3012fdcec368de842e977758a362dbc4a7cd5c5a"
result: "fail"
captured_at: "2026-10-01T22:17:19.659192618+00:00"
exit_status: "fail"
path: "/tmp/atelier-session-bench-r4-committed.log"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xz11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Earlier session release benchmark exceeded 100 ms"
updated_at: "2026-10-01T22:17:19.659914576+00:00"
---

Historical failed run on session commit 3012fdcec368de842e977758a362dbc4a7cd5c5a: issue show maximum 120.0 ms exceeded S4. The initial run is retained; a later recheck passed, and final integrated acceptance still requires a fresh production benchmark. This record captures the earlier report, not a rerun on the current commit.
