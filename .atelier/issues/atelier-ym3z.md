---
created_at: "2026-11-10T21:30:48.574415359+00:00"
id: "atelier-ym3z"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add `setup` for the Git hooks, Claude Code and Codex, keeping a hook a clone already has"
updated_at: "2026-11-10T21:30:48.574415359+00:00"
---

## Goal

Install what orients and guards: `atelier setup` puts the three Git hooks in a clone, `setup claude` and `setup codex` make a session start with `status`, and `status` says when a clone has no hooks.

## Done when

- [ ] `atelier setup` writes `pre-commit`, `pre-push` and `post-checkout` into the clone's hooks directory, the one all its worktrees share, each a one-line script that runs `atelier check --hook <name>` with the arguments and input Git gave it. Run again it changes nothing and says so. A test installs them in a scratch repository with a second worktree and sees a hook refuse a commit in each.
- [ ] A hook the clone already has is kept: `setup` moves a hook it did not write to `<name>.local` and says so, and `check --hook <name>` runs `<name>.local` after its own rule passes, with the same arguments and the same input, and fails if it fails. A test with an existing `pre-push` hook that reads its input sees that hook still run and still able to refuse a push.
- [ ] `status` says in one line, with the command that fixes it, when the clone has none of Atelier's hooks.
- [ ] `setup claude` writes Claude Code's project hooks so that a session starts with `atelier status` and ends with `atelier check`, keeps any hook already in that settings file, and changes nothing when run again. The merging of hook settings is copied from the old workspace's `setup claude`, trimmed to what this calls, and named in the Report.
- [ ] `setup codex` makes a Codex session in the repository start with `atelier status`. The Report says which Codex 0.167 hook event it uses and shows one real Codex session in a fixture repository whose transcript begins with `status` output. If no event fits, `setup codex` says so and prints the line for the repository's `AGENTS.md` that tells an agent to run `atelier status` first, and the Report shows that route working instead.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built, and "Which Codex hook event runs at session start" leaves its Open questions list.

## Scope

`setup`, `setup claude` and `setup codex` in `next/`, the chaining of a kept hook in `check --hook`, and the one line in `status`. `setup` installs nothing outside the repository it is run in, and no test or hand run touches this host's own Claude or Codex configuration or any real repository's hooks: fixtures only. `init` is not built by this mission. Copied from the old workspace: only the merging of hook settings into a harness's config, as the architecture specification lists. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, "Hooks and the bypass" and Threads and roles (harness hooks). Behaviour: `docs/strategies/redesign-product.md`, "Hooks and the bypass". The old code is `crates/atelier-cli/src/commands/setup.rs` and what it calls. Keeping a clone's existing hook and passing Git's inputs through to it answers planning finding atelier-1l4r; the cutover's rollback (mission 3) restores a clone's hooks directory from a copy, so `setup` only has to keep such a hook working. `check --hook` is atelier-pz1y's.

This is the second of step D's three items in `docs/strategies/redesign-2-core.md`.
