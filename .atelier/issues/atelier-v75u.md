---
created_at: '2026-06-18T16:20:11.473760961+00:00'
id: atelier-v75u
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: atelier.issue
schema_version: 1
status: superseded
title: 'Superseded by 0v3f: Simplify session and PR attribution model'
updated_at: '2026-06-18T16:44:49.653223443+00:00'
---

## Description

Redesign Atelier sessions and PR coordination around minimal issue-scoped attribution. Sessions should become derived worker/reviewer/validator attempts from canonical activity, not standalone workflow overhead. PRs should stay linked to issues, enforce one active PR per issue or epic, and record useful attribution for comments, reviews, validation, and merge activity.

## Outcome

### Constraints

- Use activity records as the canonical story source; do not make `.atelier/sessions` the live source of truth.
- Sessions are issue-scoped role attempts named by issue, role, and serial, such as `atelier-hw9t-reviewer-1`.
- PRs link to issues or epics, never directly to sessions.
- Only worker, reviewer, and validator attempts are in scope; admin and coordination sessions are out of scope.
- Keep session commands inspection-only; normal commands create and close attempts automatically at meaningful milestones.

### Risks

- Changing PR issue inference can regress existing `pr` workflows if branch and linked-PR resolution are not tested carefully.
- Activity metadata must stay readable and stable enough for history, status, and future Mission Control views.
- Ignoring existing `.atelier/sessions` files may hide current local session state, so status/help text must make the new model clear.

## Evidence

- Manual check: Mission is complete when docs, CLI help/man text, activity history, PR commands, status output, and tests all describe and enforce the new model.
- Manual check: Validation must cover one-active-PR enforcement, PR issue inference, automatic worker/reviewer/validator attempt attribution, `session list/show` projections, and `pr merge` behavior.
- Manual check: Final checks must pass: `cargo fmt -- --check`, `cargo nextest run`, `cargo nextest run --profile extended --run-ignored=only`, `atelier export --check`, `atelier lint`, and `atelier doctor`.

## Notes

Migrated from `.atelier/missions/atelier-v75u.md` as a declared mission objective issue.
