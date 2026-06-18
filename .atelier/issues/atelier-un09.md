---
created_at: "2026-06-18T22:41:51.377989902+00:00"
id: "atelier-un09"
issue_type: "epic"
labels:
- "architecture"
- "pr"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0wyy"
  - kind: "issue"
    id: "atelier-cp7i"
  - kind: "issue"
    id: "atelier-r0k7"
  - kind: "issue"
    id: "atelier-tilv"
  - kind: "issue"
    id: "atelier-vd9e"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Harden PR artifact workflow seams"
updated_at: "2026-06-18T22:42:55.142286766+00:00"
---

## Description

Harden the PR artifact workflow so Agent Factory review artifacts are reliable
during ordinary work. The current shape is conceptually sound: PRs are review
artifacts, Forgejo owns remote review policy, and Atelier workflow transitions
remain explicit. The weak seams are around failure atomicity, command
ergonomics, guidance drift, and a remaining CLI-owned orchestration boundary.

This epic groups focused fixes that make PR artifact handling less fragile. It
does not require Atelier to duplicate Forgejo branch-protection or approval
policy.

## Outcome

- `atelier pr open` cannot create an unlinked remote PR because local branch
  ownership and target branch expectations are checked before the Forgejo
  create call.
- PR discussion commands show or clearly distinguish the comment/review
  surfaces operators need during review.
- Product docs and role guidance describe the current PR artifact workflow,
  validator role, and Forgejo policy boundary without contradicting command
  behavior.
- Larger command-boundary cleanup for PR orchestration is tracked separately so
  it can be handled at the right architecture boundary.

## Evidence

- Child issue evidence proves each focused behavior with targeted tests or CLI
  transcripts.
- Epic closeout maps each outcome above to child proof and attached validation
  evidence.

## Notes

Suggested first task: `atelier-r0k7`.
