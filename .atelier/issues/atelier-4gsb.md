---
created_at: "2026-10-30T06:49:44.817530340+00:00"
id: "atelier-4gsb"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Drop severity from the review request brief to match Agent Factory's review output"
updated_at: "2026-10-30T06:49:44.817530340+00:00"
---

## Goal

`atelier review request <id>` asks the reviewer for severity twice: the finding command's summary placeholder is `"<severity>: <path:line> <problem, impact, suggested fix>"` and the Report line asks for "findings (R-number, severity, recommendation)" (crates/atelier-cli/src/commands/review_request.rs:491 and :502 on master 2f1eea77). Agent Factory removed severity on purpose (spadaval/.agents a2c1050): its `procedures/review.md` Output uses the recommendation (FIX NOW, DEFER, NO ACTION) as the only scale and tells reviewers to ignore a brief that asks for severity. Reviewers get conflicting instructions, and findings recorded from the brief carry a label the process no longer uses.

## Done when

The brief's finding placeholder reads `"<path:line> <problem, impact, suggested fix>"`, its Report line lists findings as R-number, path:line and recommendation with no severity, and `the_brief_renders_as_dense_markdown` asserts the new text and fails if either `severity` string is restored. `git grep -n -i severity crates/atelier-cli/src/commands/review_request.rs` prints nothing.

## Scope

`review_request.rs` and its unit test only. Existing review records keep their summaries.

## Context

Skill output format: `skills/agent-factory/procedures/review.md`, Output, in spadaval/.agents (ADR 0023). Sibling atelier-abdp also changes this brief (round number); whichever lands second merges the other's text.
