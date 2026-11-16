---
schema: atelier.review
schema_version: 3
round: 3
commit: 258e048c96fb2210a58120b681ae017df76ebbed
reviewers:
- thread:delegated-task:command%3Amcp%3Ab0793ffb-9a53-4e52-8cee-0a327e66ac85%3Adelegate-task%3A5156f81c-fed0-4d92-9d59-ec86cb01d75e
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T03:10:57.527769436Z
---

## Summary

Complexity: high. These reads establish a contract for repository identity
and current ownership across item and plan review lifecycles.

Value: high. The embedding host needs them to replace its repository table
and find the record for a thread in one read.

Risk of merging it: medium. A wrong identity or current tie could prevent
the host from finding the right page, but the endpoints change no records
and can be corrected without a data migration.

The value justifies the size. The shared record index reuses the existing
record-selection and response-cache rules; I found no materially smaller
change that delivers the required lookups.

Reviewed commit 258e048c96fb2210a58120b681ae017df76ebbed in the existing
atelier-prx5 checkout after fetching origin and pulling the branch with
--ff-only. Read the full code diff and the change since round 2. Checked
each Done when line against the route handlers, cross-branch record
selection, core item and plan round rules, generated TypeScript and the
field documentation. The current/history rules match the requirement,
including the Report's interpretation that a plan round remains open
while any review of that round lacks a verdict. The existing holds map,
/api/threads and its consumers are unchanged. The fixture update keeps
RepoSummary typing valid; the URL dependency implements the requested
canonicalization rather than maintaining another URL parser.

Validation passed: cargo nextest run -p atelier-desk (47 tests, including
the embedding routes and generated-type comparison), cargo clippy
--all-targets -- -D warnings, cargo fmt -- --check, git diff --check,
pnpm test (41 tests), and pnpm exec tsc --noEmit. Ran the local T3 Code
checkout's actual parseGitRemoteRepository function with Node and compared
its results with the Rust regression cases for HTTPS, SCP and SSH forms,
case-insensitive .git stripping, IPv6, an internationalized HTTPS host,
dot-segment paths and a path containing a space. They agree. GitHub CI's
changes check passed; test and desk were still pending when checked.
I did not exercise the external embedding extension or browser navigation,
which are outside this item's scope. No new findings.

## Finding 1: Canonicalize URL hosts and paths as T3 Code does
Recommendation: fix now
Was: 2-mihq.2
Response: fixed in 5c7122fd4
Now: fixed

Confirmed that url_identity now uses url::Url instead of splitting the URL
as text. The expanded IPv6 host becomes [2001:db8::1], the HTTPS host
börse.example.com becomes xn--brse-5qa.example.com, and the path
group/../other/repo becomes other/repo. Each agrees with T3 Code's parser
and is covered by a passing regression case. The existing preprocessing
and SCP handling are retained.
