---
created_at: "2026-06-16T16:18:15.539492142+00:00"
id: "atelier-2sut"
issue_type: "task"
labels:
- "cli"
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1xmi"
  - kind: "issue"
    id: "atelier-a7gd"
  - kind: "issue"
    id: "atelier-jezn"
  - kind: "issue"
    id: "atelier-m1r7"
  - kind: "issue"
    id: "atelier-vuqb"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Define normal, admin, and debug command boundaries"
updated_at: "2026-06-16T16:18:15.539492142+00:00"
---

## Description

Define the product contract before implementation changes the command surface. The current docs already imply that export and rebuild are low-level diagnostics, but older specs, Agent Factory guidance, role guides, issue evidence boilerplate, and command help still disagree.

## Outcome

- `docs/product/cli-surface.md`, command audits, and architecture docs consistently define four command categories: normal workflow, admin maintenance, hidden debug diagnostics, and temporary migration.
- The contract states that tracked `.atelier/` Markdown is authoritative; local runtime/projection state is repairable cache and must not overwrite canonical records during normal recovery.
- The contract names the intended owner for projection freshness: normal commands refresh safely when possible, `doctor` reports health, and `doctor --fix` repairs ignored local state when explicit repair is needed.
- The contract states whether any replacement for `export` remains, and if so, frames it as hidden/admin migration or deterministic-renderer testing rather than normal workflow.
- Adjacent commands with ambiguous placement are listed for confirmation or cleanup: `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, `maintenance`, `branch`, and `worktree`.

## Evidence

- Documentation file diff updates `docs/product/cli-surface.md`, `docs/product/command-audit/export.md`, `docs/product/command-audit/rebuild.md`, and related command-audit index pages with the final category contract.
- Review artifact maps each command category to at least one example command and one excluded non-example.
- Search command transcript shows no target-state product doc presents `atelier export` as a normal health, handoff, or validation command.
- `atelier lint atelier-2sut`, `atelier doctor`, and `git diff --check` pass.

## Notes

This is the contract-first blocker. Implementation issues should not choose hidden vs removed vs renamed behavior independently.
