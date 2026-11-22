---
schema: atelier.mission
schema_version: 3
id: atelier-ww7w
title: Cut review, CI, and branch overhead, and close the garden and Desk history loops
status: open
labels: []
lead: null
planners:
- 92ddb56e-5c41-43a2-a697-03aa5ee83459
created_at: 2026-11-22T05:30:19.649159093Z
---

## Why

Sumeet asked for this mission in conversation on 2026-11-22 after a full review of Atelier's effectiveness and overhead on `master` (`ac44653e3`). During that conversation he explicitly approved all six areas below, including having `atelier merge` delete merged remote branches (lifting `AGENTS.md`'s blanket ban strictly for `atelier merge` using a lease check) while leaving `t3-extension/` untouched for now.

Recent rewrites cut CLI+core size sharply and put the lead in charge, but concrete overhead and reliability gaps remain across six areas:

1. **Plan reviews loop without narrowing or stopping:** `atelier-to83` took 7 plan-review rounds, `atelier-osel` took 7 across its initial plan and amendment, and `atelier-kw8u` took 4 plus a forced skip — repeatedly splitting 1-approve / 1-changes while fresh round 2+ reviewers re-read the whole plan from scratch (`atelier-s8wm`). Code reviews also lack a clear escalation cap after 3 rounds.
2. **Record-only CI does unnecessary work and filters stale paths:** `.github/workflows/ci.yml` installs Node and `pnpm` and runs `scripts/t3-extension-release.test.sh` inside the `changes` gate on every record-only PR (taking ~26–39s instead of ~10–15s). `scripts/ci-jobs.sh` still filters deleted 0.4 paths (`.atelier/issues/*`, `.atelier/reviews/*`, `.atelier/evidence/*`) instead of `.atelier/items/*` and `.atelier/missions/*` (`atelier-i70l`), so non-Markdown validation artifacts trigger the full Rust suite. CI's `test` job also never runs `cargo clippy`.
3. **Branch lifecycle leaves sprawl and takes two branches to drop a started item:** Dropping a started item requires a drop+commit+push on the item's branch *and* a second drop on a `backlog/*` branch. Nothing deletes merged branches on `atelier merge` or reports branch/worktree accumulation (this clone holds 285 merged local branches including `t3/*`, 266 merged remote branches, and 238 worktrees, slowing `atelier list` to ~1.0–1.4s vs. ~0.7s in a clean clone).
4. **Five recurring CLI friction bugs keep causing failed calls:** `submit` omits the item checkout path in reviewer handoffs (`atelier-u49o`, hit in 3 of 7 smoke scenarios), `status` on base points delegated threads at unrelated items (`atelier-o2vv`) and tells held items that `start` switches the base checkout (`atelier-yhaf`), `pre-push`'s recovery command fails when the base branch is checked out (`atelier-jfos`), and one malformed `### Responses` line breaks every command on the item (`atelier-o91z`).
5. **Garden runs stall at the human boundary:** `garden.md` holds approved garden branches unmerged until a synchronous human session, leaving `atelier-0019` and `atelier-qb4f` unmerged (approved locally, still `in_review` on `origin`) for over a week and stranding their upkeep fixes and deferred-finding settlements.
6. **Desk mission history (M5) is still unimplemented and `atelier_core::log` omits mission-plan lifecycle actions:** `GET /api/repos/{repo}/missions/{id}/history` still returns 404, and `atelier_core::log::Happened` does not record when a mission was planned or when its plan merged, preventing the Desk from deriving exact mission timelines from `atelier-core`.

## Outcome

- **CI is fast for records and enforces lints:** Record-only PRs skip Node/`pnpm` and `t3-extension-release.test.sh` in the `changes` job; `scripts/ci-jobs.sh` uses current `.atelier/items/*` and `.atelier/missions/*` paths (including non-Markdown artifacts); and CI's `test` job enforces `cargo clippy --all-targets --locked -- -D warnings`.
- **Reviews converge instead of spiraling:** Round 2+ plan reviews diff only the plan's own changed files (excluding `reviews/` and unrelated items), split plan reviews stop at `submit`, `status`, and `merge` after 2 consecutive split rounds (covering both mission branches and plan amendments on backlog branches) and bring remaining findings to the human, code review `later_round` guards missing/rebased/base-merged commits, and code items at `changes_requested` in round 3+ replace `Then: atelier submit` with escalation to the mission lead.
- **Branch operations are lean and clean:** A started item can be dropped in a single `backlog/*` branch (closing its open PR on merge and naming any newly filed items on its branch); `atelier merge` safely deletes the merged remote branch with `--force-with-lease=refs/heads/<branch>:<head>` and detaches/deletes the merged local branch at `<head>`; and `atelier status` prints a read-only summary when merged local branches or detached worktrees pile up.
- **The five CLI paper cuts are fixed:** `submit` names the item checkout path, `status` never misdirects delegated threads or base checkouts, `pre-push`'s recovery steps move the commit off a checked-out base without discarding uncommitted work, and malformed `### Responses` lines report cleanly without crashing `status`, `show`, or `review`.
- **Garden runs merge on approval:** Reviewed upkeep fixes and deferred-finding settlements merge immediately, while proposals and newly found bugs go to the human via a single maintained `needs-human,garden` item (and `bug,needs-human` items) on a backlog branch, with a planner applying answers and closing it.
- **Desk mission history (M5) is backed by `atelier_core::log::history`:** `atelier_core::log` records mission creation (`Plan`) and plan merge (`PlanMerge`) with a bounded walk, and `GET /api/repos/{repo}/missions/{id}/history` serves exact record-diff history from `atelier-core`. Existing `/api/history` and `/api/launch` stay untouched so `t3-extension/` (including `t3-extension/fixture-desk.mjs`'s empty-commit fixture) and pre-cutover history keep working with zero `t3-extension/` edits.

## Success criteria

- **fast-record-ci:** In `.github/workflows/ci.yml`, the `changes` job has no Node/`pnpm` setup and does not run `t3-extension-release.test.sh` (which still runs when `t3-extension/` or `scripts/t3-extension-release*` changes). `scripts/ci-jobs.test.sh` passes with zero references to `.atelier/issues/`, `.atelier/reviews/`, or `.atelier/evidence/`, proving both `.md` and non-`.md` files under `.atelier/items/` and `.atelier/missions/` skip `rust` while `.atelier/config.toml` runs `rust`. CI's `test` job runs `cargo clippy --all-targets --locked -- -D warnings` and passes on `master`.
- **converging-reviews:** CLI tests prove: (a) round 2+ `atelier review <mission>` prints a delta diff restricted to the plan's own paths (`plan_changed_at`, excluding `.atelier/missions/<id>/reviews/` and unrelated items) and narrows the brief (falling back when `prev` is missing, non-unique across the round, or not an ancestor); (b) after 2 consecutive split plan-review rounds (on either a mission branch or an amending backlog branch), `submit`, `status`, and `merge`'s refusal tell the planner to bring open findings to the human instead of opening another round; (c) at `changes_requested` in round 3+, `status` replaces `Then: atelier submit` with escalation to the mission lead; (d) `roles/planner.md`, `roles/lead.md`, and `roles/implementer.md` document these rules without modifying `roles/reviewer.md`.
- **clean-branches-and-drop:** CLI tests prove: (a) `atelier drop <started-item> --reason "<why>"` works directly on a `backlog/*` branch, merges cleanly, stops the item on base, names any newly added item records on the dropped branch, and closes the item's open PR on forge merge; (b) `atelier merge` deletes the merged remote branch via `--force-with-lease=refs/heads/<branch>:<head> --delete` (with a race test proving a late push is preserved, not deleted) and detaches + deletes the merged local branch at `<head>` (leaving it intact if uncommitted dirt was skipped); (c) `atelier status` prints a read-only count + inspection hint (`git branch --merged <base>`, `git worktree list`) above threshold without suggesting destructive deletion; (d) `AGENTS.md`, `roles/lead.md`, and `docs/design.md` reflect both changes.
- **cli-paper-cuts-fixed:** CLI, hook, and unit tests prove all five fixes (`u49o`, `o2vv`, `yhaf`, `jfos` running the printed recovery steps end-to-end without losing uncommitted base work, and `o91z` keeping `status`/`show`/`review` working while `submit`/`verdict`/`merge` refuse malformed `Responses`), `docs/design.md` and `docs/product.md` are updated, and `evals/agent-factory/test.sh` passes against the updated binary.
- **garden-unblocked:** `skills/agent-factory/procedures/garden.md`, `skills/agent-factory/SKILL.md`, `docs/product.md`, and `docs/design.md` specify merging approved upkeep/settlements immediately, filing/updating one `needs-human,garden` proposals item (and `bug,needs-human` items) on a backlog branch, reading carried proposals from `needs-human,garden` items, and having a planner apply human answers and close it; `atelier check` and `evals/agent-factory/test.sh` pass.
- **desk-mission-history:** `atelier_core::log` records mission creation and plan merge and supports a bounded walk; `GET /api/repos/{repo}/missions/{id}/history` returns exact record-diff mission history with generated `ts-rs` types and Rust tests; existing `/api/history` and `/api/launch` tests, `desk/` tests (`pnpm typecheck && pnpm test && pnpm build && pnpm e2e`), and `node t3-extension/run-tests.mjs` pass with zero edits under `t3-extension/`.
- **all-checks-pass:** On `master` after the final item merges, `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo nextest run --locked`, `atelier check`, `evals/agent-factory/test.sh`, `desk/` tests, and `node t3-extension/run-tests.mjs` all pass.

## Scope

**In:**
- `.github/workflows/ci.yml`, `scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`
- `crates/atelier-core`, `crates/atelier-cli`, `crates/atelier-desk`
- `desk/src/api/` (generated TypeScript types for mission history) and Desk docs (`docs/architecture/desk.md`, `docs/product/desk.md`)
- `skills/agent-factory/SKILL.md`, `roles/planner.md`, `roles/lead.md`, `roles/implementer.md`, `procedures/garden.md`, `AGENTS.md`, `docs/product.md`, `docs/design.md`
- Dropping the 8 superseded loose items (`atelier-i70l`, `atelier-s8wm`, `atelier-u49o`, `atelier-o2vv`, `atelier-yhaf`, `atelier-jfos`, `atelier-o91z`, `atelier-y0ba`) on this mission's branch.

**Out:**
- **`t3-extension/`:** Zero file changes in `t3-extension/` (deferred by Sumeet on 2026-11-22, including `atelier-yy4j` and migrating `t3-extension/fixture-desk.mjs` off empty commits). Because of that freeze, `/api/history` and `/api/launch` keep their existing commit-subject parser in this mission; migrating them is filed as a follow-up for when `t3-extension/` reopens.
- **`atelier-dh7a`:** Removing `draft`/`ready` and letting `atelier new` file on the current branch stays in its already-agreed mission.
- **`atelier-kw8u` / `atelier-oiwy`:** Do not edit `skills/agent-factory/roles/reviewer.md` or run model eval matrices; keep `oiwy`'s reviewer experiment isolated.
- **Moving all CLI actions into `atelier-core` & plain-English/docs-site overhaul:** Deferred to Mission 2 after `atelier-dh7a` merges so CLI wording and docs are rewritten once against the `open`-only model.
- **Post-cutover 2-week scorecard (`atelier-pwhn`)** and **settling existing garden branches (`atelier-0019`, `atelier-qb4f`)**: handled outside this mission.

## Authority

- **Approved by Sumeet on 2026-11-22:** Lifting `AGENTS.md:71`'s blanket prohibition on deleting remote branches specifically for `atelier merge` deleting a just-merged branch using `--force-with-lease=refs/heads/<branch>:<head> --delete`. Manual remote branch deletion and force-pushing `master` remain forbidden.
- **The lead and implementers may decide:** exact helper module layout inside `atelier-core`/`atelier-cli`/`atelier-desk`, exact wording of new CLI messages (in plain, direct English), threshold values for read-only stale-branch/worktree notices in `status`, caching/timeout details for Desk's mission history route, and execution order across independent items.
- **The lead may shrink or drop** an individual sub-fix if it creates an unexpected conflict with `atelier-dh7a`, `atelier-oiwy`, or `evals/agent-factory/test.sh`, recording the reason and filing a follow-up.
- **Reserved for the human:** changing the round-cap thresholds (3 code rounds / 2 split plan rounds), touching `t3-extension/`, altering `atelier-dh7a` or `atelier-kw8u` plans, editing `roles/reviewer.md`, or breaking record format compatibility across repositories.

## Result
