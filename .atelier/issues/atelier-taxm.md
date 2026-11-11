---
created_at: "2026-11-08T07:07:49.807640842+00:00"
id: "atelier-taxm"
issue_type: "task"
labels:
- "agent-factory"
- "setup"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:57.017713230+00:00"
status: "declined"
title: "Use one function to detect a skill linked to a checkout, in setup --link and the update check"
updated_at: "2026-11-11T03:55:57.017713230+00:00"
---

## Goal

`atelier-45gr` (`freshness::linked_checkout`) and `atelier-8bj8` (`setup --link`) each detect whether the installed agent-factory skill is a symlink into a checkout. Two copies of the rule can drift, so a skill one reports as linked the other could treat as a copy.

## Done when

- [ ] One function decides whether `~/.agents/skills/agent-factory` or `~/.claude/skills/agent-factory` is linked to a checkout and which one, and both `setup --link`/`--unlink` and the update check call it. Existing tests for both still pass.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new binary has no `setup --link` and no update check (`atelier setup --help`), so there is no second copy of the rule.
