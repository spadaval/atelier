---
schema: atelier.review
schema_version: 2
id: atelier-9a6e
title: finding on atelier-0uaj @46a269e9
target: atelier-0uaj
identity: Independent reviewer (Claude Sonnet subagent)
commit: 46a269e98ef40a08ce787c618d8dd0f7b5e42bee
created_at: 2026-10-23T15:58:52.343424033Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 580b5de04abdfbb3dfbcea538311a4483fa3a9a0
    recorded_at: 2026-10-23T16:06:36.752580140Z
    resolution_commit: 1ac26495b9012b9615d9eaa522cc72906f4acf96
    reason: 'base_mismatch had no fix and owned items got atelier start; fixed. Invalid-policy prose fix and not-found wording kept: only a person can edit the policy, and the error names the ID.'
    migrated_from: atelier-h6cd
---

low: crates/atelier-cli/src/commands/session.rs, merge.rs: invalid-policy precondition fix is prose ('fix .atelier/workflow.yaml and commit it', not a command) and land_base_mismatch has no Next at all; the fix helper (next_steps.rs fixes()) silently drops non-command fixes so tests never see them. Mission rule says a fix line must be a runnable command. Also 'land <id>' unknown id prints 'issue nope not found' not the 'no issue matches' wording in atelier-ns04 Done when (names the ID, so equivalent). Land from a non-owner branch with tracker changes suggests 'atelier start <id>' though a local branch already owns the item (git switch would be right).
