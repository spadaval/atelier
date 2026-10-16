---
created_at: "2026-10-15T22:08:58.936198054+00:00"
id: "atelier-9m4k"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-9m4k
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T23:53:26.797888557+00:00"
status: "done"
title: "Link a mission and its strategy without a circular edit"
updated_at: "2026-10-15T23:53:26.797888557+00:00"
---

## Goal

A mission's strategy path can only be set by a bundle; no CLI command sets or changes it. The strategy file's front matter needs `mission: <id>`, which doesn't exist until the mission is created, so the strategy has to be edited after every create or re-import.

## Done when

`issue update <mission> --strategy <path>` sets the path, and the link is resolvable from one side only (for example the mission's strategy path is canonical and `check` reports a strategy whose front matter disagrees), or create fills the strategy's front matter in. Documented in the mission guide.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). astromech re-imported its bundle four times during review, and each time the strategy's `mission:` had to be fixed by hand.

## Close Reason

issue create/update --strategy <path> set a mission's strategy (missions only). The mission's path is canonical; check reports a missing strategy file and front matter whose mission: names a different mission; strategies need not name their mission. Documented in docs/product/mission-authoring.md. Test: test_mission_strategy_is_set_from_the_cli_and_checked_from_the_mission_side.
