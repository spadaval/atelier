---
created_at: "2026-11-08T05:52:52.985101458+00:00"
id: "atelier-45gr"
issue_type: "feature"
labels:
- "agent-factory"
- "setup"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Warn in doctor and status when the binary or skill is out of date, without blocking"
updated_at: "2026-11-08T05:52:52.985101458+00:00"
---

## Goal

Tell agents and Sumeet when the atelier binary or the Agent Factory skill is out of date, with the command that fixes it, without ever blocking work. Today nothing notices a stale install until a command or guidance disagrees.

## Done when

- [ ] `atelier doctor` reports the binary's freshness: it compares the running version (ignoring `+<sha>` build metadata) with the latest release of spadaval/atelier from the GitHub API. Older: a warning naming both versions and the release installer line from `README.md`. Same or newer: current. No network or an API error: one line saying the check could not run. Never a non-zero exit for any of these.
- [ ] `atelier doctor` reports the skill's freshness with Vercel's `skills` CLI at a pinned version (`npx -y skills@<version>`, 1.7.6 as of 2026-11-08): `skills ls -g --json` decides the case. Tracked: `skills check agent-factory -g`; exit 1 is stale (warn, print `npx skills update agent-factory -g`); exit 0 is current only when the output has no "Failed to check" line, otherwise "could not check". `~/.agents/skills/agent-factory` or `~/.claude/skills/agent-factory` is a symlink into a git checkout: "linked to a checkout at <path>", no warning. Untracked and not a link: warn that updates cannot be checked and print the `npx skills add spadaval/atelier --skill agent-factory -g` line. Missing: print the same add line. No `npx`: say the skill check needs Node and print the add line.
- [ ] Each network call has a timeout of a few seconds, and the results are cached in the user's cache directory (`$XDG_CACHE_HOME/atelier` or the platform equivalent) for about a day. `doctor` refreshes the cache.
- [ ] `atelier status` (and its hook form) shows a warning line only when the cached result says the binary or skill is stale, and never makes a network call or runs `npx` itself. When the cache is missing or older than a day, `status` starts a detached refresh and prints nothing about freshness. A `status` run stays within 50 ms of its time without this change (measured in the Report on this repository).
- [ ] An environment variable (`ATELIER_NO_UPDATE_CHECK=1`) turns off the network calls and the refresh; tests and CI set it.
- [ ] Tests cover each case above with a fake `npx` on `PATH`, a fake release endpoint (or an injected fetcher), and a temporary home and cache: stale binary, current binary, network failure, tracked-current, tracked-stale, tracked with "Failed to check" and exit 0, linked, untracked, missing, no `npx`, and `status` reading a stale cache.
- [ ] `docs/product` describes the check and the variable.

## Scope

Atelier's code and `docs/product`. Not the skill's files.

## Context

Strategy: docs/strategies/agent-factory-in-atelier.md (freshness, not versioning; a fast `status` beats a fresh result). Facts from testing `skills` 1.7.6 in a scratch home on 2026-11-08: `check` is undocumented in `--help` but takes `update`'s flags, so `-g` covers global installs; it exits 1 when an update exists and 0 when up to date, but also 0 when it could not reach the source (it prints "✗ Failed to check skills from <source>" and then "All global skills are up to date"); its output has ANSI codes and no JSON form, while `skills ls -g --json` reports each skill's `status` as tracked, untracked or missing; it compares the skill folder's tree hash on the default branch with `~/.agents/.skill-lock.json` (or `$XDG_STATE_HOME/skills/.skill-lock.json`); a warm check took about 0.6 s, a cold `npx` download about 2.5 s; anonymous GitHub API calls are limited to 60 an hour.

## Unknowns

- How to start a detached refresh from `status` portably (a re-exec of the binary with a hidden flag is one way). Record the choice as a decision note.
