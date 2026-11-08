---
created_at: "2026-11-08T05:52:14.763375354+00:00"
id: "atelier-dkul"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-dkul
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
  - kind: "issue"
    id: "atelier-l58v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-pmib"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Move the skill and its evals here with their history, so a command and its guidance change in one commit"
updated_at: "2026-11-08T22:28:40.135053564+00:00"
---

## Goal

Move the skill and its evals into this repository unchanged, keeping their git history if that is cheap, so the skill and atelier's commands can change in one commit. Content edits wait for the next item, so this one can be reviewed as a move.

## Done when

- [ ] `skills/agent-factory/` and `evals/agent-factory/` in this repository match spadaval/.agents `main` at the commit that merged `atelier-l58v` or later (`diff -r` against a `git archive` of that commit is empty, apart from the path fixes below), and the Report names that commit.
- [ ] History: `git log --follow skills/agent-factory/SKILL.md` shows spadaval/.agents commits (via `git subtree` or `git filter-repo` keeping only those two paths, merged with `--allow-unrelated-histories`). If `atelier land` or `atelier doctor` on a trial merge refuses that history, the Report says so and the move is a plain copy whose commit message names the source commit.
- [ ] The harness runs from here: `python3 evals/agent-factory/harness/run.py` finds the repository root and stages `skills/agent-factory` with `git archive`, `harness/test_run.py` passes, and the canary plus one scenario (`trivial-typo`, skill arm, one run) produce valid, graded results, recorded under `evals/agent-factory/results/`.
- [ ] Every link to the old location points at the new one: `AGENTS.md` (Process Sources), `docs/index.md`, `docs/product/*.md`, `docs/architecture/**/*.md`, live strategies, the skill's own `references/atelier.md` and `maintainers/`, and the eval README. `grep -rn 'spadaval/.agents\|agents-skills' --exclude-dir=.atelier --exclude-dir=target .` finds only closed records, superseded or completed strategies, ADRs 0020 and 0023, and history notes that describe the old location.
- [ ] spadaval/.agents gains a README (it has none) and a first line in `skills/agent-factory/maintainers/skill-evaluation.md`, both saying, in a commit on a branch there merged with this item (its live checkout never changes branch), that Agent Factory moved to spadaval/atelier and the copy there is frozen until `atelier-6oxt` deletes it. Pushing waits for Sumeet.
- [ ] CI passes on the pull request, and nothing under `crates/` reads files under `skills/` or `evals/`.

## Scope

Moving files and fixing paths and links. No wording changes to the skill. spadaval/.agents is not changed here.

## Context

Waits on `atelier-l58v`, which edits the skill in spadaval/.agents; the skill must not be edited in two places. The ADR item records why the move is allowed. The memory note on this host says spadaval/.agents' live checkout (`~/projects/agents-skills`) must never change branch; read from it or from a fresh clone.

## Unknowns

- Whether `atelier land` and `doctor` accept unrelated imported history (they check commits on the base). Tried on a scratch clone before the real merge.

## Notes

- 2026-11-08 [note] Import from the spadaval/.agents commit that merged atelier-l58v (or a later one), but before this item's own frozen notice there, so the moved maintainers/skill-evaluation.md does not carry the notice.
