---
created_at: "2026-06-21T02:54:31.341182327+00:00"
id: "atelier-q15l"
issue_type: "task"
labels:
- "validation"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Require base branch for epic and mission workflow gates"
updated_at: "2026-06-21T02:54:52.266778235+00:00"
---

## Description

Add workflow guards so epic start transitions and mission terminal closeout
checks only pass from the configured base branch. Missions remain hardcoded for
now; the change must not make missions workflow-owned.

## Outcome

Epic `start` in the checked-in and starter workflow policy requires the current
checkout to be `branch_policy.base_branch`, and mission closeout uses the same
hardcoded terminal validator. Off-base attempts fail before branch preparation
or closeout mutation.

## Evidence

- test: `cargo nextest run -p atelier-cli -p atelier-workflow -E 'test(parses_valid_policy) or test(test_off_base_branch_blocks_mission_closeout) or test(test_epic_start_requires_base_branch)'`
- test: `cargo nextest run -p atelier-cli -E 'test(test_branch_actions_prepare_and_integrate_epic_workflow) or test(test_child_branch_prepare_action_checks_out_parent_epic_branch) or test(test_branch_integrate_action_failure_rolls_back_status_with_recovery)'`
- test: `cargo nextest run -p atelier-workflow`
- test: `cargo nextest run -p atelier-cli -E 'test(default_validators_are_target_and_transition_aware)'`
- check: `cargo fmt -- --check`
- check: `git diff --check`
