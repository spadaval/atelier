---
schema: atelier.review
schema_version: 3
round: 1
commit: 793bf0b5cafb2450014f9fcf945aea2d9af0e2b1
reviewers:
- thread:delegated-task:command%3Amcp%3Aae95d3d6-81a4-4c33-b5ac-f63e316d3640%3Adelegate-task%3A4fdcfabe-30f8-4e49-a2ac-dae86021a293
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T19:46:57.058273885Z
---

## Summary

Complexity: high. This imports a separate 36-commit history and requires a
privacy audit of every historical tree before public publication.
Value: high. Keeping the extension beside the Desk lets one item cover changes
to their shared contract.
Risk of merging: high. Publishing private history can expose information that
removing a file from the tip cannot undo.
The value warrants the import and its audit. I found no smaller change that
would preserve the requested history and shared repository workflow.

Reviewed commit `793bf0b5cafb2450014f9fcf945aea2d9af0e2b1` from an independent
detached worktree at `origin/atelier-n4g7`. The CLI refuses item reviews in a
detached checkout, so only the review record and verdict use the item's branch
checkout, as `atelier review` directed.

I compared all 36 imported commits with a separate private source clone at
`a24241057c2b2138553fcf777813ef865e9fdd9c`. The parent graph, author/committer
identities and timestamps, file modes, and remaining paths match after removing
`artifacts/` and adding the `t3-extension/` prefix. Every imported tree was
checked, including both merge commits: none contains an `artifacts` path or a
path outside `t3-extension/`. The only altered historical content is three
file blobs and two commit messages, matching the Report's privacy scrub. Every
file edit is exactly the private repository name and ID-prefix replacement.

I independently scanned all 110 distinct historical file blobs (2,858,161
bytes) and every imported commit message. I checked credential/key formats and
assignments, security-related keywords, opaque strings and entropy candidates,
URLs/domains, IPv4/IPv6 addresses, email addresses, user/home paths, UUIDs,
repository names and item IDs, phone/SSN/long-number patterns, and hidden Unicode
characters. I also read the historical messages and deduplicated documentation
and comments, and inspected identity-like string candidates. The removed
private repository name and both IDs have no case-insensitive match anywhere
in the imported files/messages. No secrets or unaccepted personal details were
found. Network addresses are loopback or fixture addresses and the accepted
`buildbot`; `supadava` is the accepted bundle ID. Git identities and repository
identities/remotes account for the owner/co-author references.

The Report's two tree commands independently return
`28de536c55eff9d45d54ede6fbfc6d98bb434116`. The 36 commits preserve the source
history; `git log --follow -- t3-extension/custom.js` reaches its initial
prototype commit. No `v0.1*` tags were imported. No historical blob contains CR
line endings or trailing whitespace. The final README and test edits are in
`2501060af`, after the import merge. The root documentation and design decision
match the requested move, and no build or release step gained Node.

Validation passed with `TMPDIR` under `$HOME`: all 26 checks from
`BROWSER=/usr/bin/google-chrome DESK=http://127.0.0.1:3780 node
t3-extension/test-extension.mjs`, `node --check` on all six JS/MJS scripts,
`scripts/ci-jobs.test.sh` (26 cases, including both new cases),
`cargo fmt -- --check`, `git diff --check origin/master...HEAD`, the import
merge's whitespace check, and `atelier check`. The PR's test and changes jobs
passed; Desk was correctly skipped. I did not repeat Windows/macOS installation
or real-T3 contract checks: those runtime files are byte-identical to the
source tip, and this item changes no extension behavior.

## Finding 1: Correct the Report's stale initial-commit hash
Recommendation: defer

`.atelier/items/atelier-n4g7/item.md:50` says the followed history reaches
`620c4bd2b`, but the imported history actually reaches
`4f4e96b7bcbf6a297d34798230c2247e5d15212b` with that same initial-prototype
title. The former hash is outside the imported ancestry. Update this reference
when editing the Report. This does not block merging: the history follows
correctly, all 36 commits match the source graph, and the final subtree hash
matches the source exactly.
