---
created_at: "2026-06-20T15:11:32.479521714+00:00"
id: "atelier-bz8g"
issue_type: "task"
labels:
- "cutting-pass"
- "removal"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-p1yz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Remove legacy command translation tests and obsolete ignored suite"
updated_at: "2026-06-20T21:29:26.392308622+00:00"
---

## Description

Remove test infrastructure that keeps obsolete command shapes alive through translation or ignored legacy suites. Replace remaining translated calls with current commands before deleting the translator layer.

## Outcome

Tests exercise the current CLI surface directly. Obsolete ignored tests for removed command families are deleted instead of preserved as permanent compatibility reminders.

## Evidence

- `rg "obsolete legacy command surface removed|issue next|issue tree|issue subissue|issue label|issue comment"` shows no remaining legacy translation layer or ignored obsolete suite.
- `cargo test -p atelier-cli --test cli_integration setup_guidance -- --nocapture` passes.
- Removed command rejection tests cover the intentionally unsupported old commands.
