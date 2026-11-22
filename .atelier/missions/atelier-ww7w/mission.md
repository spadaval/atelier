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

Sumeet asked for this mission in conversation on 2026-11-22 after a full review of Atelier's effectiveness and overhead on `master` (`ac44653e3`).

Recent rewrites cut CLI+core size sharply and put the lead in charge, but concrete overhead and reliability gaps remain across six areas:

1. **Plan reviews loop without narrowing or stopping:** `atelier-to83` took 7 plan-review rounds, `atelier-osel` took 6, and `atelier-kw8u` took 4 plus a forced skip — repeatedly splitting 1-approve / 1-changes while fresh round 2+ reviewers re-read the whole plan from scratch (`atelier-s8wm`). Code reviews also lack a clear escalation cap after 3 rounds.
2. **Record-only CI is slow and misconfigured:** `.github/workflows/ci.yml` installs Node and `pnpm` and runs `scripts/t3-extension-release.test.sh` inside the `changes` gate on every record-only PR. `scripts/ci-jobs.sh` still filters deleted 0.4 paths (`.atelier/issues/*`, `.atelier/reviews/*`, `.atelier/evidence/*`) instead of `.atelier/items/*` and `.atelier/missions/*` (`atelier-i70l`), so non-Markdown validation artifacts trigger the full Rust suite. CI's `test` job also never runs `cargo clippy`.
3. **Branch lifecycle leaves sprawl and takes two branches to drop a started item:** Dropping a started item requires a drop+commit+push on the item's branch *and* a second drop on a `backlog/*` branch. Nothing deletes merged branches on `atelier merge` or warns on worktree sprawl (this clone has 285 merged local branches, 266 merged remote branches, and 238 worktrees, slowing `atelier list` to ~0.94s).
4. **Five recurring CLI friction bugs keep causing failed calls:** `submit` omits the item checkout path in reviewer handoffs (`atelier-u49o`, hit in 3 of 7 smoke scenarios), `status` on base points delegated threads at unrelated items (`atelier-o2vv`) and tells held items that `start` switches the base checkout (`atelier-yhaf`), `pre-push`'s recovery command fails when the base branch is checked out (`atelier-jfos`), and one malformed `### Responses` line breaks every command on the item (`atelier-o91z`).
5. **Garden runs stall at the human boundary:** `garden.md` holds approved garden branches unmerged until a synchronous human session, leaving `atelier-0019` and `atelier-qb4f` unmerged for over a week and stranding their upkeep fixes and deferred-finding settlements.
6. **Desk history still parses commit subjects instead of reading records:** `crates/atelier-desk/src/history.rs` regex/prefix-parses commit messages even though `atelier_core::log::history` already derives exact action history from Git, and `/api/repos/{repo}/missions/{id}/history` (M5) is still unimplemented.

## Outcome

- **CI is fast for records and enforces lints:** Record-only PRs skip Node/`pnpm` and `t3-extension-release.test.sh` in the `changes` job; `scripts/ci-jobs.sh` uses current `.atelier/items/*` and `.atelier/missions/*` paths (including non-Markdown artifacts); and CI's `test` job enforces `cargo clippy --all-targets --locked -- -D warnings`.
- **Reviews converge instead of spiraling:** Round 2+ plan reviews are delta-scoped (carried findings + diff since the prior round, with clean fallbacks), split plan reviews escalate to the human after 2 consecutive split rounds, code review `later_round` handles base merges/rebases cleanly, and code items at `changes_requested` in round 3+ escalate to the mission lead.
- **Branch operations are lean and clean:** A started item can be dropped in a single `backlog/*` branch (closing its open PR on merge and naming any newly filed items on its branch); `atelier merge` cleans up the merged remote and unattached local branch; and `atelier status` surfaces accumulated merged branches and detached worktrees.
- **The five CLI paper cuts are fixed:** `submit` names the item checkout path, `status` never misdirects delegated threads or base checkouts, `pre-push`'s recovery command works on a checked-out base, and malformed `### Responses` lines report cleanly without crashing `status`, `show`, or `review`.
- **Garden runs merge on approval:** Reviewed upkeep fixes and deferred-finding settlements merge immediately, while the ≤3 proposals are filed as a `needs-human,garden` item for asynchronous human review.
- **Desk reads exact history from `atelier-core`:** `crates/atelier-desk/src/history.rs` uses `atelier_core::log::history` (keeping existing `/api/history` and `/api/launch` compatibility for `desk/` and `t3-extension/`), and `GET /api/repos/{repo}/missions/{id}/history` is live.

## Success criteria

- **fast-record-ci:** In `.github/workflows/ci.yml`, the `changes` job has no Node/`pnpm` setup and does not run `t3-extension-release.test.sh` (which still runs when `t3-extension/` or `scripts/t3-extension-release*` changes). `scripts/ci-jobs.test.sh` passes with zero references to `.atelier/issues/`, `.atelier/reviews/`, or `.atelier/evidence/`, proving both `.md` and non-`.md` files under `.atelier/items/` and `.atelier/missions/` skip `rust` while `.atelier/config.toml` runs `rust`. CI's `test` job runs `cargo clippy --all-targets --locked -- -D warnings` and passes on `master`.
- **converging-reviews:** CLI tests prove: (a) round 2+ `atelier review <mission>` shows the delta diff from the prior reviewed commit and narrows the brief (falling back on missing/non-ancestor commits or base merges); (b) after 2 consecutive split plan-review rounds, `status` and `merge`'s refusal tell the planner to bring open findings to the human instead of starting round 3+; (c) at `changes_requested` in round 3+, `status` tells implementer and lead to settle via the lead; (d) `roles/planner.md`, `roles/lead.md`, and `roles/implementer.md` document these rules without modifying `roles/reviewer.md`.
- **clean-branches-and-drop:** CLI tests prove: (a) `atelier drop <started-item> --reason "<why>"` works directly on a `backlog/*` branch, merges cleanly, stops the item on base, warns about any newly added item records on the dropped branch, and closes the item's open PR on forge merge; (b) `atelier merge` deletes the merged remote branch (via `git push --delete`, without breaking linked worktrees) and deletes the local branch when not checked out; (c) `atelier status` reports accumulated merged branches / detached worktrees above threshold.
- **cli-paper-cuts-fixed:** CLI and unit tests prove all five fixes (`u49o`, `o2vv`, `yhaf`, `jfos`, `o91z`), and `evals/agent-factory/test.sh` passes against the updated binary.
- **garden-unblocked:** `skills/agent-factory/procedures/garden.md`, `docs/product.md`, and `docs/design.md` specify merging approved upkeep/settlements immediately and filing proposals as a `needs-human,garden` item; `atelier check` and `evals/agent-factory/test.sh` pass.
- **desk-core-history:** `crates/atelier-desk/src/history.rs` derives events from `atelier_core::log::history` with no commit-subject prefix parser left; `GET /api/history` and `GET /api/launch` pass all existing Desk Rust + app tests; `GET /api/repos/{repo}/missions/{id}/history` returns structured mission history with tests; `t3-extension/` has zero file changes.
- **all-checks-pass:** On `master` after the final item merges, `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo nextest run --locked`, `atelier check`, `evals/agent-factory/test.sh`, and `desk/` tests (`pnpm typecheck && pnpm test && pnpm build`) all pass.

## Scope

**In:**
- `.github/workflows/ci.yml`, `scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`
- `crates/atelier-core`, `crates/atelier-cli`, `crates/atelier-desk`
- `desk/src/api/` (generated TypeScript types only if needed by mission history) and Desk docs (`docs/architecture/desk.md`, `docs/product/desk.md`)
- `skills/agent-factory/SKILL.md`, `roles/planner.md`, `roles/lead.md`, `roles/implementer.md`, `procedures/garden.md`, `AGENTS.md`, `docs/product.md`, `docs/design.md`
- Dropping the 8 superseded loose items (`atelier-i70l`, `atelier-s8wm`, `atelier-u49o`, `atelier-o2vv`, `atelier-yhaf`, `atelier-jfos`, `atelier-o91z`, `atelier-y0ba`) on this mission's branch.

**Out:**
- **`t3-extension/`:** No changes to the T3 Code extension (explicitly deferred by Sumeet on 2026-11-22, including `atelier-yy4j`).
- **`atelier-dh7a`:** Removing `draft`/`ready` and letting `atelier new` file on the current branch stays in its already-agreed mission.
- **`atelier-kw8u` / `atelier-oiwy`:** Do not edit `skills/agent-factory/roles/reviewer.md` or run model eval matrices; keep `oiwy`'s reviewer experiment isolated.
- **Moving all CLI actions into `atelier-core` & plain-English/docs-site overhaul:** Deferred to Mission 2 after `atelier-dh7a` merges so CLI wording and docs are rewritten once against the `open`-only model.
- **Post-cutover 2-week scorecard (`atelier-pwhn`)** and **settling existing garden branches (`atelier-0019`, `atelier-qb4f`)**: handled outside this mission.

## Authority

- **The lead and implementers may decide:** exact helper module layout inside `atelier-core`/`atelier-cli`/`atelier-desk`, exact wording of new CLI messages (in plain, direct English), threshold values for stale-branch/worktree notices in `status`, caching/timeout details for Desk's `log::history` calls, and execution order across independent items.
- **The lead may shrink or drop** an individual sub-fix if it creates an unexpected conflict with `atelier-dh7a` or `evals/agent-factory/test.sh`, recording the reason and filing a follow-up.
- **Reserved for the human:** changing the round-cap thresholds (3 code rounds / 2 split plan rounds), touching `t3-extension/`, altering `atelier-dh7a` or `atelier-kw8u` plans, editing `roles/reviewer.md`, or breaking record format compatibility across repositories.

## Result
