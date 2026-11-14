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

- Every change, to code or to records, lives on a branch that `atelier
  start` makes, and reaches `master` only by `atelier merge`. There are
  three kinds: `<item-id>` for an item's code and records, `<mission-id>`
  for a new mission's plan, its items and its plan reviews, and
  `backlog/<slug>` for record changes tied to neither, such as filing,
  readying or dropping an item, leading or closing a mission, or a lead's
  notes.
- Only `start`, `submit` and `merge` act on Git. `new`, `set`, `ready`,
  `drop`, `close`, `lead`, `review` and `verdict` write files and commit
  nothing; commit and push them with Git.
- A mission starts from Sumeet's direction: `atelier start --mission
  "<title>"`, write the plan and its items, `atelier submit`, and two
  reviewers who did not plan it each run `atelier review <mission>` and push
  their verdict to the branch. It merges once both approve, with no go-ahead
  after.
- An item, in order: `atelier start <id>` (a branch named `<id>`),
  implement, tick every Done when line and write the `## Report`, `atelier
  submit`. A reviewer, in a thread that did not write the change, runs
  `atelier review <id>` and `atelier verdict approve` or `changes`. The
  implementer fixes and submits again, and once approved runs `atelier
  merge`, which marks the item done.
- A question the implementer cannot settle goes to the thread that started
  it, as advice. A finding it disputes is rebutted in the Report's
  Responses, and a fresh reviewer in the next round keeps or withdraws it; `atelier
  list --disputed` lists each one.
- A mission is read with `atelier show <mission>`, which lists its items,
  their status and what is disputed in them, and with the files in its
  directory. `atelier list` shows all open work.

## Merge Policy

- Nothing is committed on `master` directly. A branch merges on what it
  holds: one that changes code needs an approving independent review of its
  head; one that adds a mission or changes a plan needs two approving plan
  reviews; one that changes only records needs `atelier check` to pass on
  the merged tree. Only an item's branch may change code.
- `atelier submit` pushes the branch and opens its pull request, so CI runs
  during the review. `atelier merge` waits for CI on the branch's
  latest code for up to 9 minutes, runs the merge checks in `config.toml`
  on the merged tree, and has the forge merge the pull request. Exit status
  3 means CI has not finished: run it again. Do not open or merge pull
  requests or push `master` by hand around it.
- Git hooks guard the rules: pre-commit validates every record a commit
  changes, and pre-push refuses every push that moves `master`, since the
  forge makes its merges. `atelier setup` installs them in a
  new clone. A rule can be passed on purpose with `ATELIER_SKIP="<reason>"`
  on the command, which the commit records; use it only when you mean to.
- Branches are pushed over `origin`, whose URL here is HTTPS. A host without
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
