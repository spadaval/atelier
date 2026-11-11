---
schema: atelier.review
schema_version: 3
round: 2
commit: 0e2100e96286bf349f3c6e72d916c5670dc568f0
reviewers:
- thread:delegated-task:command%3Amcp%3A952a8bcb-a049-4c76-bd07-27d029a0ca95%3Adelegate-task%3Ac53f8e28-2011-4d39-8af4-baa1a90a4164
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T17:53:13.803391024Z
---

## Summary

Complexity: medium. Snapshot capture, thread attribution and both scorecard entry points now share the frozen log as an input.

Value: high. The post-cutover scorecard rerun needs the current record layout and reproducible attribution from the log.

Risk of merging it: high. Incorrect attribution or record parsing can silently change scorecard populations.

The value justifies the change. I found no substantially smaller change that meets the frozen-input and log-based attribution requirements, and no new merge blockers.

I reviewed the full change from df9c84a43 to 0e2100e9, the changes since round 1, every Done when line and the Report. I compared the log reader with docs/design.md and the CLI formatter, and the finding reader with atelier-records' document and body parsers. I checked removal of old-layout readers, history walks and title/message matching, and confirmed that the baseline reproduction commit is the branch's base and no baseline numbers, samples or labels changed.

All 22 tests passed with `python3 -m unittest discover scripts/scorecard`, including every attribution rule and identical counted outputs, mission membership and findings after advancing the fixture base and deleting its item branches. `git diff --check` passed. A separate synthetic fixture exercised freeze.copy(), hash_() and verify(): it captured the log verbatim, included its hash in the manifest and retained owner-only permissions. No live sources were used. The new fence filter also matched a literal port of the record reader on 5,000 generated cases. I separately reran the earlier null-front-matter/fenced-mission reproduction and got no membership.

I did not read the private baseline snapshot, live T3 database or transcripts, take a live snapshot, or run a live scorecard. The production presence of nativeItemRef.driver remains unverified. GitHub's test check was pending when inspected; the local Python suite passed. The earlier deferred empty-item-repository case remains recorded in 1-c6ma.3. I edited only this review file.

## Finding 1: Exclude fenced examples when reading review findings
Recommendation: fix now
Was: 1-c6ma.1
Response: fixed in fefff2355
Now: fixed

The body is filtered through unfenced() before headings and fields are read. Its fence rules match document.rs. The regression fixture contains both a phantom fix-now finding in a tilde fence and a fenced Was line inside a real finding; the population correctly includes the real finding and excludes the example.

## Finding 2: Read mission membership only from item front matter
Recommendation: fix now
Was: 1-c6ma.2
Response: fixed in fefff2355
Now: fixed

mission_items() now considers the mission field only between the first two front-matter delimiters and omits null membership. The fixture checks an item's fenced example naming another mission, including an extra delimiter. The separate reproduction without that extra delimiter also returns no membership. Both earlier failure cases are fixed.
