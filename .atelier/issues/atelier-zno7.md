---
created_at: "2026-11-09T05:11:56.745208397+00:00"
id: "atelier-zno7"
issue_type: "loose_end"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-06jc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-5xvl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jqzl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-tvfd"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-zzwi"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Re-count agents' Atelier command overhead a week after the CLI fumble fixes ship"
updated_at: "2026-11-09T05:11:56.745208397+00:00"
---

## Goal

Check that the mission's fixes cut the turns agents spend on Atelier in real use, once the installed binary and skill on this host include all of them and agents have worked a week with them.

## Done when

- [ ] A note on this item gives, for a window of at least seven days after all five fixes are installed, the same per-role measures as the mission's Background (Sol reviews: turns per review and the share spent on `status`/`land`/`check`, history reads and record-help; non-reviewer `--help` lookups per session; failed `merge-ci-passed.sh` runs), next to the 2026-11-04 to 2026-11-09 figures, and files a bug for any pattern that did not fall.

## Context

The 2026-11-09 audit's extractor parsed Claude Code transcripts under `~/.claude/projects` and Codex rollouts under `~/.codex/sessions`; the mission's Background describes the method and cost model.
