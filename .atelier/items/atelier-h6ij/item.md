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

- [ ] On plan review round 2+, `atelier review <mission>` finds the prior completed round's reviewed commit. When that commit is unique across the round's reviews, exists, and is an ancestor of `HEAD`, `atelier-core::plan` computes the plan's own changed paths (do not reuse `plan_changed_at`, which keeps `notes.md` and includes all mission item directories even from `base` merges):
  - For a proposed mission (`!agreed`): `.atelier/missions/<id>/` (excluding `reviews/` and `notes.md`) plus `.atelier/items/<item>/` for items of the mission at `prev` or `head` (excluding any item `reviews/`).
  - For an amendment of an agreed mission (`agreed`): `.atelier/missions/<id>/` (excluding `reviews/` and `notes.md`) plus `.atelier/items/<item>/` (excluding any item `reviews/`) for items of the mission at `prev` or `head` **that the amendment branch itself changed relative to `base`** (`git diff <base>...<head>`), intersected with paths differing between `prev` and `head`. That keeps the amendment's own item edits in the delta while excluding item Reports and reviews merged into `base` between rounds.
  - When that path list is **non-empty**, `plan_brief` prints `git diff <prev> <head> -- <paths>` first and instructs the reviewer to verify carried findings and changes since the last round, blocking on unchanged plan sections only if a serious defect was missed (otherwise `defer`).
  - When that path list is **empty** (e.g., a re-submit whose only new commits are review files), `plan_brief` prints that the plan is unchanged since the prior round and tells the reviewer to judge carried findings — it **never** prints `git diff` with an empty pathspec.
  - When `prev` is missing, non-unique across the round's reviews, or not an ancestor of `HEAD`, it falls back cleanly to the full plan brief.
  - Tests verify: (a) no `.atelier/missions/<id>/reviews/*` or `notes.md` path appears in the delta diff, (b) on an amendment whose `base` merged an item of the mission between rounds AND whose amendment branch edited another item of the mission, the base-merged item's Report/reviews are excluded while the amendment's own item edit is included, and (c) an empty changed-paths list prints the plan-unchanged message with no `git diff` command.
- [ ] On code review round 2+, `review::brief` guards `previous`: omits a broken delta diff if `previous` is missing or not an ancestor of `commit`, and scopes the delta diff to branch-touched paths (or falls back cleanly) when `<previous>..<commit>` contains a merge from `base`.
- [ ] `atelier-core::plan` counts consecutive trailing split plan-review rounds (completed rounds with at least one `approve` and at least one `changes`) **only among rounds added by the plan change under review** — i.e., rounds present at `head` that are not on `base` (`base_tip`). That way an agreed mission whose initial plan merged after two splits (such as `atelier-kw8u` or `atelier-to83`) starts any later amendment at 0 splits, while two consecutive splits on the amendment itself still trigger the exit. After 2 consecutive split rounds on the current plan change (on either a mission branch or an amending `backlog/*` branch):
  - `atelier submit` refuses to open another review round (offering `ATELIER_SKIP` so the human can authorize another round after an amendment) and tells the planner to bring the remaining open findings to the human to rule.
  - `atelier status` (printing plan review status on mission branches and amending backlog branches) and `merge`'s `plan_unapproved` refusal print the same human-ruling exit instead of telling the planner to submit another round.
  - `output::plan_submitted` notes the 2-split-round exit instead of unconditionally telling the planner to run `submit` again on any split.
- [ ] When an item is in `changes_requested` at round 3 or higher, `status` (for both implementer and mission lead) notes that automated review reached the 3-round cap and **replaces** `Then: atelier submit` with guidance to settle via the mission lead (or starting thread): merge with `ATELIER_SKIP="<ruling>"`, adjust `Scope`/`Done when`, `drop`, or rule to fix (noting that once the lead rules to fix, `atelier submit` opens the next round).
- [ ] `skills/agent-factory/roles/planner.md`, `roles/lead.md`, `roles/implementer.md`, `docs/product.md`, and `docs/design.md` document these rules. `skills/agent-factory/roles/reviewer.md` is not modified.
- [ ] Unit and CLI tests cover incremental plan briefs (excluding reviews, `notes.md`, and merged item paths on amendments, plus empty-delta handling), delta fallback after missing/non-ancestor/base-merge commits, 2-split-round escalation across `submit`/`status`/`merge` on both mission and amendment branches (including an amendment of a mission whose initial plan merged after 2 splits opening its first round cleanly), and the 3-round code-review cap; `cargo nextest run --locked` and `evals/agent-factory/test.sh` pass.

## Scope

In: `crates/atelier-core/src/plan.rs`, `crates/atelier-core/src/say.rs`, `crates/atelier-cli/src/review.rs`, `crates/atelier-cli/src/output.rs`, `crates/atelier-cli/src/commands.rs`, `crates/atelier-cli/src/merge.rs`, `skills/agent-factory/roles/{planner,lead,implementer}.md`, `docs/product.md`, `docs/design.md`, and tests.

Out: `skills/agent-factory/roles/reviewer.md` (owned by `atelier-oiwy`), reusing reviewer chat threads, `t3-extension/`, and changing the 2-approval requirement on round 1.
