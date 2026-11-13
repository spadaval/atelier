---
schema: atelier.review
schema_version: 3
round: 1
commit: 0e1540d7bf78835cd99c1bd03aeb186463045573
reviewers:
- thread:delegated-task:command%3Amcp%3A9f96e1ea-9eb7-432f-85d1-e61ac8cede94%3Adelegate-task%3Addb9d6bf-0a74-44bf-8022-b38d89c51f42
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T08:49:27.152870040Z
---

## Summary

Complexity: high. These two documents define a new branch, merge and record-reading contract that several later items must implement together.

Value: high. The mission needs a reviewed specification before those implementations begin.

Risk of merging it: medium. Contradictory instructions can produce incompatible implementations and weaken review gates, although this change itself ships no executable code.

The value warrants updating both documents. I read both documents in full, compared their changes at 0e1540d7bf78835cd99c1bd03aeb186463045573 with master, and checked the Done when lines against the mission's mission.md and design.md. I also read atelier-2vdq, atelier-ob9u, atelier-dciw and atelier-t5am to check which later implementations depend on this text. The decision and rejected-alternative rows are carried over, and the branch names, prefix, mission format, base-checkout behavior and derived done choice are documented. The merge gates and some retained instructions do not yet agree with that model.

The documentation-only diff passes git diff --check. atelier check passes for all records. gh pr checks 153 reports no checks on the branch. The full diff also reports an extra blank line at the end of the item's tracker record; that is outside the two-document validation claim and is not a finding here. I ran no Rust tests: the branch model is deliberately ahead of the binary, so its runtime guarantees cannot be proved by this documentation change.

## Finding 1: Apply the content-based review gates on every kind of branch
Recommendation: fix now

docs/design.md:950-959 applies the item approval condition only on an item's branch and the two-plan-review condition only on a mission's branch. It rejects code on a backlog branch, but gives no corresponding restriction on code on a mission's branch, or on plan changes on an item's branch. This contradicts docs/product.md:124-128 and the mission's reviews-are-the-gate criterion, which gate what a branch contains. For example, an approved item that changes an agreed mission's Authority meets the item conditions in step 5 with no two approving plan reviews. Conversely, a mission branch containing a change under crates/ meets its listed conditions with only reviews of the defined plan, which excludes that code. The mission's Authority explicitly forbids merging such code without its independent review. Make the detailed merge steps and act table apply every gate the changed contents require, or explicitly refuse an unsupported combination and name its permitted path. A branch name alone cannot select the only review gate.

## Finding 2: Define a usable review and merge path for amending an agreed plan
Recommendation: fix now

docs/design.md:372-384 allows mission submission, plan review, plan verdicts and mission merge only from proposed. start --mission creates a new mission, while backlog merge refuses a change to an existing plan (line 385), and review of a plan on an item's branch cannot be skipped (lines 1339-1342). No command or branch flow is specified for amending the Why, Scope or design.md of a mission already agreed on the base. Yet docs/product.md:415-419 explicitly permits these changes and requires two new approving plan reviews, and atelier-2vdq must implement a merge gate for them. Reusing a merged mission branch also needs an explicit rule for its creation or resumption and for which approvals are current; the existing start command only creates new missions. Specify where an agreed-plan amendment is made, how its two reviews are obtained, and how it merges, and make the act table accept that flow. Fixing the missing content gate alone would otherwise leave legitimate amendments impossible to land.

## Finding 3: Exclude config.toml changes from the unreviewed record-only path
Recommendation: fix now

docs/design.md:958-982 lets a backlog branch merge on atelier check alone when every changed path is under .atelier/ and no mission or plan changes. A branch changing only .atelier/config.toml satisfies that rule. However, docs/product.md:601-605 explicitly says a config change counts as code and is reviewed and merged like any other. The new definition in docs/product.md:92 has the same omission. A branch could therefore remove merge checks or change the forge through the unreviewed record-only path. Keep the stated configuration review policy by excluding config.toml from that path and reflecting the exclusion in both documents' definitions and merge conditions. If reversing that policy is intended instead, settle it explicitly rather than leaving contradictory instructions for the implementations.

## Finding 4: Make the product's push rule publish the head the forge must merge
Recommendation: fix now

docs/product.md:542-545 still says merge pushes only when the forge's branch is missing or is not a commit whose CI counts, so it never cancels an existing run. In the normal flow, CI on submit's commit still counts after the reviewer commits its verdict locally. The product rule therefore leaves that verdict commit unpublished. In contrast, docs/design.md:933-936 requires a push whenever the forge's copy is not the checked head, and lines 985-988 require the forge to merge exactly that head. Following the product rule makes that merge fail, or merges the earlier head without the approving review if the head guard is omitted. Update the product's rule and its CI-wait explanation to match the exact-head push design, including the acknowledged additional CI run described at docs/design.md:1027-1034.

## Finding 5: Remove the old claim that every Atelier merge writes done
Recommendation: fix now

docs/design.md:1151-1157 still explains Git history simplification by saying an Atelier merge never matches either parent because it marks its item done. The newly chosen design at lines 1038-1045 explicitly writes no done change at merge. When the base has not moved since the branch started, the new merge tree matches the branch parent's tree, including its item record. The old explanation is therefore false under the model this item establishes. Rewrite it to describe why --full-history remains necessary when an ordinary Atelier merge can match a parent; do not leave the log implementation guided by an invariant the new merge design removes.
