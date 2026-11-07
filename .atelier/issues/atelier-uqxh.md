---
created_at: "2026-11-07T15:35:46.504583439+00:00"
id: "atelier-uqxh"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-uqxh
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
status: "in_progress"
title: "Clear the contradictions the mission's cold read found"
updated_at: "2026-11-07T15:38:41.980076666+00:00"
---

## Goal

The mission validation (atelier-2bho, receipt atelier-pznw) found that a fresh agent reading only the agent-factory skill, astromech's AGENTS.md and the CLI help gets the lifecycle right but meets contradictions. Remove them so the legible criterion passes.

## Done when

- [ ] The skill (references/atelier.md:144-147) says a loose end is planning like any other and is filed straight to the base, even for an item in flight (its `blocked_by` link lives on the loose end, so the in-flight item's record is not touched), matching astromech's AGENTS.md:36-39; one rule, no exception.
- [ ] The skill's Words section (references/atelier.md) says that the `close_reason` policy key and `--reason` write the Report on every transition that asks for one, so "nothing else closes" does not contradict them; and that `done` and `superseded` are both terminal, the policy's done category, with `done` meaning merged or done on the base and `superseded` meaning retired.
- [ ] The skill says one thing about decision records and ADRs: a decision record holds the question and decision, and an ADR is the document a decision record may produce (`issue update --adr`); tracker-records.md, decide.md and repository-shape.md agree.
- [ ] The skill no longer mentions `issue note --one-way` (tracker-records.md:237-238), which does not exist.
- [ ] The skill, Atelier's AGENTS.md (:30-32), astromech's AGENTS.md (:21-22, changed through astromech's own review and merge) and `atelier --help` say the same thing about a Done when line rewritten after start: it is shown in the review brief and by `merge --check`, and refuses nothing; record edits never make an approval stale on their own. The skill's lines 65-67 ("gets a fresh approval") go, since lines 91-92 already say this.
- [ ] A fresh cold read of the same three sources finds no contradiction: the spadaval/.agents worktree commit that will merge, astromech's AGENTS.md on main after its change merges plus its workflow policy, and the CLI help from this branch's binary; its report is attached as a note.

## Scope

The agent-factory skill in spadaval/.agents (worktree; reviewed at a named commit; merged into main no-ff with this item), Atelier's AGENTS.md and CLI help text, astromech's AGENTS.md lines 21-22 (its own light-path branch or issue, reviewed and merged there).

## Context

Receipt atelier-pznw lists each contradiction with file and line. docs/strategies/simpler-lifecycle.md criterion legible.
