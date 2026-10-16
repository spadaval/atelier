---
schema: atelier.review
schema_version: 1
id: atelier-2mno
title: 'crates/atelier-cli/src/commands/status.rs:16-23,85,133-136: status still counts needs-human items as ready work, so it now disagrees with the list it points to. Reproduced in a /tmp scratch repo: atelier -q work ready printed 3 IDs, while atelier -q status printed ready=4 and the page said ''Choose ready work (4 ready issue(s) available): atelier work ready''. If every ready item is needs-human, status tells an agent N items are available, work ready -q prints nothing, and the ''Inspect blocked work'' next action never appears. Before this change the two counts always matched. Fix: split view.work.ready on row.needs_human in status, the same way list() does: count only agent-ready items as ready, add a separate needs-human count (ready=N human=M in quiet output), and base the next action on the agent-ready count.'
target: atelier-q8ok
identity: Independent reviewer (Claude subagent)
commit: bd03289966af07bb8f13ec3c855590bf6e8504af
created_at: 2026-10-16T00:09:55.757738313Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/status.rs:16-23,85,133-136: status still counts needs-human items as ready work, so it now disagrees with the list it points to. Reproduced in a /tmp scratch repo: atelier -q work ready printed 3 IDs, while atelier -q status printed ready=4 and the page said 'Choose ready work (4 ready issue(s) available): atelier work ready'. If every ready item is needs-human, status tells an agent N items are available, work ready -q prints nothing, and the 'Inspect blocked work' next action never appears. Before this change the two counts always matched. Fix: split view.work.ready on row.needs_human in status, the same way list() does: count only agent-ready items as ready, add a separate needs-human count (ready=N human=M in quiet output), and base the next action on the agent-ready count.
