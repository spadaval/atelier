---
created_at: "2026-10-17T01:10:57.983373200+00:00"
id: "atelier-mi3x"
issue_type: "bug"
labels:
- "cli-output"
- "status"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Choose status Next Actions from the checkout's state"
updated_at: "2026-10-17T01:10:57.983373200+00:00"
---

## Goal

`print_status_view` (crates/atelier-cli/src/commands/status.rs:136-155) picks Next Actions only from the ready, human and blocked counts. Branch, owner, checkout and land state play no part. These states all print the same two lines, `Choose ready work (6 ready issue(s) available): atelier work ready` and `Inspect selected work transitions: atelier issue transition <issue-id>`:
- an owner branch with an uncommitted record right after `atelier start`;
- an owner that is `done on branch …, not merged` and approved;
- master with an unreviewed code commit that `land` refuses (`land_base_unreviewed`);
- an unowned feature branch.

With zero issues, status says `No specific next action is available from checkout state.` An agent working an owner is told to pick new work. docs/product/human-cli-output.md:17 says "Next actions come from actual state: … record proof, resolve a finding, or commit record changes."

## Done when

Next Actions is chosen in this order, and each case has a CLI test:
1. Dirty `.atelier/` paths: `Commit record changes: git add .atelier && git commit`.
2. Active owner: `Continue atelier-x: atelier work parent atelier-x` (or `issue show` without children), then `Check handoff: atelier land`.
3. Done owner: `Check merge readiness: atelier merge atelier-x --check`.
4. Base ahead of its upstream with unreviewed code: the `land_base_unreviewed` recovery.
5. No owner and ready work: `Start the top ready item: atelier start atelier-abcd`, naming the real top ID instead of `<issue-id>`. The generic `issue transition <issue-id>` line is removed.
6. No issues at all: `Create the first issue: atelier issue create "…"`.

## Context

Reproduced in a scratch clone of this repository. This builds on atelier-sdb0, which makes status the session view.
