---
schema: atelier.item
schema_version: 3
id: atelier-vl5n
title: Write the readiness checklist, mapped to the three codebase properties
status: changes_requested
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:10:10.404909184Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.972580408Z
---

## Goal

Give the gardener a short static list of preconditions to check before it reads any numbers: can an agent find its way, build, change, test and launch in this repository. The scorecard shows effects weeks later; the checklist shows the cause on the day.

## Done when

- [x] `skills/agent-factory/procedures/readiness.md` holds the default checklist: at most twelve lines, grouped under the three properties (easy to navigate, easy and reliable to change, easy to validate). Each line gives the precondition, the command or observation that tests it, and the measure that would show its failure, named as `docs/product.md`'s Scorecard names it or as one of the log's numbers (time in each status, rounds, take-overs, escalations, skips). A candidate with no such measure is not on the list.
- [x] Every test on the list can be made by an agent without changing the repository, and a command it runs finishes inside ten minutes at `-j 4`. Where the honest test is longer, the line is an observation instead, such as the last CI run on the base.
- [x] The file says a repository's own lines live in `.atelier/readiness.md` in that repository, in the same form, checked after the defaults, and that a line is added or removed there by a reviewed change. It adds that file to no repository.
- [x] The list is drawn from Factory's agent-readiness pillars without the team-shaped criteria. The Report names each pillar and what was taken from it or why nothing was.
- [x] Trial: every line was tested in atelier and in demiurge, read only. The Report holds the result of each line in each (met, not met, could not test), and a line that could not be tested as written was reworded or cut.

## Scope

In: the one file `skills/agent-factory/procedures/readiness.md`.

Out, and a review finding that asks for one of these is escalated, not fixed:

- `garden.md`, `audit.md` and `SKILL.md`. atelier-9dku rewrites the gardener's procedure and links this file from it.
- A command, script or check that runs the checklist. An agent reads the list and runs each line.
- Any change in a product repository, and any fix for a line a repository does not meet. What the trial finds is recorded in the Report for the first runs (atelier-613a).
- Levels, scores or a pass mark. A line is met or not met.

## Starting candidates

The lead may choose the lines (mission Authority); these are a start, to cut and reword freely. Fewer is better.

- Navigate (tool calls before an agent's first edit): `AGENTS.md` is loaded by each harness in use and names the build, test, lint and launch commands; the relative links in `AGENTS.md` and the docs it names resolve; the docs it names describe directories that exist.
- Change (rework; rounds per item; time in `building`): a fresh worktree builds with the documented command and no manual step; format and lint run from one documented command and pass on the base; `.atelier/config.toml` lists them as merge checks.
- Validate (defects found after merge; where each defect was caught; skips): the tests run from one documented command and pass on the base; a merge check or CI runs them on every merge; a verify skill says how to launch the product and prove a feature, and the launch works; `atelier status` reports the clone's hooks installed; a bug is filed as an item with its `caused-by`.

## Context

Mission: `.atelier/missions/atelier-295u/` (design.md: Two layers; Risks, The checklist grows). `atelier check` accepts a file at `.atelier/readiness.md`, and the pre-push hook counts it as code, so it reaches the base only through a reviewed merge.

The trial is read only: use each repository's base checkout or a detached worktree, change and push nothing, and send nothing from a repository to an outside service. Demiurge is at `~/projects/demiurge`.

## Report

The gardener now has nine readiness lines in `skills/agent-factory/procedures/readiness.md`: three to navigate, two to change, four to validate. Each gives the precondition, its test, and the Scorecard measure or log number that would show its failure. Every test runs in a scratch clone of the base, so the repository is never changed, and each command in the trial finished in under 45 seconds at `-j 4`. The file says where a repository's own lines live, `.atelier/readiness.md`, and adds that file nowhere.

The trial found real gaps in both repositories. atelier fails four lines: `cargo clippy` is its documented lint but runs in neither its merge checks nor CI, the Desk has no format or lint command, there is no verify skill, and none of its 39 bug items names its cause. demiurge fails eight, mostly because `AGENTS.md` describes the old root app: 756 of the last month's file changes are in `app/`, which it never names.

### Lines cut or folded from the starting candidates

- "`atelier status` reports the clone's hooks installed" is cut. `status` already says "This clone has none of Atelier's Git hooks" at every session start, which is a sign where the agent trips; a checklist line would repeat it.
- "The docs it names describe directories that exist" is folded into line 3, which checks every path `AGENTS.md` names.
- "`.atelier/config.toml` lists format and lint as merge checks" is folded into line 7, which accepts a merge check or a CI workflow, since atelier runs most of its checks in CI.

### Factory's pillars

Read from Factory's public pages: the docs' overview at `docs.factory.com/agent-readiness/overview.md`, which names nine pillars, and the launch post at `factory.com/news/agent-readiness`, which names eight and adds Code Quality. Neither publishes the full list of criteria, so this works from the examples they give.

| Pillar | What was taken, or why nothing was |
| --- | --- |
| Style & Validation | Linter, type checker and formatter became line 5, and line 7 runs them on every merge. Pre-commit hooks were not taken: a merge check catches the same thing, and no measure tells the two apart. |
| Build System | "Build command documented" and "dependencies pinned" became line 4. VCS CLI tools were left out: `gh` is the host's setup, not the repository's, and `atelier merge` already refuses without it. |
| Testing | "Tests runnable locally" became line 6, and line 7 runs them on every merge. The unit and integration split was left out: no measure distinguishes them, and where each defect was caught shows the gap either way. |
| Documentation | `AGENTS.md` became lines 1 to 3, and "documentation freshness" became line 3's check that what it names exists. README was left out: it is written for people, and agents start at `AGENTS.md`. |
| Development Environment | "Environment template" and "local services setup" became line 4's "no step the docs leave out" and line 8's launch. Devcontainer was left out: agents here run on the host, and a devcontainer is one way to meet line 4, not the precondition. |
| Debugging & Observability | Nothing. Structured logging, tracing and metrics serve a service in production. These repositories are a CLI and local web apps, and no measure would show the absence. Line 8's verify skill is where an agent observes the running product. |
| Security | Nothing. CODEOWNERS and branch protection are team-shaped: they route review among people. Atelier's pre-push hook does branch protection's job for agents. Secret scanning shows in no measure. |
| Task Discovery | Issue and PR templates and labelling systems are team-shaped, and Atelier's records replace them. What the scorecard needs from an issue is its cause, which became line 9. |
| Product & Experimentation | Nothing. Analytics and experiments serve a product with users at scale, and no measure would show their absence. |
| Code Quality (launch post only) | Nothing. The Scorecard's rework row measures it after the fact, and it names no precondition with a test. |

Factory's levels were not taken; the item's Scope rules out levels and scores. Level 4's "fast CI feedback" became line 7's bound, the 9 minutes `atelier merge` waits. Flaky test detection was left out: it needs many runs, and a flake shows as a failed last run in line 7.

### Trial

Both run read only on 2026-11-11, in scratch clones of each base under `~/.cache/atelier-vl5n-tmp`: atelier at `0f0d8f334`, demiurge at `13b07f3`. Commands ran one at a time with `-j 4`. Nothing from either repository went to an outside service. Line 7's `gh run list` asked GitHub for atelier's own CI runs. The Desk's `pnpm install --frozen-lockfile` ran `--offline` from the host's store, and demiurge's `app/` checks used the base checkout's `node_modules` through a symlink in the clone.

| Line | atelier | demiurge |
| --- | --- | --- |
| 1. Harness loads the map, starts with `status` | Met. `CLAUDE.md` is `@AGENTS.md`; both hook files run `atelier status`. | Not met. There is no `CLAUDE.md`. Claude Code 2.1.322 does read `AGENTS.md` on this host today, under its default Project instructions setting with nothing above the checkout, but a host setting can turn that off. Both hook files run `atelier status`. |
| 2. `AGENTS.md` names build, test, lint, launch | Met. The install, `cargo nextest run`, clippy and fmt for `crates/`; the Desk's through `desk/README.md`. | Not met. It gives the old root app's build and no test command, and never names `app/`, where 756 of the last month's file changes landed. |
| 3. What `AGENTS.md` points to exists | Met. The script printed nothing over 17 named paths and the links in the 6 Markdown files among them. | Not met. It names `/src/docs/editor`, which does not exist. |
| 4. Fresh clone builds from the docs alone | Met. `cargo install --path crates/atelier-cli --locked` to a scratch root in 13 s; the Desk's `pnpm install` and `pnpm build` passed. | Not met. `bun run build` fails with `Script not found "vite"`. The `bun install` it needs is only in `README.md`, which `AGENTS.md` does not link, and `app/` has no documented build. |
| 5. Format, lint, type check documented and passing | Not met. `cargo fmt -- --check` and `cargo clippy --all-targets -- -D warnings` pass (10 s), and the Desk's `pnpm typecheck` passes, but the Desk has no format or lint command. | Not met. The documented format and lint commands write (`--write`, `--fix`) and are the root app's. In `app/`, `tsc --noEmit` and `biome check` pass (8 warnings) but are not documented. |
| 6. Tests run from one documented command and pass | Met. `cargo nextest run`: 292 passed in 27 s. The Desk: `pnpm test` 43 and `pnpm e2e` 52 passed. | Not met. No documented test command. `bun test` in `app/` passes 1,154 tests in 43 s. |
| 7. Every merge runs them | Not met. Clippy runs in neither `config.toml`'s merge checks nor `ci.yml`. The rest runs in CI, whose last full run on `master` passed in 532 s. | Not met. `config.toml` lists no merge check and names no forge. |
| 8. Verify skill, and its launch works | Not met. No verify skill. | Not met. No verify skill; `.agents/skills/` holds `frontend-design`, `openrouter-typescript-sdk` and `story-sketch`. |
| 9. Every bug names its cause | Not met. 39 bug items have `caused_by: null`, among them `atelier-o91z`, filed today. | Not met. 2: `worldcrafter-08kf` and `worldcrafter-yxfp`. |

No line was left could not test. Changed after the trial:

- Line 1 first failed demiurge for having no `CLAUDE.md`. Before submitting, I loosened it to Claude Code's rule for reading `AGENTS.md` directly, from `code.claude.com/docs/en/memory`, which made demiurge met.
- Line 3's script skipped package names such as `@legendapp/state`, reads a leading `/` as the repository's root, and follows the `.md` paths `AGENTS.md` names in backticks as well as its links. After review round 1 it also checks and follows Markdown files named at the root, such as `GUIDE.md`, skips URLs and anchors, and no longer takes the slash command `/agent-factory` for a path. On a fixture it reported a missing `MISSING.md`, a broken link inside a root `GUIDE.md`, and a broken relative link beside a valid `https` link, which it did not report. Its results on atelier and demiurge did not change.
- Line 1, after review rounds 1 and 2, no longer infers whether Claude Code reads `AGENTS.md` directly, which turns on its version, its Project instructions setting, its built-in AGENTS.md plugin and files above the checkout. It asks for what the repository controls: a `CLAUDE.md` that imports `@AGENTS.md` or is a symlink to it, which Claude Code reads under every setting but `managed-only`. That changed demiurge's result from met to not met.
- How to Check gained "install into a scratch root": atelier's documented `cargo install` would have replaced the host's shared `atelier` binary.

### Validation

The trial above is the validation: every line was run as written, in both repositories, and the commands in the file were copied from the ones that ran. `atelier check` in a scratch clone with a staged `.atelier/readiness.md` printed "Every record is valid", so the file's claim that `check` accepts it holds.

### Responses

1-pv9w.1: fixed in a453d7061
1-pv9w.2: fixed in a453d7061
