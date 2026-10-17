---
created_at: "2026-10-17T01:10:57.977106633+00:00"
id: "atelier-2tux"
issue_type: "task"
labels:
- "cli-output"
- "merge"
priority: "P3"
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
status: "backlog"
title: "Trim successful merge output and point to the next step"
updated_at: "2026-10-17T01:10:57.977106633+00:00"
---

## Goal

A successful `atelier merge` re-prints every `pass …` gate line. It then prints `Git hooks are disabled during gated integration and cleanup; put required read-only checks in the base policy's merge_checks.` on every run (crates/atelier-cli/src/commands/merge.rs:167), which is the kind of static guidance ADR 0022 Decision 2 removes. It ends with `Merged task/atelier-nqgr into master at c24af21afe568762ad5d513e4506721879522dd5; deleted local work branch. No commits or tracker records were created.` and never says that the base is now ahead of its upstream or what to run next. Agents piped 11 of 37 merges to `tail`.

## Done when

- A successful merge prints `✓ Merged task/atelier-nqgr into master at c24af21a; branch deleted. Next: atelier land, then git push.` The detached-worktree note stays when it applies.
- Gate lines aren't repeated on success (they print only when a gate fails). The hooks notice appears only when the repository has hooks configured, as one short line.
- A test pins the success output.
