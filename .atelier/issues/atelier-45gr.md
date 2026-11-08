---
created_at: "2026-11-08T05:52:52.985101458+00:00"
id: "atelier-45gr"
issue_type: "feature"
labels:
- "agent-factory"
- "setup"
workflow_branch:
  branch_base: master
  work_branch: atelier-45gr
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
status: "in_progress"
title: "Warn in doctor and status when the binary or skill is out of date, without blocking"
updated_at: "2026-11-08T06:31:48.373573191+00:00"
---

## Goal

Tell agents and Sumeet when the atelier binary or the Agent Factory skill is out of date, with the command that fixes it, without ever blocking work. Today nothing notices a stale install until a command or guidance disagrees.

## Done when

- [x] `atelier doctor` reports the binary's freshness: it compares the running version (ignoring `+<sha>` build metadata) with the highest `vX.Y.Z` tag of spadaval/atelier from `git ls-remote --tags https://github.com/spadaval/atelier` (no new HTTP crate, no API rate limit). Older: a warning naming both versions and the release installer line from `README.md`. Same or newer: current. No network, or `git ls-remote` failing or timing out: one line saying the check could not run. Never a non-zero exit for any of these.
- [x] `atelier doctor` reports the skill's freshness with Vercel's `skills` CLI at a pinned version (`npx -y skills@<version>`, 1.7.6 as of 2026-11-08), run with `DISABLE_TELEMETRY=1`: `skills ls -g --json` decides the case. Tracked: `skills check agent-factory -g`; exit 1 is stale (warn, print `npx skills update agent-factory -g`); exit 0 is current only when the output has no "Failed to check" line, otherwise "could not check". `~/.agents/skills/agent-factory` or `~/.claude/skills/agent-factory` is a symlink into a git checkout: "linked to a checkout at <path>", with a warning only when that checkout's `HEAD` is behind its local `master` (or the repository's base branch), giving the commit count and `git -C <path> checkout --detach master`; this uses local refs only. Untracked and not a link: warn that updates cannot be checked and print the `npx skills add spadaval/atelier --skill agent-factory -g` line. Missing: print the same add line. No `npx`: say the skill check needs Node and print the add line.
- [x] Each network call has a timeout of a few seconds, and the results are cached in the user's cache directory (`$XDG_CACHE_HOME/atelier` or the platform equivalent) for about a day. `doctor` refreshes the cache.
- [x] `atelier status` (and its hook form) shows a warning line only when the cached result says the binary or skill is stale, and never makes a network call or runs `npx` itself. When the cache is missing or older than a day, `status` starts a detached refresh and prints nothing about freshness. A `status` run stays within 50 ms of its time without this change (measured in the Report on this repository).
- [x] An environment variable (`ATELIER_NO_UPDATE_CHECK=1`) turns off the network calls and the refresh; tests and CI set it.
- [x] Tests cover each case above with a fake `npx` on `PATH`, a fake release endpoint (or an injected fetcher), and a temporary home and cache: stale binary, current binary, network failure, a linked checkout behind `master`, tracked-current, tracked-stale, tracked with "Failed to check" and exit 0, linked, untracked, missing, no `npx`, and `status` reading a stale cache.
- [x] `docs/product` describes the check and the variable.

## Scope

Atelier's code and `docs/product`. Not the skill's files.

## Context

Strategy: docs/strategies/agent-factory-in-atelier.md (freshness, not versioning; a fast `status` beats a fresh result). Facts from testing `skills` 1.7.6 in a scratch home on 2026-11-08: `check` is undocumented in `--help` but takes `update`'s flags, so `-g` covers global installs; it exits 1 when an update exists and 0 when up to date, but also 0 when it could not reach the source (it prints "✗ Failed to check skills from <source>" and then "All global skills are up to date"); its output has ANSI codes and no JSON form, while `skills ls -g --json` reports each skill's `status` as tracked, untracked or missing; it compares the skill folder's tree hash on the default branch with `~/.agents/.skill-lock.json` (or `$XDG_STATE_HOME/skills/.skill-lock.json`); a warm check took about 0.6 s, a cold `npx` download about 2.5 s; anonymous GitHub API calls are limited to 60 an hour.

## Unknowns

- How to start a detached refresh from `status` portably (a re-exec of the binary with a hidden flag is one way). Record the choice as a decision note.

## Notes

- 2026-11-08 [decision] status starts its refresh by re-executing the binary as the hidden `atelier refresh-update-check`, with no stdio and in its own process group, and starts at most one every 10 minutes (a stamp file in the cache directory).
  Reason: It is portable across Linux and macOS with std alone, needs no daemon or shell, the hook runner never waits on it since it holds no pipe, and the stamp stops several sessions opening at once from each starting a check.
  Rejected: fork/daemonize through libc: unsafe and Unix-only for no gain
  Rejected: `sh -c '... &'`: depends on a shell and quoting
- 2026-11-08 [decision] A symlinked skill is detected before any `npx` call, and only counts as linked when its resolved target is a Git work tree outside `~/.agents/skills`. Its base branch is the one `origin/HEAD` names, else `master`, else `main`. The detection is `freshness::linked_checkout(home, path)`, a public function the setup --link work can call.
  Reason: A linked checkout needs only local refs, so it works without Node. `skills add` links agents' copies (`~/.claude/skills/<name>`) into its own store, so a link into the store is a tracked install, not a checkout. This host's checkout's origin/HEAD is `main`, so a fixed `master` would compare with a missing branch.
  Rejected: Letting `skills ls` decide first: it needs npx for a case that needs none
- 2026-11-08 [decision] The cache stores the latest release version and the skill's warning line, not a binary verdict; `status` compares the cached latest release with its own version. A plain `key: value` file at `<cache>/atelier/update-check`, written to a temporary file and renamed.
  Reason: Upgrading the binary clears its warning at once instead of up to a day later; the rename means `status` never reads half a file.
- 2026-11-08 [decision] `ATELIER_NO_UPDATE_CHECK` is set for every test by `.cargo/config.toml`'s `[env]` (nextest honours it) and for CI by the workflow's `env`. When set, `status` does not even read the cache.
  Reason: About 40 test call sites spawn the binary; one setting covers them all and every future test. Not reading the cache keeps a developer's real cache from changing test output. The update-check tests remove the variable and fake the network and npx; making them pass without `env_remove` fails all ten that need it, which shows the setting reaches the tests.
  Rejected: Setting the variable at each spawn site: about 40 edits, and new tests would miss it
- 2026-11-08 [decision] Timeouts: 5 s for `git ls-remote`, 10 s for each `npx` call, killing the child's whole process group. Doctor runs the update check in a thread beside its history checks. Quiet `doctor` and `status` keep stdout unchanged and print only ⚠ lines to stderr.
  Reason: A cold `npx` download alone takes about 2.5 s, so 5 s would fail first runs on a slow link; killing the group also stops the `node` process npx starts, whose open pipe would otherwise hang the read. Quiet output is a parsed token contract.
- 2026-11-08 [handoff] Implemented at 29c80985; see ## Report. Review outstanding: not yet requested (no reviewer under this assignment). Scratch dirs under ~/.cache/atelier-tmp removed.

## Report

All seven Done when lines hold. `atelier doctor` prints an `Updates` section after its history checks, with one line for the binary and one for the skill, and its exit status still comes from those checks alone; `atelier status` and its hook show an `Updates` section only when the cache says something is behind. The tests are in `crates/atelier-cli/tests/update_check.rs` (11, all pass), each with its own home, cache, fake `npx` on `PATH` and a local release repository that Git's `url.<path>.insteadOf` puts in place of GitHub: stale binary and tracked-stale skill with doctor still passing (`doctor_warns_of_a_stale_binary_and_skill_and_still_passes`, which also checks the `-y skills@1.7.6` arguments, `DISABLE_TELEMETRY=1` and the cache it writes), current binary and tracked-current (`doctor_reports_a_current_binary_and_skill`), network failure plus "Failed to check" with exit 0 (`doctor_says_when_the_checks_cannot_run`), a linked checkout 2 commits behind `master` and then current, with no `npx` call (`doctor_reports_a_skill_linked_to_a_checkout_from_local_refs`, `doctor_finds_a_link_in_the_claude_skills_directory`), untracked, missing and no `npx` (three `doctor_*` tests), `status` reading a stale cache in plain, quiet and hook form while every live check would fail (`status_shows_the_cached_warnings_without_checking`), a day-old cache that `status` does not show but refreshes in the background (`status_refreshes_a_day_old_cache_in_the_background`), and the variable (`the_variable_turns_the_check_off`). Unit tests in `commands/freshness.rs` cover version parsing (`+<sha>` ignored, pre-releases rejected), the timeout killing a child and its children (`a_command_that_outlives_its_timeout_is_stopped`, 0.2 s), the cache format, and the ANSI-coloured "Failed to check" line. Breaking the stale, "Failed to check", version-order and behind-count logic made 6 of the 11 fail; dropping the tests' `env_remove` made all 10 that need the check fail, which shows `.cargo/config.toml`'s `ATELIER_NO_UPDATE_CHECK=1` reaches every test. Full `cargo nextest run -j 4 -p atelier-cli`: 816 passed; with `--features test-hooks`: 822 passed. `docs/product/sessions-and-hooks.md#update-check` describes the check and the variable, with pointers from `cli-surface.md` and `development-setup.md`.

`status` timing, debug build, 30 interleaved runs each in this repository: median 4160 ms with the check off (the pre-change path plus one environment read), 4152 ms reading a fresh cache with two warnings, 4165 ms with no cache, where it also starts the refresh. In a fresh one-commit repository, where noise is smaller, 60 runs each: 32.0, 30.2 and 28.2 ms. Both are within the 50 ms bound; the difference is below run-to-run noise. Manual runs of `target/debug/atelier doctor` in this worktree with a scratch home, against the real GitHub and real `npx`: binary current (0.3.1 against v0.3.1); skill missing; a real untracked skill directory (warns, reinstall line); a symlink to `~/projects/agents-skills` ("linked to a checkout ..., not behind main"); no `npx` and an unreachable release repository (both "could not check" lines); and the variable set. Doctor refused there on the uncommitted record, and still printed the Updates section.

### Deviations

- The linked-checkout warning names the checkout's base branch rather than always `master`: the branch `origin/HEAD` names, else `master`, else `main`. This host's `agents-skills` checkout tracks `main`.
- When the variable is set, `status` also skips reading the cache, not only the network and the refresh, so a developer's real cache cannot change test output.

### Decisions

See the decision notes below: the detached refresh (a hidden `atelier refresh-update-check`, re-executed in its own process group, at most one per 10 minutes); link detection before `npx` and outside the `skills` store, as the public `freshness::linked_checkout(home, path)`; the cache storing the latest release rather than a verdict; setting the variable through `.cargo/config.toml`; and the timeouts (5 s Git, 10 s `npx`) and quiet-mode behaviour.

### Open items

- The tracked cases ran only against a fake `npx`. The real `skills` CLI's tracked output was taken from this item's Context and the 1.7.6 source, not from a live tracked install, because `spadaval/atelier` does not publish the skill yet (atelier-hrgx checks that).
- The `git ls-remote` timeout is proven through the shared runner's unit test, not with a hanging Git server.
- macOS's cache path (`~/Library/Caches/atelier`) is untested here; CI runs the suite on macOS with the variable set.
