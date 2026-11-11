---
schema: atelier.review
schema_version: 3
round: 1
commit: 23b13d37ac50b1a072ee02cf29a56df4f9c406de
reviewers:
- thread:delegated-task:command%3Amcp%3A952a8bcb-a049-4c76-bd07-27d029a0ca95%3Adelegate-task%3A88b1dd08-9e2e-46da-8e72-fe1ccdf563c0
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T17:47:59.716608615Z
---


## Summary

Complexity: medium. The input contract changes across snapshot capture, the log reader, thread attribution and both scorecard entry points.

Value: high. The post-cutover rerun needs the current record layout, frozen logs and stable thread attribution.

Risk of merging it: high. Incorrect record parsing silently changes the populations from which the scorecard samples are drawn.

The value justifies the change. I found no substantially smaller change that meets the required frozen-input and log-based attribution rules.

I reviewed the diff from df9c84a43 to 23b13d37, each Done when line and the Report, and compared the readers with docs/design.md and atelier-records' actual record parser. I checked the removal of history walks and title/message attribution, the baseline reproduction commit and unchanged baseline artifacts. All 22 tests passed with `python3 -m unittest discover scripts/scorecard`, including the branch-deletion/base-advancement rerun. `git diff --check` passed. A separate temporary fixture exercised freeze.copy(), hash_() and verify() with mocked sources: the log was captured verbatim, hashed in the manifest and owner-only. Additional temporary fixtures reproduced the three findings below without changing source files.

I did not read the private baseline snapshot, T3's live database or transcripts, take a live snapshot, or run a live scorecard. The production presence of nativeItemRef.driver therefore remains unverified. GitHub's test check was pending when inspected; the local Python suite passed. I changed only this review file.

## Finding 1: Exclude fenced examples when reading review findings
Recommendation: fix now

`scripts/scorecard/draw_samples.py:98-100` splits the raw Markdown body at every `## ` line and searches the resulting text for Recommendation and Was lines, including lines inside fenced code blocks. The supported record format explicitly treats fenced headings and fields as text (docs/design.md, Records; crates/atelier-records/src/document.rs and body.rs).

I passed draw_findings() a review with one real fix-now finding and a fenced `## Finding 99: Example` / `Recommendation: fix now` example in Summary. It reported two findings and sampled the nonexistent finding 99. In a second case, one real fix-now finding containing a fenced `Was: 1-zz00.1` example produced a population of zero. These are valid review bodies, and the second case silently removes a real finding from the population.

Ignore fenced headings and fields using the record format's fence rules before identifying findings, recommendations and carries. Add regression cases for both the phantom finding and the omitted real finding.

## Finding 2: Read mission membership only from item front matter
Recommendation: fix now

`scripts/scorecard/draw_samples.py:155-157` searches entire item files for `^mission: `, so Markdown examples in Goal, Scope or Report become additional mission memberships. The item's mission is its front-matter field; its body is arbitrary Markdown.

I committed a fixture item whose front matter says `mission: null` and whose Scope contains a fenced YAML example saying `mission: demo-cccc`. mission_items() returned `{'null': ['demo-aaaa'], 'demo-cccc': ['demo-aaaa']}`. closed_missions() then treats this standalone item as belonging to demo-cccc, includes its logged threads, and can sample unrelated intervention messages. The required membership at the manifest commit is therefore wrong even though every input is frozen.

Read the mission field from each item's front matter at the manifest commit, excluding body text and null memberships. Test an item with a different mission shown in its Markdown body.

## Finding 3: Treat a repository with no item records as an empty population
Recommendation: defer

`scripts/scorecard/draw_samples.py:155` uses snapshot.git(), which sets subprocess check=True. Git grep exits 1 for no matches, so closed_missions() raises CalledProcessError whenever a manifest repository has no item records, even if that repository has no closed missions to sample. I reproduced this with a temporary repository containing only a mission record and a frozen log with its ready and close lines.

An empty item population should return an empty membership map while actual Git failures still fail. This case is not exercised by the populated five-repository rerun, so it can follow separately. Add an empty-record fixture when handling it.
