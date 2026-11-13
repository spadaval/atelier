---
schema: atelier.review
schema_version: 3
round: 1
commit: 4f83519224ee47ad4eaf0ba6196029458807ce12
reviewers:
- thread:delegated-task:command%3Amcp%3A64389874-c2e8-4544-b20a-b4c28f7b418f%3Adelegate-task%3A57a9e909-2865-4a7a-b177-d81f8fc8baae
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T08:33:59.098771573Z
---

## Summary

Complexity: low. This is a bounded rewrite of four documents with one small embedded shell script and no runtime changes.
Value: high. The gardener needs a purpose, a reproducible focus and thread evidence to produce useful proposals.
Risk of merging: medium. These instructions guide agents in other repositories, and a wrong checklist result can misdirect a published run.

The value warrants the size. No smaller replacement is needed, but the path classification below needs correction before merging.

I checked the full diff at 4f83519224ee47ad4eaf0ba6196029458807ce12 against master, each Done when line, the mission's authority and atelier-h201's gaps. I compared the product and skill descriptions, verified the unchanged upkeep/proposal boundary and session requirement, and checked the focus/window rules against `atelier list`, `atelier show atelier-0019`, `atelier log atelier-0019` and the show/log implementation. The numbering changed, but threads still precede the code analysis and proposal grouping.

`git diff --check` passed. The file counts are 188 to 192 for garden.md and 110 to 106 for readiness.md, retaining the 298-line total. I extracted and ran readiness line 3's exact shell script on this repository, the Report's sample, and additional scratch fixtures. The repository produced no diagnostics, and the Report's sample produced the claimed three. PR #151's applicable checks passed; Rust and Desk checks were skipped for this documentation change. I did not run a complete gardener session or the repository builds, since this change defines the procedure rather than exercising a run.

## Finding 1: Check missing agent setup paths without reporting HTTP routes
Recommendation: fix now

In `skills/agent-factory/procedures/readiness.md:85-88`, the new `*.*/*` branch skips any dotted first component that does not already exist. That includes repository paths, not just domains. Running the exact script in a scratch directory whose AGENTS.md names `.agents/skills/verify/SKILL.md`, `.claude/settings.json` and `docs.v2/README.md`, with all three absent, prints nothing. The old script reported all three. A missing agent setup directory can therefore make line 3 appear met, contrary to its requirement that repository paths resolve.

The same existence heuristic does not reliably exclude routes: with directories `api/` and `docs/` present and AGENTS.md naming the HTTP routes `/api/items` and `/docs/health`, the script prints both as missing files. This also leaves the item's explicit requirement to stop reporting routes unresolved.

Distinguish home paths, URLs and routes from repository paths without treating a missing first directory as evidence that a repository path is external. Recheck missing dot-directory paths and routes whose first component happens to exist, alongside the Report's original sample.
