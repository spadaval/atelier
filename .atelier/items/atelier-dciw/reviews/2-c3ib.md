---
schema: atelier.review
schema_version: 3
round: 2
commit: c40cafc7842caaf9223480c409e8e050cef2b936
reviewers:
- thread:delegated-task:command%3Amcp%3A861a38e9-fcfa-4297-967f-6aaf69c945b3%3Adelegate-task%3A21314f46-3485-41fb-b958-8bd02b0aa771
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T00:11:16.543063849Z
---

## Summary

Complexity: medium. Generic item and mission lookup now spans local and
remote refs, with copy selection and provenance used by several views.
Value: high. Plans and started work must be visible across clones for the
branch model to work across hosts.
Risk of merging: high. Incorrect selection or readiness can silently give
agents the wrong account of what work exists and what they may start.

The value warrants the change. I found no materially smaller approach that
meets the branch discovery and provenance requirements, and no remaining
fix-now finding.

I reviewed `master...c40cafc7842caaf9223480c409e8e050cef2b936` and the
changes since round 1, against each Done when line and the copy-selection
rules in `docs/design.md`. I followed ref pairing, branch-change discovery,
checkout-file reads, generic record selection, mission review loading,
read provenance, proposed/dropped rendering, and CLI/Desk consumers. I
checked the Report against the code and the two-clone integration tests.

`cargo nextest run --workspace --run-ignored all` passed all 336 tests,
including the three regression tests for the carried findings. Running the
200-branch test separately with output enabled measured `atelier list` at
827.479689 ms on the debug binary, below two seconds. `cargo fmt -- --check`
and `git diff --check master...c40cafc` passed. PR #157's CI test check is
successful at the reviewed head.

The two-clone tests cover plans and plan verdicts after an explicit fetch,
no implicit fetches, dropped plans, backlog additions and record changes,
and remote item holders. I did not repeat the Report's release benchmark
or clippy run. Mission records still use the old `draft`/`ready` schema;
`open` and old-format compatibility are assigned to atelier-2vdq. Selecting
the remote base tip is assigned to atelier-t5am. Those later changes are
not implemented or proved by this item.

## Finding 1: Discover new records in the current checkout before looking them up
Recommendation: fix now
Was: 1-d4f6.1
Response: fixed in 3e145311f
Now: fixed

Fixed. `Repo::touching` now discovers records on the current named branch
with `git::changed_since` from its merge base, including tracked and
untracked checkout paths. The regression test
`a_record_written_but_not_committed_on_the_branch_checked_out_is_found`
passed for both a branch with no new commit and a branch with an earlier
committed addition, through `list --proposed` and `show`.

## Finding 2: Do not advertise proposed items as startable before their branch merges
Recommendation: fix now
Was: 1-d4f6.2
Response: fixed in 3e145311f
Now: fixed

Fixed. `Entry::startable` now excludes proposed entries, and item views
call `mark` before rendering their standing. The regression test
`a_ready_item_only_on_an_unmerged_branch_cannot_start_and_is_not_said_to`
passed for `list --ready`, `list --proposed --ready`, `show`, `status` and
the refusal from `start`.

## Finding 3: Apply the narrowing flags to proposed record changes as well as additions
Recommendation: fix now
Was: 1-d4f6.3
Response: fixed in 3e145311f
Now: fixed

Fixed. Both list groups and proposed record changes use `narrows_item`
and `narrows_mission`, so the readiness, blocker, needs-human, ownership,
mission and label predicates agree. The regression test
`a_proposed_record_change_is_narrowed_by_the_flags_its_record_would_be`
passed for an unheld draft item's change with `--mine`, `--ready`,
`--needs-human`, `--blocked` and a nonmatching label.
