---
created_at: "2026-10-16T18:49:01.640765109+00:00"
id: "atelier-hyew"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Harden the base done-claim check"
updated_at: "2026-10-16T18:49:01.640765109+00:00"
---

## Goal

Close the gaps review found in atelier-fj1t's land_base_done_claim check.

## Done when

1. A record made unparsable in one unpushed commit and restored as done in the next is still refused: compare against the nearest parseable earlier version or the upstream record (atelier-2rse).
2. A test covers a done claim inside a conflicted (non-clean) pull merge, so the every-parent rule is exercised (atelier-q9a9, atelier-5krb).

## Context

Deferred findings from the atelier-fj1t reviews (approval atelier-oobm). crates/atelier-cli/src/commands/git.rs base_done_claim.
