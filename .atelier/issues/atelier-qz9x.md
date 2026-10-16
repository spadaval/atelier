---
created_at: "2026-10-16T02:55:55.608836824+00:00"
id: "atelier-qz9x"
issue_type: "bug"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "land on a work branch behind its base fails with a raw merge-base error"
updated_at: "2026-10-16T02:55:55.608836824+00:00"
---

## Goal

When the base branch has moved past a work branch's merge base, `atelier land` on the work branch fails with `git merge-base --is-ancestor <base tip> <head> failed:`. It gives no invariant name or recovery step. Seen on bug/atelier-0349 after master gained the b7hb approval.

## Done when

`land` (and `merge`) on a branch whose base tip is not an ancestor of HEAD report a named invariant, such as `land_base_ahead`, that says to merge the base into the branch. That advice should agree with atelier-j48s, which is about `merge_diverged` advising a rebase. A CLI test covers it.

## Context

The error comes from `ReadContext::edges` (git_freshness.rs), which requires the pin to be an ancestor of the end commit. Session evaluation uses the base tip as the claim base. This existed before atelier-0349: the old code had the same check, but master's binary failed earlier on the epic parse error.
