# Readiness Checklist

Checked by `skills/agent-factory/procedures/readiness.md` on 2026-11-11, in a scratch clone of `master` at 87ee3ccd0, with `TMPDIR` under the home directory outside any repository, one command at a time, `-j 4` for cargo, and the CLI installed with `--root` into the scratch directory.

| # | Line | Result | Evidence |
| --- | --- | --- | --- |
| 1 | Each harness loads `AGENTS.md` and starts with `atelier status` | met | `CLAUDE.md` is `@AGENTS.md`; `.claude/settings.json` and `.codex/hooks.json` run `atelier status` at SessionStart |
| 2 | `AGENTS.md` names the commands for the code agents change now | met | The last 30 days landed in `crates/`, `docs/`, `evals/`, `skills/`, `desk/`, `scripts/`; `AGENTS.md` names the Rust install, format, lint and test commands and the Desk's `pnpm test` and `pnpm e2e`, and points to `desk/README.md` and `evals/agent-factory` for the rest |
| 3 | What `AGENTS.md` points to exists | met | The script printed nothing |
| 4 | A fresh clone builds with the documented commands alone | met | `cargo install --path crates/atelier-cli --locked --root <scratch>` built in 16 s; `pnpm install --frozen-lockfile` and `pnpm build` in `desk/` passed |
| 5 | Format, lint and type check run from documented commands and pass | not met | Rust: `cargo fmt -- --check` and `cargo clippy --all-targets -- -D warnings` pass. TypeScript (`desk/`, 161 files changed in 30 days): `pnpm typecheck` passes, but no format or lint command exists |
| 6 | Tests run from one documented command and pass | met | `cargo nextest run`: 309 of 309 passed in 33 s; `pnpm test`: 43 of 43; `pnpm e2e`: 52 of 52 |
| 7 | Every merge runs the checks of lines 5 and 6 | not met | `config.toml`'s merge checks are `git diff --check`, `cargo fmt -- --check` and `cargo metadata --locked`; `ci.yml` runs fmt, build, `atelier check` and nextest, and the desk job typecheck, Vitest, build and Playwright, but neither runs `cargo clippy`. The last base run with tests, 43072803941, passed in 64 s |
| 8 | A verify skill says how to launch the product and prove a feature | not met | No `.claude/skills/` or `.agents/skills/` in the repository |
| 9 | Every bug names its cause | not met | The command printed 39 items: every `bug` item lacks `caused_by` (38 draft, 1 ready), all from the cutover's conversion |

No `.atelier/readiness.md`, so no repository lines.

Found while checking: `desk/README.md` and `docs/architecture/desk.md` said to `cd next` before building atelier-desk; `next/` is gone. Fixed on this branch in 94bbfa149.
