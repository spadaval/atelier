---
schema: atelier.review
schema_version: 3
round: 2
commit: ed7221b488c785e57d9b257f9fa736f82cf9c64b
reviewers:
- thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3A2b7e5f06-ed0a-406c-9e31-9645d6316db8
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T16:09:26.583379807Z
---

## Summary

Complexity: medium. The change consolidates two specifications, carries
governing ADR decisions, removes obsolete documentation and relocates eval
dependencies across several directories.
Value: high. Agents need guidance for the installed tracker and a durable
record of the decisions that still govern it.
Risk of merging: low. The changes affect documentation and eval fixtures;
mistakes are visible and reversible through Git.

The value warrants the size. Most of the diff deletes obsolete material,
and a smaller edit would leave conflicting guidance in the repository.

I reviewed master...ed7221b488c785e57d9b257f9fa736f82cf9c64b and the
changes since round 1. I compared the two specifications with their source
pages at the merge base, checked the ADR mapping and retained decisions,
inspected the retained documentation and entry points, and checked edits
outside docs for scope. Words, Lifecycle and Review are byte-for-byte
unchanged. The two ADR 0028 rows preserve its decisions 9 and 10 without
extending them. The carried findings are both fixed; I found no new problem.

Validation:

- All ten next-* unit suites passed, 99 tests total, with NEXT_ATELIER
  pointing at the installed binary and TMPDIR under ~/.cache. Each setup.sh
  also exited 0 and created repo/ in its own temporary directory.
- An independent Python check scanned 29 Markdown files under docs/ and
  skills/, the root guidance files and the additional changed entry points.
  It checked 97 relative Markdown links, including heading fragments, and
  found 0 broken.
- atelier check passed: 133 items, 7 missions and config.toml. cargo fmt --
  --check and git diff --check master...ed7221b passed.
- Removed-path searches agreed with the Report's explained exceptions.
  The old-command search found no instruction to use a removed command.
  I checked open missions' mission.md and design.md references: the named
  fleet-dashboard.md remains in atelier-bnek, and the two specifications'
  links point to their new locations. Recorded eval results are unchanged.
- Help for all 18 built commands succeeds; log and init are absent, and
  atelier-8b0a is the item to build log. The local cutover counts and verify
  reports agree with Transition.

I did not rerun the cutover, operate the work host, run live eval agents or
rerun the Rust suite for comment-only Rust edits. PR #143 points at the
reviewed head; GitHub currently reports no checks on that head. CI remains
for the implementer's merge gate to verify.

## Finding 1: Preserve the attribution and feed boundaries before deleting their ADRs
Recommendation: fix now
Was: 1-rzbt.1
Response: fixed in 9e0e1d3a5
Now: fixed

docs/design.md:1247-1248 now carries each data kind's writer, attribution
from each command's session environment rather than MCP, the daily feed's
exclusive access to other sources, one adapter per source, provenance and
the prohibition on live pages reading T3 Code's database. The Report's ADR
0030 and 0032 mapping was updated accordingly.

## Finding 2: Make a newly created contributor item ready before telling the reader to start it
Recommendation: fix now
Was: 1-rzbt.2
Response: fixed in 9e0e1d3a5
Now: fixed

CONTRIBUTING.md:10-13 now tells contributors to select a ready item, or
complete a new draft's Goal and Done when and run ready --by on the base
before start. This matches the commands' readiness requirement.
