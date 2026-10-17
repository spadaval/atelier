---
schema: atelier.review
schema_version: 1
id: atelier-i5n2
title: 'crates/atelier-cli/src/commands/issue.rs:570 — issue show --full''s Transition Readiness still prints each transition''s command as a bare line (''    atelier issue transition <id> block''), the unlabeled format the Goal names for retirement, while issue transition options now prints the same pointer as ''Command: ...'' (workflow.rs:419) and human-cli-output.md''s new Next Footer rule says a transition''s Command: line reads ''<label>: <command>''. Scenario: run ''atelier issue show <id> --full'' on any issue with an allowed transition; the bare lines appear. The new fixture test runs ''issue show --full'' but checks only the Next section, so it passes. Fix: print ''    Command: {}'' and assert it in rendered_views_use_one_next_footer_and_no_retired_headings.'
target: atelier-onwh
identity: Independent reviewer (Claude subagent)
commit: 81e00ba8711cfead5a7dd7743deb6cf53d00af80
created_at: 2026-10-17T07:08:38.017475261Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/issue.rs:570 — issue show --full's Transition Readiness still prints each transition's command as a bare line ('    atelier issue transition <id> block'), the unlabeled format the Goal names for retirement, while issue transition options now prints the same pointer as 'Command: ...' (workflow.rs:419) and human-cli-output.md's new Next Footer rule says a transition's Command: line reads '<label>: <command>'. Scenario: run 'atelier issue show <id> --full' on any issue with an allowed transition; the bare lines appear. The new fixture test runs 'issue show --full' but checks only the Next section, so it passes. Fix: print '    Command: {}' and assert it in rendered_views_use_one_next_footer_and_no_retired_headings.
