---
created_at: "2026-11-08T05:52:53.329572276+00:00"
id: "atelier-8bj8"
issue_type: "feature"
labels:
- "agent-factory"
- "setup"
priority: "P2"
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
title: "Add setup --link so the live skill can be edited from a checkout of this repository"
updated_at: "2026-11-08T05:52:53.329572276+00:00"
---

## Goal

Let Sumeet edit the live Agent Factory skill from a checkout of this repository. Today `~/.agents/skills/agent-factory` and `~/.claude/skills/agent-factory` are hand-made symlinks into spadaval/.agents.

## Done when

- [ ] `atelier setup --link [<checkout>]` (the harness argument is not needed with `--link`; `<checkout>` defaults to the current repository's root) points `~/.agents/skills/agent-factory` and `~/.claude/skills/agent-factory` (honouring `CLAUDE_CONFIG_DIR`) at `<checkout>/skills/agent-factory`, replacing an existing symlink or a directory that `skills add` installed, and prints what each path pointed at before.
- [ ] It refuses, changing nothing, when `<checkout>/skills/agent-factory/SKILL.md` does not exist, or when a target path is a real directory that is not a `skills`-installed copy (not listed in the lock file); the message says what to move aside.
- [ ] It removes `agent-factory` from the `skills` lock file (`~/.agents/.skill-lock.json`, or under `$XDG_STATE_HOME/skills/`) if present, leaving other entries unchanged, so `npx skills update -g` skips it. Proven in a scratch home with `skills` 1.7.6: install the skill with `skills add` from a local fixture or a public repository, run `setup --link`, then `skills update -g`, and both links still point at the checkout. The Report shows the commands and output.
- [ ] `atelier setup --unlink` removes those links and prints the `npx skills add spadaval/atelier --skill agent-factory -g` line to reinstall.
- [ ] Its help says to link a checkout that stays on `master`, since the live skill follows whatever that checkout has checked out.
- [ ] Tests cover linking over a symlink, over a lock-tracked copy, refusing an untracked real directory, refusing a checkout with no skill, the lock entry removed with others kept, and unlink.

## Scope

Atelier's code (`setup`) and `docs/product`. Running it on this host is the install loose end.

## Context

Strategy: docs/strategies/agent-factory-in-atelier.md. Facts from testing `skills` 1.7.6 on 2026-11-08: a hand-made symlink that is not in the lock file is "untracked", and `check` and `update` leave it alone; once the skill is in the lock, `update` and `add` delete the symlink (not its target) and put a copy in its place. `add -g` writes the real files to `~/.agents/skills/<name>` and makes `~/.claude/skills/<name>` a relative symlink to them.
