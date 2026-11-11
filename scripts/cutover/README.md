# The cutover

This directory moves atelier, astromech, demiurge, life-on-rails and
cosplay-studio onto the new tool on this host, in one sitting, and can put
everything back. The protocol is the Transition section of
`docs/strategies/redesign-architecture.md`. Read this page first on the day.

| File | What it is |
| --- | --- |
| `cutover.sh` | The runbook: `preflight`, the cutover, and `adopt` for another host |
| `rollback.sh` | Undoes what the runbook's journal says it started |
| `common.sh` | The settings both read, with this host's values as defaults |
| `convert_records.py` | The converter, run by the runbook in each repository |
| `agents/atelier.md` | atelier's new `AGENTS.md`, whole |
| `agents/product.md` | The process section each product repository's `AGENTS.md` gets (`@BASE@` is its base branch) |
| `astromech-atelier.yml` | astromech's new `.github/workflows/atelier.yml` (`@ATELIER_REV@` is atelier's conversion commit) |
| `trials.sh` | Runs the runbook and the rollback in sandboxes, at every point it can stop |
| `first-use-forge.sh`, `rehearse.sh` | The new binary's first read-only questions to GitHub, and mission 2's rehearsal |

## Before: quiet

All of this must hold in the five repositories, and `cutover.sh preflight`
checks it:

- every worktree is clean, untracked files included;
- each base branch is at its remote's tip;
- no item is in progress, as the installed old binary reports it, except
  atelier-u1ko itself before its merge;
- no T3 thread other than the cutover's own (and the threads waiting on it)
  is running in any of them or their worktrees. The preflight reads that
  from T3's database with one read-only query. If it cannot, it prints the
  hand check; once you have made it, run it again with
  `CUTOVER_THREADS_CHECKED=yes`.

It also lists, as information, each local branch not merged into its base
(its reviews are not carried over) and each worktree not on its base.

## The three commands

1. From atelier-u1ko's checkout: `scripts/cutover/cutover.sh preflight`.
   It changes nothing. Fix what it names; never work around it.
2. From the same checkout, under the installed old binary: `atelier merge
   atelier-u1ko --push --wait --timeout 9`, again if CI is still pending.
3. Straight after, from the worktree that has atelier's `master` checked
   out (it refuses to run anywhere else): `scripts/cutover/cutover.sh`.

The runbook stops at the first failure, says which step failed and what
`rollback.sh` will undo, and keeps its journal in `~/.cache/atelier-cutover/`
(`CUTOVER_STATE`). Run again after a stop, it picks up from the journal.
Its exit status is 0 when it finished, 1 when it stopped, and 3 when CI on
atelier's conversion commit had not finished within 7 minutes
(`CUTOVER_CI_WAIT`, in seconds): that is not a failure, so run it again and
it goes on waiting. Each run fits in a 10-minute foreground command.

## The steps, and how long they took

The full run in the sandbox took 11 to 13 s, every step under 6 s. Two
steps take longer here than in the sandbox, which installs a prebuilt
binary and answers CI from a double:

| Step | Does | Sandbox | Here |
| --- | --- | --- | --- |
| 1. Preflight | The checks above; then records each base, each worktree's checkout, a copy of each hooks directory, the installed binary, the skill checkout's commit and the service | 2 s | the same |
| 2. Convert atelier | In the master worktree: converter, `AGENTS.md`, `skills/agent-factory-next/` to `skills/agent-factory/`, `setup claude` and `setup codex`; commit; `--verify` and the tree's `atelier check` | 1 s | plus the release build, 13 s on this host (18 s without sccache) |
| 3. Install | `cargo install` from master, reusing that build; `atelier setup` for the Git hooks and each harness; the skill checkout to the conversion commit; `atelier-serve` stopped and disabled | 1 s | a few seconds |
| 4. Push atelier's master | `ATELIER_SKIP="cutover conversion" git push` | 1 s | the same |
| 5. Convert the product repositories | Each in a fresh clone of its base: converter, its `AGENTS.md` section, astromech's workflow, harness hooks; commit; `--verify` and `check`; push with the same skip | 5 s | the same |
| 6. Bring each clone onto the converted base | Fetch; move the base; the main worktree onto the base; `atelier setup` | 1 s | the same |
| 7. Accept | `check`, `status` and the counts in each main and base worktree; the new version; the new skill text; the service off; CI passed on the conversion commit, its `atelier check` step included | 2 s | plus CI: its test job took about a minute on this branch's pull request |
| 8. Close atelier-wd1i | `atelier close`, which commits; `git push` | 0 s | the same |

## What you will see

Each step prints `== Step <n>`, what it did and how long it took. At the end
the report gives, for each repository, the conversion commit, the count of
items and missions, `--verify`'s result, how many relative links resolve,
and where each worktree is.

Where the checkouts are left:

- **atelier.** The worktree that has `master` is on `master` at the closing
  commit. The main worktree, `~/projects/atelier`, which is detached, is
  detached at the converted `master` (the conversion commit; `master` then
  gains the closing commit). The skill checkout, `~/projects/atelier-skill`,
  is detached at the conversion commit. Other worktrees are as they were.
- **astromech.** The main worktree, which is on a feature branch today,
  is switched to `main`; the feature branch is kept. Its other worktrees
  are as they were.
- **demiurge, life-on-rails, cosplay-studio.** The main worktree is on its
  base, at the conversion commit. Other worktrees are as they were.

A worktree left on a branch that still has the old layout is printed with
its path and branch: `atelier` and the hooks refuse there until it takes the
converted base (below).

## How to tell it worked

The runbook finished with exit status 0 and printed its report; step 7 had
passed before step 8 ran. Then, in any of the five repositories,
`atelier --version` prints `atelier 0.5.0`, `atelier status` runs, and
`atelier check` says every record is valid. `systemctl --user is-active
atelier-serve` says `inactive`.

## When to roll back

- **It stopped at or before step 7** (exit status 1): run
  `scripts/cutover/rollback.sh` from the same worktree. Nobody chooses by
  hand which ref to reset: the rollback reads each remote's tip, reverts what
  was published, and brings every clone to where its remote then is. Then
  diagnose before trying again; not in the same sitting.
- **It exited with status 3**: CI has not finished. Run `cutover.sh` again.
- **It stopped in step 8**, as when the closing push fails: run `cutover.sh`
  again. Past step 7 the cutover has happened, and `rollback.sh` is not the
  answer to a later doubt; what breaks is fixed forward (atelier-2mlh).

The rollback, in order: every hooks directory back from its copy; for each
repository, nothing published means the conversion is reset away, and
published commits are each reverted, newest first, and the reverts pushed (a
remote holding anything else stops that repository, untouched); every clone's
base to where its remote now is and every worktree back to what it had
checked out; and, if the install had started, the v0.4.0 release
reinstalled from its archive with its SHA-256 checked, its `setup claude`
run where a repository uses it, the skill checkout moved back, and
`atelier-serve` enabled, started and checked to answer. It is safe to run
twice. Afterwards, the trials' "restored" holds: a base with nothing
published is at its recorded commit; one with something published holds one
revert per pushed commit, has the recorded tree, and agrees with its remote;
every worktree has what it had; every hooks directory is as copied; no file
the conversion wrote is left; the old `atelier check` passes; and the old
binary, skill and site are back.

## A worktree left on an old-layout branch

Bring the converted base into the branch, and take the base's `.atelier/`
whole; the branch's old records and reviews are not carried over:

```bash
cd <worktree>
git merge --no-commit <base>          # conflicts under .atelier/ are expected
git rm -r -q --cached .atelier
git checkout <base> -- .atelier
git clean -fdq -- .atelier
git commit --no-edit                  # after resolving any conflict outside .atelier/
atelier status
```

A branch with nothing worth keeping can instead be left, or its worktree
moved with `git switch --detach <base>`.

## Another host adopts

A host that still has the old binary, the old skill and old records, whose
remotes this host converted, adopts with the same script. It must not fetch
a converted repository before then.

1. Make a fresh clone of atelier's converted `master`, for the purpose:
   `git clone https://github.com/spadaval/atelier ~/atelier-adopt`. That is
   where the host gets the reviewed script, the new tool's source and the
   new skill, without moving any checkout it works in.
2. Run `~/atelier-adopt/scripts/cutover/cutover.sh preflight adopt`, with
   the settings for that host (below), and make it pass. Each base may be
   behind its remote, never ahead.
3. Run `~/atelier-adopt/scripts/cutover/cutover.sh adopt`.

It records what it will change, installs the new binary from that clone
(`cargo install --path crates/atelier-cli --locked`), installs the skill by
the command the settings give, stops and disables `atelier-serve` where the
host has it, brings each clone onto the converted base as step 6 does, and
accepts as step 7 does without the CI check, printing the count of items and
missions `atelier check` reads. It converts, pushes and closes nothing.
`~/atelier-adopt/scripts/cutover/rollback.sh` undoes it from its journal and
pushes nothing: hooks, refs and worktrees back to what was recorded, the
fallback reinstalled, the skill and the service as they were.

## Settings

Everything outside the scripts comes from `common.sh`'s settings block or
the environment. For another host, set at least `CUTOVER_PROJECTS` (or
`CUTOVER_REPOS`, `<name>:<clone>:<base>` each), and, where the skill is not
a checkout at `~/projects/atelier-skill`, `CUTOVER_SKILL_INSTALL` (the
command that installs it, run in the fresh clone) and `CUTOVER_SKILL_DIR`
(where it installs, copied aside first). `trials.sh` sets every one to a
sandbox; read it for the full list in use.

## Trying it again

`scripts/cutover/trials.sh <a directory under ~/.cache> <release build of
this tree>` builds a sandbox per trial from the five real repositories,
which it only reads, and prints one line per result. It takes about six
minutes. Its logs hold the private repositories' records: read them there
and delete the directory afterwards.
