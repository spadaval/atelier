---
created_at: "2026-06-18T22:42:09.947786211+00:00"
id: "atelier-vd9e"
issue_type: "task"
labels:
- "docs"
- "forgejo"
- "pr"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-18T23:34:37.505101138+00:00"
status: "done"
title: "Record Forgejo merge-policy boundary for Atelier PR commands"
updated_at: "2026-06-18T23:34:37.505101138+00:00"
---

## Description

Atelier should not silently duplicate Forgejo branch-protection and review
policy. The current code checks facts Atelier depends on, such as linked PR
number, branch match, and merged state, while Forgejo should own fine-grained
approval and merge policy. Record that boundary so future validators and PR
commands do not drift into a second policy engine without a deliberate ADR or
product decision.

## Outcome

- Product or architecture docs state which PR facts Atelier validates locally
  and which review/merge policies are delegated to Forgejo.
- `linked_pr_merged`, `review_complete`, and `pr merge` docs/help no longer
  imply stronger review-policy enforcement than the code provides.
- Any remaining ambiguity about merge strategy is recorded as either an
  intentional Forgejo-owned policy or a follow-up implementation item.

## Evidence

- Documentation diff maps each PR workflow claim to the command or Forgejo
  policy surface that owns it.
- Search transcript over docs and role guides shows no contradictory claim that
  Atelier enforces Forgejo approval policy.
- `target/debug/atelier lint` passes.
