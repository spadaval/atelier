---
schema: atelier.review
schema_version: 3
round: 1
commit: 1ac55a66658a3f3ef161b6dec912f7450a2fe0a8
reviewers:
- thread:delegated-task:command%3Amcp%3A134a3ba0-c5f0-4828-a968-a4cfb219e451%3Adelegate-task%3A38f93980-536b-4051-b51f-5d3804096b08
model: gemini-4-neon
verdict: approve
created_at: 2026-11-22T18:47:12.552644457Z
---

## Summary

- **Complexity:** low — targeted UI, model, test, and doc updates across `desk/` and `t3-extension/` with no new routes, controls, or server changes (+360/-96 across 34 files, including the Report and four screenshots).
- **Value:** high — finishes removing `draft`/`ready` item wording across the Desk and T3 extension, sorts the single loose-open list by priority then ID so urgent items are not hidden behind "Show more", explains every unstarted item on mission pages, and keys proposed mission glyphs on `MissionState` rather than `draft`.
- **Risk of merging it:** low — purely client-side presentation and documentation changes, covered by unit, Playwright e2e, and extension browser tests; any regression is immediately visible and cheap to revert.

Value clearly outweighs the cost and risk; a smaller change would leave the remaining `atelier-v06d` gaps open. No blocking findings.

**What I checked and how:**
- Read the full diff (`master...1ac55a66658a3f3ef161b6dec912f7450a2fe0a8`, 34 files) and inspected every modified and related file in `desk/src/`, `t3-extension/src/`, `crates/atelier-desk/src/read.rs`, `crates/atelier-core/src/{view,work,actions,say,model/config}.rs`, `docs/product/desk.md`, `docs/architecture/desk.md`, and `t3-extension/README.md`, checking each Done-when item, Scope boundary, and Report claim.
- Grepped `desk/src/` and `t3-extension/src/` for `draft`, `ready`, and `readied` to verify no page shows `draft` or `ready` as an item's status and that `desk/src/api/Status.ts` uses `open` in place of both.
- Inspected all four screenshots in `.atelier/items/atelier-rme3/artifacts/` (`desk-project.png`, `desk-mission.png`, `extension-project.png`, `extension-mission.png`) and the screenshots produced by the extension tests (`home-full-light.png`, `mission-running-light.png`, `thread-nothing-light.png`). The Desk screenshots show `Built from 1d731b5f9`, which is the last commit touching `desk/` or `t3-extension/src/` (later commits `229c64438` and `1ac55a666` change only extension tests, the item record, and screenshots).
- Ran in `desk/`: `pnpm typecheck`, `pnpm test` (10 files, 61 passed), `pnpm build`, and `pnpm e2e` (138 passed).
- Ran in `t3-extension/`: `pnpm typecheck`, `pnpm build`, `BROWSER=/usr/bin/google-chrome node run-tests.mjs` against `target/debug/atelier-desk` built in this checkout (7 of 7 test suites passed), and `bash scripts/t3-extension-release.test.sh` (14 of 14 passed).
- Ran `cargo fmt -- --check`, `git diff --check`, and `atelier check` (all exit 0), and checked PR #283 CI (`changes`, `plan`, `bundle`, `desk`, and `extension` all green; Rust/evals/host skipped per `scripts/ci-jobs.sh`).

**What I could not check:**
- Running the rebuilt extension inside a live T3 Code instance (`t3-extension/test-contract.mjs`).
- Re-running `desk-matches-cli` or replaying against the running Desk service's five repositories (verified the server-to-client contract directly against `crates/atelier-desk/src/read.rs` and `crates/atelier-core/src/{view,work,actions,say}.rs`, and via `test-extension.mjs` against the checkout's `atelier-desk`).

## Finding 1: Branch-name suffix heuristic misclassifies an adding branch ending with the item ID, and `startNote` has no empty-condition fallback

Recommendation: defer

Location: `desk/src/lib/mission.ts:20`, `t3-extension/src/mission/model.ts:215-217`

Both `cannotStartUntil` (`desk/src/lib/mission.ts:20`) and `startNote` (`t3-extension/src/mission/model.ts:215`) distinguish an unmerged branch that adds an item from the item's own branch with `!on.name.endsWith(row.id)`. If an open item is added on an unmerged backlog branch whose slug ends with the item's ID (for example `backlog/<id>` or `origin/backlog/fix-<id>`, valid under `Config::named` in `crates/atelier-core/src/model/config.rs:84-88`), `endsWith(row.id)` is true, so `added` is empty while `row.cannot_start` is also empty. In that case `cannotStartUntil` returns `[]` (no reason shown on the Desk's mission page or Mission Control) and `startNote` falls through to `` `cannot start until ${row.cannot_start.join("; ")}` ``, rendering `"cannot start until "` with a trailing blank. Because `atelier new` allocates random IDs after `start --backlog <slug>`, this is rare in practice; a follow-up can either expose `proposed` on `ItemRow` in `atelier-desk` or check for own-branch naming without matching `backlog/` prefixes and fall back to `"waits"` in `startNote` when `row.cannot_start` is empty.

## Finding 2: Open-item sort comparator returns `1` for equal IDs

Recommendation: no action

Location: `desk/src/lib/group.ts:23`, `t3-extension/src/home/model.ts:36`

In `openByPriority`, `(a.id < b.id ? -1 : 1)` returns `1` instead of `0` when `a.id === b.id`. Item IDs are unique within a repository so real `GET /api/roadmap` responses are unaffected; noted only as a minor comparator detail.
