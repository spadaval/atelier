---
created_at: "2026-06-13T20:37:01.941584397+00:00"
id: "atelier-rgd1"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "ux"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T23:50:13.622831Z"
status: "done"
title: "Audit focused output for common operator workflows"
updated_at: "2026-06-13T23:50:13.622831Z"
---

## Description

Review default output for status, mission status/show/list, issue show/list, evidence record/show/list, dependency/link/graph, worktree, lint, doctor, export, and rebuild. Identify output that exposes too much diagnostic detail, too little next-action context, or command lists that are not tied to the current purpose.

## Outcome

- Each audited command has a concise default answer and a documented path for more detail.
- Next actions are intent-labeled and do not send ordinary operators into low-level repair commands unless health is actually degraded.
- Quiet output remains minimal and composition-friendly.

## Evidence

- Command transcript artifact captures representative healthy, empty, blocked, degraded, and closeout-ready states.
- Tracker issue links or review artifact list every failed output classification and its follow-up implementation issue.
- Focused output tests or snapshot tests cover changed behavior.

## Notes

- Claimed manually because `atelier issue update atelier-rgd1 --claim` and
  `atelier start atelier-rgd1` could not run while canonical evidence
  `.atelier/evidence/atelier-06rb.md` is invalid.
- Output audit sampled `status`, `mission status`, `mission show`,
  `mission list`, `issue show`, `issue list --ready`, `evidence record/list`,
  dependency and link list output, `graph impact`, `worktree status`, `lint`,
  `doctor`, `export --check`, and `rebuild`.
- Failed classification found: fresh `atelier init` suggested issue creation
  before workflow setup. Fixed init output to route through
  `atelier workflow init` and `atelier workflow check`, updated product docs,
  and added a focused integration assertion.
- No additional failed output classifications were found inside the owned
  operator-output scope. Workflow policy internals, record-store internals, and
  status selection logic were intentionally left unchanged.
- Validation proof: `cargo test test_init_creates_atelier_directory`,
  `cargo test test_mission_status_cli_reports_control_state`,
  `cargo test test_orientation_commands_enter_degraded_mode_for_malformed_records`,
  `cargo test test_work_lifecycle_human_output_and_guards`,
  `cargo test command_surface`,
  `cargo test test_top_level_help_only_shows_core_commands`,
  `cargo test test_evidence_help_hides_predecessor_subcommands`,
  `cargo test test_mission_help_uses_show_not_view`,
  `cargo fmt -- --check`, and `git diff --check` passed.
- Tracker health proof: `atelier doctor` completed and reported
  `rebuild_ready: not ok` and `projection_fresh: not ok`; `atelier lint` and
  `atelier export --check` failed on pre-existing invalid canonical evidence
  `.atelier/evidence/atelier-06rb.md` (`Missing string front matter key
  'data'`), outside this issue's scope.
