---
schema: atelier.item
schema_version: 3
id: atelier-zc9b
title: Fix recurring CLI friction in submit handoffs, status on base, pre-push recovery, and Responses parsing
status: in_review
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A337c6277-e094-40d2-a1c8-de83cd3a5839%3Adelegate-task%3Af8749286-70ab-419c-98a1-f19557181b40
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.741617559Z
---

## Goal

Eliminate five small, recurring CLI friction bugs already observed in real agent runs and evals:
1. **`atelier-u49o`:** `atelier submit` tells the reviewer `Run atelier review <id> and follow what it prints.` without naming the checkout path, causing reviewers started from another directory (often the main checkout) to hit a refusal on their first call.
2. **`atelier-o2vv`:** On `base` when a thread holds nothing, `atelier status` ends with `Then: atelier start <top-ready-item>`, which can steer a delegated thread toward an unrelated item.
3. **`atelier-yhaf`:** In the base's own checkout, `atelier status` for a thread holding a started item prints `Then: atelier start <id> (switches to its branch)`, even though `start` does not switch the base checkout — causing agents that follow `Then:` lines to loop.
4. **`atelier-jfos`:** `pre-push`'s refusal tells the agent to run `git branch -f <base> <old>`, which Git refuses whenever `<base>` is checked out in the current worktree.
5. **`atelier-o91z`:** Any malformed line under `### Responses` makes the whole `item.md` fail to parse (exit 2), wedging `status`, `show`, and `review` on the item and preventing `ATELIER_SKIP`.

Covers `atelier-u49o`, `atelier-o2vv`, `atelier-yhaf`, `atelier-jfos`, and `atelier-o91z`.

## Done when

- [x] `submit`'s reviewer handoff task line names the item checkout's path (e.g., `In <path>, run atelier review <id> and follow what it prints.`), with no backticks; a test covers a reviewer following that task from the main checkout.
- [x] Away from an item branch, when the caller holds nothing, `atelier status` lists startable items without printing `Then: atelier start <id>`. `skills/agent-factory/SKILL.md` and `AGENTS.md` add one line noting that if a session starts in a different repository from its task, run `atelier status` again in the task's checkout.
- [x] In the base checkout, when `atelier status` shows a started item the caller holds, it names the existing worktree path (via `git::checkout_of`) or prints `git worktree add <path> <branch>`, never `Then: atelier start <id> (switches to its branch)`. Elsewhere, `status` still suggests `atelier start <id>`.
- [x] The recovery steps `pre-push` prints on a refused base push, run verbatim on a checked-out base, move the commit onto a new item branch (not orphaned) and restore `<base>` to `<old>` via `git reset --keep <old>` without discarding uncommitted changes (or, when uncommitted changes overlap files touched by the bad commit, `git reset --keep <old>` refuses cleanly without losing work, and the printed instructions state how to finish restoring `<base>` after stashing/saving); hook scenario tests run the printed steps end-to-end when local `<base>` is strictly ahead of `<old>` (no empty cherry-pick), with non-overlapping uncommitted work (base restored), and with overlapping uncommitted work (work preserved cleanly).
- [x] Malformed lines under `### Responses` do not cause `ItemDir` loading to fail with exit 2: valid responses still parse, while malformed lines are reported as validation problems by `atelier check`, `atelier status`, `atelier submit`, `atelier verdict`, and `atelier merge` (naming the line and expected syntax, so a malformed `not built:` line is never silently ignored), while `status`, `show`, and `review` remain usable on the item.
- [x] `docs/design.md` and `docs/product.md` are updated to match the changed `status`, `pre-push`, and `Responses` validation behavior.
- [x] `cargo nextest run --locked` and `evals/agent-factory/test.sh` pass (update any fixture test assertions that only matched the old buggy `status` or `pre-push` output, keeping every scenario's contract intact).

## Scope

In: `crates/atelier-core/src/{records/body.rs,records/validate.rs,records/store.rs,actions.rs,review.rs,disputed.rs,say.rs,work.rs}`, `crates/atelier-cli/src/{commands.rs,output.rs,check.rs,review.rs}`, `crates/atelier-desk/src/read.rs`, `skills/agent-factory/SKILL.md`, `AGENTS.md`, `docs/design.md`, `docs/product.md`, `evals/agent-factory/` (only if a scenario's unit test asserts the old buggy wording), and Rust tests.

Out: `skills/agent-factory/roles/reviewer.md`, `t3-extension/`, and removing `draft`/`ready` (`atelier-dh7a`).

## Report

The five commands that sent agents into a dead end now name a step that
works.

- **Reviewer task (u49o):** `submit`, the implementer's `status` and the
  refusal of an author's `review` print `In <path>, run atelier review <id>
  and follow what it prints.`
- **`status` with nothing held (o2vv):** it lists what can start, then
  `Start one only if that is your task: atelier start <id>`, with no
  `Then:`. The skill and `AGENTS.md` have the line about another repository.
- **`status` in the base's checkout (yhaf):** it ends `Then: cd <worktree>`,
  or `Then: git worktree add <path> <branch>`, after a `git worktree prune`
  where the listed worktree's directory is gone.
- **pre-push (jfos):** it prints one line,
  `atelier start --new "<title>" && git reset --keep <old>`. The `&&` is
  there so that a failed `start` leaves the base holding the commits. It
  also covers a base checked out in another worktree (`cd` first) and one
  checked out nowhere (`git branch -f`). Where a new branch would not hold
  the commits, such as `feature:main` or a forge base that moved on, it
  prints no reset, which would have orphaned them.
- **Responses (o91z):** a malformed line is a problem of that line, quoted
  with the three forms. `check`, `submit` and both verdicts refuse it as a
  rule.

### Judgment calls

- `merge` takes no skip of a malformed response. Its check of the merged
  tree refuses an invalid record whatever is skipped, so offering a skip
  would have been a false way out, and the line may be a `not built` for
  the lead.
- A malformed line's problem names the record from the repository's root.
  The first smoke test showed a skip writing the checkout's absolute path
  into the commit's `Atelier-Skip` trailer; a test now covers it.
- `submit` and `merge` run off an item's branch no longer say that `start`
  switches to it, which was the same false advice as yhaf.
- The change reaches `atelier-core`'s `view.rs`, `repo.rs` and `plan.rs`,
  which Scope does not list: `Entry` and `Facts` carry the checkout's path
  and the malformed lines to what `status` prints. `README.md`'s sample of
  `submit` follows the new task line.

### Validation

On the tree with origin/master (5d1834854) merged in:

- `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D
  warnings` and `git diff --check`: clean.
- `cargo nextest run --locked`: 433 passed, 1 skipped.
- `evals/agent-factory/test.sh` with the built binary: all pass. The one
  fixture changed is refused-push's, which now moves main as the hook says.
- `target/debug/atelier check` on this repository: every record is valid.
- The hook tests run the printed steps in a shell: with and without a
  forge, with non-overlapping and overlapping uncommitted work, from
  another checkout, with no checkout of the base, and with a failing
  `start`.
- By hand, in scratch repositories with hooks installed: the refused push,
  its steps, both `Then:` lines in a path with a space, and a malformed
  line through submit, review, verdict and merge.
- Not driven: the Desk's item page with a malformed line. Its read no
  longer fails on one, and no test covers that page.

### Found and left for the lead to file

- `status` away from the base still says `atelier start <id>` when another
  worktree has the branch out, and `start` refuses there. Done when asks
  for `start` elsewhere, so this item keeps it.
- `status` in a base checkout ahead of its remote says `Push with: git push
  origin master`, which pre-push refuses.
- The commit line `verdict` prints after a skip carries the skip's trailers
  but not `ATELIER_SKIP`, so pre-commit refuses it until the reviewer adds
  the variable before `git commit`.
- `evals/agent-factory/test.sh` leaves about 60 directories in `TMPDIR`
  each run.
