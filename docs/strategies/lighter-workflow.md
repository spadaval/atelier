---
status: draft
last_changed: "2026-11-06: created from Sumeet's direction (LoR thread 'Launcher usability vs Smart Launcher')"
---

# Why

Atelier was built when agents needed every safeguard, and the bookkeeping around its gates has outgrown what those gates protect. Between 2026-10-15 and 2026-11-06:

- **Record-only commits dominate history.** Most commits that weren't merges touched nothing but `.atelier/`:

  | Repository | Commits that weren't merges | Record-only |
  | --- | --- | --- |
  | atelier | 2017 | 1667 (83%) |
  | astromech | 2719 | 2115 (78%) |
  | demiurge | 1018 | 743 (73%) |
  | cosplay-studio | 201 | 138 (69%) |

- **Every finding becomes two files.** A finding and its disposition are separate immutable events: 577 of each in atelier, 1054 and 1069 in astromech.
- **Fixes trigger a fresh full approval.** Many approved targets were approved more than once: 42 of 155 in atelier, 70 of 195 in astromech.
- **Catch-up merges.** 143 of atelier's 200 merges brought the base into a branch.

The review findings themselves are real bugs and are worth keeping. The records around them are what costs.

Atelier also has no honest place for proof that only use can give, such as a real device, the deployed service or the human's judgment. Such a check is either forced in before the merge or split off as a separate validation issue. For example, cosplay-studio-ojg1 holds phone steps that its parent closed blocked.

# Outcome

Atelier records what proves work, at the time it can be proven, and nothing more:

- A review finding and what was done about it are one record.
- Checking a fix is the review of that fix, not a fresh approval of the whole branch.
- Recording proof and review adds as few commits of its own as the pinning of records to commits allows.
- An item's proof is split, when it is planned, into what must pass before merge and what can only be shown after. Merged work that is still awaiting validation is visible until it is validated.
- The workflow's statuses and issue types are the ones repositories actually use.

What this mission does not change: no non-trivial change reaches the base branch without an independent review of its code, and every gate that refuses today still refuses. Partial outcome worth shipping: findings that carry their disposition, together with fix re-checks.

# Success Criteria

- lifecycle-cost: A scenario test drives one standard lifecycle in a fixture repository:
  - start an issue;
  - record evidence;
  - close it;
  - a review with three findings, one fixed, one deferred and one no-action;
  - the fix checked;
  - merge.

  The test counts the record-only commits and review files it produces. The same scenario run on master as it stands at this mission's start gives the baseline, recorded as evidence by the first increment. At close, review files per finding are 1, and record-only commits are at most half the baseline.
- proof-timing: A scenario test covers an item with one criterion checked before merge and one after:
  - merge refuses while the before-merge criterion lacks evidence;
  - a criterion marked for after merge without a reason is refused;
  - the merged item shows as awaiting validation in `status` and `work` views;
  - validation recorded on the base completes it;
  - a mission with open validation does not close unless that validation is explicitly carried to another item.
- gates-hold: The existing gate tests in `atelier merge` and `land` still pass, except where an ADR from this mission replaces the rule they test. Each such change is listed in the debrief.

# Shape

- **Review stays before merge.** The reviewer reads the branch's diff against the base. Reviewing after merge, on the base, was considered and rejected: the diff is harder to isolate there, and it would let unreviewed code onto the base.
- **One record per finding.** A finding carries its disposition (`fix_now` with a resolution commit, `defer` with a follow-up issue, or `no_action` with a reason) as fields that change, not as a separate event. Who set each disposition, and at which commit, stays visible. The non-author rules for `defer` and `no_action` stay.
- **A fix's re-check counts as its review.** A reviewer who confirms a `fix_now` disposition at the resolution commit has reviewed that fix, which satisfies Agent Factory's rule that changes after review get reviewed. A fresh approval of the whole head is needed only for commits that no finding's resolution covers, or for a finding the reviewer marks as needing re-review.
- **Proof has a time.** "Done when" names, for each criterion, whether it is checked before merge (the default) or after merge.
  - A criterion checked after merge must give its reason: it needs the deployed or merged system, a device or place no agent can reach, the human's judgment, or time and real use.
  - Its nearest stand-in still runs before merge (an emulator for a device, a test server for production). The after-merge check covers only the gap between the stand-in and the real thing.
  - The split is fixed when the work is planned. Moving a criterion to after merge later is a recorded plan change that the reviewer sees, in line with atelier-ll3p's showing of a Done when edited after the item was started.
  - Validation is recorded on the base as a planning change, needs no reviewer identity, and a failure files a follow-up bug rather than reopening the merged claim.
  - Agent-facing text says it plainly: post-merge validation is only what cannot be checked before the merge.
- **Fewer records, then fewer commits.** Record-only commits are cut first by needing fewer records. What remains is cut by letting records ride with the commit they describe, or batching them, wherever pinning a record to a commit allows.
- **Statuses and types follow use.** Each status and issue type is kept only if records across these repositories use it, or a gate needs it. A post-merge state replaces whatever it makes redundant rather than being added on top.
- **Existing records keep working.** Each record change ships with an `atelier migrate` step, as the epics change did.
- **Simplicity wins.** When a rule can be kept only by adding a new rule about when it applies, prefer removing it, unless removing it reopens a shortcut that atelier-ll3p documents.

# Scope

In:
- Atelier's records, workflow policy, review, merge, land and status;
- the product docs and ADRs those change;
- migrations for existing records;
- the Agent Factory text that describes these flows (`~/projects/agents-skills`, the source of `~/.agents/skills/agent-factory`, per ADR 0023).

Out:
- Adopting Atelier in other repositories, including life-on-rails, which waits on this mission. Re-measuring them after release is a backlog item to pick up a few weeks after release.
- atelier-ll3p's own work. It is complementary; where its showing of shortcuts would cover a rule this mission removes, this mission keeps the showing.
- External review backends.

Environment: Atelier's CLI on Linux and macOS, as released.

# Authority

The orchestrator may change the increments, their order and their issues, and make choices within this Shape: record field names, migration form, which statuses and types are dropped based on the counts. It records each choice under its item.

These come back to Sumeet:
- any change that lets a non-trivial change reach the base branch without independent review of its code;
- dropping a gate that refuses today, beyond the ADRs this mission writes for the rules above;
- the success criteria;
- Agent Factory's Core Rules or its Independent Review section, beyond wording the rules above require.

# Risks and Unknowns

- **Pinning may need record-only commits.** A review event or receipt pins a commit and so cannot be part of that commit, which puts a floor under record-only commits. The first increment measures how much of the baseline is that floor.
- **Fewer records may reopen shortcuts.** atelier-ll3p found agents getting past gates in form rather than substance. Each removal is checked against its findings.
- **Re-checks may be too lenient.** If scoping re-review to the fix lets problems through, the re-review mark becomes the default. This is judged from the review findings during the mission.
