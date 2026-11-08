---
created_at: "2026-11-08T05:35:31.390561285+00:00"
id: "atelier-stnv"
issue_type: "feature"
labels:
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let review say no: a won't-fix decision, a declined status, and deferring an item's findings in one command"
updated_at: "2026-11-08T05:35:31.390561285+00:00"
---

## Goal

Give findings and work items a recorded "no". Findings today end as fix_now, defer (now or later, with a follow-up issue each) or no_action (the finding is wrong). Add won't-fix for a real finding nobody will fix, a declined status for work that will not be done, and one command that defers all of an item's deferred findings so deferring costs no more than fixing.

## Done when

- [ ] `atelier review disposition --decision wont_fix <finding> "<reason>"` records that a real finding will not be fixed. The reason is required and non-empty. It satisfies merge's disposition gate as defer and no_action do, needs no follow-up issue, and the same identity rules apply as for no_action (an author cannot decide findings on its own work). `review list`, `review show`, the review brief, `debrief` and `doctor` show it as won't fix, distinct from no_action.
- [ ] The help for `review finding` and `review disposition` says no_action means the finding is wrong (mistaken, a duplicate, or without consequence) and wont_fix means it is real but not worth fixing.
- [ ] Work items have a terminal `declined` status (category done) and a `decline` transition from backlog, todo and in_progress with a required close reason, in the workflow `atelier init` writes and in this repository's `.atelier/workflow.yaml`. Work views, `debrief` and `doctor` show declined work as finished without merging, labelled declined, as they treat superseded. A repository whose workflow.yaml lacks the status loads and works unchanged.
- [ ] One command defers every open finding on an item that was recommended defer and has no disposition yet: it creates one backlog follow-up per finding (discovered from that finding) under a single parent per item, creating the parent if missing, and records each defer disposition. A dry-run flag lists what it would do and writes nothing. Its records can be committed in one commit, like other commands' `--commit-records`.
- [ ] Tests cover: won't-fix by the author refused; merge passes with a wont_fix disposition; decline without a reason refused; an older workflow.yaml without declined loads; the defer command's dry run and real run.
- [ ] The new binary runs `atelier check`, `atelier doctor` and `atelier work missions` in scratch clones of each local Atelier repository (atelier, astromech, life-on-rails, cosplay-studio, demiurge, worldcrafter) with no error the installed binary does not also report. The Report lists the commands and results.

## Scope

Atelier's code, `docs/product` and this repository's workflow.yaml and records. Not other repositories' workflow.yaml (a loose end adds declined there once the binary is installed). Not the agent-factory skill.

## Context

Agreed with Sumeet in conversation on 2026-11-08 after an audit of review records in atelier, astromech and life-on-rails (about 175 findings sampled). Review found real defects about three times as often as it did harm, and no sampled finding was wrong. The harm came after review: orchestrators fixed findings the reviewer marked defer (atelier: 122 of about 242 deferred findings were fixed anyway, 80 with no reason beyond "Fixed in <sha>"), several "easy" fixes grew to hundreds of lines (astromech-u6ak, astromech-emsi, the astromech clock-skew chain, atelier-hwxm), and nothing in the process lets anyone decide not to do real work. A defer costs a follow-up issue per finding, and backlog is the only place a no can go (atelier has 100 backlog items, astromech 71 open follow-ups from dispositions). The skill changes that use these commands are a sibling item.
