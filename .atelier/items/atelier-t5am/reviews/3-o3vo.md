---
schema: atelier.review
schema_version: 3
round: 3
commit: 05e372c20c55af943f5d7bab84b30e40e4c4c4d5
reviewers:
- thread:delegated-task:command%3Amcp%3Aba3b9e6f-8a49-4e3c-a239-c913be6c8ada%3Adelegate-task%3Af6a04b44-bfcc-44d4-b2e7-474c25848072
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T01:27:21.850804077Z
---

## Summary

Complexity: high. The forge owns the final merge, and completion is derived across record readers, planning, views and history rather than written into the item.
Value: high. Finishing through the pull request is necessary for Atelier to work on a protected base.
Risk of merging: medium. This changes the merge and completion paths used by other repositories, with contract tests covering failures, retries and local work preservation.
The value warrants the change. The carried finding is fixed, and I found no further problem requiring a change.

I reviewed commit 05e372c20c55af943f5d7bab84b30e40e4c4c4d5 against master and the changes since round 2. I checked each Done when line against the forge merge and refusal paths, merged-tree checks, plan-review and record-only gates, pre-push rules and skip, merge lock and moved-input checks, completion readers, deferred findings, history and design. The Report describes the gap between the checked tree and the forge's eventual merge and what catches it.

I ran 195 CLI integration tests across merge, hooks, branches, deferred findings, log, views, planning, scenarios, skip and disputes, plus all 41 atelier-app tests. All 236 passed; one additional CLI test was skipped. These include the protected-base item loop, fresh-clone completion, dirty and diverged local-base regressions, repeated merge recognition, and the earlier deferred-findings regression. `git diff --check master...05e372c20c55af943f5d7bab84b30e40e4c4c4d5` and `cargo fmt -- --check` passed. PR #159's head matched the reviewed commit; its test job was still running when inspected. I did not perform a live GitHub merge or change branch protection, and did not repeat the full workspace suite or clippy.

## Finding 1: Read the fetched forge base when the local base cannot fast-forward
Recommendation: fix now
Was: 2-4ppj.1
Response: fixed in 1f8ddca32
Now: fixed

`atelier_app::repo::base_tip` now reads the fetched forge base when the local base diverges, retaining the local ref only when it is strictly ahead or no tracking ref exists. The new `a_forge_merge_onto_a_base_the_local_one_diverged_from_still_reads_as_done` regression passes: the forge merge succeeds, the local base and its checkout retain their local-only commit, `show` reports done, and a repeated merge reports already merged without asking the forge for a second merge. The dirty-checkout regression also still passes.
