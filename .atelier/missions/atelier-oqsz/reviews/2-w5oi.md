---
schema: atelier.review
schema_version: 3
round: 2
commit: 427a0be326dc8c54b412989f9c0dd50bdf9b4ef6
reviewers:
- thread:delegated-task:command%3Amcp%3A86f1daeb-3c4b-499b-87f9-1b337cdbb39a%3Adelegate-task%3Aoqsz-plan-r2-sol
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-27T07:34:09.980670514Z
---

## Summary

Complexity: medium. Four items touch CLI output, merge checks and the skill, with an explicit boundary around the reviewer experiment and no new record format.

Value: high. The plan addresses observed truncation of a lead's status, misleading merge-check results and review overhead on paths in current use.

Risk of merging it: medium. Merge-check decisions and instructions reach other repositories through one installed binary; the plan keeps installation out of the running Hub mission and preserves the record format.

Checked the round-1-to-round-2 diff of the mission and atelier-qut2, both carried findings, and the measurement notes. Verified the named historical commit and the two round-1 review files: both review 6a2c26e, their models are claude-opus-5-5 and grok-4.6, and they contain 16 and 4 findings respectively. Read plan_brief and later_plan_round in crates/atelier-cli/src/output.rs to confirm the CLI brief is a place the plan reviewer necessarily reads. Compared the changed scope with atelier-oiwy's experiment and the preserved role-file restrictions.

Both carried findings are fixed. The plan also assigns the version bump to the lead after every crates/ item merges, and atelier-qut2 now requires an account of what its counts leave open rather than treating one review as proof. The value warrants the proposed scope. No new findings.

This is a review of the plan changes, not an implementation review. I did not reproduce the Hub's measurements, run the proposed historical re-review, or run code tests; those remain the items' work. The host's installation timing is taken from the plan's recorded authority.

## Finding 1: Name the plan that skill-measured re-reviews
Recommendation: fix now
Was: 1-xnyq.1
Now: fixed

mission.md:63 and atelier-qut2/item.md:55 now name atelier-ww7w at 6a2c26e, the two original models and the original review files. The historical records agree with the stated 20 findings. Implementers can no longer satisfy the criterion by choosing a different plan.

## Finding 2: The skill change skill-measured cites is off the path a plan reviewer reads
Recommendation: fix now
Was: 1-xnyq.2
Now: fixed

atelier-qut2/item.md:65 adds the plan-review brief in crates/ and its tests to Scope. Lines 69-73 require the historical re-review to use a binary built from that item's branch, leave the item-review brief alone, and file the frozen role-file work as a blocked follow-up. mission.md:79 carries the same boundary. This takes the finding's first option and puts the plan-review instructions on a required read path without changing the files atelier-oiwy is testing.
