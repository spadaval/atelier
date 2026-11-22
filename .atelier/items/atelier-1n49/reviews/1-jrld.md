---
schema: atelier.review
schema_version: 3
round: 1
commit: b5f6fba9e62df4242291359453f19e82764bdbc7
reviewers:
- 01a20211-e629-7840-b6ba-4e636ca81766
model: grok-4.6
verdict: approve
created_at: 2026-11-22T07:04:32.099184395Z
---

## Summary

Complexity: low. The change updates the existing CI workflow, job-filter script and tests, plus two docs; it adds no job or mechanism.
Value: high. Record-only PRs currently install Node and run the extension release test, non-Markdown artifacts under the new record paths start the Rust job, and AGENTS.md already requires clippy.
Risk of merging: low. A wrong skip or a clippy miss shows on the pull request, and the change reverts in one merge.

The value is worth it. No smaller change gets the three outcomes.

I read `git diff master...b5f6fba9e` for `.github/workflows/ci.yml`, `scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`, `scripts/t3-extension-release.test.sh`, `AGENTS.md` and `docs/design.md`, and checked each Done when line. I ran `scripts/ci-jobs.test.sh` (pass) and `scripts/t3-extension-release.test.sh` with pnpm 12.18.0 on PATH (pass). I ran `CARGO_BUILD_WARNINGS=warn cargo clippy -j 4 --all-targets --locked -- -D warnings` (pass), `cargo fmt -- --check` and `git diff --check master...HEAD` (pass). In a throwaway crate, `v.len() == 0` under those same clippy flags exits 101 with `clippy::len_zero`. Master's `ci-jobs.sh` fails the four new artifact and release-script cases; this branch selects rust, extension and evals for this PR and skips desk. Walking master's first-parent history, the old and new extension rules both pick `6850e181e`. GitHub Actions run 44996155457 (record-only `backlog/lead-ww7w`) spent 8s on pnpm setup, 6s on Node and 4s on the release test: 18s of a 26s `changes` job. PR 270 run 44996533328 at `b5f6fba9e` ran `changes` without Node or the release test (13s), ran `cargo clippy --all-targets --locked -- -D warnings` in `test` (pass, 19s), and ran `scripts/t3-extension-release.test.sh` in `extension` (pass); desk skipped; nextest 403 passed.

I did not rerun the Rust suite locally. I did not rebuild from a cold cache.

Each Done when line holds. One documentation gap remains, already named for the lead.

## Finding 1: Extension README still omits the release-script trigger and test
Recommendation: defer

`t3-extension/README.md` "In CI" (lines 1008-1019) lists the extension job's paths as `t3-extension/`, `desk/`, `crates/atelier-desk/`, the workspace Cargo files, `rust-toolchain.toml`, `.github/workflows/ci.yml` and `scripts/ci-jobs.sh`, and its steps as package install, typecheck, build, then `run-tests.mjs` after building `atelier-desk` and the Desk app. This change adds `scripts/t3-extension-release*` to `needs_extension` in `scripts/ci-jobs.sh:46` and runs `scripts/t3-extension-release.test.sh` as a step of the `extension` job in `.github/workflows/ci.yml:163`. The README names neither. The item's scope leaves `t3-extension/` out, so this is later work.
