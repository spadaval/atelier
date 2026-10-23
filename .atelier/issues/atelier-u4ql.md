---
created_at: "2026-10-17T19:00:36.340066193+00:00"
id: "atelier-u4ql"
issue_type: "feature"
labels:
- "cross-cutting"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T17:33:00.085158728+00:00"
status: "done"
title: "Say reinstall atelier when records need a newer binary"
updated_at: "2026-10-23T17:33:00.085158728+00:00"
---

## Goal

After o6h9 migrated records, the installed binary failed to parse them and the Stop hook's land failed on master with a parse error, not an instruction (b32c39e6).

## Done when

When a record's schema version or field is newer than the running binary supports, every command reports 'records need atelier >= X; reinstall' with the install command, instead of a parse error. A test with a future schema version covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. schema_version check is exact equality (atelier-records lib.rs:811-817, review.rs:79-83); status and hook advise 'atelier check'. Scope to version numbers (higher than supported -> reinstall); an unknown field does not prove a newer binary.
- 2026-10-23 [decision] A record with a higher schema_version is reported without a code ('<path> has schema_version N, and this atelier reads M, so records need a newer atelier. Next: from Atelier's source checkout, reinstall it with `cargo install --path crates/atelier-cli --locked`'); a higher workflow policy version gets the new workflow_config_schema_newer code with the same step. One phrasing serves every repository, with no detection of whether the checkout is Atelier's source.
  Reason: human-cli-output.md's error contract gives records that fail to parse no code and allows new codes only in workflow_*; the binary cannot know where an adopter's Atelier source lives, and 'from Atelier's source checkout' is right both here and elsewhere.
  Rejected: A records_schema_newer code; cargo install --git with the Cargo.toml repository URL (it names another fork than origin); detecting crates/atelier-cli in the repo root

## Close Reason

An issue, evidence or review record whose schema_version is higher than this binary reads now fails as one line, `<path> has schema_version N, and this atelier reads M, so records need a newer atelier. Next: from Atelier's source checkout, reinstall it with `cargo install --path crates/atelier-cli --locked``, in every command (status, its SessionStart context, land, the Stop hook, issue list, check); a newer workflow policy fails `workflow_config_schema_newer` with the same step, which land uses as the fix. Lower versions keep the existing errors.

### Deviations
No code: the issue text asked for "records need atelier >= X", but the binary cannot know which release reads a version, and the output contract gives a record that fails to parse no code. Field-level newness is out of scope, per the regroom note.

### Decisions
See the decision note (no code for records; workflow_* code for policy; one install phrasing for every repository).

### Surprises
`load_repository` rewrote every load error as a string with an `atelier check` step, losing the error type; it now passes a newer-schema error through. Review front matter is deny_unknown_fields, so its version is read before the strict parse. Two existing tests used schema_version 99 as a generic drift fixture; they now use a lower version and gain the newer case.

### Open items
None. Other commands that fall back to the last valid policy still say "fix .atelier/workflow.yaml" after the reinstall diagnostic; proposed as a follow-up, not filed.
