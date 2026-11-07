---
created_at: "2026-11-07T05:00:04.971587750+00:00"
id: "atelier-mmpv"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/simpler-lifecycle.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T21:23:38.930216499+00:00"
status: "done"
title: "Take one issue from start to merge in one obvious order, where only a code change makes an approval stale"
updated_at: "2026-11-07T21:23:38.930216499+00:00"
---

## Outcome

An agent takes one issue from start to merge in one obvious order: start, implement, tick every Done when line, review, fix and re-review, approve, merge. Only a code change makes an approval stale, merge marks the issue done, post-merge work is a loose end outside the mission, records are checked when written and correctable by their author until they reach the base, and only the records and gates review needs remain. Independent review before merge, and re-review of fixes, are unchanged. The strategy, docs/strategies/simpler-lifecycle.md, owns the outcome, success criteria, shape and authority.

## Notes

- 2026-11-07 [handoff] 2026-11-07: all eleven work items are merged (3jdm, 4yqd, ilba, jnrh, 8bh4, md0v, 4nsl, l457, ml3r, nn67, uqxh), the skill rewrite is on spadaval/.agents main (4d88ffb), and all five repositories run the new policy. Criteria: lifecycle pass (atelier-oh52; 12 author + 3 reviewer commands), legible pass on re-check (atelier-l6jx, after uqxh cleared the four contradictions atelier-pznw found), every-repo partial (atelier-ulrl): policy, check and doctor hold everywhere, but four After merge lines remain on astromech items in flight on other threads' branches (1hh9, v3u4, v6r3, y6d6), which migrate when those branches merge. Open for Sumeet: the every-repo carve-out and decision atelier-mzpn. Follow-ups outside the mission: atelier-hfme, atelier-f4xz, atelier-xz5c, astromech-vfsu.
- 2026-11-07 [human] Sumeet accepted the every-repo carve-out (2026-11-07, T3 chat: "The astromech carve-out is fine."): the four After merge lines on astromech items in flight on other threads' branches (1hh9, v3u4, v6r3, y6d6) migrate when those branches merge, and do not hold this mission open.

## Report

Agents now take one issue from start to merge in one order: start, implement, tick every Done when line and write the Report, review request, fix and re-review, approve, merge, and merge marks the work done. Only a code change makes an approval stale. Post-merge work is a loose end outside any mission, records are checked when written and correctable until they reach the base, the gates review does not need are gone, a change with no issue takes the light path, and `init` gives a repository a policy that merges as written. The agent-factory skill says this once, and all five repositories on this host run the new policy. Criteria: lifecycle passes (receipt atelier-oh52: one issue in 12 author and 3 reviewer commands, no refusal, one approval beyond the fix's re-review); legible passes on re-check (atelier-l6jx, after atelier-uqxh cleared the four contradictions atelier-pznw found); every-repo is partial (atelier-ulrl): policy, check and doctor hold everywhere, and four After merge lines on astromech items in flight on other threads' branches (1hh9, v3u4, v6r3, y6d6) migrate when those branches merge, which Sumeet accepted on 2026-11-07.

### Deviations
- atelier-ml3r was split: the rollout to the other repositories became atelier-nn67, and the contradictions the first cold read found became atelier-uqxh.
- nn67 dropped the disposition migration (optional; about 1,400 records in repositories with work in flight) and moved the branch_base write and hidden `start --base` removal to atelier-hfme.
- The on-base workflow keeps `backlog`, so policy moves strand no parked validations (found while reviewing astromech's move).

### Decisions
- `fast_forward_only` and `merge_unproven` are retired: merge always makes a merge commit that marks the work done (atelier-jnrh; tradeoff recorded for Sumeet's read).
- Approvals and trivial exemptions stay append-only even before the base, so a hand-edited approval cannot cover unseen code (atelier-ilba review).
- A fix_now finding is resolved only by an approval whose commit holds the finding (atelier-md0v review).
- Loose ends are filed on the base, even for an item in flight; the line they replace is edited on its branch (atelier-uqxh).

### Surprises
- The rollout to the shared binary broke astromech's pinned CI until its pin moved (astromech-5sp5, now at 12efdfd9).
- Using the light path for real found that `land` treated a policy change on an unowned branch as an untraceable tracker edit (fixed in atelier-nn67).

### Open items
- Follow-ups outside the mission: atelier-hfme (branch_base contract), atelier-f4xz (judge a branch's records by the merged result), atelier-xz5c (command output wording), atelier-wiri (Atelier docs links into the cut skill), atelier-jdpf (one agent review of a debrief under the new lifecycle).
- For Sumeet's after-merge read: the merge-strategy tradeoff (atelier-jnrh), the light-path risk that a plain `git merge` skips the merge checks (atelier-4nsl), and that a finding can be deleted before merge (atelier-ilba).
