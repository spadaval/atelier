---
schema: atelier.item
schema_version: 3
id: atelier-kb2g
title: Stop naming one person in the code, the skill and the docs
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- 2ebdc870-0f5d-4afb-9d4a-204040808b53
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T16:12:19.601986899Z
---

## Goal

The repository's owner asked on 2026-11-17 that the tool, the skill and the
docs stop naming them: nothing in Atelier depends on who its human is, and
the Agent Factory skill is installed in other repositories, where a first
name in its rules means nothing. The skill, the product and design docs,
tests and three cutover scripts named one person, used "he" for the human,
or held that person's home directory.

## Done when

- [x] The skill (`skills/agent-factory`), `AGENTS.md`, `docs/product.md`, `docs/product/desk.md`, `docs/design.md`, `docs/architecture/desk.md` and `CHANGELOG.md` say "the human" and "they" where they named the person or said "he".
- [x] Tests, fixtures and doc examples use the placeholder name `ada` (`--by ada`, `caller = "ada"`) where they used the owner's name, and assert the same things.
- [x] `scripts/cutover/first-use-forge.sh`, `rehearse.sh` and `trials.sh` hold no home directory: they use the checkout they are in or `$HOME/projects`.
- [x] `git grep -i` for the owner's first name, family name and home directory name finds nothing outside `.atelier/`, `evals/agent-factory/results/` and `docs/architecture/quality/scorecard-baseline*`.
- [x] No command, record field, check or printed text changes.

## Scope

In: the files above, comments in `crates/` and `desk/` that said "he" or
"his", and the macOS bundle identifier in `t3-extension/install.py`.

Out, by the owner's answer on 2026-11-17 ("no need to rewrite history", and
two questions put to them):

- Records under `.atelier/`, open or finished. They say what happened and
  who said it, and other branches hold some of them.
- Dated results: `evals/agent-factory/results/` and the scorecard baseline
  (`docs/architecture/quality/scorecard-baseline-2026-11.md` and its
  directory, whose `manifest.json` keys transcripts by checkout path).
- The GitHub account in real URLs (`Cargo.toml`, `README.md`, `AGENTS.md`,
  the cutover kit, `.gitattributes`) and in fixtures that copy such URLs.
  They work only with that name until the repository moves.

## Report

Nothing outside the records and dated results names the owner now. 53 files
change, almost all one line for one line: the name becomes "the human" in
prose and `ada` in tests and examples, and "he", "him" and "his" for the
human become "they", "them" and "their". No behaviour changes.

Three changes are more than a rewording:

- `scripts/cutover/first-use-forge.sh` cloned a fixed path under one home
  directory. It now clones the checkout the script is in
  (`git -C "$KIT" rev-parse --show-toplevel`).
- `scripts/cutover/rehearse.sh` reads the projects from
  `${REHEARSE_PROJECTS:-$HOME/projects}`, and `trials.sh` defaults
  `TRIAL_REAL_PROJECTS` to `$HOME/projects`. On the home host these are the
  same directories as before. I checked the three scripts with `bash -n`
  and did not run them: each needs the host's real repositories and takes
  minutes.
- `t3-extension/install.py` writes the macOS bundle identifier
  `local.t3-custom`, where it held the home directory name. Nothing in the
  repository reads the identifier. On a Mac that already has T3 Custom, the
  next install rewrites `Info.plist` with the new identifier, and macOS may
  treat the launcher as a new app (a Dock pin or a granted permission may
  need setting again). I have no Mac to try it on.

The skill's two headings that held the name are now "Who Works With the
Human" and "Never Without the Human". No file links to either.

One choice the reviewer may question: pronouns. "The human ... he" read as
one known person, so I changed every "he" for the human in the living docs,
which is most of the diff in `docs/product/desk.md`. Where "them" would have
been ambiguous beside a plural I wrote "the human" again
(`procedures/garden.md` line 7, two rows of the Decisions table in
`docs/design.md`).

### Validation

- `cargo fmt -- --check` and `git diff --check`: pass.
- `cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk`: 390
  passed, 1 skipped.
- `pnpm test` in `desk/`: 57 passed. `pnpm typecheck`: passes.
- `python3 -m unittest test_grade` in
  `evals/agent-factory/scenarios/next-loop`: OK. One expected string there
  is the first 20 characters of a command, so it changed with the name's
  length.
- Not run here: `pnpm e2e` and `node t3-extension/run-tests.mjs`, which CI
  runs on the pull request.
