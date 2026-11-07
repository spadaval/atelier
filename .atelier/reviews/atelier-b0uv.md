---
schema: atelier.review
schema_version: 2
id: atelier-b0uv
title: finding on atelier-8bh4 @70b45d46
target: atelier-8bh4
identity: independent reviewer subagent (claude-opus-5-5), atelier-8bh4
commit: 70b45d464142a56eba268b9270dc764ea7e13de8
created_at: 2026-11-07T09:40:08.475416381Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 83e9a3a91f34fb82d9281a01aba87137f9d86089
    recorded_at: 2026-11-07T09:46:07.455962295Z
    follow_up: atelier-ml3r
    reason: atelier-ml3r settles each repository's validation items before moving its policy, and fixes the doc (master 0dd66661).
---

low: docs/product/workflow-configuration.md:89 tells a repository without loose_end to copy the on_base workflow from a fresh init. Copied as written, it puts validation in two workflows (a clear policy error), and moving validation out of work leaves astromech's 6 backlog validations with a status on_base lacks, which workflow_issue_status_invalid turns into a refusal of every command (checked on a scratch clone). Defer to atelier-ml3r, which decides astromech's validation: the doc should say to add only loose_end to on_base unless validation moves too, and to bring parked validations to todo first.
