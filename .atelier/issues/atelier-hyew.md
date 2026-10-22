---
created_at: "2026-10-16T18:49:01.640765109+00:00"
id: "atelier-hyew"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-hyew
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-2rse"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-5krb"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-q9a9"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:41:22.395468651+00:00"
status: "done"
title: "Harden the base done-claim check"
updated_at: "2026-10-16T20:41:22.395468651+00:00"
---

## Goal

Close the gaps review found in atelier-fj1t's land_base_done_claim check.

## Done when

1. A record made unparsable in one unpushed commit and restored as done in the next is still refused: compare against the nearest parseable earlier version or the upstream record (atelier-2rse).
2. A test covers a done claim inside a conflicted (non-clean) pull merge, so the every-parent rule is exercised (atelier-q9a9, atelier-5krb).

## Context

Deferred findings from the atelier-fj1t reviews (approval atelier-oobm). crates/atelier-cli/src/commands/git.rs base_done_claim.

## Close Reason

base_done_claim reads earlier copies of a record directly or through the epic migration; when a copy still does not parse, the pushed upstream copy decides whether the item was already done, so breaking a record and restoring it as done is refused. Test base_done_claims_survive_a_broken_record_and_merges_judge_every_parent covers that and a conflicted pull merge whose resolution differs from both parents (land passes); replacing the upstream fallback or judging only the first parent each fails it. The epic migration test still passes.
