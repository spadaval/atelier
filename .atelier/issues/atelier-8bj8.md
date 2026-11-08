---
created_at: "2026-11-08T05:52:53.329572276+00:00"
id: "atelier-8bj8"
issue_type: "feature"
labels:
- "agent-factory"
- "setup"
workflow_branch:
  branch_base: master
  work_branch: atelier-8bj8
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
status: "in_progress"
title: "Add setup --link so the live skill can be edited from a checkout of this repository"
updated_at: "2026-11-08T06:09:55.757347156+00:00"
---

## Goal

Let Sumeet edit the live Agent Factory skill from a checkout of this repository. Today `~/.agents/skills/agent-factory` and `~/.claude/skills/agent-factory` are hand-made symlinks into spadaval/.agents.

## Done when

- [x] `atelier setup --link [<checkout>]` (the harness argument is not needed with `--link`; `<checkout>` defaults to the current repository's root) points `~/.agents/skills/agent-factory` and `~/.claude/skills/agent-factory` (honouring `CLAUDE_CONFIG_DIR`) at `<checkout>/skills/agent-factory`, replacing an existing symlink or a directory that `skills add` installed, and prints what each path pointed at before.
- [x] It refuses, changing nothing, when `<checkout>/skills/agent-factory/SKILL.md` does not exist, or when a target path is a real directory that is not a `skills`-installed copy (not listed in the lock file); the message says what to move aside.
- [x] It removes `agent-factory` from the `skills` lock file (`~/.agents/.skill-lock.json`, or under `$XDG_STATE_HOME/skills/`) if present, leaving other entries unchanged, so `npx skills update -g` skips it. Proven in a scratch home with `skills` 1.7.6: install the skill with `skills add` from a local fixture or a public repository, run `setup --link`, then `skills update -g`, and both links still point at the checkout. The Report shows the commands and output.
- [x] `atelier setup --unlink` removes those links and prints the `npx skills add spadaval/atelier --skill agent-factory -g` line to reinstall.
- [x] Its help says to link a checkout that stays on `master`, since the live skill follows whatever that checkout has checked out.
- [x] Tests cover linking over a symlink, over a lock-tracked copy, refusing an untracked real directory, refusing a checkout with no skill, the lock entry removed with others kept, and unlink.

## Scope

Atelier's code (`setup`) and `docs/product`. Running it on this host is the install loose end.

## Context

Strategy: docs/strategies/agent-factory-in-atelier.md. Facts from testing `skills` 1.7.6 on 2026-11-08: a hand-made symlink that is not in the lock file is "untracked", and `check` and `update` leave it alone; once the skill is in the lock, `update` and `add` delete the symlink (not its target) and put a copy in its place. `add -g` writes the real files to `~/.agents/skills/<name>` and makes `~/.claude/skills/<name>` a relative symlink to them.

## Notes

- 2026-11-08 [decision] Both install paths link straight to <checkout>/skills/agent-factory with absolute symlinks, rather than keeping the claude path a relative link to the agents path.
  Reason: Each path then names the checkout on its own, so a later skills add or a hand edit of one path cannot silently redirect the other, and --unlink can remove each link without following a chain.
- 2026-11-08 [decision] Unlink (setup --unlink) removes only symlinks, and leaves both paths alone when the lock file lists agent-factory.
  Reason: A listed skill means skills add has reinstalled it since the link, so its relative claude link and copy belong to skills, not to setup --link; removing them would break a working install.
- 2026-11-08 [decision] The lock file is rewritten through serde_json::Value, which sorts its keys; entries keep their values but not their order.
  Reason: skills reads the file as JSON and never depends on key order. Keeping order needs serde_json's preserve_order feature, which would change key order in every JSON the workspace writes, including .claude/settings.json from setup claude.
  Rejected: enable serde_json preserve_order for the workspace
- 2026-11-08 [decision] A lock file that is not valid JSON counts as listing nothing and is not rewritten.
  Reason: skills 1.7.6 reads an unparseable lock the same way (createEmptyLockFile), so update will not touch the links, and a real directory in a path is refused as untracked rather than deleted.

## Report

All six Done when lines hold. `atelier setup --link [<checkout>]` links `~/.agents/skills/agent-factory` and `$CLAUDE_CONFIG_DIR`-or-`~/.claude` `/skills/agent-factory` to `<checkout>/skills/agent-factory` (default: the repository root), replacing a symlink or a lock-listed copy and printing what each path held; it refuses, changing nothing, a checkout without `skills/agent-factory/SKILL.md` or an unlisted real directory, naming the `mv` to run; it drops `agent-factory` from the `skills` lock file (`$XDG_STATE_HOME/skills/` or `~/.agents/`) and keeps the other entries. `setup --unlink` removes the links and prints the `npx skills add spadaval/atelier --skill agent-factory -g` line. `setup --help` says to link a checkout that stays on master. `setup claude` is unchanged. Tests in `crates/atelier-cli/tests/cli_integration/skill_link.rs` cover each case (link over symlinks, over a lock-tracked copy, both env overrides, refusing an untracked directory, refusing a checkout with no skill, the default checkout and quiet token, unlink, and the harness still required): 8 passed, and they failed (3 of 8) with the lock removal and the untracked-directory check mutated out. `cargo nextest run -j 4 -p atelier-cli`: 808 passed. `cargo fmt -- --check` and `git diff --check` pass. Product docs: `docs/product/sessions-and-hooks.md` (new section "Linking the Agent Factory skill"), `cli-surface.md`, and the quiet-token table in `human-cli-output.md`.

### Proof with skills 1.7.6

Scratch root `$S` = `/home/supadava/.cache/atelier-tmp/8bj8-skills`, with HOME `$S/home`, `DISABLE_TELEMETRY=1`, `CLAUDE_CONFIG_DIR` and `XDG_STATE_HOME` unset; the checkout is a fixture `co/skills/agent-factory/SKILL.md` with `name: agent-factory`. `spadaval/atelier` has no skill yet, so the skill came from `spadaval/.agents`, its current public home.

```text
$ npx -y skills@1.7.6 add spadaval/.agents --skill agent-factory -g -a claude-code -a codex -y
  ✓ ~/.agents/skills/agent-factory   universal: Codex   symlinked: Claude Code
$ npx -y skills@1.7.6 add anthropics/skills --skill skill-creator -g -a claude-code -a codex -y
# Both lock entries' skillFolderHash set to 0000... so update has work to do.
$ ls -l ~/.agents/skills ~/.claude/skills
~/.agents/skills: agent-factory/ (dir)  skill-creator/ (dir)
~/.claude/skills: agent-factory -> ../../.agents/skills/agent-factory  skill-creator -> ../../.agents/skills/skill-creator
$ atelier setup --link $S/co
Linked the Agent Factory skill to $S/co/skills/agent-factory.
  $S/home/.agents/skills/agent-factory: was a copy skills add installed, now removed
  $S/home/.claude/skills/agent-factory: was a link to ../../.agents/skills/agent-factory
Removed agent-factory from $S/home/.agents/.skill-lock.json, so `npx skills update -g` leaves the links alone.
The live skill follows whatever $S/co has checked out; keep it on master. Undo with `atelier setup --unlink`.
$ cat ~/.agents/.skill-lock.json      # dismissed, skills.skill-creator (unchanged values), version 3; no agent-factory
$ npx -y skills@1.7.6 update -g -y
Checking skills from source: anthropics/skills
Found 1 global update(s)
Updating skill-creator…
  ✓ Updated skill-creator
✓ Updated 1 skill(s)
$ readlink ~/.agents/skills/agent-factory ~/.claude/skills/agent-factory
$S/co/skills/agent-factory
$S/co/skills/agent-factory
```

Control, in a copy of the home before linking: hand-made symlinks to the checkout with the lock entry kept (stale hash), then `npx -y skills@1.7.6 update -g -y` printed `Updated agent-factory` and put back a real directory in `~/.agents/skills/agent-factory` and the relative link in `~/.claude/skills`. So the lock removal is what keeps the links.

### Decisions

See the four decision notes: absolute links from both paths, unlink leaves a lock-listed install alone, the lock file's keys come back sorted, and an unparseable lock lists nothing.

### Surprises

`skills add -g -a claude-code` alone copies the skill straight into `~/.claude/skills/agent-factory` as a real directory, with no `~/.agents` copy; `--link` handles that as a lock-listed copy too.

### Open items

- `cargo nextest run --features test-hooks` was not run; the change adds no fault hooks.
- Running `setup --link` on this host is atelier-bxs7.
