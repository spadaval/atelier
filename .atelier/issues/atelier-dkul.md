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

- [x] `skills/agent-factory/` and `evals/agent-factory/` in this repository match spadaval/.agents `main` at the commit that merged `atelier-l58v` or later (`diff -r` against a `git archive` of that commit is empty, apart from the path fixes below), and the Report names that commit.
- [x] History: `git log --follow skills/agent-factory/SKILL.md` shows spadaval/.agents commits (via `git subtree` or `git filter-repo` keeping only those two paths, merged with `--allow-unrelated-histories`). If `atelier land` or `atelier doctor` on a trial merge refuses that history, the Report says so and the move is a plain copy whose commit message names the source commit.
- [x] The harness runs from here: `python3 evals/agent-factory/harness/run.py` finds the repository root and stages `skills/agent-factory` with `git archive`, `harness/test_run.py` passes, and the canary plus one scenario (`trivial-typo`, skill arm, one run) produce valid, graded results, recorded under `evals/agent-factory/results/`.
- [x] Every link to the old location points at the new one: `AGENTS.md` (Process Sources), `docs/index.md`, `docs/product/*.md`, `docs/architecture/**/*.md`, live strategies, the skill's own `references/atelier.md` and `maintainers/`, and the eval README. `grep -rn 'spadaval/.agents\|agents-skills' --exclude-dir=.atelier --exclude-dir=target .` finds only closed records, superseded or completed strategies, ADRs 0020 and 0023, and history notes that describe the old location.
- [x] spadaval/.agents gains a README (it has none) and a first line in `skills/agent-factory/maintainers/skill-evaluation.md`, both saying, in a commit on a branch there merged with this item (its live checkout never changes branch), that Agent Factory moved to spadaval/atelier and the copy there is frozen until `atelier-6oxt` deletes it. Pushing waits for Sumeet.
- [ ] CI passes on the pull request, and nothing under `crates/` reads files under `skills/` or `evals/`.

## Scope

Moving files and fixing paths and links. No wording changes to the skill. spadaval/.agents is not changed here.

## Context

Waits on `atelier-l58v`, which edits the skill in spadaval/.agents; the skill must not be edited in two places. The ADR item records why the move is allowed. The memory note on this host says spadaval/.agents' live checkout (`~/projects/agents-skills`) must never change branch; read from it or from a fresh clone.

## Unknowns

- Whether `atelier land` and `doctor` accept unrelated imported history (they check commits on the base). Tried on a scratch clone before the real merge.

## Notes

- 2026-11-08 [note] Import from the spadaval/.agents commit that merged atelier-l58v (or a later one), but before this item's own frozen notice there, so the moved maintainers/skill-evaluation.md does not carry the notice.

## Report

The skill and its evals are in this repository with their history, imported from spadaval/.agents `c75a9b1` (the merge of atelier-l58v). `diff -r` of `skills/agent-factory` against a `git archive` of `c75a9b1` is empty, and `evals/agent-factory` differs only by the new `results/2026-11-08-moved-to-atelier.md`. `git log --follow -- skills/agent-factory/SKILL.md` lists 34 spadaval/.agents commits. `harness/test_run.py` passes (4 tests), and the Haiku canary plus one `trivial-typo` skill-arm Opus run pass, read by hand. The item's grep finds only ADRs 0020, 0023 and 0029, the completed `lighter-workflow` and `simpler-lifecycle` strategies, this mission's strategy (which plans the cleanup of spadaval/.agents), the closeout's history note and the new results entry. spadaval/.agents has a README and a first-line notice on branch `atelier-dkul` (`84102d9`), unmerged and unpushed. `cargo fmt -- --check`, `git diff --check master...HEAD`, `atelier land` and `atelier doctor` pass. The CI line waits for the pull request.

### Decisions

- History came through `git filter-repo --path skills/agent-factory --path evals/agent-factory` on a clone at `c75a9b1`, merged with `--allow-unrelated-histories` (`661aeef5`). The filtered trees have the same hashes as `c75a9b1`'s. filter-repo rewrote commit hashes, including hashes quoted in imported commit messages, so old messages name the new hashes. On a scratch clone, `atelier doctor` and `atelier land` passed both on the branch and on master after a trial merge, so no plain-copy fallback was needed.
- Two imported files end in a blank line (`results/2026-11-04-mission-shaping/input.md`, `scenarios/costly-hardening/files/base/flags.py`), which CI's `git diff --check` would refuse. `.gitattributes` marks them `-whitespace` rather than editing them, so the move stays byte for byte.
- Two doc links were already broken at the old location: `references/tracker-records.md` and `plan.md#ready-missions`. They now point at the sections that replaced them (`references/atelier.md#records` and `#ready-work`).

### Surprises

- Nothing in the moved tree named the old location. Its relative links resolve unchanged and `run.py` finds the root as `parents[3]`, so the skill, its `references/atelier.md`, `maintainers/` and the eval README needed no edits.
- `crates/atelier-cli/src/commands/skill_link.rs` (`setup --link`, ADR 0029 Decision 4) checks that `<checkout>/skills/agent-factory/SKILL.md` is a file before linking it. It does not read the skill's contents; no other code under `crates/` touches `skills/` or `evals/`. A test fixture directory named `agents-skills` in `tests/update_check.rs` became `skill-checkout`; `update_check` passes (11 tests).
- In the scratch trial, a plain `git merge --no-ff` of the unreviewed import branch into master passed `atelier land`, which reported "2 unpushed record-only commits outside review" and did not flag the imported code. A direct code commit on master was refused. This looks like a gap in land's base check for merge commits; not filed.

### Open items

- The CI line stays open until the pull request's run passes.
- spadaval/.agents `atelier-dkul` (`84102d9`) merges with this item; pushing waits for Sumeet.
- This host's installed skill still links `~/projects/agents-skills` (`atelier doctor` says so); atelier-bxs7 relinks it.
