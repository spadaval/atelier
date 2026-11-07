---
schema: atelier.review
schema_version: 2
id: atelier-s4rd
title: 'crates/atelier-cli/src/human_output.rs:677 — the doc comment on render_unassigned_groups still says it writes a ''<label> (<count>)'' line per group; this change made the helper write ''<label>: N open issues'', so the shared helper''s own contract now contradicts its output and the product doc. One-line comment fix.'
target: atelier-u3vj
identity: Independent reviewer (Claude subagent)
commit: b8288f8555508d70806ebecd8ed5197ffe4f426c
created_at: 2026-10-17T09:45:13.990439657Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 06739335351f0973408dda1e032298644e735650
    recorded_at: 2026-10-17T09:48:18.339839400Z
    resolution_commit: 371ecf05393267ecfacc354cf622589980100aa7
    reason: Doc comment describes the label-first count.
    migrated_from: atelier-7zk8
---

crates/atelier-cli/src/human_output.rs:677 — the doc comment on render_unassigned_groups still says it writes a '<label> (<count>)' line per group; this change made the helper write '<label>: N open issues', so the shared helper's own contract now contradicts its output and the product doc. One-line comment fix.
