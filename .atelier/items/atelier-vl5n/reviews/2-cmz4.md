---
schema: atelier.review
schema_version: 3
round: 2
commit: a577cc3f3132f1b4e8bd295ce2f64b1f0fa20849
reviewers:
- thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3Ac88fdfbd-7254-4bc4-a211-911d677dbc66
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T17:25:16.229129093Z
---

## Summary

Complexity: low. One static procedure with nine entries and two shell snippets; no runtime behavior changes.

Value: high. The gardener needs these checks to identify missing preconditions before interpreting the scorecard.

Risk of merging it: low. A wrong check can mislead a readiness report, but this document is easy to correct and changes no product data.

The value is worth the size. Keep the checklist, with the remaining correction to line 1 below.

I reviewed `master...a577cc3f3132f1b4e8bd295ce2f64b1f0fa20849`, the changes since round 1, every Done when line, the Report, the mission design, and the Scorecard's measure names. The nine entries use the required three groups, name measures, and specify reviewed repository additions without adding them. I checked the Factory sources against the pillar table, the merge and record code behind the `.atelier/readiness.md` claims, and the repositories' maps, hooks, documented commands and CI configuration. `git diff --check` passed. PR #144's checks completed successfully; the Rust and Desk jobs were skipped for this documentation change.

I extracted and ran the exact line 3 and line 9 snippets against the retained trial clones, atelier `0f0d8f334c06fe4cf8c52d804aa20138a6fceeb7` and demiurge `13b07f334aa5fbcf64889a39e57f191ce1e86caa`. They reproduce the reported path results and the 39 and 2 bugs without causes. Isolated fixtures now report a missing root document and a broken link in a root document, ignore a valid external URL alongside a broken relative link, and accept valid links with fragments while ignoring the slash command and scoped package name. I checked the revised Claude rule against its current memory documentation and confirmed the installed version is 2.1.322. I did not repeat builds, linters, unit tests or browser suites, so their complete results and timings remain the author's validation rather than independently repeated runs. I did not start a Claude session to observe its loaded context.

## Finding 1: Make the path check cover root documents and ignore external links
Recommendation: fix now
Was: 1-pv9w.1
Response: fixed in a453d7061
Now: fixed

The revised snippet includes root Markdown filenames and filters external URLs before testing links. The fixtures above reproduce all three cases from the original finding with the correct results. The affected trial line still prints nothing in atelier and reports only the missing `src/docs/editor` in demiurge.

## Finding 2: Test whether Claude loads the map using its documented loading rules
Recommendation: fix now
Was: 1-pv9w.2
Response: fixed in a453d7061
Now: kept

The revision fixes the parent-directory, `CLAUDE.local.md`, version and symlink omissions, but still omits the instruction-loading mode and plugin availability identified in the original finding. In `skills/agent-factory/procedures/readiness.md:26-31`, the test says Claude reads `AGENTS.md` when the listed files are absent and its version is recent enough. A supported `claude-md` setting or a disabled built-in AGENTS plugin leaves that same layout unloaded. Conversely, `claude-md-and-agents-md` loads the map even when a parent `CLAUDE.md` exists, contrary to the line's "only when" condition. These are documented settings, not version or path edge cases. See [Choose which instruction files load](https://code.claude.com/docs/en/memory#choose-which-instruction-files-load) and [When AGENTS.md support is unavailable](https://code.claude.com/docs/en/memory#when-agentsmd-support-is-unavailable).

Include the applicable mode and plugin conditions, or replace the inferred loading rule with an observation that the intended map actually loaded. Otherwise a gardener can still mark line 1 met when the harness did not receive the map. The Report's version and file-layout observations do not establish the missing conditions for demiurge's direct-loading case.
