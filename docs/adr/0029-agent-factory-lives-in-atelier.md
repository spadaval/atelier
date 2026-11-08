# ADR 0029: Agent Factory Lives in This Repository

## Status

Accepted (2026-11-08, human-directed: Sumeet decided to move the skill into
this repository; mission atelier-g6rp, docs/strategies/agent-factory-in-atelier.md).

Supersedes [ADR 0023](0023-agent-factory-is-not-vendored.md). Supersedes
[ADR 0020](0020-agent-factory-targets-atelier-only.md) Decision 3's rule that
a command change updates the upstream skill in the same piece of work, and the
plan-file fallback in its Decision 1. The rest of ADR 0020 stands.

The decisions take effect as mission atelier-g6rp's items merge. Until the
skill moves (atelier-dkul), its one copy stays in `spadaval/.agents` and ADR
0023's consequences describe where to edit it.

## Context

Agent Factory and Atelier are one product split across two repositories. The
skill names Atelier more than fifty times, ADR 0020 made Atelier its only
tracker, and ADR 0020 Decision 3 exists only to keep the two repositories in
step. In practice a command change and the guidance that describes it land in
separate commits, reviews and CI runs, and the guidance drifts until someone
ports it.

The skill is also installed globally, and its description matches most
multi-step software work, so agents load it in repositories that do not use
Atelier at all.

ADR 0023 removed a vendored second copy of the skill from this repository
because copies diverged. Its reasoning was about the number of copies, not
their location.

## Decision

1. **The skill's one copy is `skills/agent-factory` in this repository**, and
   its evaluations are `evals/agent-factory`. `spadaval/.agents` no longer
   holds it. There is still exactly one git-tracked copy, so ADR 0023's reason
   holds.

2. **It installs with Vercel's `skills` CLI:**
   `npx skills add spadaval/atelier --skill agent-factory -g`, unpinned, so it
   follows the default branch. The skill's first lines give that line and the
   binary's release installer.

3. **The binary carries no copy of the skill and makes no version promise
   about it.** Instead `atelier doctor` warns, and never fails, when the binary
   is older than the latest release or the installed skill is behind, and
   prints the command that updates it. `atelier status` shows the same warning
   from a cached result.

4. **`atelier setup --link` links the installed skill to a checkout** of this
   repository for editing it, in a form `skills update` leaves alone.

5. **The skill steps aside outside Atelier.** Its first step is
   `atelier status`. In a repository without Atelier, or without `atelier`
   installed, it stops and offers its install procedure only when the user
   asked to set up Atelier. This replaces ADR 0020 Decision 1's plan-file
   fallback.

6. **Process and records stay separate, by directory.** The skill says what to
   do and why; the CLI's help owns syntax, as ADR 0020 Decision 3 put it, and
   nothing under `crates/` reads the skill's files. Because both now live
   here, a command change that invalidates the skill updates it in the same
   commit as a matter of course, and Decision 3's cross-repository sync rule is
   retired.

## Consequences

- One pull request, review and CI run cover a command and its guidance.
- Documentation links to the skill point into this repository once it moves.
  Closed records, review events and superseded strategies keep the paths they
  were written against.
- A stale install is reported, not prevented. Someone running an old binary
  with a new skill, or the reverse, sees a warning and the fix.
- Installing the skill needs Node (`npx`). Checking its freshness does too;
  without it, `doctor` says so and prints the install line.
- `spadaval/.agents` is left with no skills; whether to retire it is a
  separate decision (atelier-rfxf).

## Alternatives Considered

- **Keep `spadaval/.agents` as the source.** Rejected: it keeps a command and
  its guidance in separate reviews, which is the drift this ADR removes.
- **Ship the skill inside the binary, or install it with `atelier setup`.**
  ADR 0020 deferred this until version skew appeared. Rejected now: it locks
  skill and binary versions together and makes every skill edit a release,
  while a freshness warning is enough for an alpha project.
- **A custom installer or updater.** Rejected: the `skills` CLI already
  installs from GitHub into every harness's skill directory and checks for
  updates.
- **Vendor a copy here and keep the source elsewhere.** Rejected again, for
  ADR 0023's reasons.
