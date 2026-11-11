---
schema: atelier.mission
schema_version: 3
id: atelier-g6rp
title: Ship Agent Factory from this repository, so a command and its guidance land in one change
status: ready
labels:
- agent-factory
lead: null
ready:
  by: conversion
  at: 2026-11-08T23:41:24.026223269Z
created_at: 2026-11-08T05:51:18.943984031Z
---

## Why

Agent Factory (the skill) and Atelier are one product split across two repositories. The skill mentions Atelier 52 times, ADR 0020 says it targets Atelier only, and ADR 0020 Decision 3 exists only to keep the two repositories in step. A command change and the guidance that describes it land in separate commits, reviews and CI runs, and the guidance drifts. Moving the skill here means both land in one commit, one review and one CI run.

The skill is also global, and its description matches almost any multi-step software work, so it loads in repositories that have nothing to do with Atelier.

## Outcome

- The skill's only copy is `skills/agent-factory` in this repository, with its git history, and its evals are `evals/agent-factory`. spadaval/.agents no longer holds it.
- Anyone installs it with `npx skills add spadaval/atelier --skill agent-factory -g` (Vercel's `skills` CLI), unpinned, and the skill's first lines give that line and atelier's release installer.
- `atelier doctor` warns, without failing, when the binary is older than the latest GitHub release or the installed skill is behind, and prints the update command. `atelier status` shows the same warning from a cached result, so it stays fast. A skill symlinked to a checkout is reported as linked, not stale.
- `atelier setup --link` points the installed skill at this checkout for editing it, and `npx skills update` leaves the link alone.
- The skill's first step is `atelier status`. Outside an Atelier repository, or without atelier installed, it steps aside.
- spadaval/.agents loses the moved skill and evals, the Codex-only skills (`artifact-hub`, `generate-html-plan`, `pr-review-explorer`) and `codex-reflect`, after anything useful from `codex-reflect` is folded into Agent Factory's `audit.md`. That leaves it with no skills; whether to retire it is Sumeet's decision (`atelier-rfxf`).

## Scope

In: this repository's code, docs, ADRs and records; `skills/` and `evals/` once moved; spadaval/.agents (deleting what moved and the Codex-only skills); the symlinks `~/.claude/skills/agent-factory` and `~/.agents/skills/agent-factory` on this host.

Out: review verdicts, which `atelier-l58v` records as `fix_now` findings (`atelier-9mdj` and `atelier-7iq0`, a separate verdict record, were superseded on Sumeet's choice); the `firecrawl*` skills, which live only in `~/.agents/skills` and not in spadaval/.agents; retiring the spadaval/.agents repository, which Sumeet decides (`atelier-rfxf`); moving `file-pr` (decided against; see the mission's decision note).

Target environment: Linux and macOS, Claude Code and Codex reading `~/.claude/skills` and `~/.agents/skills`, `npx` from Node available on the machines that install the skill.

## Authority

The orchestrator may reorder and split items, and change flag names and cache details, without asking. It asks Sumeet before any push (to either repository), before deleting anything from spadaval/.agents beyond the skills named in the Outcome, and before changing the Outcome.
