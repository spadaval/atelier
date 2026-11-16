---
schema: atelier.review
schema_version: 3
round: 3
commit: 0fbb46486de596cdb9a2b2ce82a095fc13b5dbb1
reviewers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3Aea689bc7-39cb-461a-b0f0-7e085e8032be
model: gpt-6-astra
verdict: approve
created_at: 2026-11-16T19:32:35.303209912Z
---

## Summary

Complexity: high. Four items coordinate a history import, a same-commit Desk fixture job, independent releases and a device updater.

Value: high. One repository and a contract test address the observed coordination failures, and bundles satisfy the requested installation model.

Risk of merging it: high. Execution publishes formerly private history and installs downloaded code; an accidental private-history publication cannot be undone by a revert.

The value warrants the scope. The import scan, explicit publication authority, ordered dependencies and required release and checksum tests set concrete boundaries. I found no smaller change that meets the requested outcomes and no remaining fix-now finding.

I read the mission and all four items at `0fbb46486de596cdb9a2b2ce82a095fc13b5dbb1`, compared its changes with its parent, and checked the carried finding against the CI workflow and `scripts/ci-jobs.sh`. I also checked the release configuration, merge policy, ADR 0032 decisions, prior reviews and the available source clone's installer. The release item now uses the same selector as CI and requires tests for later records-only merges, failed relevant runs and skipped jobs. The installer stand-in is owned by the release item before the updater consumes it.

This is a plan review. I did not execute the future workflows, scan all private history, prove the six-minute runtime or test Windows, macOS or a real T3 Code. Those remain implementation evidence required by the items, with platform limits stated in the plan.

## Finding 1: The release refuses most `master` commits, because their CI run skipped the extension's job
Recommendation: fix now
Was: 2-soy4.8
Now: fixed

The mission's Outcome and own-release criterion and atelier-451s now accept a later unrelated merge using successful CI on the newest relevant first-parent commit. atelier-451s requires the selector to come from `scripts/ci-jobs.sh`, avoiding a separate path list, and explicitly tests a tag two records-only merges later. It still refuses failed or skipped extension validation on the relevant commit. This implements the earlier finding's second proposed remedy.
