---
created_at: "2026-10-30T06:39:58.922719757+00:00"
id: "atelier-ll3p"
issue_type: "mission"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make review and gate shortcuts visible where work is judged"
updated_at: "2026-10-30T06:39:58.922719757+00:00"
---

## Outcome

A review of every Agent Factory session on this host from 2026-10-15 to 2026-10-29 found agents getting past Atelier's gates in form rather than substance: gate output piped so a refusal's exit status was lost before a push, approvals recorded under a reviewer's label for heads no reviewer saw, Done when rewritten after a finding so an item could close, and review loops that ran five to seven rounds without anyone seeing the count. After this mission, publishing an integration is one gated command, and the shortcuts Atelier can detect are shown where a human or agent decides whether work is done: branch readiness, the review request brief, `merge --check` and the debrief.

## Success criteria

- one-command-publish: `atelier merge <id> --push` integrates and pushes the base, or exits non-zero with nothing pushed when any gate or the push fails, and no product doc recommends piping a gate's output.
- review-history-shown: branch readiness and the review request brief show how many review rounds an owner has had, and `merge --check` and the debrief show a Done when edited after a finding or failing receipt on the same item.

## Background

Source: the usage review in /home/supadava/.t3/scratch/2026-10-30-the-agent-factory-skill-has-0de9a47e/af-analysis/ (REPORT.md, synth/S1.md to S4.md), patterns 1, 3, 4 and 5. Each candidate was checked against master 2f1eea77 on 2026-10-30. Already fixed there and not part of this mission: the rebased-branch refusal now says to merge the base (merge.rs), close-then-approve ordering is one rule since atelier-uw4x, and a passing receipt keeps its byte count, digest and test-summary line rather than a cut log. Base edits to a branch-owned record are atelier-uf97's (see its new item). Skill wording belongs in spadaval/.agents (ADR 0023).
