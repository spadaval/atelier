---
schema: atelier.review
schema_version: 1
id: atelier-9a6e
title: finding on atelier-0uaj @46a269e9
target: atelier-0uaj
identity: Independent reviewer (Claude Sonnet subagent)
commit: 46a269e98ef40a08ce787c618d8dd0f7b5e42bee
created_at: 2026-10-23T15:58:52.343424033Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/session.rs, merge.rs: invalid-policy precondition fix is prose ('fix .atelier/workflow.yaml and commit it', not a command) and land_base_mismatch has no Next at all; the fix helper (next_steps.rs fixes()) silently drops non-command fixes so tests never see them. Mission rule says a fix line must be a runnable command. Also 'land <id>' unknown id prints 'issue nope not found' not the 'no issue matches' wording in atelier-ns04 Done when (names the ID, so equivalent). Land from a non-owner branch with tracker changes suggests 'atelier start <id>' though a local branch already owns the item (git switch would be right).
