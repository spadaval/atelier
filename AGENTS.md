# Repository Map

This file is a map to the repository's durable guidance. Keep process and
product policy in the linked sources instead of duplicating it here.

## Starting

Run `atelier status` first; Claude Code and Codex run it at session start
through the hooks in `.claude/settings.json` and `.codex/hooks.json`. It
says who you are in this repository, what you hold, and the next command.
Agent Factory (`skills/agent-factory`, loaded as `/agent-factory`) says how
to decide; `atelier <command> --help` says what each command does.

## Work

Records live in `.atelier/`: a mission in `missions/<id>/mission.md`, an
item in `items/<id>/item.md` with its reviews beside it, and the rules in
`config.toml`. They are committed with the code they describe.

- Planning happens on `master`: `atelier new`, `atelier set`, then `atelier
  ready <id> --by <who said go>`. `ready`, `drop`, `close` and `lead` make
  their own commit; push `master` with `git push` when you are done.
- An item, in order: `atelier start <id>` (a branch named `<id>`),
  implement, tick every Done when line and write the `## Report`, `atelier
  submit`. A reviewer, in a thread that did not write the change, runs
  `atelier review <id>` and `atelier verdict approve` or `changes`. The
  implementer fixes and submits again, and once approved runs `atelier
  merge`, which marks the item done.
- A question the implementer cannot settle goes to `atelier escalate`, and
  whoever `status` names answers it with `atelier decide`.
- A mission is read with `atelier show <mission>`, which lists its items,
  their status and their open questions, and with the files in its
  directory. `atelier list` shows all open work.

## Merge Policy

- Unreviewed non-trivial code never reaches `master`. Planning records may
  be committed there directly.
- `atelier submit` pushes the item's branch and opens its pull request, so
  CI runs during the review. `atelier merge` waits for CI on the branch's
  latest code for up to 9 minutes, runs the merge checks in `config.toml`
  on the merged tree, merges and pushes `master`. Exit status 3 means CI has
  not finished: run it again. Do not push branches, open pull requests or
  push `master` by hand around it.
- Git hooks guard the rules: pre-commit validates every record a commit
  changes, and pre-push refuses a push to `master` that brings code or sets
  an item done outside an Atelier merge. `atelier setup` installs them in a
  new clone. A rule can be passed on purpose with `ATELIER_SKIP="<reason>"`
  on the command, which the commit records; use it only when you mean to.
- `merge` pushes over `origin`, whose URL here is HTTPS. A host without
  HTTPS credentials for GitHub needs one command in a fresh checkout:
  `git config remote.origin.pushurl git@github.com:spadaval/atelier.git`,
  or `gh auth setup-git`. `gh` must be signed in either way.
- Never force-push `master` or delete remote branches without the human.

## Sources

- What it is for and what it does: `docs/product.md`. How it is built, what
  was tried and removed, and the decisions that govern it: `docs/design.md`,
  the one design record.
- The Desk, the human's web tool: `docs/product/desk.md` and
  `docs/architecture/desk.md`.
- Agent Factory's evals: `evals/agent-factory`.

## Command References

- Installed CLI: `atelier`, installed from this tree with `cargo install
  --path crates/atelier-cli --locked`.
- The Desk's app: `desk/` (`pnpm test`, `pnpm e2e`; see `desk/README.md`).
- Formatting: `cargo fmt -- --check`
- Lints: `cargo clippy --all-targets -- -D warnings`
- Rust tests: `cargo nextest run`
- Whitespace: `git diff --check`
- Locally, run formatting and the tests that cover your change
  (`cargo nextest run -p <crate>` or a filter). The full suite runs on Linux
  in CI on the pull request (`.github/workflows/ci.yml`), and that run gates
  the merge. CI skips the Rust jobs when a change touches only tracker
  records, Markdown outside `crates/`, and the Desk's app outside
  `desk/src/api/` (`scripts/ci-jobs.sh` holds the rule).
- The cutover kit, `scripts/cutover/`, stays until the work host has
  adopted the new tool (`scripts/cutover/README.md`).
