---
schema: atelier.review
schema_version: 3
round: 2
commit: 6739d6783a179ee339906cd23492844c0cbf7b7a
reviewers:
- thread:delegated-task:command%3Amcp%3A5c01d7ea-dd61-466c-857a-4beef42607cc%3Adelegate-task%3A326bef4a-0fa1-42e2-b630-93bb263f226f
model: gemini-4-neon
verdict: approve
created_at: 2026-11-22T10:01:24.761986317Z
---

## Summary

Checked:
- Inspected all commits since round 1 (`git diff 919e3952bdaef9566e796afa15a1a548cac0b2dd 6739d6783a179ee339906cd23492844c0cbf7b7a`: `d511edd8f`, `ce50941a9`, and `6739d6783`), and verified the surrounding code in `crates/atelier-desk/src/server.rs`, `crates/atelier-desk/src/server/views.rs`, `crates/atelier-desk/tests/{actions.rs,sessions.rs}`, `crates/atelier-cli/src/{plan.rs,cleanup.rs,commands.rs,merge.rs,git.rs,output.rs}`, `crates/atelier-cli/tests/cleanup.rs`, `crates/atelier-core/src/{forge.rs,plan.rs,repo.rs,say.rs}`, evals, and updated docs (`AGENTS.md`, `skills/agent-factory/roles/lead.md`, `docs/product.md`, `docs/design.md`, `docs/architecture/desk.md`) against every Done when line and Report claim.
- Confirmed round 1's blocking finding (`1-ca9z.1`) is fixed in `d511edd8f`: `Desk::cache` now includes `TypeId::of::<T>()` in its key (`crates/atelier-desk/src/server.rs:60,226`) and both `GET /api/repos/{repo}/missions/{id}` and `POST /api/sessions` share `Desk::mission_read` (`server.rs:292-297,582,897`), covered by unit test `a_route_kept_as_one_type_is_not_missing_to_a_reader_of_another` (`server.rs:918-941`) and integration test `a_missions_page_and_its_session_are_each_found_whichever_is_read_first` (`tests/sessions.rs:270-284`).
- Confirmed round 1's deferred finding (`1-ca9z.2`) was also fixed in `ce50941a9`: `added_there` (`crates/atelier-cli/src/plan.rs:381-423`) now inserts `<remote>/<branch>` ahead of local `<branch>` when the local tip is a strict ancestor of the fetched remote tip, matching `Repo::tip` while still reporting items only on the other side when diverged, covered by `a_drop_names_a_filed_item_as_the_newer_of_the_branchs_two_copies_has_it` (`tests/cleanup.rs:266-294`).
- Ran `cargo fmt -- --check`, `git diff --check`, and `cargo clippy --all-targets --locked -- -D warnings`: clean.
- Ran `cargo nextest run --locked` (whole workspace) and verified the 3 new regression tests pass.
- Built `target/debug/atelier` and ran `evals/agent-factory/test.sh target/debug/atelier`: all eval fixtures and harness tests pass.
- Ran `desk/` tests (`./node_modules/.bin/vitest run`, as the `pnpm` corepack shim failed to locate its cached binary): 10 files, 57 tests passed.
- Verified PR #278 (`6739d6783a179ee339906cd23492844c0cbf7b7a`) CI checks: `changes`, `plan`, `test`, `evals`, and `extension` all passed (10 passed, 0 pending, 0 failed).

Could not check:
- Live GitHub lease-checked remote branch deletion and PR close (only exercised via local bare remote and fake `gh`).
- `node t3-extension/run-tests.mjs` locally (covered by CI's passing `extension` check on `6739d6783a179ee339906cd23492844c0cbf7b7a`).

## Finding 1: Desk's `"mission {id}"` cache entry collides in type between `mission` and `session`, causing 404s
Recommendation: fix now
Was: 1-ca9z.1
Response: fixed in d511edd8f
Now: fixed
`d511edd8f` keys `Desk::cache` by `(repo, route, TypeId::of::<T>())` (`crates/atelier-desk/src/server.rs:60,226`) so different types on the same route string can no longer collide, shares `Desk::mission_read` across both `GET /api/repos/{repo}/missions/{id}` and `POST /api/sessions` (`server.rs:292-297,582,897`), and adds regression tests in `crates/atelier-desk/src/server.rs:918-941` and `crates/atelier-desk/tests/sessions.rs:270-284` covering both request orders with unchanged refs.
