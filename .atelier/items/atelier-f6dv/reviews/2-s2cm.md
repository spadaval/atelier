---
schema: atelier.review
schema_version: 3
round: 2
commit: 272d823cf311d7cb2ffc523bfd708d64a4628b21
reviewers:
- thread:delegated-task:command%3Amcp%3A7b31cbcf-39a2-4bb0-b4aa-53c1f77ac8b6%3Adelegate-task%3A5db8b0dc-2f1d-4c66-b073-0e0eae48b727
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T18:19:07.269457738Z
---

## Summary

Complexity: low. Two procedure files change, with no new runtime behavior.
Value: high. Four more first gardener runs need these instructions to avoid repeating the first run's mistakes.
Risk of merging: medium. Other repositories follow these commands, so an incorrect instruction can interrupt their runs; the text is easy to correct.
The value justifies the small change. No smaller correction would cover the requested gaps.

Read the complete diff against master at 272d823cf311d7cb2ffc523bfd708d64a4628b21, the changes since round 1, both procedures, and the item's Done when and Report. Checked every Done when line. Confirmed the worktree instruction, no mission and one-line Scope, seconds-based comparison, carried-finding counting and exact thread IDs, separate filing paths, fixed-finding drop reason, template additions and delayed merge instruction. Readiness line 4 now includes every documented install and build command for every repository component.

Checked the filing instructions against crates/atelier-cli/src/plan.rs: ordinary new and set require the base, while new --from and drop on a finding allow the held item branch. The revision restores the base route for fresh bugs and limits post-merge labeling to items taken from deferred findings. Checked approval and next-action handling in crates/atelier-app/src/acts.rs: approved holders are offered merge, and Report edits preserve approval.

Ran git diff --check successfully and documentation assertions confirming 176 to 188 lines (+12), one full artifacts-directory reference, Responses before Session, and the requested additions. PR #149's head matches the reviewed commit; changes and plan passed, while build and test jobs were skipped for this documentation change. Did not execute a full gardener session or readiness builds; this review checks the changed procedure against the CLI, not the repository's readiness. No new findings.

## Finding 1: Preserve a base-checkout filing path for newly discovered bugs
Recommendation: fix now
Was: 1-3oy0.1
Response: fixed in 6d22f5ae0. Step 4 keeps its base route for a fresh bug, and step 5's post-merge labels apply only to items taken from deferred findings.
Now: fixed

Step 4 again files newly discovered bugs on the base with atelier set for their label and cause. Step 5 now applies deferred post-merge labels and cause only to an item taken from a deferred finding. This resolves 1-3oy0.1 without changing the CLI.
