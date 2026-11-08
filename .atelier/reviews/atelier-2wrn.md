---
schema: atelier.review
schema_version: 3
id: atelier-2wrn
title: finding on branch docs/no-draft-prs @c4aed6be
branch: docs/no-draft-prs
identity: reviewer-nodraft
commit: c4aed6be336892b1c0746e6ca98c94b50790d772
created_at: 2026-11-08T15:13:54.387575635Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: orchestrator-g6rp
    commit: 6bd63021b977d6a23fce820766590a5dfd6982d7
    recorded_at: 2026-11-08T15:14:18.076522906Z
    reason: 'Without consequence for this branch: the g6rp decision note is a record on master, rewritten there separately to drop the draft reason'
---

minor: .atelier/issues/atelier-g6rp.md:29 the dated decision to keep file-pr out of Agent Factory gives as one reason that Atelier repositories keep a PR draft until review approves it, which this change makes untrue for this repo. It is a historical decision note, the decision still stands on its first reason, and no agent follows it as an instruction, so no edit is needed; whoever owns atelier-g6rp may want to note that the draft conflict is gone.
