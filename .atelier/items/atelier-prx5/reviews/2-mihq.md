---
schema: atelier.review
schema_version: 3
round: 2
commit: 134ef2a1ba317d6591c441082f342322943e2179
reviewers:
- thread:delegated-task:command%3Amcp%3Ab0793ffb-9a53-4e52-8cee-0a327e66ac85%3Adelegate-task%3A3233e7f7-51c8-4e64-a4c4-3226cf67fe48
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-16T03:06:13.730906666Z
---

## Summary

Complexity: high. These reads establish a contract for repository identity
and current ownership across item and plan review lifecycles.

Value: high. The embedding host needs them to remove its repository table
and find the record for its current thread in one read.

Risk of merging it: medium. A wrong identity or current tie prevents the
host from finding the right page, but these reads change no records and
can be corrected without data migration.

The value justifies the size. Reusing the record-selection rules and the
existing response cache is appropriate; I found no materially smaller
change that delivers these lookups.

Reviewed commit 134ef2a1ba317d6591c441082f342322943e2179 on the item's
existing branch checkout, after fetching origin and registering round 2.
Read the full diff and the changes since round 1. Checked every Done when
line against the handlers, core item and plan round rules, cross-branch
record selection, generated TypeScript and field documentation. The
current/history rules match the requirement, including the Report's
interpretation of a plan round that still has an outstanding verdict.
The existing holds map and its consumers are unchanged.

Validation passed: cargo nextest run -p atelier-desk (47 tests), cargo
clippy --all-targets -- -D warnings, cargo fmt -- --check, git diff
--check, pnpm test (41 tests) and pnpm typecheck. The generated-type test
passed. GitHub CI's changes, test and desk checks passed for the submitted
commit. I compiled the submitted identity functions unchanged in a
temporary standalone Rust probe and compared their outputs with the local
T3 Code checkout's actual builtInSourceControlDefinitions.repositoryKeyFromRemote.
The two examples from round 1 now agree, but further valid URL forms do
not, as Finding 2 records. I did not exercise the external extension or
browser navigation, which are outside this item's scope.

## Finding 1: Match T3 Code's normalization for HTTPS and SSH repository identities
Recommendation: fix now
Was: 1-ctz4.1
Response: fixed in cb7ff9fd2
Now: fixed

Confirmed that https://GitHub.com/Spadaval/Atelier.GIT now returns
github.com/spadaval/atelier, and
ssh://git@[2001:db8::1]:22/org/repo.git now returns
[2001:db8::1]/org/repo. Both agree with T3 Code's registry and are covered
by the new unit cases. Finding 2 addresses URL canonicalization beyond
those two repaired cases.

## Finding 2: Canonicalize URL hosts and paths as T3 Code does
Recommendation: fix now

In crates/atelier-desk/src/fetch.rs:87-100, url_identity splits the URL
as text, whereas T3 Code's parseGitRemoteRepository reads hostname and
pathname from new URL(normalized). Keeping brackets and dropping ports
does not reproduce that URL parser's canonicalization. For example:

- ssh://git@[2001:0db8:0000:0000:0000:0000:0000:1]:22/org/repo.git
  returns [2001:0db8:0000:0000:0000:0000:0000:1]/org/repo here, but
  [2001:db8::1]/org/repo in T3 Code.
- https://börse.example.com/org/repo.git returns
  börse.example.com/org/repo here, but
  xn--brse-5qa.example.com/org/repo in T3 Code.
- https://gitlab.example.com/group/../other/repo.git returns
  gitlab.example.com/group/../other/repo here, but
  gitlab.example.com/other/repo in T3 Code.

These are URL forms, not the Azure host-specific rewriting that the
Report excludes. I reproduced each result with the submitted Rust
functions unchanged and T3 Code's actual registry. For such an origin,
/api/repos returns a different identity from the host's project, so the
first Done when line's matching contract still fails and the host cannot
replace its manual mapping reliably.

Use URL parsing with equivalent host and path canonicalization for the
supported URL schemes, retaining the existing preprocessing and SCP
handling. Add regression cases checked against T3 Code, including the
expanded IPv6 host, an internationalized HTTPS host and a dot-segment
path.
