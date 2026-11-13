---
schema: atelier.review
schema_version: 3
round: 3
commit: 2f6f26d37d099164a590f6909b6dc8d5b5a0b650
reviewers:
- thread:delegated-task:command%3Amcp%3Aa410b15e-b763-48ac-8edb-eb330175253e%3Adelegate-task%3A45a36a82-c807-4973-9da1-d6702e2bac9e
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T23:29:35.269981238Z
---

## Summary

Complexity: high. This changes the Git contract across record commands,
branch naming, remote claims, hooks, record-only merges and cross-clone drops.
Value: high. Keeping record writes off the base requires this branch model.
Risk of merging it: high. A readable working-copy config can still disable
its own base-commit guard and let record changes land directly on the base.

The value warrants the scope; I found no substantially smaller change that
delivers the required branch model. The five carried findings' reported
cases are fixed. The two new findings below need fixing before merge.

I read the implementation and tests against master...2f6f26d and the
round-3 diff, checked the Done when lines, Report, act table and command
help, and inspected the regression cases for the carried findings. All 324
tests in atelier-cli, atelier-app, atelier-core, atelier-records and
atelier-desk passed locally. Formatting, Clippy with warnings denied and
whitespace checks passed. PR #156's test and changes checks pass.

I also ran scratch-repository probes with the built binary and a real
pre-commit hook for staged and unstaged valid base-policy changes, and a
fake gh that records the review brief's pull-request query on a prefixed
branch. These reproduce the new findings despite the passing suite. The
suite's remote tests use local bare repositories. I did not exercise live
GitHub claim races or the Desk UI.

## Finding 1: Keep the base-commit guard active when config.toml is deleted
Recommendation: fix now
Was: 2-coi1.5
Response: fixed in c10d09ad9
Now: fixed

The hook falls back to HEAD's config when the checkout's config cannot be
read. The regression test passes for staged config deletion and an
unreadable unstaged config beside a staged record change. Finding 6 below
covers the separate case of a readable config changing the base policy.

## Finding 2: Refuse review writes while the checkout is behind its fetched branch
Recommendation: fix now
Was: 2-coi1.6
Response: fixed in c10d09ad9
Now: fixed

Both review and verdict now refuse a remote-backed record before writing
it. The two-clone regression test passes for review takeover and verdict,
checks the pull guidance, and verifies that HEAD and the checkout stay put.

## Finding 3: Reject branch prefixes that start with a dash
Recommendation: fix now
Was: 2-coi1.7
Response: fixed in c10d09ad9
Now: fixed

Prefix validation rejects components starting with a dash. The passing
validation tests include both -/ and a/-b/.

## Finding 4: Apply list filters to pending drops too
Recommendation: fix now
Was: 2-coi1.8
Response: fixed in c10d09ad9
Now: fixed

The pending-drop section now respects ready, blocked, needs-human, mine,
done, mission and label filters. The regression test passes for an
unfiltered list, --ready and unmatched label and mission filters.

## Finding 5: Give record writes guidance for their actual branch and act
Recommendation: fix now
Was: 2-coi1.9
Response: fixed in c10d09ad9
Now: fixed

save_with selects guidance from the parsed branch kind and whether the
write is inside the item's own directory. The passing regression test
checks the complete output of an ordinary backlog ready and a started-item
drop, including the drop's commit and backlog steps.

## Finding 6: Read the established base policy before accepting a record commit
Recommendation: fix now

In crates/atelier-cli/src/check.rs:218-230, the hook prefers the checkout's
config over HEAD's whenever it parses. An edit to base_branch therefore
changes which branch the guard protects before that edit is committed or
reviewed. The deletion fallback fixes Finding 1's cases but does not make
this policy trustworthy.

In a valid scratch repository on main, with the built binary installed as
the pre-commit hook and no ATELIER_SKIP, changing base_branch from main to
other and committing the config succeeds directly on main. More starkly,
I left that config edit unstaged, staged an ordinary .atelier/probe.md,
and committed it. That also succeeded on main, although HEAD's config
still names main as the base. The commit violates the required base guard
without even committing the policy change used to pass it.

Judge the commit against the established config at HEAD, or otherwise
ensure that a working-copy policy change cannot unprotect the current
base. Cover both a staged base_branch change and an unstaged valid change
beside another staged record.

## Finding 7: Use the prefixed branch when finding a review's pull request
Recommendation: fix now

In crates/atelier-cli/src/review.rs:394-400, pull_request still passes the
item ID directly to Github::open_pull_request and its output instead of
using repo.branch_of(id). With branch_prefix = "x/", submit publishes
x/demo-ab12, but review queries head=owner:demo-ab12. It therefore misses
the real pull request and its CI link, or can report an unrelated pull
request from an old unprefixed branch.

I reviewed demo-ab12 on x/demo-ab12 with a fake gh that returns a pull
request for head=owner:x/demo-ab12 and records all calls. The command
queried head=owner:demo-ab12 and printed "The forge has no open pull
request for branch demo-ab12, so CI's result cannot be read there."
The existing prefix lifecycle test has no forge, so it misses this lookup.
Use the configured branch name for both the query and the branch wording,
and cover a prefixed item review with a forge.
