---
schema: atelier.review
schema_version: 3
round: 1
commit: d13bd5c712aec547f0dcda999c892140b0b984a7
reviewers:
- thread:delegated-task:command%3Amcp%3A134a3ba0-c5f0-4828-a968-a4cfb219e451%3Adelegate-task%3A9c5a30af-d537-4699-92d2-a90ab6cc1b34
model: gemini-4-neon
verdict: approve
created_at: 2026-11-22T18:17:31.074567138Z
---

## Summary

- **Complexity:** medium — spans `atelier-core`, `atelier-cli`, `atelier-desk`, `desk/`, `t3-extension/`, `evals/`, `skills/`, and docs, splitting mission transitions (`actions::ON_MISSION`) from item statuses while shrinking non-test Rust by 76 lines.
- **Value:** high — removes the ceremonial `draft`/`ready` gate across the workflow and documents while keeping existing repositories' records and history readable without migrating old files.
- **Risk of merging it:** medium — reaches every repository the host's installed binary serves and requires reinstalling `atelier`, `atelier-desk`, and the T3 extension together (plus updating `astromech`'s pinned CI rev) before writing `open` records, as noted in the Report and `docs/design.md`; failures are visible and reversible.

Value outweighs the cost and risk; a smaller change would leave the status model split across the CLI, Desk, extension, and docs. No blocking findings.

**What I checked and how:**
- Read the full diff (`d13bd5c712aec547f0dcda999c892140b0b984a7` against `master`, 122 files) and verified every Done-when line and Scope boundary in `atelier-v06d/item.md` against `atelier-dh7a/mission.md` and `atelier-rme3/item.md`.
- Verified Rust formatting, whitespace, lints, tests, builds, and line counts: `cargo fmt -- --check`, `git diff --check master...HEAD`, `cargo clippy -j 4 --all-targets --locked -- -D warnings`, `cargo nextest run -j 4 --locked`, `cargo build -j 4 --locked -p atelier-cli -p atelier-desk`, `git diff --shortstat master...HEAD -- crates/` (48 files, +1,191/-1,097; `src/` +371/-447, `tests/` +820/-650), and `python3 scripts/count_non_test_rust.py` (`master` 24,409 -> `HEAD` 24,333, -76 lines).
- Ran `target/debug/atelier` read-only (`GIT_OPTIONAL_LOCKS=0`) across all six repositories (`atelier`, `astromech`, `life-on-rails`, `cosplay-studio`, `demiurge`, and `worldcrafter` symlinked to `demiurge`): `check`, `list`, `list --startable`, and `log` exited 0 with `check` and `log` identical to the installed `atelier` (1,179 log lines on this branch, +1 over the Report's `fc6c137a8` count from the submit commit); `status` byte counts matched the Report (`astromech` 9,713 -> 6,817, `life-on-rails` 3,798 -> 3,516, `cosplay-studio` 850 -> 1,549, `demiurge`/`worldcrafter` 687 -> 1,573); all eight legacy `draft` missions (`lor-l9a7`, `cosplay-studio-ro6g`, `atelier-b7f5`, `atelier-w19u`, `atelier-c77k`, `atelier-0tvt`, `atelier-wih3`, `worldcrafter-3otd`) read as `proposed`; `show astromech-05k2` prints `Status: open, can start now` and `Made ready by conversion at ...`.
- Exercised `target/debug/atelier` end-to-end in an isolated sandbox repo: confirmed `atelier --help` and `atelier help` omit `ready`; `atelier ready`, `atelier ready <id>`, `atelier ready --help`, and `atelier ready -h` all exit 1 with "Items need no go-ahead"; `atelier new` writes `status: open`, `start` refuses unmerged open items and accepts merged ones directly, and `log` writes no `ready` line; legacy `draft` and `ready` items read as `open` (`show` prints `Made ready by ...` only when `ready` is populated and never prints `Not made ready`), read commands and no-op `set` leave legacy files byte-identical, and the pre-commit hook allows `draft -> open` on a real `set` change while refusing `draft -> done` on a legacy `draft` mission; `lead` and item `start` are refused on a legacy `draft` mission.
- Ran `bash evals/agent-factory/test.sh`, `scripts/cutover` tests (16/16), `scripts/scorecard` tests (22/22), `bash -n scripts/cutover/{rehearse,first-use-forge}.sh`, `desk/` (`pnpm typecheck`, `pnpm test` 57/57, `pnpm build`, `pnpm e2e` 134/134), and `t3-extension/` (`pnpm typecheck`, `pnpm build`, all 7 tests in `run-tests.mjs` against the built `atelier-desk`, and `bash scripts/t3-extension-release.test.sh`), and verified PR #281 CI checks are green (`test`, `evals`, `desk`, `extension`, `changes`, `plan`, `bundle`).

**What I could not check:**
- Live model runs of `evals/agent-factory` cases.
- Reinstalling `atelier`, `atelier-desk`, and the rebuilt T3 extension onto the live host or running `scripts/cutover/rehearse.sh` end-to-end.
- Running `desk-matches-cli` against clones of the other repositories (verified `atelier-desk` via `crates/atelier-desk` tests and `t3-extension/test-extension.mjs` instead).

## Finding 1: `atelier help ready` prints a bare usage line instead of refusing

Recommendation: no action

Location: `crates/atelier-cli/src/main.rs:158-162`, `crates/atelier-cli/src/main.rs:277`, `crates/atelier-cli/src/main.rs:348`

`Ready` is hidden with `#[command(hide = true, disable_help_flag = true)]`, so `atelier --help`, `atelier help`, and every visible subcommand omit `ready`, and `atelier ready`, `atelier ready <id>`, `atelier ready --help`, and `atelier ready -h` all exit 1 with "Items need no go-ahead". Two minor edges remain: `atelier help ready` still resolves through clap's built-in `help` subcommand and prints `Usage: atelier ready` with exit 0, and `Command::Ready` runs after `open(&here, &skip)` so running `atelier ready` outside a Git checkout exits 2 ("not a Git checkout") rather than refusing. Neither misleads an agent in normal use and disabling clap's `help` subcommand is not worth extra parsing code.

## Finding 2: Stale function reference in `docs/design.md` rules list

Recommendation: no action

Location: `docs/design.md:1645-1647`

The Rules bullet reads "a status changed as no action on that kind of record (an item or a mission, `actions::changes`) changes it", whereas `actions::changes` now covers only items and `actions::changes_mission` covers missions (as `docs/design.md:503` accurately records). Purely cosmetic documentation reference.
