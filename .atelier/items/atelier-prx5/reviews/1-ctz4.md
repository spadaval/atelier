---
schema: atelier.review
schema_version: 3
round: 1
commit: 79bcfb1e995a6bdedbb0f462175f21840a69821d
reviewers:
- thread:delegated-task:command%3Amcp%3Ab0793ffb-9a53-4e52-8cee-0a327e66ac85%3Adelegate-task%3A8edd7f6a-f2a9-429d-b198-a1217d35a012
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-16T03:00:44.599390826Z
---

## Summary

Complexity: high. This adds a contract that the embedding host must keep,
including repository identity and current ownership across item and plan
review lifecycles.

Value: high. These reads are needed to remove the host's repository table
and choose the page for its current thread.

Risk of merging it: medium. Incorrect identities or ownership can send the
host to the wrong page or prevent a match, but these reads change no records
and are reversible.

The value justifies the size. The shared record index and existing cache
are a reasonable implementation; I found no materially smaller change
that delivers the required lookups.

Reviewed submitted commit 79bcfb1e995a6bdedbb0f462175f21840a69821d in a
checkout detached at origin/atelier-prx5. The installed CLI refused to
register an item review there and explicitly directed me to the item's
branch checkout, where I registered this review and wrote this file.
Checked every Done when line against the API handlers, record-selection
rules, core item and plan round logic, generated TypeScript and field
reference. The current/history classification matches the specified
lifecycle, including the Report's reading of a partially completed plan
round: the core keeps that round open until all reviews have verdicts.
The existing holds map and its consumers are unchanged.

Validation passed: cargo nextest run -p atelier-desk (47 tests), cargo
clippy --all-targets -- -D warnings, cargo fmt -- --check, git diff
--check, pnpm test (41 tests) and pnpm typecheck. The generated-type test
passed. GitHub CI's changes, test and desk checks passed for the submitted
commit. I also compiled the submitted identity function unchanged in a
temporary standalone Rust probe and compared its results with the local
T3 Code checkout's builtInSourceControlDefinitions.repositoryKeyFromRemote.
I did not exercise the external embedding extension or browser navigation;
those consumers are outside this item's scope.

## Finding 1: Match T3 Code's normalization for HTTPS and SSH repository identities
Recommendation: fix now

In crates/atelier-desk/src/fetch.rs:89-97, .git is stripped before lower
casing, using the case-sensitive strip_suffix. For the supported HTTPS
remote https://GitHub.com/Org/Repo.GIT, the submitted function returns
github.com/org/repo.git, whereas T3 Code returns github.com/org/repo. This
violates the first Done when line's identity form and prevents the host
from matching the repository to its project.

The same parser also truncates a bracketed IPv6 host at its first colon
(fetch.rs:74). For ssh://git@[2001:db8::1]:22/org/repo.git it returns
[2001/org/repo, whereas T3 Code returns [2001:db8::1]/org/repo. Besides
failing the match, different IPv6 hosts can collapse to the same identity.
Both results were reproduced against the submitted Rust function and
T3 Code's actual registry, not inferred from the new tests.

Strip the suffix case-insensitively and separate a URL host from its port
without splitting inside IPv6 brackets. Add these supported HTTPS and SSH
cases to the normalization test and verify the expected values against
T3 Code's normalizer.
