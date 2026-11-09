---
created_at: "2026-11-09T02:01:46.682662332+00:00"
id: "atelier-1gb3"
issue_type: "mission"
labels:
- "forge"
strategy:
  path: docs/strategies/forge-delivery.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Take approved work to the published base in one command, through a forge"
updated_at: "2026-11-09T02:41:45.818322753+00:00"
---

## Outcome

In a repository whose policy names a forge, `atelier merge <id> --push` takes an approved branch to the published base in one command: it pushes the branch, makes sure it has a pull request, checks CI passed on the branch head's code, merges, pushes the base, and the pull request shows as merged. A base that moved by docs or records alone needs no new CI run. This repository runs that way, without `scripts/merge-ci-passed.sh` or the hand-run routine. A repository with no forge in its policy behaves as today. See `docs/strategies/forge-delivery.md`.

## Success criteria

- one-command-merge: in this repository, an approved item merges and publishes with `atelier merge <id> --push` alone, with no `gh` or `git push` by hand, while another thread commits a strategy doc to master after its CI run; shown by the transcript and the pull request's merged state.

## Background

Planned 2026-11-08 at Sumeet's direction after atelier-3sk9 needed three CI reruns for docs-only base changes. Decision atelier-vmq2 and ADR 0031 govern it.
