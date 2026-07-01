---
created_at: '2026-06-11T00:40:11.669040461+00:00'
id: atelier-6i3y
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments:
  - kind: evidence
    id: atelier-tm8c
    role: validates
  relates:
  - kind: issue
    id: atelier-ap3r
    type: advances
  - kind: issue
    id: atelier-pukf
    type: advances
  - kind: issue
    id: atelier-v851
    type: advances
  - kind: issue
    id: atelier-wnf1
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-11T02:08:39.707242883+00:00'
status: closed
title: Refine human CLI output ergonomics
updated_at: '2026-06-11T02:08:39.707242883+00:00'
---

## Description

Atelier default human CLI output still leaks implementation-shaped data and inconsistent formatting even after the first output improvement pass. This mission tracks a second pass focused on eliminating raw key/value tails, full machine timestamps in human views, and command-specific formatting drift across the core CLI.

Problem surfaces identified:

- `atelier mission show`: linked work rows append raw `relation=...` and `open_blockers=...` fields after the title; `Created` and `Updated` print full RFC3339/UTC timestamps; progress uses mixed terse counters such as `Mission blockers=0`; related work does not present relation and blocker context as first-class readable columns or metadata rows.
- `atelier issue show`: `Created`, `Updated`, `Closed`, and recent activity timestamps print full RFC3339 values; dependency rows append shouty `OPEN BLOCKER`; recent activity prints raw field-change bodies (`field`, `old`, `new`) instead of a readable event summary.
- `atelier issue list`, `atelier issue ready`, and `atelier issue search`: queue rows still expose raw `parent=...` and `blocked_by=...` suffixes; summaries use mixed machine-style `status: open=...` and `blocked=...`; row field order differs from the documented grammar and can make titles look like a dumping ground.
- `atelier mission list`, `atelier plan list`, and `atelier evidence list`: flat fixed-width rows lack the shared grouped queue/detail grammar, useful headings, empty-state guidance, and consistent metadata labels.
- `atelier plan show` and `atelier evidence show`: detail views are too sparse compared with issue/mission detail views and omit created/updated/links/provenance context or readable sections.
- `atelier link list`: prints edge syntax such as `mission X --advances--> issue Y` rather than a human section grouped by relation or target kind.
- `atelier dep list`: prints a long ungrouped list of quoted IDs (`"A" is blocked by "B"`) with no status, title, priority, readiness grouping, or bounding.
- `atelier workflow validate`: prints validator result as `pass durable_state_current: ...`, mixing status token, validator key, and reason without headings or actionable next steps.
- `atelier work status` and `atelier worktree status`: outputs are terse ad hoc lines, with worktree status especially using raw path/branch/dirty columns and nested `work:` / `export:` fragments instead of a scan-friendly status view.
- Mutation acknowledgements across mission, plan, evidence, link, work, dependency, issue, export, rebuild, init, and delete commands use one-off sentences; they should be audited for consistent identity lines, next commands, and quiet-mode boundaries.

End state:
Default non-JSON output follows `docs/architecture/human-cli-output.md` across core commands. Human views use readable dates (relative where useful plus exact when needed), explicit labels instead of raw suffix fields, stable grouping, bounded long lists, consistent empty states, and shared formatter helpers. JSON output remains the automation contract and is not changed by this mission.

Recommended subskill: agent-factory orchestrate.

## Outcome

### Constraints

- Preserve existing JSON schemas unless a separate migration explicitly approves a contract change.
- Keep human output useful in colorless logs and narrow terminals; color, if added, cannot be the only carrier of meaning.
- Use shared formatter/date helpers where more than one command needs the same policy.

### Risks

- Changing human text may affect users scraping default output; mitigate by documenting JSON as the stable scripting interface and keeping quiet mode minimal.
- A broad formatting pass can devolve into cosmetic churn; mitigate by prioritizing the identified rough surfaces and adding focused fixtures.

## Evidence

- Manual check: Golden or behavior tests cover mission show, issue show, issue list/ready/search, mission/plan/evidence lists, link list, dep list, workflow validate, and work/worktree status.
- Manual check: Tests or deterministic formatter units prove readable date rendering and raw key/value suffix removal.
- Manual check: Run cargo fmt -- --check, cargo nextest run or focused equivalent, git diff --check, atelier export --check, atelier lint, and atelier doctor.

## Notes

Migrated from `.atelier/missions/atelier-6i3y.md` as a declared mission objective issue.
