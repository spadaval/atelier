---
schema: atelier.review
schema_version: 3
round: 2
commit: d195f04eb705ea12b16bc194283e5b445f391300
reviewers:
- thread:delegated-task:command%3Amcp%3A4e6430b6-baa9-4aee-82a6-523fdb40596f%3Adelegate-task%3A1735dd79-497c-414d-805e-3aea942ff7d6
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T00:13:19.708605151Z
---

## Summary

- **Complexity:** low. The policy and output changes fit in one sitting;
  passing skip trailers through the existing commit formatter adds no state.
- **Value:** high. The skill must give the lead the authority the human
  assigned it, including settling a kept finding without losing the reason.
- **Risk of merging it:** medium. Agents in other repositories use this
  skill to decide when to depart from independent review.

The value warrants the size. I reviewed the full diff at
d195f04eb705ea12b16bc194283e5b445f391300 against master and the changes
since round 1. I checked every Done when line and the Report against the
skill, product and design text, CLI messages, record-writing paths, drop
instructions, and skip and dispute tests. The lead's authority and limits
agree across the changed documents. The dispute test demonstrates a lead
merging past a kept fix-now finding and checks the recorded reason; its
existing thread-attribution limitation is assigned to atelier-nv22. The
word counts reproduce: 1,231 before and 1,533 after.

Validation: `cargo nextest run -p atelier-cli -p atelier-core` passed all
325 run tests, with one skipped. `cargo build -p atelier-cli`,
`evals/agent-factory/test.sh`, `cargo clippy --all-targets -- -D warnings`,
`cargo fmt -- --check`, and the non-record `git diff --check` passed. I
also exercised a skipped `ready` with installed hooks in a temporary
repository, as described in Finding 3. GitHub's test and eval jobs were
pending when checked. I did not run live model evaluations; passing the
fixtures does not establish how agents behave under the new policy.

Both carried findings are fixed. Finding 3 is a pre-existing limitation
for later work and does not block approval.

## Finding 1: Preserve reasons for departures on commands that make no commit
Recommendation: fix now
Was: 1-zfhi.1
Response: fixed in 214d5b328
Now: fixed

`plan::save_with` and both verdict paths now pass `repo.skip.passed()` to
`say::commit_line`, which emits a quoted `--trailer` argument per rule.
The updated skip tests execute the printed commands and check the Git
messages, and the verdict test checks the reason in `atelier log`.
The lead's instructions now explicitly require making that printed commit.

## Finding 2: Align the product principles with the lead's new authority
Recommendation: fix now
Was: 1-zfhi.2
Response: fixed in 214d5b328
Now: fixed

The product principles now describe role separation and independent review
as defaults, and explicitly allow a lead's recorded departure. They agree
with the skill and dated design decision.

## Finding 3: Carry a skip through the printed commit when a hook needs it
Recommendation: defer

`crates/atelier-core/src/say.rs:418-428` adds the command's skip trailers
to the printed commit but does not pass its reason to Git's hooks. A
command that skips a lifecycle transition can therefore write its files
successfully and then print a commit that is refused as written.

I reproduced this in a temporary repository with `atelier setup` installed:
on a backlog branch holding a dropped item, run
`ATELIER_SKIP="Revive cancelled work" atelier ready demo-ab12 --by test`.
It succeeds and prints the new trailer-bearing commit. Running that
printed command without the variable exits 1 because pre-commit refuses
`dropped to ready`. The hook asks for the skip again. The current skip
tests use scratch repositories without installed hooks, so they do not
exercise that boundary.

This is deferred because the same generated-command limitation existed
before this change. The new trailers preserve the reason once the commit
succeeds, and the hook's refusal is explicit rather than a silent loss.
Follow-up work should carry the reason into the printed Git invocation,
or explain the second skip, and cover it with an installed-hook test.
