---
created_at: "2026-10-16T19:14:57.806198561+00:00"
id: "atelier-kqt9"
issue_type: "bug"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Land on the base misreads claims when HEAD is a merge commit"
updated_at: "2026-10-16T19:14:57.806198561+00:00"
---

## Goal

With no work owner, `land` on the base compares done claims with `HEAD^`, its first parent. When HEAD is a merge commit, such as the tip `atelier merge` fast-forwards to after a branch merged the base in last, or a `git pull --no-rebase` merge, claims that arrived through the other parent look new. Their approvals are not retained at `HEAD^`, so land fails with `findings.disposed: ... unresolved FIX NOW` or `review.current`.

## Done when

Land on the base treats a claim as retained when it matches any parent of HEAD, using that parent for retention, and a test lands on a base whose tip is a merge that brought in an approved claim with a FIX NOW finding.

## Context

Found 2026-10-16: master at 71943018 (the 749l merge commit, first parent the 749l side) failed land with atelier-l6zy (atelier-83xi) and atelier-nr0w (atelier-lq42) unresolved, though both merged through the gates. Worked around by committing this record on top, so HEAD^ is the merge. crates/atelier-cli/src/commands/session.rs claim_base.
