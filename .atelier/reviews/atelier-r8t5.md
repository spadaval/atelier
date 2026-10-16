---
schema: atelier.review
schema_version: 1
id: atelier-r8t5
title: 'crates/atelier-app/src/git_freshness.rs:322-375 and 619-645: the in-process tree diff and entry walk compare raw tree-entry mode strings, while git canonicalizes modes (canon_mode). A zero-padded directory mode ''040000'' or a legacy ''100664'' file mode is treated as a non-tree or a changed mode, and the git diff cross-check runs only on the first edge of each read phase. I traced the reachable cases: a canonical-to-crafted edge lists every path under the directory as changed and entry() returns None (the gate fails closed), and both sides can only be non-canonical once such an edge has already been crossed. So no gate is bypassed. Optional hardening: treat any non-canonical mode as a tree-diff mismatch and fall back to git diff.'
target: atelier-0349
identity: Independent reviewer (Claude subagent)
commit: 6d0300f6202777ceeb5a1b28f93efc8d5541dd00
created_at: 2026-10-16T02:50:22.202186391Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/git_freshness.rs:322-375 and 619-645: the in-process tree diff and entry walk compare raw tree-entry mode strings, while git canonicalizes modes (canon_mode). A zero-padded directory mode '040000' or a legacy '100664' file mode is treated as a non-tree or a changed mode, and the git diff cross-check runs only on the first edge of each read phase. I traced the reachable cases: a canonical-to-crafted edge lists every path under the directory as changed and entry() returns None (the gate fails closed), and both sides can only be non-canonical once such an edge has already been crossed. So no gate is bypassed. Optional hardening: treat any non-canonical mode as a tree-diff mismatch and fall back to git diff.
