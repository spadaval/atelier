---
schema: atelier.review
schema_version: 3
round: 1
commit: 432cdd3722e2d57259f2351df382d5d52f278955
reviewers:
- thread:delegated-task:command%3Amcp%3A01d01b6c-cd17-4b66-866b-dc56811fa3f4%3Adelegate-task%3A0a15827a-2525-4301-a2af-bd4122cf145e
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-27T08:36:55.773245588Z
---

## Summary

Complexity: low. The CLI adds a flag and a presentation pass over the existing history, with no new stored state or core-history change.

Value: medium. The command is in current use, and its latest actions now fit in a cut view.

Risk of merging: low. Failures are visible in CLI output, and the change is easy to revert.

The change is worth its size. I checked the complete diff at
`432cdd3722e2d57259f2351df382d5d52f278955`, the core history's ordering and
filtering, the CLI help, the garden procedure and the scorecard readers.
`cargo nextest run -p atelier-cli --test log --test disputes --test skip
--test merge` passed all 100 tests. The scorecard's 22 Python tests,
`cargo fmt -- --check` and `git diff --check` also passed.

I compared the new binary's `log --all` with the installed binary's `log`:
they match byte for byte on this repository and on the Hub, including the
Hub's item `hub-v2-524p`. The garden counter gives the same merge IDs.
The Hub's default output is exactly the reported 6,168 bytes in 31 lines,
with a maximum line width of 271 characters; the whole log is 174,415 bytes
in 695 lines. Its mission `hub-v2-j0ms` prints 6,713 bytes, and both that
mission and `hub-v2-524p` reach the intended 300-character cap.

The ordinary repository, item and mission paths, the new default order,
`--all` compatibility and the documented limit hold. A separate valid
scratch repository exposes an exception to the promised line cap, below.
I did not rerun the full workspace suite or clippy, or inspect remote CI.

## Finding 1: Critical: Long valid record IDs exceed the 300-character cap
Recommendation: fix now

Done when line 3 says, "No line is over 300 characters by default; a cut
reason says where to read it whole." The cap does not hold for all valid
IDs. In `crates/atelier-cli/src/history.rs:51`, the older-actions footer
bypasses the cap entirely. In `history.rs:79`, the cut suffix includes the
whole command without bounding its own length; `saturating_sub` only
limits the retained prefix, so a suffix longer than `WIDTH` is still
printed whole.

I reproduced both in a scratch repository whose slug is `"a" * 245` and
whose item ID is that slug plus `-abcd` (250 characters, within the
filesystem's component limit). `atelier check` reports every record valid.
With 32 recorded drop actions, `atelier log <id>` prints 30 actions but a
349-character footer. Adding `--since 2000-01-01` produces 323-character
cut action lines and a 378-character footer. This is stdout, not an
existing diagnostic on stderr.

Keep the recovery directions and footer within the advertised width for
long valid IDs too, preserving a complete command the reader can run.
Splitting the directions over bounded lines is one option. Cover both the
footer and cut suffix with and without `--since` in a regression test.

## Finding 2: Useful: The stated 9 KB bound counts characters as bytes
Recommendation: defer

`crates/atelier-cli/src/history.rs:13` and `docs/design.md:1350` say that
30 lines of 300 characters bound the output at 9 KB; the Report repeats
that claim. The code counts Unicode scalar values with `chars().count()`,
not UTF-8 bytes, and the footer and newlines add output too. A retained
non-ASCII reason can therefore make the output substantially larger than
9 KB even when every action line is at most 300 characters.

The measured Hub output meets Done when line 4, so this does not block
merging. Describe the limit as 9,000 characters of action text plus line
breaks and recovery directions, and keep the measured byte counts as
measurements rather than a general byte bound. No new byte limit is needed
to satisfy this item's Hub requirement.
