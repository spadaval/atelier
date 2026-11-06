---
status: draft
last_changed: "2026-11-06: on Sumeet's direction, dropped the fix re-check cut (atelier-8s1m), turned the after-merge refusals into things shown, and added evidence that records only what nothing else records (after an evidence audit); earlier that day, planning review fixes; created that day from Sumeet's direction (LoR thread 'Launcher usability vs Smart Launcher')"
---

# Why

Atelier was built when agents needed every safeguard, and the bookkeeping around its gates has outgrown what those gates protect. Between 2026-10-15 and 2026-11-06:

- **Record-only commits dominate history.** Most commits that weren't merges touched nothing but `.atelier/`. These counts include planning commits (filing and grooming issues), which this mission does not aim to reduce; the lifecycle scenario below measures only the bookkeeping of doing one item.

  | Repository | Commits that weren't merges | Record-only |
  | --- | --- | --- |
  | atelier | 2017 | 1667 (83%) |
  | astromech | 2719 | 2115 (78%) |
  | demiurge | 1018 | 743 (73%) |
  | cosplay-studio | 201 | 138 (69%) |

- **Every finding becomes two files.** A finding and its disposition are separate immutable events: 577 of each in atelier, 1054 and 1069 in astromech.
- **Most evidence repeats what CI already shows.** An audit of the 1,725 receipts recorded in these repositories since 2026-10-15 judged about a quarter of a 72-receipt sample useful and more than half busywork. The busywork is mostly whole-suite, fmt and diff-check runs that CI or the merge checks already gate, re-recorded after review fixes and base merges: base merges alone caused 47 re-records in atelier and 90 in astromech, of which 8 were needed. No policy requires a passing receipt to close or merge any more; agents record these because merge errors, `status` and the docs tell them to. What is worth keeping: checks nothing else records (a browser at phone width, a real build host, a live service), failures, measurements, and mission criteria receipts.

The review findings themselves are real bugs and are worth keeping, and so is re-reviewing a fix: a reviewer reading the fix's diff and approving the head again is cheap. The records around review and proof are what costs.

Atelier also has no honest place for proof that only use can give, such as a real device, the deployed service or the human's judgment. Such a check is either forced in before the merge or split off as a separate validation issue. For example, cosplay-studio-ojg1 holds phone steps that its parent closed blocked.

# Outcome

Atelier records what proves work, at the time it can be proven, and nothing more:

- A review finding and what was done about it are one record.
- Evidence records only what nothing else records.
- Recording proof and review adds as few commits of its own as the pinning of records to commits allows.
- An item's proof is split, when it is planned, into what is checked before merge and what can only be shown after. Merged work that is still awaiting validation is visible until it is validated.
- The workflow's statuses and issue types are the ones repositories actually use.

What this mission does not change: no non-trivial change reaches the base branch without an independent review of its code, and a fix made after review is reviewed again before merge, as today. Every gate that refuses today still refuses, except the rules this mission removes as described in Shape: the separate disposition event, the rule that a pass stops clearing an older failure once later commits make it stale, and `merge --sync` rerunning receipts. The mission adds no new refusals; what it adds is shown, not enforced. Partial outcome worth shipping: findings that carry their disposition, and the evidence changes.

In this strategy and the mission's items, a **Done when check** is one line of an item's Done when. It is not a mission success criterion and gets no slug.

# Success Criteria

- lifecycle-cost: In the scenario test below, review files per finding are 1 and record-only commits are at most half of atelier-5fus's baseline, with reductions coming from product changes rather than from changing how the scenario commits. The test drives one standard lifecycle in a fixture repository that uses the `atelier init` starter policy. Commands that accept `--commit-records` use it, as the review request brief instructs; `atelier start` and `issue create` are each followed by one commit of exactly the records they wrote:
  - start an issue, record one manual evidence receipt, close it;
  - one review, by an identity other than the author, that raises three findings, one of them `fix_now`, and does not approve;
  - a fix commit;
  - the author's dispositions: `fix_now` with that resolution commit, `defer` with a follow-up issue, `no_action` with a reason, the last two accepted by the reviewer;
  - the reviewer reviews the fix and approves the head;
  - merge.

  It counts record-only commits (commits on the work branch that change only files under `.atelier/`) and, for each finding, the review files about it (the finding plus any event that names it). The baseline is the test's result before any product change, recorded as evidence by atelier-5fus.
- proof-timing: A scenario test of an item with one Done when check before merge and one after shows the after-merge path, without any new refusal:
  - the review request brief lists the after-merge check with its reason and its before-merge stand-in, and says when a reason is missing;
  - the merged item shows as awaiting validation in `status` and `work` views;
  - validation recorded on the base clears it, and `land` accepts that;
  - closing a mission whose validation is still open warns and names it, unless that validation is carried to another item.
- gates-hold: The existing gate tests in `atelier merge` and `land` still pass, except where this mission removes the rule they test, and the debrief lists each such change.

# Shape

- **Review stays before merge.** The reviewer reads the branch's diff against the base. Reviewing after merge, on the base, was considered and rejected: the diff is harder to isolate there, and it would let unreviewed code onto the base.
- **Fixes are re-reviewed as today.** After a `fix_now`, a reviewer (the same one is fine) reviews the fix's diff and approves the head. Replacing that approval with a scoped confirmation was considered and dropped (atelier-8s1m): it saves one record per fix round and needs new rules to stay safe. The docs say plainly that the re-review may cover only the fix's diff.
- **One record per finding.** A finding carries its disposition (`fix_now` with a resolution commit, `defer` with a follow-up issue, or `no_action` with a reason) as fields that change, not as a separate event. This departs from append-only review events (ADR 0019, `PRODUCT_INTENT.md`, `CONTEXT.md`), and the ADR for it says so. Who set each disposition, and at which commit, stays visible. The non-author rules for `defer` and `no_action` stay. Recording a disposition does not make an approval stale.
- **Evidence records only what nothing else records.** A receipt is for a check that leaves no other record: a manual, browser, device or deployed check, a mission criterion, a measurement, or a failure (including a test that fails before its fix). Suites, fmt and lint that CI or the merge checks run need no receipt; the close reason names the targeted test that proves the claim. Repositories without CI put their suite in the policy's merge checks. To get there:
  - merge and land errors, the docs and Agent Factory's text stop telling agents to record fresh proof, and `status` and `work` views stop treating an item with no receipt as a proof gap;
  - `merge --sync` stops rerunning receipts, since CI or the merge checks test the merged tree;
  - a later passing receipt clears an older failure for good, rather than only while it is fresh;
  - receipts, failure output, criteria receipts and their marks, and the newer-failure block stay. So does Agent Factory's rule to prove claims with fresh evidence: a check is still run after the last change and its output read; what changes is only whether it is recorded as a receipt.
- **Proof has a time.** Each Done when check is checked before merge (the default) or after merge.
  - A check after merge gives its reason: it needs the deployed or merged system, a device or place no agent can reach, the human's judgment, or time and real use.
  - Its nearest stand-in still runs before merge (an emulator for a device, a test server for production). The after-merge check covers only the gap between the stand-in and the real thing.
  - The split is fixed when the work is planned. Moving a check to after merge later is a plan change that the reviewer sees, through atelier-ll3p's showing of a Done when edited after the item was started (atelier-r9hb).
  - The review request brief lists each after-merge check with its reason and stand-in. Agent Factory's review text tells the reviewer to flag any that could have run before merge, as a finding.
  - Validation is recorded on the base as a planning change, needs no reviewer identity, and a failure files a follow-up bug rather than reopening the merged claim.
  - Agent-facing text says it plainly: post-merge validation is only what cannot be checked before the merge. Everything that can be checked on the branch is checked there.
- **Show, don't enforce.** What this mission adds is shown where work is judged (the review brief, `status`, `work` views, the mission debrief) and judged by the reviewer, not refused by a command.
- **Fewer records, then fewer commits.** Record-only commits are cut first by needing fewer records. What remains is cut by letting records ride with the commit they describe, or batching them, wherever pinning a record to a commit allows.
- **Statuses and types follow use.** Each status and issue type is kept only if records across these repositories use it, or a gate needs it. They are counted (atelier-rkxm) before the after-merge model is designed. A post-merge state replaces whatever it makes redundant rather than being added on top.
- **Existing records keep working.** Each record change ships with an `atelier migrate` step, as the epics change did.
- **Simplicity wins within this Shape.** When one of the rules above can be kept only by adding a new rule about when it applies, prefer removing it, unless removing it reopens a shortcut that atelier-ll3p documents. This does not extend to rules outside this Shape or to anything the Authority section returns to Sumeet.

# Scope

In:
- Atelier's records, evidence, workflow policy, review, merge, land and status;
- the product docs and ADRs those change;
- migrations for existing records;
- the Agent Factory text that describes these flows (`~/projects/agents-skills`, the source of `~/.agents/skills/agent-factory`, per ADR 0023).

Out:
- Adopting Atelier in other repositories, including life-on-rails, which waits on this mission. Re-measuring them after release is atelier-akkp, outside the mission.
- Adding merge checks to repositories without CI. demiurge and cosplay-studio have neither CI nor `merge_checks`, so once `merge --sync` stops rerunning receipts nothing tests their merged tree until they add a suite merge check (atelier-bp3s, outside the mission). The ADR for the evidence change says so.
- atelier-ll3p's own work. It is complementary; where its showing of shortcuts would cover a rule this mission removes, this mission keeps the showing.
- Catch-up merges of the base into a branch (143 of atelier's 200 merges). They come from keeping a branch in sync with the base for CI and conflicts, not from record bookkeeping, so this mission does not target them.
- External review backends.

Environment: Atelier's CLI on Linux and macOS, as released.

# Authority

The orchestrator may change the increments, their order and their issues, and make choices within this Shape: record field names, migration form, which statuses and types are dropped based on the counts. It records each choice under its item.

These come back to Sumeet:
- any change that lets a non-trivial change reach the base branch without independent review of its code, including a fix made after review;
- dropping a gate that refuses today, beyond the rules this Shape removes;
- adding a refusal;
- the success criteria;
- Agent Factory's Core Rules or its Independent Review section, beyond wording the rules above require.

# Risks and Unknowns

- **Pinning may need record-only commits.** A review event or receipt pins a commit and so cannot be part of that commit, which puts a floor under record-only commits. Keeping the fresh approval after a fix (atelier-8s1m dropped) removes one source of savings, so the cut now rests mostly on atelier-5822. atelier-5fus measures how much of the baseline is that floor, and says whether "at most half" is reachable.
- **Repositories without CI lose their merged-tree check.** Accepted by Sumeet on 2026-11-06 and tracked in atelier-bp3s.
- **Most of the evidence saving is behavior, not code.** Agents recorded suite receipts because the text told them to; the scenario cannot show that change. It shows in real repositories, which atelier-akkp re-measures after release.
- **Fewer records may reopen shortcuts.** atelier-ll3p found agents getting past gates in form rather than substance. Each removal is checked against its findings. Several ll3p items depend on the shapes this mission changes: open fix-now findings in readiness (atelier-efnf) and review history (atelier-ma4a). Whichever mission lands second adapts to the first. atelier-n0qh waits on atelier-r9hb, because a check moved after merge is a shortcut nothing shows until r9hb ships. If atelier-ll3p has not scheduled r9hb when n0qh is next, the orchestrator may do r9hb as this mission's work, leaving it in ll3p, rather than stall.
