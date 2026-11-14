---
schema: atelier.review
schema_version: 3
round: 4
commit: 38c5d6b4f387f7cd03744a818b78ff7f208359db
reviewers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3A143c95ad-658e-47f7-9b04-394658caab74
model: gpt-6-astra
verdict: approve
created_at: 2026-11-14T07:04:36.472953827Z
---

## Summary

Reviewed the plan at `38c5d6b4f387f7cd03744a818b78ff7f208359db`:
the mission, all three items and their reports, the amendment, and the
separate cleanup item atelier-f3ax. Checked the remaining move against
the core's Git helper and commit reader, the CLI's forge implementation
and its callers, and the Desk's fetch and stop implementations. Compared
the cleanup list with every `atelier-yclc` occurrence outside `.atelier/`
and read the affected design Decision and API rationale.

- Complexity: medium. The plan crosses the library, both callers, tests
  and documents, but introduces no new runtime mechanism.
- Value: high. A single tracker crate and explicit homes for Git and forge
  calls directly serve the requested outcome.
- Risk of merging it: medium. Moving subprocess calls can change flags,
  working directories, errors or deadlines; the plan requires preserving
  behavior and running the existing tests before each item merges.

The value warrants the scope. The crate merge and terminology rename are
already done; the remaining move is bounded and does not require moving
submit or merge. I found no smaller change that delivers the remaining
outcome, and no new problem requiring a plan amendment.

`atelier check` passed for 166 items, 10 missions and the configuration.
`cargo metadata --no-deps --format-version 1` confirmed the three workspace
crates and the core's dependencies. Source searches confirmed the listed
Git and gh construction sites and no forbidden dependency in `model`.
This is a plan review: I did not rerun the completed items' test suites or
verify the unimplemented move's runtime behavior. In particular, keeping
the Desk's command options and errors and the CLI's forge deadline intact
remains part of atelier-x5al's implementation review.

## Finding 1: x5al's yclc correction names four lines and misses the rest
Recommendation: fix now
Was: 3-k13f.7
Now: fixed

The mission's Scope (lines 115-121) and atelier-x5al's Scope explicitly
exclude the other yclc references and the Desk's refusal text. The final
Done-when line of atelier-x5al now requires correcting this mission's own
Decision at `docs/design.md:1698`.

Atelier-f3ax's Done when (lines 25-28) covers every remaining occurrence
found by `git grep atelier-yclc -- ':!.atelier'`, including the generated
type and app comment, and separately preserves the reason for the single
API endpoint. Its refusal edits are outside this mission's unchanged-output
requirement. The cleanup is deliberately deferred to that draft item;
this review does not claim the stale text is already corrected.

## Finding 2: Stopping at the Git half leaves one-git unmet
Recommendation: fix now
Was: 3-k13f.8
Now: fixed

Atelier-x5al's move criterion and Context now name both `serde_json` and
`tempfile`, and mission Authority (lines 128-129) permits both. The latter
is needed by `forge.rs:312-313`; both are existing workspace dependencies.
The revised one-git criterion (lines 83-87) and the item's fallback agree
that a Git-only finish leaves the gh part unmet and requires the Result
to say why. This removes the contradictory promise without treating a
partial result as full consolidation.
