---
created_at: "2026-11-07T15:35:46.504583439+00:00"
id: "atelier-uqxh"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Clear the contradictions the mission's cold read found"
updated_at: "2026-11-07T15:35:46.504583439+00:00"
---

## Goal

The mission validation (atelier-2bho, receipt atelier-pznw) found that a fresh agent reading only the agent-factory skill, astromech's AGENTS.md and the CLI help gets the lifecycle right but meets contradictions. Remove them so the legible criterion passes.

## Done when

- [ ] astromech's AGENTS.md says planning changes go straight to `main`, except a loose end for an item in flight, which is filed on that item's branch, as the skill says (references/atelier.md:144-147); changed through astromech's own review and merge.
- [ ] The skill's Words section (references/atelier.md) says that the `close_reason` policy key and `--reason` write the Report on every transition that asks for one, so "nothing else closes" does not contradict them; and that `done` and `superseded` are both terminal, the policy's done category, with `done` meaning merged or done on the base and `superseded` meaning retired.
- [ ] The skill says one thing about decision records and ADRs: a decision record holds the question and decision, and an ADR is the document a decision record may produce (`issue update --adr`); tracker-records.md, decide.md and repository-shape.md agree.
- [ ] The skill no longer mentions `issue note --one-way` (tracker-records.md:237-238), which does not exist.
- [ ] The skill, Atelier's AGENTS.md and `atelier --help` say the same thing about a Done when line rewritten after approval: it is shown to the reviewer, who looks again; record edits never make an approval stale on their own.
- [ ] A fresh cold read of the same three sources (the skill, astromech's AGENTS.md on main and workflow policy, and the CLI help) finds no contradiction; its report is attached as a note.

## Scope

The agent-factory skill in spadaval/.agents (worktree; reviewed at a named commit; merged into main no-ff with this item), Atelier's AGENTS.md and CLI help text, astromech's AGENTS.md (its own light-path branch or issue, reviewed and merged there).

## Context

Receipt atelier-pznw lists each contradiction with file and line. docs/strategies/simpler-lifecycle.md criterion legible.
