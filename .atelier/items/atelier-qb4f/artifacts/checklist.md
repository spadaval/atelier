# Readiness checklist, atelier, 2026-11-13

Checked on master at 609abbdd6, in a scratch clone, before this run's fixes.

1. Met. `AGENTS.md` exists; `CLAUDE.md` is `@AGENTS.md`; `.claude/settings.json` and `.codex/hooks.json` run `atelier status` at SessionStart.
2. Met. Last 30 days landed in `crates` (2,448 paths), `docs`, `next` (deleted), `skills`, `evals`, `desk`, `scripts`; `AGENTS.md` names cargo's commands and `desk/README.md` names pnpm's.
3. Met. The script prints nothing.
4. Not met, for the Desk's server: `desk/README.md:33` says `cd ../next && cargo build -p atelier-desk`, and `next/` no longer exists. atelier-0019's branch repairs it, unmerged. The root `cargo build --locked -j 4 --all-targets` exits 0 in 22s; in `desk/`, `pnpm install --frozen-lockfile` and `pnpm build` exit 0.
5. Not met, for the Desk's app: `desk/` has `pnpm typecheck` (passes) but no format or lint command. Rust: `cargo fmt -- --check` and `cargo clippy --all-targets -- -D warnings` pass.
6. Met. `cargo nextest run --locked`: 309 of 309 passed in 15s. `pnpm test`: 43 of 43 passed.
7. Not met. Neither `.github/workflows/ci.yml` nor `.atelier/config.toml` runs clippy. The other checks run in CI. No run in the window changed Rust, so each skipped the test job; the last master run that ran it, on 2026-11-11 at 17:55Z, passed in 51 seconds.
8. Not met. No skill under `.claude/skills/` or `.agents/skills/`.
9. Not met. atelier-k55o and atelier-o91z are labelled `bug` with no `caused_by`.

Total: 4 met, 5 not met (4, 5, 7, 8, 9), 0 could not test.
