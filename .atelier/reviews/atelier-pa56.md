---
schema: atelier.review
schema_version: 2
id: atelier-pa56
title: 'crates/atelier-cli/src/main.rs:223: ''atelier check [ID]'' help still says ''Optional issue ID or imported source ID''. This is the same leftover from the old import model that item 2 removed from --parent, and lint.rs resolve_issue_id only resolves issue IDs and key suffixes, so the help claims a behaviour that does not exist. Fix: ''Optional issue ID'' (one-line change) and extend the help assertion in test_issue_options_do_what_their_help_says.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.976967776Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 0f489f2f14870835465001d44cd3f6e024830c81
    recorded_at: 2026-10-15T23:50:09.813551353Z
    resolution_commit: 0f489f2f14870835465001d44cd3f6e024830c81
    reason: Fixed in 0f489f2f14870835465001d44cd3f6e024830c81
    migrated_from: atelier-51x3
---

crates/atelier-cli/src/main.rs:223: 'atelier check [ID]' help still says 'Optional issue ID or imported source ID'. This is the same leftover from the old import model that item 2 removed from --parent, and lint.rs resolve_issue_id only resolves issue IDs and key suffixes, so the help claims a behaviour that does not exist. Fix: 'Optional issue ID' (one-line change) and extend the help assertion in test_issue_options_do_what_their_help_says.
