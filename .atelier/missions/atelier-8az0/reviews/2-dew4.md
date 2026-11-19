---
schema: atelier.review
schema_version: 3
round: 2
commit: e2d95130f2cac8fc5b2dd0fc59cbc91f3dddbb82
reviewers:
- 01a1f20c-18bd-71a3-ba66-eb01482aaefc
model: grok-4.6
verdict: approve
created_at: 2026-11-19T04:24:40.880690477Z
---

## Summary

Reviewed the plan as commit e2d9513 holds it: the mission, all five draft
items, the diff from bdcab8d that answers round 1, and the files those
items name. Rechecked the Why against
`~/.cache/af-evals/review-survey-2026-11-18/` (143 reviews, 65 items, 136
findings all `valid: yes`; round shares 75/50/39; 18 `family_of`; 7 of 18
from round 3 on important; 27 of 117 `fix now` useful or nit, all 27
`outcome: fixed`; exact agreement 23/28, important/useful line 27/28;
`read_code` true on three). Rechecked the later-round brief in
`crates/atelier-cli/src/output.rs` (`brief`, whole-change diff first),
Responses in `crates/atelier-core/src/records/body.rs` and
`crates/atelier-core/src/model/item.rs` (two forms only),
`roles/implementer.md` Findings (two exits), `roles/reviewer.md`,
`roles/lead.md` settle/SKIP, `docs/design.md` on `escalate`/`decide` and
Responses, atelier-kw8u Scope and atelier-oiwy (still writes those role
files, blocked by atelier-id3v), the `costly-hardening` rubric, CI's
`evals` job, and `evals/agent-factory/test.sh`. Did not run a model, read
another reviewer's file of this round, or replay the 143 reviews one by
one.

- Complexity: medium. Five items, a later-round brief, a third Responses
  form, and eight eval cases; no new command or record section is in Scope.
- Value: high. The survey's three holes are in the current loop, and the
  items map onto the Outcome.
- Risk of merging it: medium. A later-round rule that hides a real defect
  would reach every re-review; the frozen wording, two matrices, and drop
  rule are supposed to catch that.

The split is worth doing. All four carried findings are fixed. No new
fix-now finding.

## Finding 1: Validate the final brief independently of the runs used to tune it
Recommendation: fix now
Was: 1-25hs.1
Now: fixed

The iterate-until-pass loop is gone. `.atelier/items/atelier-d73k/item.md:45-47`
settles the brief's wording before any run and does not change it after one.
`.atelier/items/atelier-d73k/item.md:51-54` and
`.atelier/missions/atelier-8az0/mission.md:83-89` drop the unchanged-code
sentence if the defect is found less often after in both matrices, and do
not try wording again. There is no selection on the same experiment, so a
second validation after tuning is not needed. The failure outcome the
finding asked for is that drop rule.

## Finding 2: d73k may rewrite the brief until the keep-bar passes
Recommendation: fix now
Was: 1-v49b.1
Now: fixed

The "or the brief's wording is changed until it is" sentence is gone. The
item now matches the mission: wording is frozen, two matrices of ten, and
if both show a drop in the defect the sentence comes out in the open, the
way `per-part-passage` leaves a passage out.

## Finding 3: Say-no has no legal place for the basic reason
Recommendation: fix now
Was: 1-v49b.2
Now: fixed

The plan took the first remedy. Scope In now names the third Responses
form, read and shown wherever the two forms are today
(`.atelier/missions/atelier-8az0/mission.md:127-132`). Outcome,
`reason-on-record`, and `.atelier/items/atelier-hlsw/item.md:40-46` say
what that line does for `status`, the next reviewer, and `merge`, and that
an item with no such line still reads. Scope Out forbids a new section of
a record, not a new form of `### Responses`.
`.atelier/items/atelier-zwh1/item.md:38-43` says what the grader reads
before hlsw (the implementer's last reply and the thread that started it)
and after (that Responses line, and either counts). Today's parser still
accepts only `fixed in` and `rebutted:`
(`crates/atelier-core/src/records/body.rs:272-294`).

## Finding 4: Skill A/B is not pinned against atelier-oiwy
Recommendation: fix now
Was: 1-v49b.3
Now: fixed

`same-base` (`.atelier/missions/atelier-8az0/mission.md:107-109`) stages
the two arms of a text comparison from commits that differ in that text
alone and names both. atelier-hlsw and atelier-ufbm both adopt it;
atelier-zwh1 names the skill commit. That stops oiwy's nits rule, or
anything else on those files, from landing in one arm only. Authority
still lets this mission run while kw8u's last items run; the parent of
each comparison is frozen.