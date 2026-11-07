---
created_at: "2026-11-07T11:19:13.110128252+00:00"
id: "atelier-nn67"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-nn67
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ml3r"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Move every Atelier repository on this host to the new lifecycle"
updated_at: "2026-11-07T14:34:53.816055218+00:00"
---

## Goal

atelier, astromech, cosplay-studio, demiurge and life-on-rails share one installed binary but keep their own policies, records and AGENTS.md. Move the four other repositories onto the new lifecycle so the every-repo criterion holds, and make `branch_base` optional on read so its write can be dropped later (atelier-hfme).

## Done when

- [x] Atelier reads `WorkflowBranch.branch_base` as optional (expand); tests show a record without it and a record with it both parse. Writing it stops later, in atelier-hfme, once astromech's pin is past this commit.
- [x] `atelier migrate after-merge` names the item when removing its last Done when line would leave the section empty, and keeps that line as an ordinary Done when line instead (finding atelier-z3z1).
- [x] docs/product/workflow-configuration.md says how to move an existing policy without stranding items (the order below).
- [x] ADR 0028 (decision 5 and Consequences) names atelier-hfme for the `branch_base` write and `start --base`, and this item for the rest of the rollout (planning finding R12).
- [ ] In each of astromech, cosplay-studio, demiurge and life-on-rails, through one issue in that repository (or the light path, which astromech's pin 12efdfd9 and the other three, which have no pinned CI, can read), reviewed independently and merged with `atelier merge`, in this order: every item in a status the new policy lacks (`backlog` validations under the on-base workflow, `blocked`, `review`) moves to `todo` or is superseded, with no play-check needed (finding atelier-b0uv; astromech's play-check validations stay `validation` items, since they assess its missions' criteria before close); its policy takes the starter's shape (`merge_commit`, no `close` for work, `loose_end` and `validation` on the on-base workflow); `atelier migrate after-merge --apply` runs where there are After merge lines (only astromech has any, 5); `atelier check` and `atelier doctor` pass; its AGENTS.md describes the lifecycle in the same words as Atelier's own.
- [ ] Nothing is pushed for demiurge or life-on-rails beyond what their owner already pushes (both carry hundreds of unpushed local commits); astromech and cosplay-studio push their base as they usually do.

## Scope

Atelier: `WorkflowBranch` read, `migrate after-merge`, workflow-configuration.md. The four repositories' policies, records and AGENTS.md, each through its own process. Not in scope, and said here because the earlier plan included them: `atelier migrate dispositions` is optional cleanup and is not run (it rewrites about 1,400 review records in repositories with work in flight, and the binary reads both forms); removing the `branch_base` write and `start --base` moves to atelier-hfme (both are invisible to agents, and removing them needs another pin move).

## Context

docs/strategies/simpler-lifecycle.md (criterion every-repo; Shape: One binary serves every repository), ADR 0028 decision 10. atelier-ml3r rewrites the skill first, so agents in those repositories follow the new order before their policies change. crates/atelier-core/src/lib.rs (`WorkflowBranch`, `deny_unknown_fields`).

## Notes

- 2026-11-07 [decision] When migrate after-merge keeps an item's last After merge line, the line loses its marker (prefix plus check) and the loose end is still made, so the check sits on the item as an ordinary Done when line and on the loose end; the output tells the owner to give the item a check it can meet before review.
  Reason: Without the marker a rerun leaves the line alone and check stops noting it; making the loose end keeps the post-merge check once the owner replaces the item's line.
  Rejected: Keep the line with its marker: every rerun plans it again and check keeps noting an open After merge line.
  Rejected: Keep the line and make no loose end: the post-merge check is lost when the owner edits the item's Done when.

## Report

Atelier's share holds: a record without `branch_base` parses and one with it still does, while `start` keeps writing it; `migrate after-merge` keeps an unfinished item's last stand-in-less After merge line as an ordinary Done when line and names the item instead of failing the batch; workflow-configuration.md has a "Moving an existing policy" section giving the order; ADR 0028 names atelier-hfme for the `branch_base` write and `start --base` and atelier-nn67 for the rest of the rollout. The four repository moves are not covered here yet.

Proof: `atelier-records an_owner_record_parses_with_or_without_branch_base` passes (with the field it round-trips; without it, it parses as absent and is written back without it), as does `atelier-workflow validates_workflow_branch_field_shape` for both forms; the `session` start tests still find `branch_base: main` written. On a scratch repository the installed binary refuses a record with the field removed (`missing field branch_base`) and this branch's binary passes `check`. `atelier-cli loose_ends migrate_after_merge_keeps_an_item_s_last_done_when_line_and_names_the_item` failed before the fix (the dry run said the line leaves) and passes after it. The documented order was run on a scratch repository on the earlier starter policy: switching the policy first made `check`, `issue transition` and `migrate after-merge` all refuse with `workflow_issue_status_invalid`; settling statuses first, then switching, then migrating passed `check` and `doctor`. Full suite: 1021/1021, and 1027/1027 with `test-hooks`; `cargo fmt -- --check` and `git diff --check` clean.

### Deviations

Docs that described the changed behaviour changed with it, beyond the four lines: validation.md and the `migrate after-merge` help (the kept line), work-model.md and markdown-first-record-store.md (`branch_base` optional, atelier-hfme stops writing it), and ADR 0028's decision 3 migration bullet and Status paragraph. The new section in workflow-configuration.md replaces three older sentences there (moving out of `blocked` and `review`, copying the `on_base` workflow, "atelier-nn67 updates the other repositories") with links to it, which also settles finding atelier-b0uv's doc point: copied as written, `on_base` put `validation` in two workflows.

### Decisions

The kept line loses its marker and its loose end is still made; see this item's decision note. The policy-move order lists every validation outside `todo`, `done` and `superseded` (so `in_progress` ones too, not only `backlog`), since `on_base` has only those, and says a repository keeping validations in `work` applies `on_base` to `loose_end` alone.

### Surprises

The earlier starter policy has no transition from `review` to `todo` (and `blocked` gets there only through `defer` then `plan`), so the doc's first step says to edit the record's `status` where no transition leads there.

### Open items

The four repository moves and the push line are another agent's and stay unticked. Independent review of this branch is outstanding. atelier-hfme waits on astromech's CI pin moving past this branch's merge.
