---
schema: atelier.review
schema_version: 1
id: atelier-pa56
title: 'crates/atelier-cli/src/main.rs:223: ''atelier check [ID]'' help still says ''Optional issue ID or imported source ID''. This is the same leftover from the old import model that item 2 removed from --parent, and lint.rs resolve_issue_id only resolves issue IDs and key suffixes, so the help claims a behaviour that does not exist. Fix: ''Optional issue ID'' (one-line change) and extend the help assertion in test_issue_options_do_what_their_help_says.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.976967776Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/main.rs:223: 'atelier check [ID]' help still says 'Optional issue ID or imported source ID'. This is the same leftover from the old import model that item 2 removed from --parent, and lint.rs resolve_issue_id only resolves issue IDs and key suffixes, so the help claims a behaviour that does not exist. Fix: 'Optional issue ID' (one-line change) and extend the help assertion in test_issue_options_do_what_their_help_says.
