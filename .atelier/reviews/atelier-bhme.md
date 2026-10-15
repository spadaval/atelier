---
schema: atelier.review
schema_version: 1
id: atelier-bhme
title: Caveat on the fix suggested in atelier-ucis (crates/atelier-cli/src/commands/git.rs:170-196); no separate change needed beyond doing ucis safely. The start^ bound does protect one case that start.rs's new check misses. That check runs only when start is invoked on the base. If someone branches from a base holding unpushed direct code D (with plain git switch -c, not atelier start), then runs 'atelier start' on that branch, merge reviews only base...branch, so D is never reviewed. A plain merge-base-with-upstream bound would then credit D. If ucis loosens the bound, also run check_base_commits (or an equivalent 'commits in the local base and not in upstream') against the existing branch in start, so D is refused there. Alternatively keep start^ but extend it back to the branch's fork from the local base.
target: atelier-3mjv
identity: Independent reviewer (Claude subagent)
commit: b6101b07af6016c366540c1caa3ccda7bd31a85d
created_at: 2026-10-15T23:36:12.368639Z
event:
  event: finding
  recommendation: no_action
---

Caveat on the fix suggested in atelier-ucis (crates/atelier-cli/src/commands/git.rs:170-196); no separate change needed beyond doing ucis safely. The start^ bound does protect one case that start.rs's new check misses. That check runs only when start is invoked on the base. If someone branches from a base holding unpushed direct code D (with plain git switch -c, not atelier start), then runs 'atelier start' on that branch, merge reviews only base...branch, so D is never reviewed. A plain merge-base-with-upstream bound would then credit D. If ucis loosens the bound, also run check_base_commits (or an equivalent 'commits in the local base and not in upstream') against the existing branch in start, so D is refused there. Alternatively keep start^ but extend it back to the branch's fork from the local base.
