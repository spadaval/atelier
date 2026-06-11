---
acceptance: []
created_at: "2026-06-10T20:59:23.907724705+00:00"
evidence_required: []
id: "atelier-u8xc"
issue_type: "validation"
labels:
- "assignee:root"
- "cli"
- "human-output"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate and document human CLI output conventions"
updated_at: "2026-06-10T22:18:35.525315733+00:00"
---

Validate the improved human-output surfaces as a coherent operator experience and document the conventions so future CLI work follows them.

Why:
The mission succeeds only if improved output is consistent across commands, stable under terminal constraints, and backed by conventions that future work can reuse.

Scope:
- Run product-style validation across mission detail, issue detail, list/ready/search, and compact hierarchy surfaces.
- Verify JSON compatibility boundaries and quiet-mode behavior.
- Add or update architecture/quality docs that explain formatter patterns, color policy, hierarchy/list/detail view selection, and test expectations.
- Record residual gaps as follow-up tracker work instead of burying them in chat.

Out of scope:
- Implementing missing surfaces during validation except for tiny doc/test fixes.
- Approving breaking JSON changes.

Acceptance criteria:
- Validation evidence demonstrates improved scanability across representative fixture graphs and terminal widths.
- `cargo fmt -- --check`, targeted CLI tests, `git diff --check`, `atelier export --check`, and `atelier lint` pass or failures are documented with owner issues.
- Documentation gives future contributors concrete rules and examples for human CLI output.

Recommended subskill: agent-factory validate and docs.
