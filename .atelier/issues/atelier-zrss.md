---
created_at: "2026-11-10T21:30:48.869299317+00:00"
id: "atelier-zrss"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add the `ATELIER_SKIP` bypass to every rule of a command or a hook"
updated_at: "2026-11-10T21:30:48.869299317+00:00"
---

## Goal

Give every rule one intentional way past it. Atelier's rules are nudges for trusted agents, not hard blocks: when a rule is wrong for the case in hand, an agent sets `ATELIER_SKIP="<reason>"` for one command, the tool says which rule it let pass and why, and where Atelier makes the commit the reason stays in history.

## Done when

- [ ] With `ATELIER_SKIP` set to a non-empty reason, a command goes ahead where a rule would have refused it. A rule is a role, a status or a condition in the act table, or a rule of a Git hook. The tool prints each rule it let pass and the reason. Tests: `merge` on an item with no approval; `submit` with an unticked Done when line; `start` on a draft item; a commit of a status change the act table does not list; and a push of the base holding a plain commit that touches code.
- [ ] Where Atelier makes the commit (`start`, `submit`, `verdict`, `merge`), its message gains one trailer for each rule passed, `Atelier-Skip: <rule>: <reason>`. A skip in a hook, or on a command that makes no commit, writes nothing.
- [ ] `ATELIER_SKIP` set to an empty or blank value is refused by every command and hook, with a line saying a reason is required.
- [ ] What is not a rule cannot be skipped, and the refusal says so: a record that does not parse, an ID or a finding that does not exist, a merge that conflicts, inputs that moved while `merge` ran, and a Git or forge command that failed. A test sets the variable against each.
- [ ] Every refusal of a rule ends with the way out first and the skip second, from one place in the code, so the two never disagree. Refusals that cannot be skipped do not mention it.
- [ ] Fresh agents do not reach for the skip when the way out is open: the `next-loop` grader fails a run in which `ATELIER_SKIP` was set, and one run on Claude Opus and one on Codex pass it with the new refusal wording, recorded in the scenario's results file.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built, including the list of what is and is not a rule.

## Scope

`ATELIER_SKIP` in every command and hook of `next/`. No flag and no config option: the variable is the whole interface. Counting skips is the scorecard's, from transcripts. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, "Hooks and the bypass". Behaviour: `docs/strategies/redesign-product.md`, Principle 6 and "Hooks and the bypass". The act table is in `atelier-app`'s `acts.rs`; every command's refusal already comes from it, which is what makes one place for the skip possible. The hooks are atelier-pz1y's, and `escalate` and `decide` are atelier-axfa's; this item follows both so that it covers every command. The fresh-agent scenario is `evals/agent-factory/scenarios/next-loop/`.

This is the third of step D's three items in `docs/strategies/redesign-2-core.md`.
