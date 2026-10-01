---
status: active
mission: none yet; tracked in the Plan section below until S8
last_changed: "2026-09-30: initial strategy (human-directed rebuild)"
---

# Why

Agent Factory now owns process: independent review, fresh proof, proportional
ceremony, branch hygiene, and record templates. Atelier was built when it had
to own that process itself, and it grew a workflow engine, transition actions,
a review provider, a SQLite cache, and per-transition activity files. That
machinery produces tracker bookkeeping in most commits, still let unreviewed
code merge to `master`, and checks that evidence exists rather than that it is
true. The repo-native record model remains valuable; the machinery around it
does not.

# Outcome

Atelier is a repo-native tracker and the orchestrator's memory. It keeps
missions, epics, issues, evidence, and reviews as Markdown versioned with the
code they describe, makes the Agent Factory process the path of least
resistance, and mechanically guards a small set of invariants at the point of
action. Everything else stays out of the way. See
[ADR 0019](../adr/0019-verified-repo-native-tracker.md).

A partial outcome is still worth shipping: the deletions alone (S5, S4) remove
most of the operational cost even before the new guards land.

# Success Criteria

- S1: `atelier merge` refuses a non-trivial change that lacks a current
  approval from an identity other than the authors, or that has an unresolved
  `FIX NOW` finding or an open blocker. Shown by CLI integration tests.
- S2: Merging writes no tracker state. The base branch never shows an item
  done without its code, and a merged checkout stays clean. Shown by a
  scenario test that closes an epic on its branch and merges it.
- S3: Evidence records the commit it ran against and an explicit result.
  Evidence older than the item's last relevant change does not satisfy a
  check, and a newer `fail` blocks closing. Shown by tests.
- S4: No SQLite cache. Read commands complete in under 100 ms (release build)
  on a 1,000-record fixture. Shown by a benchmark.
- S5: Forgejo, review rooms, transition actions, the SQLite cache, activity
  files, Beads import, export, telemetry, `doctor`, `man`, and the hidden
  `branch` commands are gone. Non-test Rust is under 20k lines. Shown by
  `wc -l` and the command list.
- S6: `atelier prime` gives a fresh session its context; `atelier land`
  refuses while record changes are uncommitted or invariants are unmet;
  `atelier setup <harness>` installs the hooks. Shown by tests and one
  dogfooded session.
- S7: Records follow Agent Factory's tracker-records shapes (Goal / Done when /
  Scope / Context; epic Outcome / Proof; mission success criteria). Shown by
  lint and fixtures.
- S8: This repository tracks the tail of this mission in the new model.

# Shape

- Agent Factory owns process; Atelier owns records, queries, and invariant
  guards (ADR 0019 supersedes ADR 0006).
- Guard invariants, not choreography. Every structural rule must trace to an
  invariant: record integrity, nothing done while dependencies are open,
  traceable active work, fresh honest evidence, independent review before
  merge, explicit trivial exemption, finding dispositions, branch cleanup.
- Transitions are a record write plus read-only checks. Side effects live in
  two explicit commands: `atelier start` (branch) and `atelier merge` (gated).
- Done claims travel with the code: the claim is the last commit on the work
  branch, reviewed with the code. Merge-time facts are derived from Git.
- Agents commit records alongside code. Atelier never creates commits.
- Policy is code: gates are evaluated from the base branch's
  `workflow.yaml`, and policy changes are reviewed like any other change.
- Reviews are in-repo, append-only records. No external review provider.
- Records are parsed on every command; no cache.
- Self-declared identity is sufficient. We guard against tired agents, not
  malicious ones.
- Sane state wins over convenience: commands validate before writing and
  never leave partial state.

# Scope

In: the `atelier` CLI and crates, this repository's `.atelier/` state and
policy, product and architecture docs, ADRs, and the in-repo Agent Factory
skill copy.

Out: an external review provider or viewer (possible later), a daemon or
agent runner, other repositories that use Atelier (`malorian`,
`cosplay-studio`; migrate when next used), and the global Agent Factory skill.

Environment: local developer machines; the installed `atelier` binary is
updated only when the new model is stable.

# Authority

The orchestrator may change the epic breakdown, sequencing, internal design,
command names, record field names, and this repository's `workflow.yaml`
through reviewed changes. Changing the success criteria or the Shape above,
adding storage other than in-repo Markdown, or adding an external provider or
daemon needs the human.

# Risks and Unknowns

- Atelier tracks its own work while being rebuilt. Until S8, track the mission
  in this file and in commit history; fall back to it if the tracker breaks.
- Removing the cache touches every read path. Do it after the deletions shrink
  the code.
- Approval staleness needs a precise rule for which post-approval commits are
  tracker-only. Settle it with tests before `atelier merge` relies on it.
- Branch-per-epic means the base branch lags in-flight work; cross-branch
  status reads are needed for orientation.

# Plan

Tracked here until S8 (this section is the temporary tracker, not strategy).
Work happens on branch `mission/atelier-rebuild`; every code commit has had
an independent review.

1. [done] Simplify this repository's `workflow.yaml`; archive terminal
   records and activity files.
2. [done] Delete Forgejo, review rooms, PR and review commands.
3. [done] Delete transition actions and most validators; add `start`.
4. [done] Replace the SQLite cache with direct parsing (S4 met: medians
   28–42 ms, max under 52 ms, release build, 32 cores).
5. [done] Delete leftovers: import, export, telemetry, `doctor`, `rules`,
   `man`, hidden `branch` commands, `history`.
6. Reshape records and evidence (S3, S7). Absorb open issue `atelier-a0h0`
   (generic `fields` map).
7. In-repo review records, gates as graph nodes, `atelier merge` (S1, S2).
8. Policy from the base branch; shell checks.
9. `prime`, `land`, `setup` (S6); cross-branch status.
10. Rewrite product docs and `PRODUCT_INTENT.md`; dogfood (S8).

Deferred review findings to resolve in later steps:

- Read-modify-write under a shared lock in `issue update`, link/unlink, and
  transitions (concurrent writers can lose changes).
- `start` treats a remote-only branch as new and branches from HEAD.
- `start` writes owner and issue records separately after switching.
- `work.terminal` passes for an empty mission; no gate uses it yet (step 7).
- `close_reason` may be required on a non-done transition and is then
  dropped; reject in policy validation.
- `check` reports only the first graph error when several exist.
- Leftover ignored files from the old model (`.atelier/state.db`,
  `.atelier/cache/`, `.atelier/rules/`, `.activity/` directories, `[review]`
  config) now fail commands; needed for the one-time migration of other repos.
- Git-cleanup failures in `prune` still exit 0; prune gives no branch
  recovery hint.
- Recovery hint missing on the evidence-capture load path.
- `nearest_parent_epic` has branches made unreachable by load-time checks.
- Test hooks (`ATELIER_TEST_*` env vars) ship in the production binary.
- Docs still describing removed behavior: `workflow-configuration.md` review
  sections, `work-model.md`, `cli-surface.md`, command-audit pages, quality
  docs (`check --fix`), `CONTEXT.md`, `PRODUCT_INTENT.md` (step 10).
