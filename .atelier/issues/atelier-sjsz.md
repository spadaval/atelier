---
created_at: "2026-07-06T17:20:25.693972086+00:00"
id: "atelier-sjsz"
issue_type: "task"
labels:
- "cli"
- "docs"
- "mission-dashboard"
- "tests"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-07T06:15:06.274296382+00:00"
status: "done"
title: "Align Mission Overview guidance and regression coverage"
updated_at: "2026-07-07T06:15:06.274296382+00:00"
---

## Description

Align `work missions` help, root examples, role guidance, product docs, and focused tests with the finished overview. Cover default hidden done missions, explicit done inclusion, linked epics, collapsed subtasks, direct and unassigned work, ordering, limits, empty state, quiet mode, TTY color, `NO_COLOR`, and drill-down guidance.

## Outcome

- Help and durable guidance call the surface Mission Overview, distinguish plural `work missions` from scoped `work mission <id>`, and do not route hierarchy back through `issue list` or the legacy broad queue.
- Focused coverage fails if done missions leak into the default, epics lose mission context, leaf subtasks expand by default, color policy regresses, or colorless structure becomes incomplete.

## Evidence

- Help and role-guidance parity: `cargo test -p atelier-cli --test cli_integration setup_guidance::test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views -- --exact`.
- Delivered CLI behavior, including done inclusion, collapsed work, exceptional accounting, quiet output, colorless output, and drill-downs: focused `mission_cache_worktree::test_work_missions_*` integration tests.
- Projection ordering, limits, membership, cycle safety, and hidden-done accounting: `cargo nextest run -p atelier-app -E 'test(mission_overview)'`.
- Recorded CLI behavior evidence: `atelier evidence show atelier-r8uf`.
- Recorded help and role-guidance parity evidence: `atelier evidence show atelier-phev`.
