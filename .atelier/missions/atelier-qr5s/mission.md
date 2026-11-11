---
schema: atelier.mission
schema_version: 3
id: atelier-qr5s
title: Move all five repositories and Agent Factory onto the new loop, one cutover per host
status: done
labels:
- redesign
lead: thread:delegated-task:command%3Amcp%3A4dfc2b56-ef24-49e1-a2d6-030165a03e35%3Adelegate-task%3A8c89f937-6fe8-496a-8f35-8d051ef99e7f
ready:
  by: conversion
  at: 2026-11-11T15:17:33.448605463Z
created_at: 2026-11-10T00:56:04.626625678Z
---

## Why

Mission 2 leaves a second binary on master that nothing uses. The installed
binary and the installed Agent Factory skill are each one per host, so every
repository on a host changes process together. This mission makes that change
once, with a rehearsal behind it and a way back.

The skill has to change with the tool. It names commands that will no longer
exist, and its own evals have never shown it beating no skill: the one
comparison found no gain and was contaminated, and no clean no-skill arm has
run since.

## Outcome

On this host, atelier, astromech, demiurge, life-on-rails and cosplay-studio
run on the new records and the new binary. Agent Factory is rewritten against
the new tool, and each procedure it keeps has an eval scenario it changed.
The old workspace is deleted, and the site with it. The new binary has no `serve`. The old docs and the ADRs are replaced by the
design record. Sumeet has a runbook to convert his work host.

## Success criteria

- five-converted: with the new binary installed, `atelier check` passes in
  all five repositories, and one item has gone from ready to done under the
  new binary in atelier and in one product repository.
- no-old-surface: a search for each removed command and record path finds
  no instruction to use it in the skill, in the five repositories'
  `AGENTS.md` and `CLAUDE.md`, or under `docs/`. Naming one as history is
  allowed: in the design record's list of what was tried and removed, and in
  the baseline's reports.
- skill-earns-its-place: a results file records skill against no skill, on
  Claude Opus and on one Codex model, at least three runs per cell. Every
  procedure the skill keeps names a scenario where it changed the outcome.

## Scope

- In: the skill, its evals, the backlog sort, the fallback release, taking
  the site out, the switch, the cutover on this host, first use, and the
  docs.
- Out: the work host, which Sumeet converts with the runbook; the gardener,
  groom and audit procedures, which mission 4 rewrites and which are carried
  over unchanged except for command names; the Desk, which is mission
  atelier-z4cr; new scenarios
  beyond those needed to judge the kept procedures.
- Target environment: this host; Claude Opus and a Codex model for the evals.

## Authority

The orchestrator plans each step at its boundary. It may cut any skill
procedure no scenario supports, drop the backlog items the sort finds
obsolete, and fix what first use breaks inside the specifications.

Sumeet names the time of the cutover. The orchestrator takes the switch as
far as its approval and stops there, with everything the cutover will run
reviewed and the rollback tried. Once Sumeet has named the time, and no work
is in flight in any of the five repositories and no other thread is running
in them, it merges the switch and runs the runbook, which pushes the
conversion commits directly.

It may not push anything else directly, weaken the independent review rule
or the three exits of a fix-now finding, change the specifications' Words,
Lifecycle or Review sections, or touch the work host.

**Scope rule.** As in mission 2: a finding that asks for behaviour the
specifications do not name is escalated, not fixed in place.

## Result

All five repositories on this host run the new records and the new binary, as of the cutover on 2026-11-11, and the three criteria hold.

**The cutover.** Sumeet named the time on 2026-11-11, about 11:40 UTC. The switch (atelier-u1ko, pull request 135) merged as b54e6f1e and `scripts/cutover/cutover.sh` ran to its end in about two minutes, with no rollback. Its first run refused to start: the preflight found the lead's own stale build directory and the merged switch's leftover worktree; nothing had been recorded, both were removed, and it was run again. Its report:

| Repository | Conversion commit | Items, missions | `--verify` | Relative links resolving |
| --- | --- | --- | --- | --- |
| atelier | 7f499285 | 133, 7 | matches, 152 files | 434 of 452 |
| astromech | 775b8544 | 240, 4 | matches, 252 files | 254 of 272 |
| demiurge | a98590be | 19, 1 | matches, 23 files | 6 of 28 |
| life-on-rails | 59088d41 | 47, 3 | matches, 57 files | 136 of 136 |
| cosplay-studio | 5a1dd41d | 37, 7 | matches, 53 files | 161 of 190 |

Step 7 passed before anything was closed: `check` and `status` in every main and base worktree, each base at its remote's tip, `atelier --version` 0.5.0, the new skill in place, `atelier-serve` stopped and disabled, and CI run 43047341532 green on atelier's conversion commit with the tree's binary. atelier-wd1i was then closed by the new tool (0916291). Each repository's main checkout was left on its converted base; astromech's is detached there, since `main` is checked out in another worktree. Worktrees on old-layout branches were left as they were.

**five-converted.** With the binary installed from master 78f7b3dd, `atelier check` passes in all five repositories (133, 240, 20, 47 and 37 items). One item went from ready to done under the new binary in atelier and in a product repository, each with no refused or errored `atelier` call:

- atelier-2mlh, in atelier: `start`, `submit` (which pushed the branch and opened pull request 141), a review by another thread that approved in round 1, `merge` (which waited for CI, ran the three merge checks, merged as 9233f75 and pushed `master`). This was the new binary's first push, pull request, CI wait and merge on real GitHub. CI's `check` step no longer has a fallback branch.
- worldcrafter-s9lp, in demiurge, which has no forge: `start`, `submit`, an approving review in round 1, `merge` (13b07f3), then `git push` of the base.

**no-old-surface.** A search for each removed command and record path finds no instruction to use one in `skills/`, in the five repositories' `AGENTS.md` and `CLAUDE.md`, or under `docs/`. The hits that remain are history where the criterion allows it: the design record's account of what was removed (`docs/design.md`) and the November baseline's reports. One sentence of astromech's `AGENTS.md` uses the words "an Atelier issue" as a noun, outside its process section; it names no command.

**skill-earns-its-place.** `evals/agent-factory/results/2026-11-11-next-skill.md` records 96 runs of ten scenarios, skill against no skill, on Claude Opus 5.5 and `gpt-6.1-sol`, three runs per cell. The skill changed the outcome by two runs of three in three scenarios, all on `gpt-6.1-sol`, and did harm in none. What it keeps beyond its entry text and the fixed Review rules is the reviewer's Size and Verdict (next-costly-hardening) and the lead's "you write no code" (next-settle-within-authority); everything else no scenario supported was cut. It is 319 agent-facing lines against the old skill's 1,223.

**What was built.** The new skill (atelier-orb2) and its evals (atelier-a4rs, atelier-t747); the backlog sort (atelier-5zre: 28 records dropped, the rest converted as they were); the fallback release, 0.4.0 (atelier-mlwx); the site out of the specifications (atelier-oci2); Sumeet's six answers of 2026-11-11 (atelier-1oel, atelier-yudj); the Desk's tracker change (atelier-psjw); the switch and the runbook with its rollback and `adopt` (atelier-u1ko); first use (atelier-2mlh, atelier-ivbq); and the docs (atelier-6w8d: `docs/product.md` and `docs/design.md` replace the old docs and 31 ADRs). Non-test Rust under `crates/` is 14,779 lines, the Desk's crate included, against the old workspace's 48,988.

**What first use showed.** A thread T3 started as a child, given only the tool's task line, did not start its own reviewer, because T3 tells a child to start no agents unless its task says so; atelier-ivbq reworded the handoff line to say that starting the other thread is the reader's own job. atelier-6w8d was merged with `ATELIER_SKIP` past a failing Desk CI job, by its implementer, with the reason in the merge commit: GitHub's runners could not reach Ubuntu's package mirrors that day, and the same step passed locally.

**The release.** `v0.5.0` is tagged on 09162916, the commit that closed the cutover. Its release built on the seventh attempt, once the same runner outage had passed (the two musl builds had failed installing `musl-tools`), and has the installer and four archives; the x86_64 archive's checksum matches and its binary reports 0.5.0. The tag predates atelier-2mlh, atelier-ivbq and atelier-6w8d, so the installed binary on this host, built from master 78f7b3dd, is newer than the release.

**Left open.** The work host adopts with `scripts/cutover/cutover.sh adopt`, after which `scripts/cutover/` is deleted. Three questions are Sumeet's and unanswered: whether a lead may `start` an item of its own mission, whether `check` should accept a status set to `done` by hand, and whether the product repositories should get a forge. Follow-ups filed: atelier-dytk, atelier-i70l, atelier-o2vv.
