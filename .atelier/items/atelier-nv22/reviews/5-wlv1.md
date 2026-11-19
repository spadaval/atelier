---
schema: atelier.review
schema_version: 3
round: 5
commit: 7070f122494806e70a7ceb9ff87bf14b20f2c79e
reviewers:
- thread:delegated-task:command%3Amcp%3Aec287d19-72ad-48dc-8e79-f6d17557d1a5%3Adelegate-task%3A6225cefa-67c0-4d19-8c8e-b41c164f5701
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T01:03:40.678386242Z
---

## Summary

- Complexity: medium. Trailer attribution, history discovery and action inference span the core and CLI, but remain within the existing Git history reader.
- Value: high. Reading a lead's departures on the mission page is the item's outcome.
- Risk of merging: high. The audit still silently omits committed departures for some items the mission page itself lists.

The value warrants the change, and the existing history reader is a reasonable place for it. One membership omission still prevents approval.

I read the full change through 7070f122494806e70a7ceb9ff87bf14b20f2c79e and the latest round's diff, checked the Done when lines and Report against the code, and inspected the tests for lead attribution, legacy trailers, empty sections, action fallbacks and branch discovery. All 398 Rust tests passed (one additional test skipped), using this checkout's target directory, cargo -j 4 and TMPDIR=$HOME/.cache/atelier-tmp. Formatting, clippy with warnings denied and git diff --check passed. CI on the reviewed head passed. I also drove the built CLI through a separate scratch repository to check mission membership for a newly added backlog item. I did not perform a live forge merge.

## Finding 1: Include fetched and backlog branches in the mission's passed-rule history
Recommendation: fix now
Was: 4-q6v2.2
Response: fixed in 0de0b98c9
Now: kept

The fetched-mission cases and the backlog case with an item already on the base are fixed, and their new regression tests pass. The requested membership lookup remains incomplete: crates/atelier-core/src/log.rs:225-227 discards backlog entries from the lookup tips, and in_mission at lines 313-324 only reads the base, the item's own branch or the mission's branch. An item added by an unmerged backlog branch is therefore excluded, although show already finds and lists it.

I reproduced this using only normal CLI commands and their printed Git commit lines:

1. Put the agreed mission demo-m1ss on main, detach from main, and run atelier start --backlog tidy.
2. Run atelier new "New greeting" --in demo-m1ss and commit its printed command. The generated item in my run was demo-5xi1.
3. Run atelier ready demo-5xi1 --by ada and commit it. Then run the same command with ATELIER_SKIP="Ada reconfirmed it" and commit its printed command.
4. Detach at main and read the mission.

The last commit, 4029ab0 in my run, holds:

    Atelier-Skip: demo-5xi1 is draft (it is ready): Ada reconfirmed it
    Atelier-Skip-By: p1

atelier show demo-m1ss lists demo-5xi1 under Ready with "on backlog/tidy", but prints no Rules passed section. atelier log demo-m1ss prints nothing. Unfiltered atelier log does print that same commit's change line, rule, reason and p1, confirming that the new walk reaches it and the membership filter removes it.

This still violates Done when 2 for a supported pending branch. Include committed items found on adding branches when resolving mission membership, consistently with show's record selection, and add regression coverage for a mission item that exists only on an unmerged backlog branch. Retain the committed-only nature of history.
