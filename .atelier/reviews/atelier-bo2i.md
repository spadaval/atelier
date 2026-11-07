---
schema: atelier.review
schema_version: 2
id: atelier-bo2i
title: finding on atelier-md0v @c4586155
target: atelier-md0v
identity: independent reviewer subagent (claude-opus-5-5), atelier-md0v
commit: c4586155e0908d88b52e5855c91c02fc66092091
created_at: 2026-11-07T09:35:43.640015481Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/session.rs:774 on_work_branch decides 'work in progress' from the working-tree policy, while land_gates judges the base from HEAD's committed policy (and merge's owned() from HEAD's too). On the base with an uncommitted edit of branch_policy.base_branch, land --push reports land_base_unreviewed as work in progress, exits 0 and pushes the unreviewed base commit to origin/<base>. Reproduced in a scratch repo: 'land: work in progress — 2 checks open (land_base_unreviewed, land_records_uncommitted), pushed to origin/main'; afterwards land on the base passes, since the upstream now holds the commit. The Stop hook stops blocking there for the same reason. A rename of the base branch, edited in the policy before it is committed, is the likely trigger. This lets land on the base skip the one refusal the Done when keeps (unreviewed code reaching the base) and judges the base by two sources in one run. Fix: decide it from the same committed policy land_gates uses (base_policy::from_ref(repo, "HEAD"), as merge does), or have land_checked report where it ran; test an uncommitted base_branch edit on the base.
