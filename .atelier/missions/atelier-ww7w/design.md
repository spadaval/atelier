# Mission atelier-ww7w: Design

## Shape

Six independent (or lightly sequenced) items targeting the highest-leverage overhead and reliability gaps identified in the 2026-11-22 codebase assessment, while keeping `t3-extension/`, `atelier-dh7a`, and `atelier-kw8u`/`atelier-oiwy` untouched.

| Item | Area | Touches | Parallelizable? |
| --- | --- | --- | --- |
| `atelier-1n49` | Fast record-only CI, `.atelier/` filters, CI `clippy` | `.github/workflows/ci.yml`, `scripts/ci-jobs*.sh`, any clippy fixes | Yes |
| `atelier-h6ij` | Incremental round 2+ plan reviews, 2-split plan exit, 3-round code cap | `atelier-core`, `atelier-cli`, `roles/{planner,lead,implementer}.md`, docs | Shares `output.rs`/`say.rs` with `gqtw`/`zc9b` |
| `atelier-gqtw` | Single-branch drop for started items, merged-branch & worktree cleanup | `atelier-core`, `atelier-cli`, docs | Shares `output.rs`/`commands.rs` with `h6ij`/`zc9b` |
| `atelier-zc9b` | Five CLI paper cuts (`u49o`, `o2vv`, `yhaf`, `jfos`, `o91z`) | `atelier-core`, `atelier-cli`, `SKILL.md`, `AGENTS.md` | Shares `output.rs`/`say.rs` with `h6ij`/`gqtw` |
| `atelier-7637` | Decouple garden upkeep merges from human proposal sessions | `procedures/garden.md`, `docs/product.md`, `docs/design.md` | Yes |
| `atelier-js4a` | Back Desk history & mission history endpoint with `atelier_core::log::history` | `crates/atelier-desk`, `desk/src/api/` (if needed), Desk docs | Yes |

All six items have zero hard `blocked_by` edges. To avoid unnecessary rebase churn in `output.rs` and `say.rs`, the lead may run `atelier-1n49`, `atelier-7637`, and `atelier-js4a` in parallel with one CLI item at a time (`atelier-h6ij` → `atelier-gqtw` → `atelier-zc9b`, or whichever order it prefers).

## Design

### 1. Fast record-only CI (`atelier-1n49`)
- Move `scripts/t3-extension-release.test.sh` out of the `changes` job and into the `extension` job (adding `scripts/t3-extension-release*` to `needs_extension` in `scripts/ci-jobs.sh`). Remove `pnpm/action-setup` and `actions/setup-node` from `changes`.
- In `scripts/ci-jobs.sh::needs_rust`, replace `.atelier/issues/* | .atelier/reviews/* | .atelier/evidence/* | .atelier/runtime/*` with `.atelier/items/* | .atelier/missions/* | .atelier/runtime/*`. Keep `.atelier/config.toml` triggering `rust`.
- Add `cargo clippy --all-targets --locked -- -D warnings` to CI's `test` job. If existing code on `master` triggers warnings, fix them in the same item.

### 2. Incremental plan reviews & round caps (`atelier-h6ij`)
- **Plan review delta (`review.rs::plan_review` + `output.rs::plan_brief`):** Compute the prior completed plan-review round (highest `round < current` where every review file has a verdict). When all reviews of that round share one commit `prev` that exists, is an ancestor of `HEAD`, and has no merge commit from `base` in `prev..HEAD`, print `git diff <prev> <head> -- .atelier/missions/<id> .atelier/items` first and narrow the brief to checking carried findings + changes since the previous round (blocking on unchanged plan sections only if a serious defect was missed). Otherwise fall back to the full plan brief.
- **Code review delta guard (`review.rs::brief`):** Guard `previous` the same way: omit `git diff <prev> <commit>` if `prev` is missing or not an ancestor of `commit`; if `prev..commit` includes a merge from `base`, scope the delta diff to paths changed on the branch (`base...commit`) or fall back cleanly so unrelated `master` commits do not flood the diff.
- **2-split-round plan exit (`atelier-core::plan`):** A completed plan round is *split* when it has at least one `approve` and at least one `changes` verdict. Count trailing consecutive split rounds; when `>= 2`, `status` (on the mission branch) and `output::plan_unapproved` advise the planner to stop opening more rounds and take the open findings to the human to rule.
- **3-round code cap (`output.rs` / `say.rs`):** When an item is `changes_requested` and its latest completed round is `>= 3`, `status` (for implementer and mission lead) advises escalating to the mission lead (or starting thread) to settle rather than starting round 4+.

### 3. Single-branch drop & branch cleanup (`atelier-gqtw`)
- **Single-branch drop:** Allow `atelier drop <id> --reason "<why>"` on a `backlog/*` branch even when `<id>` is started. Write `status: dropped` + `Dropped: <reason>` to `.atelier/items/<id>/item.md` (using `carry_drop`'s `Goal`/`Done when`/`Scope` copy when `<id>` only exists on its branch via `start --new`). Inspect paths added on `<id>`'s branch relative to `base` for any other `.atelier/items/<other>/item.md` not on `base`, and print them so `atelier-dh7a`/`atelier-u6oy` items are never silently lost. Keep branch-side `drop` on `<id>`'s branch working too.
- **PR close on merge:** When `atelier merge` merges a backlog branch that transitions a started item (whose branch exists) to `dropped` and a forge is configured, best-effort close any open PR for that item's branch (`Github::close_pull_request`), warning on failure without failing the merge.
- **Merged-branch cleanup on `atelier merge`:** After a forge merge succeeds, best-effort delete the remote head branch via `git push <remote> --delete <branch>` (explicitly *not* `gh pr merge --delete-branch`, which tries to switch the current linked worktree to `base` and fails when `base` is checked out elsewhere). Locally, delete the merged branch (`git branch -d <branch>`) only if `git::checkout_of` returns `None`; otherwise print a one-line hint to remove the worktree and branch.
- **Stale hygiene notice:** In `atelier status` away from an item branch, count merged local named branches and detached worktrees; when either meets its threshold (e.g., ≥ 10 merged local branches or ≥ 5 detached worktrees), print a one-line summary and safe cleanup command.

### 4. Five CLI paper cuts (`atelier-zc9b`)
- `u49o`: `submit` passes the item checkout's root path (`repo.root`) into `start_reviewer` so the task reads `In <path>, run atelier review <id> and follow what it prints.` (using shell-safe quoting if needed, no backticks).
- `o2vv`: Remove the automatic `Then: atelier start <first-ready>` line in `output::overview` when the caller holds nothing; list startable items and let the thread's task choose. Add a note in `SKILL.md` and `AGENTS.md` about running `atelier status` in the task's checkout when started from another repo.
- `yhaf`: In `output::overview`, when on `base` and the caller holds an item, look up `git::checkout_of` for that item's branch and print the existing checkout path or `git worktree add`, never `Then: atelier start <id> (switches to its branch)`.
- `jfos`: Update `output::push_refused` and `output::push_moves_base` so the recovery command works when `base` is checked out in the current worktree (e.g., `git switch --detach && git branch -f <base> <old> && git switch <base>`).
- `o91z`: In `crates/atelier-core/src/records/body.rs`, parse `### Responses` line-by-line into `(Vec<Response>, Vec<Problem>)` (or equivalent) so a malformed line does not abort record loading. Surface malformed lines in `validate`, `atelier check`, `atelier status`, and `atelier submit`, while `status`, `show`, and `review` remain usable.

### 5. Garden decoupling (`atelier-7637`)
- Update `skills/agent-factory/procedures/garden.md` so approved garden runs merge their upkeep fixes and deferred-finding settlements immediately instead of waiting for a synchronous human session. Proposals (≤ 3) are filed as a single `needs-human,garden` item referencing the merged run's Report. Update `docs/product.md` and `docs/design.md` to match.

### 6. Desk history from `atelier_core::log::history` (`atelier-js4a`)
- Replace `crates/atelier-desk/src/history.rs`'s `git log` subject parser with calls to `atelier_core::log::history` via `Repo::open_committed`, translating `Logged` entries into Desk's `HistoryEvent` / `EventKind` (preserving existing fields and ordering for `/api/history` and `/api/launch` so `desk/` and `t3-extension/` work unchanged).
- Implement `GET /api/repos/{repo}/missions/{id}/history` in `crates/atelier-desk/src/server.rs` using `atelier_core::log::history` filtered to the mission and its items, returning `HistoryResponse` (or a dedicated response struct if cleaner, with `ts-rs` types regenerated in `desk/src/api/`).
- Preserve caching (keyed on ref tips or short TTL) and bounded execution so Desk responses remain fast.

## Sizing

Six focused items, each small-to-medium (1–2 review rounds expected). No item changes `t3-extension/`, no item alters the record schema version, and no item requires running live LLM eval matrices.

## Derisking

- **No spike needed:** Every item modifies well-understood existing modules in `crates/`, `.github/`, `scripts/`, and `skills/`.
- **Riskiest assumption:** Whether `atelier_core::log::history` is fast enough on repositories with hundreds of branches/commits when called from `atelier-desk`'s HTTP handlers (including `/api/launch` and `/api/history`). Settled by benchmarking/testing `atelier_core::log::history` on this repository inside `atelier-js4a` and keeping Desk's ref-keyed cache around it.
- **Second risk:** Existing eval fixtures (`evals/agent-factory/test.sh`) assert exact CLI behavior and strings on `status`, `submit`, and `pre-push`. Every CLI item must run `evals/agent-factory/test.sh target/debug/atelier` locally before submitting and keep all fixture tests green.

## Risks and unknowns

- **`output.rs` / `say.rs` merge churn:** Items `atelier-h6ij`, `atelier-gqtw`, and `atelier-zc9b` all touch CLI output wording. Running them sequentially (or merging cleanly) avoids repeated rebases.
- **Hidden `clippy` warnings on `master`:** Adding `clippy` to CI in `atelier-1n49` may surface existing warnings across `atelier-core`, `atelier-cli`, or `atelier-desk`; `atelier-1n49` budgets for fixing those cleanly.
- **Interaction with `atelier-dh7a`:** `atelier-dh7a` is agreed on `master` and removes `draft`/`ready`. None of these items introduce new `draft`/`ready` dependencies, and `atelier-gqtw`'s single-branch drop explicitly checks for newly added item records on the dropped branch so `atelier-u6oy` integrates smoothly.
