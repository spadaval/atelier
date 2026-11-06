---
created_at: "2026-11-06T22:08:25.195127217+00:00"
id: "atelier-rkxm"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-rkxm
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-06T23:51:04.707094722+00:00"
status: "done"
title: "Count how repositories use each status and issue type"
updated_at: "2026-11-06T23:51:10.661590811+00:00"
---

## Goal

Give the mission the facts that its after-merge model (atelier-n0qh) and the status and type cut (atelier-bqbz) are designed from: how often each workflow status and issue type is used across atelier, astromech, demiurge and cosplay-studio, including statuses that appear only on work branches and never at rest on the base.

## Done when

- A note on this issue gives, per repository, a table of each status and each issue type in that repository's `.atelier/workflow.yaml`: records currently in it on the base, and records that passed through it in git history on any branch (counted from the record files' history, `git log --all -p -- .atelier/issues`).
- The same note names, for each status and type, what in Atelier reads it: a gate or command in the code (file:line), or a policy key such as `done_statuses` or a transition that refers to it. Code mostly reads statuses by policy category rather than by name, so "none" needs both checked.
- An evidence receipt on this issue records the command or script used, so the counts can be rerun; a script, if any, is committed under `scripts/` or kept in the receipt itself.

## Scope

Measurement only. No change to policies, records or product code. Repositories are read in place at ~/projects/{atelier,astromech,demiurge,cosplay-studio}; astromech-atelier and astromech-orch share astromech's history and are counted once.

## Context

Strategy: docs/strategies/lighter-workflow.md (Shape, "Statuses and types follow use"). Sumeet asked to measure before cutting: every type is believed to be in use today.

## Notes

- 2026-11-06 [result] Status and issue-type use, counted 2026-11-06 at the refs of that day (receipt atelier-8oxm holds the script and the rerun command; counts move as branches move). Counts are distinct issue records (`.atelier/issues/<id>.md`, schema atelier.issue) that were in that status or type, a record git sees as renamed counted once.

  Columns: **base** = at rest on the base tip now. **tips** = at any tip the local base ref has had, from its reflog (this clone only; start date per repo). **fp** = at any commit on the base's first-parent line; with `fast_forward_only` (astromech, demiurge, cosplay-studio) that includes work-branch commits, so **tips** is the at-rest measure there. **branches** = any commit reachable from refs/heads, refs/remotes or tags. **all** = `git log --all`, adding T3 Code checkpoint refs (refs/t3/*). `*` = in records, not in today's policy.

  All four policies declare the starter's statuses, categories, types, workflows and transitions unchanged (`STARTER_POLICY_YAML`, crates/atelier-workflow/src/lib.rs:11). They differ only in base branch, merge_strategy (atelier merge_commit, the rest fast_forward_only), the mission `ready` description, atelier's `close` required report fields and atelier's merge_checks.

  ### atelier (master 05f01d83; reflog from 2026-10-15, 384 tips)

  | status | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | draft | 6 | 13 | 14 | 18 | 18 |
  | backlog | 132 | 249 | 249 | 249 | 249 |
  | todo | 4 | 229 | 659 | 812 | 812 |
  | in_progress | 0 | 6 | 71 | 377 | 377 |
  | blocked | 0 | 0 | 2 | 13 | 13 |
  | review | 0 | 1 | 32 | 80 | 80 |
  | done | 238 | 1147 | 1163 | 1164 | 1164 |
  | superseded | 13 | 14 | 16 | 16 | 16 |
  | closed* | 0 | 35 | 35 | 367 | 367 |
  | open* | 0 | 0 | 0 | 165 | 165 |
  | validation* | 0 | 0 | 20 | 197 | 197 |
  | ready* | 0 | 0 | 12 | 16 | 16 |
  | publish_review* | 0 | 6 | 7 | 7 | 7 |

  | type | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | bug | 139 | 152 | 154 | 154 | 154 |
  | decision | 7 | 7 | 7 | 13 | 13 |
  | feature | 112 | 210 | 210 | 210 | 210 |
  | mission | 15 | 57 | 59 | 59 | 59 |
  | spike | 0 | 5 | 6 | 6 | 6 |
  | task | 113 | 643 | 660 | 660 | 660 |
  | validation | 7 | 83 | 85 | 85 | 85 |
  | epic* | 0 | 190 | 194 | 194 | 194 |
  | closeout* | 0 | 0 | 5 | 5 | 5 |

  ### astromech (main 5d6acaae; reflog from 2026-10-11, 338 tips; 300 renamed records merged)

  | status | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | draft | 0 | 0 | 0 | 7 | 7 |
  | backlog | 125 | 143 | 143 | 150 | 150 |
  | todo | 59 | 392 | 385 | 455 | 455 |
  | in_progress | 0 | 0 | 59 | 294 | 294 |
  | blocked | 0 | 0 | 0 | 0 | 0 |
  | review | 0 | 1 | 8 | 59 | 59 |
  | done | 336 | 336 | 336 | 337 | 337 |
  | superseded | 15 | 15 | 15 | 15 | 15 |

  | type | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | bug | 56 | 56 | 56 | 56 | 56 |
  | decision | 15 | 15 | 15 | 15 | 15 |
  | feature | 49 | 49 | 49 | 49 | 49 |
  | mission | 2 | 2 | 2 | 2 | 2 |
  | spike | 6 | 6 | 6 | 6 | 6 |
  | task | 374 | 374 | 374 | 377 | 377 |
  | validation | 33 | 33 | 33 | 33 | 33 |
  | epic* | 0 | 0 | 0 | 19 | 19 |

  ### demiurge (master b9c8cbc; reflog from 2026-10-15, 104 tips)

  | status | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | draft | 1 | 3 | 3 | 3 | 3 |
  | backlog | 17 | 33 | 35 | 35 | 35 |
  | todo | 2 | 112 | 116 | 118 | 118 |
  | in_progress | 0 | 0 | 70 | 80 | 80 |
  | blocked | 0 | 0 | 0 | 0 | 0 |
  | review | 0 | 0 | 0 | 0 | 0 |
  | done | 107 | 107 | 107 | 111 | 111 |
  | superseded | 27 | 27 | 27 | 27 | 27 |

  | type | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | bug | 3 | 3 | 3 | 3 | 3 |
  | decision | 20 | 20 | 20 | 22 | 22 |
  | feature | 35 | 35 | 35 | 35 | 35 |
  | mission | 4 | 4 | 4 | 4 | 4 |
  | spike | 2 | 2 | 2 | 2 | 2 |
  | task | 86 | 91 | 91 | 91 | 91 |
  | validation | 4 | 4 | 4 | 4 | 4 |

  ### cosplay-studio (main 03057e8; reflog from 2026-10-11, only 16 tips, records since 2026-07-11)

  | status | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | draft | 6 | 7 | 10 | 10 | 10 |
  | backlog | 7 | 9 | 9 | 9 | 9 |
  | todo | 31 | 49 | 76 | 76 | 76 |
  | in_progress | 0 | 0 | 21 | 21 | 22 |
  | blocked | 0 | 0 | 0 | 0 | 0 |
  | review | 0 | 0 | 1 | 1 | 4 |
  | done | 53 | 53 | 53 | 53 | 53 |
  | superseded | 0 | 0 | 0 | 0 | 0 |
  | publish_review* | 0 | 3 | 3 | 3 | 3 |
  | ready* | 0 | 0 | 2 | 2 | 2 |

  | type | base | tips | fp | branches | all |
  |---|---|---|---|---|---|
  | bug | 4 | 4 | 4 | 4 | 4 |
  | decision | 3 | 3 | 3 | 3 | 3 |
  | feature | 17 | 17 | 17 | 17 | 17 |
  | mission | 10 | 10 | 10 | 10 | 10 |
  | spike | 7 | 7 | 7 | 7 | 7 |
  | task | 45 | 45 | 45 | 45 | 45 |
  | validation | 11 | 11 | 11 | 11 | 11 |
  | epic* | 0 | 8 | 8 | 8 | 8 |

  ### What in Atelier reads each one (at 05f01d83, production code only, `#[cfg(test)]` excluded)

  The policy is the same in all four repositories, so this holds for each. Policy keys come first, then code that names the value; code otherwise reads statuses by category (`status_category`, crates/atelier-workflow/src/lib.rs:318) or by `done_statuses` (`issue_status_is_terminal`, lib.rs:324, 22 call sites).

  | status (category) | policy | code that names it |
  |---|---|---|
  | draft (todo) | objective `initial_status`; `ready` and `supersede` from | record_history.rs:267 (a mission has started at its first non-draft version); cli mission_review.rs:465, 1030 (debrief label and dot); epic_migration.rs:31 (legacy epic draft) |
  | backlog (todo) | work `plan` from, `defer` to, `supersede` from | cli migrate.rs:78-80 (epic migration target); cli mission_review.rs:1030. Parked because no transition reaches the active category (`can_enter_active`, lib.rs:356, used at work_classification.rs:237) |
  | todo (todo) | work and decision `initial_status`; `ready`/`plan` to; `start`, `block`, `defer`, `decide` from | only as the no-policy fallback: work_classification.rs:239, 252; bundle.rs:749 |
  | in_progress (active) | `start` to; `block`, `request_review`, `close`, `supersede` from | record_index.rs:1072 (mission dependency check, together with legacy status "ready"); otherwise category `active` |
  | blocked (blocked) | `block` to; `start`, `decide`, `supersede` from | none by name; the only status in category `blocked`, read by category (work_classification.rs:234, cli mission_review.rs:1016, 1029, human_output.rs:776, 1009). The other `"blocked"` literals are bucket, evidence-result or category names |
  | review (active) | `request_review` to; `revise`, `close`, `block` from | none; every production `"review"` literal is the review record kind. Category `active` only |
  | done (done) | `close`/`decide` to; in every `done_statuses` | workflow_policy.rs:19 (no-policy fallback only); category `done` drives the validators (review_gate.rs:119, 135; workflow_validation.rs:228) |
  | superseded (done) | `supersede` to; in every `done_statuses` | excluded from "done" claims by name: mission_review.rs:342, 656, 668, 687, 719; decisions.rs:52, 238; review_gate.rs:1337, 1666, 1689; cli git.rs:575 (base_done_claims), 1119; evidence_freshness.rs:293; gate_report.rs:445; cli workflow.rs:305; cli mission_review.rs:1014, 1025 |

  | type | policy | code that names it |
  |---|---|---|
  | mission | objective `applies_to` | about 60 lines, e.g. workflow_policy.rs:69, 96, 110, 130, 181; criteria.rs:84, 89, 141; record_index.rs:941, 1071; objective_graph.rs:312, 324, 339; lint.rs:268, 324, 341; mission_overview.rs:58, 74, 128; decisions.rs:130; issue_read.rs:98; cli git.rs:574; cli issue.rs:1092 |
  | decision | decision `applies_to` | workflow_policy.rs:187 (no work branch); work_classification.rs:224 (waits on the human); mission_review.rs:502, 734, 960, 1427; decisions.rs:151, 193, 237; core lib.rs:1012, 1033, 1095; records lib.rs:758 (Decision body); cli issue.rs:283, 1092, 1219, 1342; cli git.rs:574; prune.rs:945; cli mission_review.rs:347 |
  | task | work `applies_to` | issue_cli.rs:28 (default `--type`) only |
  | bug, feature, spike, validation | work `applies_to` | none. `feature` appears only as the epic-migration default (cli main.rs:325 `migrate epics --to`, cli git.rs:559); `"validation"` literals are the evidence kind |

  ### Surprises

  - `blocked` was never used in astromech, demiurge or cosplay-studio, on any ref; atelier has 13 records in history and none at rest on any recorded tip.
  - `review` was never used in demiurge, and in cosplay-studio only once on a branch (three more only in T3 checkpoints). It sat at rest on a base tip once each in atelier and astromech. Nothing reads it by name.
  - `superseded` was never used in cosplay-studio; `draft` never reached astromech's base.
  - `in_progress` never sat at rest on a base tip in astromech, demiurge or cosplay-studio (atelier: 6 tips, under merge_commit). `review`, `in_progress` and `blocked` are effectively branch-only statuses.
  - Every type has records in every repository (atelier `spike` none at rest now). `bug`, `feature`, `spike` and `validation` differ only in label: no code reads them, and all share the work workflow.
  - Legacy values survive in history: atelier's pre-rebuild statuses `closed`, `open`, `validation`, `ready`, `publish_review` and types `epic`, `closeout`; `epic` in astromech and cosplay-studio. record_index.rs:1072 still matches status `ready`, which no current policy declares.
  - Method: plain `git log --all -p` undercounts astromech, because rename detection hides the status lines of 300 records whose ids were regenerated on 2026-10-15 (c5766703, 2470d281); the script reads every blob and merges renames. astromech also has a stray `.atelier/issues/pey5.md` (id astromech-pey5), counted once by id.
  - The reflog covers only this clone: cosplay-studio has 16 recorded tips against records since July, so its `tips` column undercounts.
- 2026-11-06 [decision] Count from every issue-record blob reachable from the refs, merging git-detected renames, and measure 'at rest on the base' by the base tip and the base reflog's tips
  Reason: git log -p rename detection hides 300 regenerated astromech records, and with fast_forward_only merges the base's first-parent line includes work-branch commits
  Rejected: parse git log --all -p status lines
  Rejected: use base first-parent history as the at-rest measure

## Close Reason

The 2026-11-06 result note on this issue gives, for atelier, astromech, demiurge and cosplay-studio, each policy status and issue type with its record counts at rest on the base and across history on any branch, and what in Atelier reads each one; receipt atelier-8oxm holds the script that reruns the counts.

### Deviations
The counts read every issue-record blob reachable from the refs (`git log -m --root --raw`, then `git cat-file`) rather than parsing `git log --all -p` as Done when suggests, because rename detection in `-p` hides status lines of 300 regenerated astromech records. A `-p --no-renames` cross-check matched the script exactly except one stray astromech file (`pey5.md`, id astromech-pey5). The tables add three columns Done when did not ask for: base reflog tips, base first-parent history, and `--all` including T3 Code checkpoint refs (refs/t3/*) apart from branches. The script lives in the receipt, not under `scripts/`, so this branch stays record-only.

### Decisions
"At rest on the base" is measured two ways: the base tip now, and every tip the local base ref has had per its reflog, since with `fast_forward_only` (astromech, demiurge, cosplay-studio) the first-parent line includes work-branch commits. Renamed record files (git -M) count once under their newest id.

### Surprises
`blocked` was never used outside atelier, `review` never in demiurge and once in cosplay-studio, `superseded` never in cosplay-studio. `in_progress`, `review` and `blocked` almost never sit at rest on a base. Every type has records in every repository, but `bug`, `feature`, `spike` and `validation` are read by no code beyond the work workflow's `applies_to`. `review` and `blocked` are never read by name; statuses are mostly read by category, while `superseded` is named in 17 places. record_index.rs:1072 still matches a status `ready` that no policy declares. All four policies keep the starter's statuses, types and transitions unchanged.

### Open items
The reflog covers only this clone (cosplay-studio: 16 tips against records since July), so the tips column is a lower bound there. Counts change as refs move (atelier `done` on branches went 1163 to 1164 during this work as atelier-5fus closed); the note uses the run recorded in atelier-8oxm. Code readers were found by grepping production source for each name and checking the context of each hit, not by tracing every caller of `status_category` and `issue_status_is_terminal`.
