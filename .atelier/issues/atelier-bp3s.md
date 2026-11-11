---
created_at: "2026-11-06T23:39:07.679326401+00:00"
id: "atelier-bp3s"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-s66u"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add a suite merge check to demiurge, cosplay-studio and life-on-rails"
updated_at: "2026-11-06T23:39:07.679326401+00:00"
---

## Goal

Give demiurge, cosplay-studio and life-on-rails a test of the merged tree. They have no CI (life-on-rails' Android builds run off-box, so its check may be limited to what runs on this host) and no suite merge check (life-on-rails has only a whitespace check).

## Done when

- [ ] Each repository's `.atelier/config.toml` names a merge check that runs its test suite on the merged tree, and one `atelier merge` there shows the check running and a failing suite refusing the merge.

## Context

Pick this up once the cutover has converted each repository (atelier-wd1i). It is work in other repositories. Strategy: ~/projects/atelier/docs/strategies/lighter-workflow.md (Scope, Risks).

Rewritten for the new tool in the cutover sort (atelier-5zre): `workflow.yaml`'s `merge_checks` became `config.toml`'s merge checks, and the sentence about `merge --sync` rerunning suite receipts went with the receipts.
