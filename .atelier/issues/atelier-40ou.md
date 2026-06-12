---
created_at: "2026-06-12T04:43:52.672217420+00:00"
id: "atelier-40ou"
issue_type: "epic"
labels:
- "lint"
- "markdown"
- "tracker"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-u6ax"
  children:
  - kind: "issue"
    id: "atelier-0j6e"
  - kind: "issue"
    id: "atelier-4eot"
  - kind: "issue"
    id: "atelier-5i4h"
  - kind: "issue"
    id: "atelier-igzl"
  - kind: "issue"
    id: "atelier-n1ys"
  - kind: "issue"
    id: "atelier-uibk"
  - kind: "issue"
    id: "atelier-v4u7"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Parse issue Markdown sections as first-class structure"
updated_at: "2026-06-12T04:55:44.832214824+00:00"
---

## Description

Issue Markdown should not be treated as one opaque description blob. Atelier
should parse known top-level Markdown headings into named issue sections and use
those sections consistently in show, lint, start/close gates, and future
workflow validation.

The immediate target is a small, explicit issue-body contract with required
Description and Outcome sections, plus an Evidence section for commands,
transcripts, evidence records, or manual checks that demonstrate the outcome is
real. Notes remain optional context, not completion criteria.

The canonical Markdown body remains the durable authoring surface. Frontmatter
stays limited to compact metadata such as id, title, status, type, priority,
labels, and relationships. Remove issue-level YAML evidence fields such as
`evidence_required`; evidence requirements belong in the Markdown Evidence
section, and durable proof artifacts belong in attached evidence records.

## Outcome

- The issue parser exposes named issue body sections instead of only returning
  one description string.
- `atelier issue show <id>` renders recognized sections in stable order and
  clearly distinguishes missing optional sections from absent required sections.
- Validators and command surfaces consume issue structure through one shared
  parsed-section API rather than ad hoc string searches or display-only splits.
- `atelier lint` and `atelier lint <id>` fail when an issue is missing a
  non-empty Outcome section.
- Lint also fails for malformed issue body structure, including duplicate
  recognized headings or content before the first required issue section, unless
  the final parser contract explicitly allows that form.
- Starting work on an issue fails when required issue structure lint fails.
- Mission or issue closeout paths cannot pass while linked implementation work
  has missing or empty Outcome.
- Existing issue records are migrated or repaired to the new section format as
  part of the same workstream, with generated changes kept reviewable.
- Issue YAML frontmatter no longer contains `acceptance` or `evidence_required`
  arrays after the schema migration; the body sections are the authoring surface
  for those concepts.
- Documentation explains the section contract, required sections, optional
  sections, and why Outcome describes the desired finished world rather than a
  mutable completion checklist.

## Evidence

- Add parser unit tests for recognized sections, unknown sections, duplicate
  headings, empty required sections, and round-trip rendering.
- Add CLI integration tests proving `atelier lint`, `atelier lint <id>`, and
  work start fail on missing or empty Outcome.
- Add transcript coverage for `atelier issue show <id>` with all required
  sections present.
- Add validator integration coverage proving missing issue sections block start
  and closeout through the same diagnostics lint uses.
- Run `cargo fmt -- --check`.
- Run focused CLI integration tests for issue parsing, lint, show, and start.
- Run `atelier export --check`, `atelier lint`, and `atelier doctor`.
- Attach durable `atelier evidence` records for closeout transcripts or broader
  validation runs when the implementation work closes.

## Notes

Do not make checklist syntax the completion mechanism in this first pass. Agents
should not mark Outcome items complete by editing checkboxes. The Evidence
section describes required proof; durable `atelier evidence` records are the
attached artifacts produced during validation or closeout.
