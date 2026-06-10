---
acceptance: []
blocks: []
created_at: "2026-06-10T16:09:12.937352005+00:00"
depends_on:
- "atelier-nwug"
- "atelier-p1yj"
- "atelier-vxte"
evidence_required: []
id: "atelier-pakd"
issue_type: "validation"
labels:
- "cli"
- "issue-show"
- "validation"
links: []
parent: "atelier-pd0w"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate better issue show human output and regressions"
updated_at: "2026-06-10T17:43:46.296355536+00:00"
---

Validate the complete Better Issue View behavior across integration, focused helper, and regression coverage.

What:
- Update CLI integration tests for `issue show` human output.
- Add focused unit tests for any new formatting helpers.
- Add regression coverage for unchanged JSON shape, shortcut behavior, nonexistent issue errors, and Unicode text.

Acceptance criteria:
- Integration tests assert richer human output includes header ID/title, status/type/priority, parent title, blocker and blocking titles, subissue progress/counts, notes/activity preview fallback, and close reason for closed issues.
- Unit tests cover empty-section behavior, dependency rows with title/status/priority, subissue summary counts, and activity preview fallback when sidecars are absent.
- Regression checks prove `atelier issue show <id> --json` remains compatible.
- Regression checks prove `atelier show <id>` still works.
- Nonexistent issue errors remain clear.
- Unicode titles/descriptions do not panic.
- `cargo fmt -- --check`, `cargo test`, `atelier export --check`, `atelier lint`, and `atelier doctor` pass or failures are documented with actionable follow-up.

Recommended subskill: agent-factory validate.
