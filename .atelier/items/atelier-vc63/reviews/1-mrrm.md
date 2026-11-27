---
schema: atelier.review
schema_version: 3
round: 1
commit: 59901278bb0f58fa9277bc78da303cfd8f3aace3
reviewers:
- thread:delegated-task:command%3Amcp%3A4676258d-0c2f-4131-be64-dffe4c284d08%3Adelegate-task%3A525aec1c-a8a9-4fa0-b2cc-c109f4bb973a
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-27T18:03:30.613343342Z
---

## Summary

Complexity: low. Three small role-file edits introduce no new mechanism.

Value: medium. These instructions guide current implementer, reviewer and lead work.

Risk of merging it: low. Wording errors are visible and easy to revert.

Reviewed the complete diff at 59901278bb0f58fa9277bc78da303cfd8f3aace3,
the three complete role files, every Done when line and the Report. The
implementer now escalates oversized or stale requirements before building;
the reviewer removes the conflicting sentence and includes oversized Done
when lines under Shrink; the lead's dispute-scoping sentence is back in its
dispute paragraph. The reviewer rule agrees with the review brief in
crates/atelier-cli/src/output.rs. Its word count falls from 515 to 508.

Ran git diff --check against master and atelier check; both pass. Atelier
check also reports existing empty mission Why sections, unrelated to this
change. No executable code changed, so no runtime tests were needed. Future
agent adherence to these prose instructions cannot be established here.

The change is proportionate to its value. No findings.
