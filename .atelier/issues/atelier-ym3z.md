---
created_at: "2026-11-10T21:30:48.574415359+00:00"
id: "atelier-ym3z"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-ym3z
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
status: "in_progress"
title: "Add `setup` for the Git hooks, Claude Code and Codex, keeping a hook a clone already has"
updated_at: "2026-11-10T23:10:16.005298271+00:00"
---

## Goal

Install what orients and guards: `atelier setup` puts the three Git hooks in a clone, `setup claude` and `setup codex` make a session start with `status`, and `status` says when a clone has no hooks.

## Done when

- [x] `atelier setup` writes `pre-commit`, `pre-push` and `post-checkout` into the clone's hooks directory, the one all its worktrees share, each a one-line script that runs `atelier check --hook <name>` with the arguments and input Git gave it. Run again it changes nothing and says so. A test installs them in a scratch repository with a second worktree and sees a hook refuse a commit in each.
- [x] A hook the clone already has is kept: `setup` moves a hook it did not write to `<name>.local` and says so, and `check --hook <name>` runs `<name>.local` after its own rule passes, with the same arguments and the same input. For pre-commit and pre-push it fails if the kept hook fails; for post-checkout it prints the kept hook's failure and still exits 0. If `<name>.local` already exists, `setup` refuses for that hook, changes neither file, and says what to do. Tests: an existing `pre-push` hook that reads its input still runs and can still refuse a push; an existing hook beside an existing `<name>.local` leaves both files byte for byte as they were; a kept `post-checkout` hook that fails does not fail the checkout.
- [x] `status` says in one line, with the command that fixes it, when the clone has none of Atelier's hooks.
- [x] `setup claude` writes Claude Code's project hooks so that a session starts with `atelier status` and ends with `atelier check`, keeps any hook already in that settings file, and changes nothing when run again. The merging of hook settings is copied from the old workspace's `setup claude`, trimmed to what this calls, and named in the Report.
- [x] `setup codex` makes a Codex session in the repository start with `atelier status`. The Report says which Codex 0.167 hook event it uses and shows one real Codex session in a fixture repository whose transcript begins with `status` output. If no event fits, `setup codex` says so and prints the line for the repository's `AGENTS.md` that tells an agent to run `atelier status` first, and the Report shows that route working instead.
- [x] `docs/strategies/redesign-architecture.md` matches what was built, and "Which Codex hook event runs at session start" leaves its Open questions list.

## Scope

`setup`, `setup claude` and `setup codex` in `next/`, the chaining of a kept hook in `check --hook`, and the one line in `status`. `setup` installs nothing outside the repository it is run in, and no test or hand run touches this host's own Claude or Codex configuration or any real repository's hooks: fixtures only. `init` is not built by this mission. Copied from the old workspace: only the merging of hook settings into a harness's config, as the architecture specification lists. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, "Hooks and the bypass" and Threads and roles (harness hooks). Behaviour: `docs/strategies/redesign-product.md`, "Hooks and the bypass". The old code is `crates/atelier-cli/src/commands/setup.rs` and what it calls. Keeping a clone's existing hook and passing Git's inputs through to it answers planning finding atelier-1l4r; the cutover's rollback (mission 3) restores a clone's hooks directory from a copy, so `setup` only has to keep such a hook working. `check --hook` is atelier-pz1y's.

This is the second of step D's three items in `docs/strategies/redesign-2-core.md`.

## Notes

- 2026-11-10 [decision] When any hook's <name>.local is taken, setup refuses the whole run and writes no hook
  Reason: A refusal then means nothing changed, as every other refusal does, and the run that follows the printed way out installs all three at once; the Done when line requires only that neither file of that hook changes
  Rejected: Install the other hooks and refuse only the one whose .local is taken
- 2026-11-10 [decision] setup refuses when the directory Git runs hooks from (git rev-parse --git-path hooks, which follows core.hooksPath) is not inside the clone's common Git directory
  Reason: A relative core.hooksPath gives each worktree its own directory, often tracked, and an absolute one can be shared with every clone on the host; writing there breaks 'the one all its worktrees share' or 'installs nothing outside the repository'. Writing into .git/hooks instead would install hooks Git never runs. No repository on this host sets core.hooksPath
  Rejected: Write into wherever core.hooksPath points
  Rejected: Write into .git/hooks regardless
- 2026-11-10 [decision] The Claude Code Stop hook's command is 'atelier check || exit 1'
  Reason: Claude Code treats a Stop hook's exit 2 as blocking and runs the hook again after the model answers; check exits 2 on an error such as a failed Git call, which would loop. Exit 1 is reported and the turn ends
  Rejected: Plain 'atelier check'
  Rejected: A SessionEnd hook, whose output nobody sees
- 2026-11-10 [decision] A hook is Atelier's only when it is the exact script setup writes and is executable; a byte-identical script that lost its mode is rewritten, not kept
  Reason: Git skips a hook that is not executable, so counting it would make status and a rerun of setup report hooks that never run

## Report

All six Done when lines hold. In `next/crates/atelier-cli/tests/setup.rs` (11 scenarios, Git driving the real hook scripts with the new binary first on `PATH`): `setup_installs_three_hooks_once_for_every_worktree_and_status_says_when_there_are_none` (run from a second worktree it creates the missing hooks directory and writes the three scripts, mode 0755; run again from each worktree it says nothing changed and changes no byte; a broken record's commit is refused in each worktree; `status` begins with the missing-hooks line before and not after; a script that lost its mode is written again); `a_kept_pre_push_hook_gets_gits_arguments_and_input_and_can_still_refuse_after_atelier_passes` (the moved hook receives `origin <url>` and Git's ref line, refuses a push that then does not happen, and does not run when Atelier's rule refuses); `a_hook_beside_a_local_one_is_left_byte_for_byte_and_the_way_out_works` (refusal, both files and their modes unchanged, no other hook written; the printed way out followed, then the kept pre-commit runs on a commit); `a_symbolic_link_or_a_hook_that_is_not_executable_is_kept_as_it_was_and_runs_only_as_git_would` (a relative link is moved as a link and still refuses; a non-executable hook is moved, said so, and not run); `a_kept_hook_with_no_interpreter_line_runs_through_the_shell_as_git_ran_it` (a script with no `#!` line, which Git runs through `/bin/sh`, runs before and after `setup` and can still refuse; added for review finding atelier-3elo, and seen failing without the fallback); `a_kept_post_checkout_hook_that_fails_does_not_fail_the_checkout` (status, then the kept hook with Git's three arguments, then its failure, exit 0); `setup_stopped_after_moving_a_hook_finishes_when_run_again`; `a_hooks_path_outside_the_shared_git_directory_is_refused_and_the_way_out_works` (a relative `core.hooksPath`, nothing changed in either directory, the printed line run as written); `setup_claude_merges_its_hooks_into_the_settings_keeping_others_and_changes_nothing_again` (another tool's hooks and the old tool's `land --hook claude` entry: the others kept, the old one replaced; a second run with a later group added changes no byte); `setup_claude_refuses_settings_it_cannot_read_and_writes_a_new_file_where_there_is_none` (invalid JSON, an event that is not a list, a settings file that is a link: each refused with the file unchanged; a new file at 0644); `setup_codex_starts_a_session_with_status_and_changes_nothing_again`. Each was seen failing under a mutation of what it covers (12 mutations: the kept hook never run, for each hook kind; its input not passed; a non-executable kept hook run; the clash, `core.hooksPath` and invalid-JSON refusals removed; nothing ever unchanged; a foreign hook not moved; the merge always appending; the old marker kept; the status line removed). Ten existing tests that compare `status` output exactly now expect the missing-hooks line first (`NO_HOOKS` in `tests/common/mod.rs`). After merging `master` (atelier-dxrj) and fixing atelier-3elo: full suite 200 passed; fmt and clippy `-D warnings` clean; the CI `next` job's steps pass in a scratch clone of the fix's commit. The architecture specification's Threads and roles paragraph on harness hooks and "Hooks and the bypass" describe the build, and the Codex question left Open questions.

### What `setup` writes where

For the cutover's runbook and its rollback:

- `atelier setup`: in the directory `git rev-parse --git-path hooks` names, which must be inside the clone's common Git directory (normally `<clone>/.git/hooks`), the files `pre-commit`, `pre-push` and `post-checkout`, each `#!/bin/sh` and `exec atelier check --hook <name> "$@"`, mode 0755, `atelier` found on `PATH`. A hook already there that is not this script is renamed to `<name>.local` in the same directory. Nothing else. The rollback's restore of the hooks directory from its copy undoes all of it.
- `atelier setup claude`: `<repo>/.claude/settings.json`, adding `hooks.SessionStart` running `atelier status # atelier-managed:SessionStart` and `hooks.Stop` running `atelier check || exit 1 # atelier-managed:Stop`, each with `timeout` 30, and removing any other handler whose command ends in that event's marker (today's `atelier status --hook claude` and `atelier land --hook claude`). The old tool's `setup claude` does the reverse, by the same marker, so the rollback's `setup` restores today's entries. The file is rewritten with keys sorted, as the old tool did.
- `atelier setup codex`: `<repo>/.codex/hooks.json`, adding `hooks.SessionStart` running `atelier status # atelier-managed:SessionStart`. The old tool has no Codex setup, so the rollback removes this file or that entry by hand.

### Codex

Codex 0.167 reads a project's hooks from `<repo>/.codex/hooks.json` (or `[hooks]` in `<repo>/.codex/config.toml`) in Claude Code's form, and its `SessionStart` event runs at session start with `source` `startup`, `resume`, `clear`, `compact` or `fork`; plain stdout is added as developer context (Codex's hooks page, learn.chatgpt.com/docs/hooks, reached from developers.openai.com/codex/hooks; the event names are in the installed binary). Project hooks run only in a trusted project and once the hook is trusted in `/hooks`, or under `--dangerously-bypass-hook-trust`. One session, `codex exec` on `gpt-6.1-sol` at default effort, in a fixture repository under `/var/tmp/` with a scratch `HOME` and `CODEX_HOME` (only a link to the sign-in, removed afterwards; the project trusted in the scratch config; hook trust bypassed by the flag), the new binary first on `PATH`, after `setup codex` and a commit. The session's rollout file holds, before the user's message, this developer message:

```text
developer ['hooks.additional_context'] [{'provenance': {'type': 'harness', 'hook': True}}]
This clone has none of Atelier's Git hooks, so nothing checks its commits and pushes. Install them with: atelier setup
You are thread human, on the base branch main.
Ready to start:
  demo-ab12 (high): Say hello
Then: atelier start demo-ab12
```

The model's reply: "The startup message says Atelier's Git hooks are missing, recommends installing them, and identifies `demo-ab12` ("Say hello") as the next task."

### Copied piece

`merge_hooks` in `next/crates/atelier-cli/src/setup.rs` is copied from `run` in `crates/atelier-cli/src/commands/setup.rs`. Trimmed: the `--binary` and `--quiet` options, the `StorageLayout` lookup, its message and the warning about `CLAUDE.md` not importing `AGENTS.md`. Changed: it takes the file and the hooks, so `setup codex` uses it too; a handler already in place is left where it is instead of removed and appended again, so a run with another hook after Atelier's changes nothing; only a group it emptied is removed; a file it cannot read is a refusal with nothing changed rather than an error; the mode is kept through one write helper shared with the hook scripts.

### Hand transcript

The new binary on `PATH` as `atelier`, a scratch repository with a second worktree, a pre-push hook of its own and a local bare remote:

```text
-- A clone with no Atelier hooks, and a pre-push hook of its own --
$ ls .git/hooks | grep -v sample
pre-push
[exit 0]
$ atelier status
This clone has none of Atelier's Git hooks, so nothing checks its commits and pushes. Install them with: atelier setup
You are thread t1, on the base branch main.
Ready to start:
  demo-ab12 (high): Say hello
Then: atelier start demo-ab12
[exit 0]
-- setup, from the second worktree, then again from each --
$ (cd ../second && atelier setup)
Installed Atelier's Git hooks in <scratch>/repo/.git/hooks, which every worktree of this clone shares:
  pre-commit: written
  pre-push: written; this clone's own pre-push hook was moved to pre-push.local, and runs after Atelier's rule passes
  post-checkout: written
[exit 0]
$ atelier setup
Atelier's Git hooks are in <scratch>/repo/.git/hooks already; nothing changed.
[exit 0]
$ (cd ../second && atelier setup)
Atelier's Git hooks are in <scratch>/repo/.git/hooks already; nothing changed.
[exit 0]
$ ls -l .git/hooks | grep -v sample
total 80
-rwxr-xr-x 1 supadava supadava   55 Nov 10 18:08 post-checkout
-rwxr-xr-x 1 supadava supadava   52 Nov 10 18:08 pre-commit
-rwxr-xr-x 1 supadava supadava   50 Nov 10 18:08 pre-push
-rwxrwxr-x 1 supadava supadava  196 Nov 10 18:08 pre-push.local
[exit 0]
$ cat .git/hooks/pre-push; cat .git/hooks/pre-push.local
#!/bin/sh
exec atelier check --hook pre-push "$@"
#!/bin/sh
while read local sha remote rsha; do echo "own pre-push hook saw $local -> $remote"; done
[ -e ../stop ] && { echo "own pre-push hook: stop file present, refusing" >&2; exit 1; }
exit 0
[exit 0]
-- A commit of a broken record, refused in each worktree --
$ (cd . && git commit -qm Broken)
Atelier's pre-commit hook refused this commit; nothing was committed.
These do not read as records. That is not a rule, and it cannot be skipped:
  .atelier/items/demo-zz99/item.md: front matter: unknown field `not`, expected one of `schema`, `schema_version`, `id`, `title`, `status`, `mission`, `labels`, `priority`, `blocked_by`, `implementers`, `ready`, `trivial`, `caused_by`, `created_at`
Fix each file named, stage it with git add, and commit again.
[exit 1]
$ (cd ../second && git commit -qm Broken)
Atelier's pre-commit hook refused this commit; nothing was committed.
These do not read as records. That is not a rule, and it cannot be skipped:
  .atelier/items/demo-zz99/item.md: front matter: unknown field `not`, expected one of `schema`, `schema_version`, `id`, `title`, `status`, `mission`, `labels`, `priority`, `blocked_by`, `implementers`, `ready`, `trivial`, `caused_by`, `created_at`
Fix each file named, stage it with git add, and commit again.
[exit 1]
-- The kept pre-push hook runs after Atelier's rule passes, and can still refuse --
$ git switch -qc topic
You are thread t1, on branch topic, which is no item's.
Ready to start:
  demo-ab12 (high): Say hello
Then: atelier start demo-ab12
[exit 0]
$ echo hi > hello.txt && git add hello.txt && git commit -qm "Say hello"
[exit 0]
$ git push -q origin topic
own pre-push hook saw refs/heads/topic -> refs/heads/topic
[exit 0]
$ touch ../stop && git commit -q --allow-empty -m Again && git push -q origin topic
own pre-push hook saw refs/heads/topic -> refs/heads/topic
own pre-push hook: stop file present, refusing
This clone's own pre-push hook, which atelier setup kept as pre-push.local in its hooks directory, refused: it exited with 1. Nothing was pushed.
Its output above says why; do what it asks, then run the Git command again.
error: failed to push some refs to '../remote.git'
[exit 1]
$ git ls-remote origin refs/heads/topic; git rev-parse HEAD~1 HEAD
2132869b31422502fee04a1b57537eff58cee613	refs/heads/topic
2132869b31422502fee04a1b57537eff58cee613
fe656f29cb2a7ff6b183b1fa0f2f56d8073e408a
[exit 0]
$ rm ../stop
[exit 0]
-- setup claude, twice --
$ cat .claude/settings.json
{
  "hooks": {
    "PreToolUse": [{"matcher": "Bash", "hooks": [{"type": "command", "command": "other-tool guard"}]}],
    "Stop": [{"hooks": [{"type": "command", "command": "atelier land --hook claude # atelier-managed:Stop", "timeout": 30}]}]
  }
}
[exit 0]
$ atelier setup claude
Wrote Atelier's hooks into .claude/settings.json, keeping every other hook there.
A Claude Code session started in this repository begins with atelier status, and each turn ends with atelier check.
Commit .claude/settings.json so that every clone has it.
[exit 0]
$ cat .claude/settings.json
{
  "hooks": {
    "PreToolUse": [
      {
        "hooks": [
          {
            "command": "other-tool guard",
            "type": "command"
          }
        ],
        "matcher": "Bash"
      }
    ],
    "SessionStart": [
      {
        "hooks": [
          {
            "command": "atelier status # atelier-managed:SessionStart",
            "timeout": 30,
            "type": "command"
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "command": "atelier check || exit 1 # atelier-managed:Stop",
            "timeout": 30,
            "type": "command"
          }
        ]
      }
    ]
  }
}
[exit 0]
$ atelier setup claude
Atelier's hooks are in .claude/settings.json already; nothing changed.
A Claude Code session started in this repository begins with atelier status, and each turn ends with atelier check.
[exit 0]
$ cat .claude/settings.json | md5sum; atelier setup claude >/dev/null; cat .claude/settings.json | md5sum
59f41c280d5bbe932ec7a6656efb855b  -
59f41c280d5bbe932ec7a6656efb855b  -
[exit 0]
```

### Surprises

- In the Codex session, `status` called the thread `human`: the SessionStart hook's environment had none of the variables identity reads, though the architecture specification says a Codex thread is known by `CODEX_SESSION_ID`. Under T3 Code the inherited `T3CODE_THREAD_ID` would name it. Not this item's to change; the planner may want it checked against commands an agent runs in its shell.
- Git sends a hook's stdout to its own stderr, so `status` printed by post-checkout reaches the agent on stderr.
- `setup` needs the base branch to have a commit, as every command but `check` opens the repository through its config and base; a repository with no commit gets `the base branch main named in .atelier/config.toml does not exist`.

### Open items

- The Claude Code hooks were not run in a real Claude Code session; they use the events today's `setup claude` uses, and SessionStart's plain stdout is context in Claude Code as in Codex.
