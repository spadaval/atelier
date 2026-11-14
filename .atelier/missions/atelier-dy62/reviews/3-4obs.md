---
schema: atelier.review
schema_version: 3
round: 3
commit: 3f0c166e53f712bec8c960c9eed0422ad57b8fb3
reviewers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3A297c84bc-fdd6-4211-a285-2ec131919f84
model: gpt-6-astra
verdict: approve
created_at: 2026-11-14T06:56:47.538010481Z
---

## Summary

Complexity: medium. The whole mission moves three library crates and their
callers, then consolidates process calls and renames terminology. It adds no
state or subsystem; two of its three items have already merged.

Value: medium. Both programs use the tracker today, and the remaining item
removes duplicate Git and forge entry points without moving commands that
have no second caller.

Risk of merging it: medium. The remaining work reaches fetch, push and pull
request operations. Preserving their arguments, environment, error wording
and the CLI's forge deadlines matters even though the intended change is
structural.

The value warrants the scope. Moving the existing forge module is a bounded
way to give the Desk access to it; moving submit and merge is unnecessary.
No smaller change delivers the remaining consolidation.

Reviewed the plan at 3f0c166e53f712bec8c960c9eed0422ad57b8fb3, all three
mission items, the dropped atelier-yclc record and the earlier plan reviews.
Compared the amendment with the workspace manifests, module exports, CLI
forge dependencies and callers, core Git command construction, records'
cat-file process, Desk fetch and stop implementations, dependency test and
the cited design and architecture text. The existing forge module uses no
CLI-specific helper beyond git::run, which is already a core re-export.
The Desk's process calls can move without adopting the CLI forge interface
or its different error handling.

The forge move also needs tempfile as a normal core dependency: forge.rs:312-313
uses it outside tests, while core currently declares it only under
dev-dependencies. This is already permitted by the plan's workspace-dependency
rule and manifest scope. Likewise, the documentation update should include
docs/design.md:1698, whose decision rationale still says submit and merge
move with atelier-yclc, as well as the four lines listed in the item.
Neither detail requires broader authority or a different plan.

atelier check passed for 165 items, 10 missions and config.toml.
git diff --check 3f0c166^ 3f0c166 passed. This is a plan review; I did not
rerun the completed items' suites or build an implementation of atelier-x5al.
Its behavior preservation and Rust and browser tests remain implementation
review checks. No fix-now findings.

## Finding 1: The Git-only fallback is a partial mission result
Recommendation: no action

The fallback at .atelier/items/atelier-x5al/item.md:33 permits stopping after
the Git half if the forge move needs a behavior change. That does not satisfy
the gh half of one-git at .atelier/missions/atelier-dy62/mission.md:83-85:
the CLI and both Desk files would still spawn gh. If that fallback is used,
the item's Report and mission's Result must describe the remaining forge
work rather than claim full consolidation. No such obstacle was found in
the current forge dependencies, so this does not block the planned move.
