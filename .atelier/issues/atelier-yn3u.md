---
created_at: "2026-06-16T15:01:48.866802571+00:00"
id: "atelier-yn3u"
issue_type: "feature"
labels:
- "evidence"
- "markdown"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Simplify evidence records around validating attachments"
updated_at: "2026-06-16T19:10:06.210307336+00:00"
---

## Description

Simplify evidence records so validating attachments carry proof semantics, front matter stays small, and detailed proof content lives in Markdown body sections.

## Outcome

- Evidence records no longer use top-level `status: pass` or result vocabulary as the primary proof meaning; an evidence record attached with role `validates` means the evidence supports the target claim.
- Command exit status remains command metadata, not evidence truth. A failing command may be positive evidence when the target claim is that a failure reproduces or that a guard rejects invalid input.
- A single evidence command can run a validation or test command, capture stdout/stderr and exit status, create the evidence record, and attach it to the target issue with the `validates` role; new proof does not require a separate attach step.
- Evidence front matter no longer renders large `output` summaries or default null fields such as `command`, `path`, `uri`, `proof_scope`, `agent_identity`, `target`, and `output` when they are absent.
- Command-backed evidence stores transcript summaries in body sections such as `## Summary`, `## Command`, `## Stdout`, `## Stderr`, and optional `## Residual Risks` instead of nested YAML.
- Manual evidence is easy for agents to write by editing Markdown directly, without filling a large metadata form.
- Parser/rebuild/show behavior can read the new body-section shape and still expose command exit status, target, relationship role, and transcript summaries to status/evidence views.
- Existing evidence records are migrated or explicitly handled by a committed-state migration with no broad compatibility alias unless the product contract requires it.

## Evidence

- CLI transcript shows an evidence record with a non-zero command exit status validating a negative/rejection claim without marking the evidence itself as failed.
- CLI transcript shows the documented one-command path running a validation command, creating a command-backed evidence record, and attaching it to `issue/<id>` with role `validates`.
- CLI transcript records command-backed evidence and shows the generated evidence Markdown has bounded stdout/stderr in body sections, not YAML `output`.
- CLI transcript records manual evidence and shows absent optional fields are omitted rather than rendered as null/default noise.
- Focused record-store tests cover parse/render round trips for manual and command-backed evidence records.
- `rg -n "^status: \"pass\"|^status: pass|^output:|command: null|path: null|uri: null|target: null" .atelier/evidence crates/atelier-records/src` proves the migrated live shape removes evidence-result duplication and front-matter output/null noise except intentional historical fixtures.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, and relevant cargo tests pass.
