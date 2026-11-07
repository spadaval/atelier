# Repository Map

This file is a map to the repository's durable guidance. Keep process and
product policy in the linked sources instead of duplicating it here.

## Missions

Open missions are listed by `atelier work missions`, and `atelier work mission
<id>` shows each one's strategy; this file does not track them. The verified repo-native tracker rebuild (mission
`atelier-kgcr`, `docs/strategies/atelier-rebuild.md`) closed on 2026-10-15.
[ADR 0019](docs/adr/0019-verified-repo-native-tracker.md) remains the
governing decision, and current product and architecture docs describe the
rebuilt model.

## Process Sources

- Agent Factory skill: the installed copy (`~/.agents/skills/agent-factory`,
  loaded as `/agent-factory`). This repository does not vendor it; its one
  source is `spadaval/.agents` ([ADR 0023](docs/adr/0023-agent-factory-is-not-vendored.md)).
- Tracker state: `.atelier/` (records are committed with the code they
  describe)
- Start a session with `atelier status`; before handoff commit the records
  and run `atelier land`. Active work may remain open, with unfinished code
  handed off honestly.
- One issue, in order: `atelier start <id>` (an item with a recorded branch
  switches to it; otherwise on `master` it creates a branch named `<id>`, and
  on another branch it takes that branch), implement, tick
  every Done when line and write the `## Report`, `atelier review request
  <id>`, fix and re-review, approve, then `atelier merge <id>` under the merge
  policy below, which marks the work done. Work has no close step. Only a code
  change makes an approval stale; a Done when line rewritten after start is
  shown in the review brief and by `merge --check`, and refuses nothing. Work
  that changes no code and can only happen after a merge or release is a loose
  end, unless it is an independent validation; a change with no issue takes
  the light path. Agent Factory's Atelier reference and
  `docs/product/work-model.md` have the detail; `CONTEXT.md` defines check,
  review, land, done and close.
- `.claude/settings.json` runs these through the installed CLI's Claude Code
  hooks (`atelier setup claude`). To try a local build, use `target/debug/atelier`
  directly, or `setup claude --binary target/debug/atelier` without committing
  the result.

## Merge Policy

- Agents may merge independently reviewed work into `master` locally without
  asking. Unreviewed non-trivial code never reaches `master`. Tracker records
  and mission strategies may be committed there directly; non-trivial
  planning is reviewed before the work it defines starts.
- A branch that changes anything besides tracker records (`.atelier/issues`,
  `reviews`, `evidence`; the workflow policy and config count as code) merges
  only after CI has passed on its latest code: push the work branch, open a
  pull request (a draft until it is approved), wait for the CI workflow to
  pass, then run `atelier merge` and `atelier land`, and push `master`.
  Record-only commits after that run, such as approvals, need no new run; a
  commit that changes code does, including an `atelier merge --sync` that
  brings in code from `master` (a clean one still keeps the approval).
  Record-only branches skip the pull request. The
  `scripts/merge-ci-passed.sh` merge check enforces this, so `atelier merge`
  runs no test suite on this host.
- Agents may push work branches, and `master` once `atelier land` passes on
  it, without asking. Never force-push `master` or delete remote branches without
  the human.

## Product Sources

- Product intent: `PRODUCT_INTENT.md`
- Domain language: `CONTEXT.md`
- Documentation map: `docs/index.md`
- Architecture: `docs/architecture/index.md`
- ADRs: `docs/adr/`
- Code standards: `docs/architecture/quality/standards.md`

## Command References

- Installed CLI: `atelier` (`cargo install --path crates/atelier-cli --locked`
  in this checkout; elsewhere the release installer in `README.md`)
- Local debug binary: `target/debug/atelier`
- Formatting: `cargo fmt -- --check`
- Rust tests: `cargo nextest run`
- Failure-path tests: `cargo nextest run --features test-hooks`
- Whitespace: `git diff --check`
- Locally, run formatting and the tests that cover your change
  (`cargo nextest run -p <crate>` or a filter). The full suite, with and
  without `test-hooks`, on Linux and macOS, runs in CI on the pull request
  (`.github/workflows/ci.yml`), and that run gates the merge.
