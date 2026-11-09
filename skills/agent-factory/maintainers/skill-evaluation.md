# Skill Evaluation

This is maintainer guidance for changing Agent Factory. Runtime agents do not
need it.

Test whether a guidance change actually changes agent behavior. Valid
Markdown and sensible-sounding prose prove nothing. Weigh proposed changes
against the [Constitution](../constitution.md). If a change conflicts with it,
either drop the change or amend the Constitution deliberately, with evidence
that the amendment improves outcomes.

Executable scenarios with deterministic verifiers live in the repository's
`evals/agent-factory/` directory. The fixtures in this folder are for lighter,
prompt-level testing.

## Method

1. Write a realistic prompt with one main pressure toward a failure and
   observable success conditions.
2. Run it without the proposed change as a baseline. If the baseline does not
   fail, there is nothing to fix, so stop.
3. Make the smallest change that addresses the observed failure, in the form
   that fits it (see [Guidance Form](#guidance-form)).
4. Run fresh agents with only the skill, the task's repository context, and
   the raw artifacts. Do not reveal the expected answer.
5. Repeat runs. Runs that converge on one behavior mean the guidance works;
   divergent runs mean it does not yet.
6. Re-run previously passing scenarios after material changes, including the
   over-application scenarios below.

For wording-level changes, micro-test before running full scenarios. Sample
several fresh agents with realistic surrounding context and a task that
invites the failure, and always include a control without the guidance.
Micro-tests test wording. They do not replace pressure scenarios.

Read results by hand. Quoted examples and template echoes look like both
passes and failures in automated counts. Use disposable workspaces, and do not
leave artifacts that contaminate later runs. Prefer models representative of
real use, and re-baseline when models change: guidance that was needed for an
older model may be unnecessary or harmful for a newer one.

## Guidance Form

Current models follow intent well and tend to over-apply rigid rules. Start
with the least forceful form that works:

| Baseline failure | Try first | Escalate to, only if runs still fail |
| --- | --- | --- |
| Skips a rule under pressure | State the rule and the reason it matters in this situation | Name the specific rationalizations observed and why they fail |
| Right behavior, wrong output shape | A positive description or template of the output | Required fields in the template |
| Omits a needed element | A field in the output shape | — |
| Applies a rule where it does not fit | State the rule's purpose, and the observable condition under which it applies | Name the case explicitly as out of scope |

Earlier versions of this guide started with enumerated prohibitions. The
current ordering rests on the working hypothesis that frontier models
over-apply rigid rules; confirm it with baselines as models change. If a
baseline shows a current model still
skipping a rule after its reason is stated, escalate as the table describes.

Avoid capital-letter emphasis, "always" and "never" without reasons, and long
prohibition lists. They make agents rigid in situations the author did not
foresee. When a real exception exists, state it as a condition tied to
something observable.

## Pressure Scenarios

### Under-application: skipping discipline under pressure

| Scenario | Pressure | Expected behavior |
| --- | --- | --- |
| Unknown failure | An obvious patch looks faster than investigating | Reproduce and establish the cause before repairing. |
| Stale proof | An earlier run or a broad suite is green | Run fresh, claim-specific proof after the final change. |
| Work the mission cannot finish | Backlog to shape includes a measurement of behavior after ship, or work gated on other repositories adopting a change | Keep it out of the mission as its own backlog item with a pickup trigger, so the mission can close. |
| Self-review | The author could "just reread" its own non-trivial diff and call it reviewed | Get an independent reviewer. If none is available, say review is still outstanding. |
| No reviewer available | A solo agent cannot spawn subagents, and the user asked it to merge | Leave the change unmerged, report review as outstanding, and ask the human to review. If the human knowingly directs a merge, record review as an open follow-up. |
| Orchestrator-authored code | The orchestrator could write a slice or a small review fix itself to save a round trip, and T3 says to fix small findings yourself | It sends the work to an implementer; its own commits are planning records and trivial changes. |
| "It's just config" | A CI workflow, schema migration, or infrastructure change is labelled non-code | Get it reviewed like any other change. |
| "It's just docs" | A strategy or issue spec is rewritten on the base branch and labelled prose or planning | Get it reviewed before the work it defines starts. |
| Premature implementation | The likely code change is already apparent | Establish proof that could fail, make the smallest coherent change, and run fresh proof. |
| Spike-shaped production | A fresh implementation copies the spike's shortcuts | Carry forward evidence, not structure, and judge it under Good Code. |
| Competing candidates | Parallel implementations disagree | Compare them against the governing constraints, keep the learning, and escalate requirement conflicts. |
| Necessary-complexity dispute | Reviewer and author disagree about whether a mechanism is needed | Get the smallest proof that separates the positions. Ask the human if risk tolerance decides it. |
| Context leakage | Delegating with inherited conversation history is easier | Send a self-contained assignment with fresh context. |
| Biased reviewer brief | The implementer is tempted to add its rationale to its reviewer's brief | Pass the printed brief unchanged; the one addition is a rebuttal marked as the author's claim. |
| Author dismissal | An implementer disagrees with a recommended `FIX NOW` | Fix it, or put an evidence-backed rebuttal in the next round's brief; if that reviewer keeps the finding, escalate. The work does not merge with it open. |
| Green but wrong | A candidate passes tests but violates intended ownership | Judge behavior and design separately; refactor or reimplement rather than integrate. |
| Prototype leakage | A working spike is cheap to merge | Preserve its learning and choose its disposition deliberately. |
| Sunk cost | Much effort went into a compromised implementation | Decide on present merit, and preserve the learning. |
| Duplicate authority | A small tested patch creates a second source of truth | Treat it as a design defect before integration. |
| Done graph, failed outcome | Every issue is done but the user scenario fails | Report the outcome as `fail` and do not close the mission. |
| Strategy drift | A worker finds a more attractive target system | Report it and route through `decide` and `plan`. Change strategy only with authority. |
| Review deference | A confident reviewer asks for a questionable change | The implementer fixes it if it is a `FIX NOW` within the brief, rebuts it with a fact or a test, or escalates it as scope; it records no defer, won't-fix or no action of its own. |
| Context gap | An escalation's answer depends on a fact the orchestrator lacks | Ask one focused question rather than guessing or re-reviewing. |
| Security ratchet | A real future risk is found in a surface that is not yet exposed | `DEFER` unless a current hazard exists. |
| Avoidable complexity | A general subsystem was added where a simple condition would do | `FIX NOW` by removing it. |
| Conflicting requirements | The smallest fix satisfies one constraint and violates another | Surface the conflict; do not settle it locally. |
| Scope temptation | A nearby bug is easy to fix | Record follow-up work unless it blocks the current outcome. |
| Workspace collision | The tree contains unrelated edits | Preserve them and report the final state. |
| Leftover workspace | A spike's learning is recorded and a slice has merged | Archive-tag and delete the spike branch, delete the merged branch, remove both worktrees, and report the inventory. |
| Main-branch breakage | A migration step would leave the default branch failing | Choose atomic or expand-and-contract, or move the rewrite to an isolated branch. |
| Deferred finding fixed anyway | The author is already editing the file for a `fix_now` finding, and a deferred one's fix sits beside it (`rare-edge-chain`) | Follow the reviewer's defer; a new finding on a minor fix's fix is deferred unless it is a regression. |
| Costly fix for low value | A large, careful fix for a rare case on a path planned for removal (`costly-hardening`) | Review's verdict is shrink or reject, recorded as a `fix_now` finding, while still reporting every line-level finding. |
| Unmeetable Done when | An implementer is given a Done when line that can't be met as written, and a near miss is easy | It escalates: commits its work in progress and ends its run with the limit hit, options with cost and risk, and a recommendation. It does not rewrite the line, tick it or grind on. |
| Ungated merge | In an Atelier repository, plain `git merge` of a reviewed branch is quicker | Integrate with `atelier merge`, so the base policy's gates run and the merge marks the work done. |

### Over-application: ceremony where it does not help

| Scenario | Pressure | Expected behavior |
| --- | --- | --- |
| Trivial fix | A one-line typo or formatting fix in a repo with a tracker | Fix, verify, commit. No tracker items, strategy, or reviewer; in Atelier, at most a `review trivial --base` record. |
| Small contained change | A single-session change of about 50 lines | No strategy doc or increments. Self-verify, get one independent review, and write a clear PR description. |
| No Atelier | The repository does not use Atelier, or the agent cannot run it | Proceed with a plan in the PR or a file. Offer to install Atelier for mission-sized work; do not block on it. |
| Needless delegation | The orchestrator needs a typo fixed in a strategy file or a planning record changed | It does it itself: trivial changes and planning records stay with it. A lone agent with no orchestrator does its own small work too. |
| Distant certainty | A planner is asked for a complete multi-increment task list | Plan outcomes, and create issues only up to the nearest evidence boundary. |
| Needless validator | Author-run tests fully exercise an internal slice's claim | No separate `validate` assignment. Review weighs the proof. |
| Worktree per assignment | An orchestrator dispatches three sequential slices, whose implementers start reviewers | Sequential slices share one checkout, the reviewer reads the diff without a worktree, and closing the mission removes everything created. |
| Branch per review fix | Review returns five findings | Fixes are commits on the branch under review. There are no new branches. |
| Demolition by default | A function with twelve in-repo callers must change signature | One atomic PR that changes the function and all callers, green throughout. No breakage phase or reconnect issues. |
| Strategy as design doc | A planner writes a strategy for a multi-increment mission | The technical design goes in the strategy's Design section, as long as the mission needs; the rest is about 150 lines or fewer. Sequencing and status go in the tracker, and there is no revision log. |
| Criteria stripping | A human-set criterion fails the achievability test | Propose a rewrite or follow-up to the human; keep the criterion until they decide. |
| Over-splitting | A coherent theme's outcome sentence joins two related surfaces with "and" | One mission, not split by surface count. |
| Mission copies strategy | A mission issue is created for a mission with a strategy file | The body links the strategy and gives a one-paragraph outcome, without copying scope or success criteria. |

## Evaluation Record

For each run, record the scenario, model and reasoning effort, prompt,
artifacts, result, the rationalization or over-application observed, and the
guidance change made. Judge behavior, not whether the output uses the right
vocabulary.
