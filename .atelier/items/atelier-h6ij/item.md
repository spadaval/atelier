---
schema: atelier.item
schema_version: 3
id: atelier-h6ij
title: Add incremental round 2+ plan reviews, 2-split-round plan exit, and 3-round code review cap
status: ready
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.622105698Z
---

## Goal

Stop multi-round plan and code reviews from spiraling into ground-up re-reviews and endless split rounds.

On `master`, code reviews already have a delta-first `later_round()` brief (`atelier-d73k`), but:
1. Plan reviews (`plan_review` / `plan_brief`) repeat round 1's full-plan brief on every round with no delta diff and no narrowing.
2. Nothing ends a split plan review (one `approve`, one `changes`), causing 4–7 round loops on both new mission branches (`atelier-to83`, `atelier-kw8u`) and plan-amendment backlog branches (`atelier-osel`; covers `atelier-s8wm`).
3. Code reviews have no escalation prompt when automated rounds reach round 3 without converging.
4. Code `later_round()` prints `git diff <prev> <commit>` even if `<prev>` was rebased away or `base` was merged in between rounds, flooding the delta diff with unrelated `base` commits.

## Done when

- [ ] On plan review round 2+, `atelier review <mission>` finds the prior completed round's reviewed commit. When that commit is unique across the round's reviews, exists, and is an ancestor of `HEAD`, `plan_brief` computes the plan's own changed paths via `atelier_core::plan::plan_changed_at` (excluding `.atelier/missions/<id>/reviews/`, `notes.md`, and unrelated items), prints `git diff <prev> <head> -- <paths>` first, and instructs the reviewer to verify carried findings and changes since the last round, blocking on unchanged plan sections only if a serious defect was missed (otherwise `defer`). Otherwise it falls back cleanly to the full plan brief. A test verifies no `.atelier/missions/<id>/reviews/*` file appears in the printed delta diff command.
- [ ] On code review round 2+, `review::brief` guards `previous`: omits a broken delta diff if `previous` is missing or not an ancestor of `commit`, and scopes the delta diff to branch-touched paths (or falls back cleanly) when `<previous>..<commit>` contains a merge from `base`.
- [ ] `atelier-core::plan` detects consecutive trailing split plan-review rounds (completed rounds with at least one `approve` and at least one `changes`). After 2 consecutive split rounds — on either a mission's branch or an amending `backlog/*` branch:
  - `atelier submit` refuses to open another review round (offering `ATELIER_SKIP` so the human can authorize another round after an amendment) and tells the planner to bring the remaining open findings to the human to rule.
  - `atelier status` (printing plan review status on mission branches and amending backlog branches) and `merge`'s `plan_unapproved` refusal print the same human-ruling exit instead of telling the planner to submit another round.
  - `output::plan_submitted` notes the 2-split-round exit instead of unconditionally telling the planner to run `submit` again on any split.
- [ ] When an item is in `changes_requested` at round 3 or higher, `status` (for both implementer and mission lead) notes that automated review reached the 3-round cap and **replaces** `Then: atelier submit` with guidance to settle via the mission lead (or starting thread) instead of looping further.
- [ ] `skills/agent-factory/roles/planner.md`, `roles/lead.md`, `roles/implementer.md`, `docs/product.md`, and `docs/design.md` document these rules. `skills/agent-factory/roles/reviewer.md` is not modified.
- [ ] Unit and CLI tests cover incremental plan briefs (excluding review files), delta fallback after missing/non-ancestor/base-merge commits, 2-split-round escalation across `submit`/`status`/`merge` on both mission and amendment branches, and the 3-round code-review cap; `cargo nextest run --locked` and `evals/agent-factory/test.sh` pass.

## Scope

In: `crates/atelier-core/src/plan.rs`, `crates/atelier-core/src/say.rs`, `crates/atelier-cli/src/review.rs`, `crates/atelier-cli/src/output.rs`, `crates/atelier-cli/src/commands.rs`, `crates/atelier-cli/src/merge.rs`, `skills/agent-factory/roles/{planner,lead,implementer}.md`, `docs/product.md`, `docs/design.md`, and tests.

Out: `skills/agent-factory/roles/reviewer.md` (owned by `atelier-oiwy`), reusing reviewer chat threads, `t3-extension/`, and changing the 2-approval requirement on round 1.
