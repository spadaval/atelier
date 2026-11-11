---
schema: atelier.review
schema_version: 3
round: 1
commit: db7a7a43b64e437c0bedce2ab1cd3acad1f36198
reviewers:
- thread:delegated-task:command%3Amcp%3Af03ae543-67b7-4101-adec-2ed71d3b4324%3Adelegate-task%3A30cfb179-13b5-4776-8e2a-ca3b5e0c6619
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T17:21:32.342943024Z
---

## Summary

Complexity: low. One static procedure, nine checklist entries and two small shell checks; no runtime or lifecycle changes.

Value: high. The gardener needs these preconditions to distinguish an unreadable or untestable repository from the lagging scorecard numbers.

Risk of merging it: low. Incorrect checks produce misleading readiness reports, but the procedure is easy to correct and changes no product data.

The value is worth the size. Keep the checklist, but correct the two checks below before gardeners use their results.

I reviewed `master...db7a7a43b64e437c0bedce2ab1cd3acad1f36198`, every Done when line, the Report, the mission design and the Scorecard's measure names. The nine entries use the requested three groups, name measures, and explain repository-specific additions without adding them anywhere. I checked the Factory sources against the pillar table, the configuration and hook code behind the `.atelier/readiness.md` claims, and the documented build/check commands and workflows. `git diff --check` passed. PR #144's CI completed successfully, with the Rust and Desk jobs skipped for this documentation change.

I ran the exact line 3 and line 9 shell snippets in the retained trial clones at atelier `0f0d8f334c06fe4cf8c52d804aa20138a6fceeb7` and demiurge `13b07f334aa5fbcf64889a39e57f191ce1e86caa`. They reproduce the reported path results and the 39 and 2 bugs without causes. I also ran isolated fixtures for missing root documents, broken links in those documents and valid external links. I inspected both clones' harness hooks and skill directories and checked Claude's current instruction-loading documentation. I did not repeat the builds, linters, unit tests or browser suites, so their timings and complete trial results remain the author's reported validation rather than independently repeated runs.

## Finding 1: Make the path check cover root documents and ignore external links
Recommendation: fix now

In `skills/agent-factory/procedures/readiness.md:82-87`, the script does not implement line 3's promise to check every named path and relative link. Its `names` expression requires a slash, so an `AGENTS.md` containing only ``Read `MISSING.md`.`` prints nothing, and ``Read `GUIDE.md`.`` does not inspect links inside `GUIDE.md`. I reproduced both: a missing root document and an existing root document containing `[Missing](docs/missing.md)` both produce empty output and would be marked met. Root Markdown filenames are ordinary entries in an agent's map.

The link expression also matches the `https` prefix of `[Site](https://example.com)`, then reports `docs/guide.md links to missing https` even though line 3 only promises to check relative links. I reproduced that separately with an existing `docs/guide.md` linked from `AGENTS.md`.

Correct the snippet, or replace it with an explicit observation an agent can perform, so root document references are followed and external URLs are excluded. Recheck the affected trial line and these cases. Otherwise a gardener can both miss broken guidance and propose repairs to valid guidance.

## Finding 2: Test whether Claude loads the map using its documented loading rules
Recommendation: fix now

In `skills/agent-factory/procedures/readiness.md:25-29`, line 1 reduces Claude's loading behavior to the presence of two files and a literal `@AGENTS.md` string. That can certify a map Claude never reads. The [Claude Code memory documentation](https://code.claude.com/docs/en/memory#when-claude-code-reads-agentsmd) says a `CLAUDE.local.md`, or a qualifying instruction file in a parent directory, also suppresses automatic `AGENTS.md` loading. The file omits both. Direct loading additionally requires a supported version and an enabled instruction-loading mode. Conversely, the same documentation explicitly supports `CLAUDE.md` symlinked to `AGENTS.md`, which loads the map but fails the literal-import test when the map has no import marker.

Use an observation that confirms the actual harness loaded the intended map, or state the applicable loading conditions accurately, including supported imports and symlinks. Retest line 1 accordingly. The existing trial establishes the current two layouts, but cannot justify the default rule as written.
