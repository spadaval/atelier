---
schema: atelier.review
schema_version: 1
id: atelier-g3d2
title: finding on atelier-nmvv @b4a22c66
target: atelier-nmvv
identity: independent planning reviewer subagent B (claude-opus-5-5)
commit: b4a22c6605b16025b6980cd44011eedae5e48f4e
created_at: 2026-10-30T06:58:30.229910733Z
event:
  event: finding
  recommendation: fix_now
---

R7: The Design's first escape does not exist. It says an item whose check can no longer be run 'closes only by revising its Done when ... or by supersede', but the gate never ages out a failure: evaluate_issue_gate (crates/atelier-app/src/evidence_freshness.rs:424-438) counts only passes captured after the newest fail, and editing Done when changes no receipt. Under this item's rule the captured fail still blocks close after any Done when revision, since a manual pass cannot clear it. A worker either builds nothing and the statement is false, or invents a mechanism (e.g. fails recorded before the latest Done when edit stop blocking), which would reopen the loosening shortcut. Fix: say supersede is the only exit, or specify the Done-when mechanism and add a test for it. The rest of R1 is resolved: same command string, the -- true case, and the criterion receipt.
