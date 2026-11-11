---
created_at: "2026-11-10T00:56:09.122058912+00:00"
id: "atelier-orb2"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-orb2
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T04:29:18.277865487+00:00"
status: "done"
title: "Write the new Agent Factory beside the live one, one file per role"
updated_at: "2026-11-11T04:29:18.277865487+00:00"
---

## Goal

Write the candidate new Agent Factory in `skills/agent-factory-next/`, against the commands of the new binary in `next/`. It says how to decide. It leaves what to run to the tool: `atelier status` already tells each thread its role, what it holds and the next command, and fresh agents took an item to done on that alone (mission atelier-fcb7's notes). So the skill starts from nothing and holds only what the tool cannot print.

The live skill, `skills/agent-factory/`, is not touched: every agent on this host loads it through a link, and the cutover moves the new directory into its place. Step B (atelier-t747) then runs this candidate against no skill and cuts whatever no scenario supports, so a small skill is an acceptable result and nothing is added here "to be safe".

## Done when

- [x] `skills/agent-factory-next/` holds `SKILL.md` (front matter `name: agent-factory`, and a description that names no removed command) and `roles/lead.md`, `roles/implementer.md`, `roles/reviewer.md`, `roles/decider.md` and `roles/planner.md`. `SKILL.md` is the only file every role reads. It says to run `atelier status` first and to stop using the skill where that reports no Atelier repository, and it sends a thread to one role file. `status` names three roles (implementer, reviewer and lead), and for those the file is the one for the role it names. It names no planner and no decider: no thread holds the planner's role, and a decider just started is a visitor to the item. So `SKILL.md` routes those two by the task the thread was given: a thread asked to plan or shape work reads `roles/planner.md`, and a thread started with the decider's handoff line reads `roles/decider.md`, whatever `status` suggests it start. No role file tells its reader to read another role's file.
- [x] The rules that are Sumeet's, and those of the product specification's Review section that the binary leaves to the skill (`docs/strategies/redesign-product.md`), are each stated once, in the file whose reader acts on it, and `grep` shows no second statement of any. `SKILL.md` holds only the invariant every role needs: every non-trivial change is reviewed before merge by a thread that did not write it, and no thread approves, or decides a finding on, its own work. `roles/implementer.md` holds the three exits of a fix-now finding (fix it, rebut it once with a fact or a test, or escalate) and when escalation is required: after three rounds without an approval, after a rejected rebuttal the implementer still will not fix, and when a fix would leave the item's scope. `roles/reviewer.md` holds the reviewer's side: a rebuttal is judged by a fresh reviewer, who withdraws the finding or keeps it, and a rebuttal gets one try. `roles/lead.md` and `roles/decider.md` hold who decides and how: the mission's lead when the item's mission has one, a fresh thread when it does not, a finding settled as deferred or as no action, and anything that touches product direction to Sumeet as a `needs-human` item.
- [x] The skill says how a reviewer and a decider are started, as Sumeet decided on 2026-11-11 (atelier-t6o6, point 5; atelier-1oel builds the rule): each is another thread, started with fresh context and given only the task line the tool prints, never a fork of its starter's context. Under Claude Code a subagent runs as its parent's thread and is refused, so the reviewer is a new thread, which under T3 Code is `delegate_task`. Under Codex a subagent has an identity of its own and may review. The skill names no harness tool that was not observed to work (`docs/strategies/redesign-architecture.md`, Threads and roles, and atelier-1oel's Report once it exists).
- [x] The Report has a table with one row per section of `skills/agent-factory/constitution.md` and per procedure and reference of the live skill (`SKILL.md`, the twelve files under `procedures/`, the four under `references/`): where its rule went in the new skill (file and heading), or why it is left out. The allowed reasons to leave something out are: the new tool prints or enforces it (with the command that shows it), the specifications removed what it governs, or no recorded eval result and no named real failure shows it changed an agent's behaviour. `evals/agent-factory/results/` holds the recorded results. No `constitution.md` is written in the new directory.
- [x] `diagnose`, `validate`, `migrate`, `install` and the old `decide` are carried only where that table gives evidence for them, and then as a few lines in the role file whose reader does that work, not as files of their own. Each one left out is named in the Report as a candidate step B may restore if a scenario shows no-skill agents fail without it.
- [x] `procedures/garden.md`, `procedures/groom.md` and `procedures/audit.md` are copied from the live skill with only command names, record paths and links changed to the new tool's, since mission 4 rewrites them. The mapping is fixed here: a digest, today a decision record labelled `needs-human`, becomes an item labelled `needs-human` and `garden` or `groom`, made with `atelier new` and `atelier set --labels`, whose Goal holds the question and its entries, as the product specification's Records rules say of a question for Sumeet; he answers in conversation or in the item's body, where the agent writes the answer under its entry, since notes are gone; finishing a digest is `atelier close <id> "<outcome>"`; applying an answer to a record is `atelier set`, `atelier drop` or an edit to its body; listing open work is `atelier list`; and a link to `plan` or to "Planning on the Base" becomes a link to `roles/planner.md`. `audit`'s clause on recording architecture decisions by the record test is removed, since decision records are gone. A carried line that still names a removed command or record kind after this mapping is removed, not kept, and the Report lists it; if removing it leaves a procedure unable to do its job, the implementer escalates instead of writing a replacement.
- [x] The two open items that change the live skill are answered as this plan settles them (Context, below), and the Report says where each one's intent landed.
- [x] No file in `skills/agent-factory-next/` names a removed command or record path. The Report gives the search and its output: each name in the product specification's "What is removed" table and "Today / Replaced by" table (`atelier land`, `atelier issue`, `atelier work`, `atelier review request` and the other `review` subcommands, `atelier evidence`, `atelier brief`, `atelier debrief`, `atelier doctor`, `atelier bundle`, `atelier migrate`, `atelier prune`, `merge --push`, `--wait`, `--sync`, `--check`, `workflow.yaml`, `.atelier/issues`, `.atelier/reviews`, `.atelier/evidence`, `docs/strategies`, `docs/adr`, loose end, validation item, decision record, light path), and `atelier serve`, `atelier init` and any site page, which the new binary does not have. Naming one as history is not allowed in the skill.
- [x] Every `atelier` command the new skill names exists in the new binary: the Report lists each distinct command with the exit status of its `--help`, run with a binary built from this branch's `next/`.
- [x] No sentence in a role file tells its reader what to run where `atelier status`, a command's refusal or its `--help` already says it for that role. The Review rules the lines above keep are the exception: they are Sumeet's, and they stay in the skill although `status` prints some of them. The Report shows the `status` output for each role and entry (implementer building, implementer with changes requested, reviewer, lead, a thread holding nothing on the base, a planner on the base, and a decider just started with `escalate`'s handoff line), taken from a scratch repository, and the reviewer of this item can check the role files and `SKILL.md`'s routing against it.
- [x] The Report gives `wc -l` for each agent-facing file and the total, against the live skill's 1,219 (its `SKILL.md`, `procedures/` and `references/`), and separately the total without the three carried procedures.
- [x] The Report ends with a table for step B: each rule or passage of the new skill, the behaviour it is there to change, and the existing scenario under `evals/agent-factory/scenarios/` that could show it, or "none".

## Scope

Only `skills/agent-factory-next/` and this item's record. Not `skills/agent-factory/`, `evals/` (step B ports the scenarios and changes the harness), `next/`, `AGENTS.md` or any doc. A gap or wrong wording in what the new binary prints is reported in the Report, not fixed and not papered over in the skill. A reviewer's finding that asks for a procedure or rule the specifications and this item do not name is escalated to the orchestrator, not written.

## Context

Mission atelier-qr5s, step A of `docs/strategies/redesign-3-cutover.md`; read its Shape, the row for step A and "What mission 2 showed". The new tool is specified in `docs/strategies/redesign-product.md` (Words, Lifecycle, Commands, Review, Hooks and the bypass) and `docs/strategies/redesign-architecture.md` (Status, Threads and roles). Build the new binary with `cargo build -j 4` in `next/` and run it from `next/target/debug/atelier` in scratch repositories only; `scripts/cutover/rehearse.sh` and `evals/agent-factory/scenarios/next-loop/setup.sh` show how to make one. It is installed nowhere.

Settled in this plan:

- **Layout.** `SKILL.md`, `roles/<role>.md` for the five roles the product specification's Words name, and `procedures/` for the three carried files. Nothing else unless the table above gives evidence for it. `agents/openai.yaml` is carried if its text is still true.
- **Who works with the human.** `SKILL.md` says in one or two sentences that the planner works with Sumeet, and that every other role runs unattended: it never waits on him, and takes a question to `escalate`, the lead, or a `needs-human` item as the Review section says.
- **atelier-8ep4** (each role reads only its own instructions) is not merged and is not started here. The new skill absorbs its intent through the layout: one file per role, and `SKILL.md` holding only what every role uses.
- **atelier-y0qe** (which roles work with the human) is not merged and is not started here. The new skill absorbs the split between the planner and the unattended roles, above. Its new `spike` procedure and its rewrite of `garden` and `groom` as two halves are not written: the first is behaviour no specification names, and the strategy carries the other two unchanged until mission 4.
- **Commits.** atelier-psjw makes `ready`, `drop`, `close` and `lead` commit what they write. The skill tells no agent to run a commit after one of them; whatever the tool leaves to Git, its own output says.
- **No site.** The new binary has no `serve`, `brief` or `debrief`. Where the lead ends a mission, the skill says Sumeet reads it with `atelier show <mission>`, and no passage sends anyone a page.
- **Sumeet's six answers.** atelier-t6o6's Report holds them. Those that touch the skill: the one-try rule, the three-round rule and the fresh reviewer for a rebuttal stay with the skill (point 1); any independent thread may decide, and the lead is the one the skill sends a question to when the mission has one (point 2); and a Codex subagent may review (point 5). atelier-1oel rewords the Review section for point 1 on another branch; write the rules as that item's Goal states them.

Expected size: 150 to 300 lines of new text, plus about 175 carried lines in the three procedures. That is a guess to judge surprise against, not a target.

## Unknowns

- Whether the garden, groom and audit procedures can work at all on the new records before mission 4, since they lean on digests, labels and views the new tool renames. The Report says what breaks.

## Notes

- 2026-11-10 [note] Handoff: approved in round 1 by GPT-6.1 Sol (atelier-jcf4 @ ee9f98e0), no findings. Outcome, proof, decisions, surprises and the step B table are in the Report.

## Report

All twelve Done when lines hold. `skills/agent-factory-next/` holds `SKILL.md`, five role files and the three carried procedures, 387 lines in all and 206 without the carried procedures, against the live skill's 1,223 (the strategy says 1,219; see Surprises). The proof for each line is below: the mapping table, the removed-name search with its output, the `--help` exit status of every command the skill names, `status` captured for each role from a scratch repository driven by the prebuilt new binary (`~/.cache/atelier-qr5s/bin/atelier-next-3ba4a62f`, built from master; this branch changes nothing under `next/`), the line counts, and the table for step B. Nothing under `skills/agent-factory/`, `evals/`, `next/` or any doc changed (`git diff --stat master...HEAD` lists only `skills/agent-factory-next/` and this record).

### Where each live rule went

Evidence keys, all from `evals/agent-factory/results/` unless named otherwise:

- **E-base**: `2026-10-30-baseline.md`. The no-skill arm passed refused-land-push, vacuous-test-review, done-when-rewrite and trivial-typo as often as the skill arm, so those scenarios show no change in behaviour from the skill.
- **E-shape**: `2026-11-04-mission-shaping.md` and its rerun in `2026-11-07-skill-cut.md`. Without the guidance, both runs kept an adoption-gated measurement as a mission criterion and 3 of 4 and 4 of 5 criteria restated single items; with it, none did.
- **E-verdict**: `2026-11-08-saying-no.md`, costly-hardening: 0/3 on main, 3/3 with the sized merge/shrink/reject verdict. Its rare-edge-chain result was recorded inconclusive.
- **E-aside**: `2026-11-08-step-aside.md`, non-atelier-repo: the agent ran `status`, saw it is not an Atelier repository and set the skill aside.
- **E-abort**: `2026-11-09-complete-or-abort.md`. Both skills passed 3/3 on settle and abort, so the complete-or-abort wording changed no outcome; the one difference was that every new-skill orchestrator handed the code to an implementer, and every old-skill one wrote it.
- **E-split**: `2026-11-09-implementer-owns-review.md`, with its baseline from transcripts: 86 of 90 delegating orchestrators edited files, and orchestrators relayed reviewers' findings 136 times. With the split, the orchestrator wrote no code, and the implementer with an unmeetable Done when escalated.
- **E-loop**: `2026-11-10-next-loop.md` and `2026-11-10-next-loop-changes.md`. Fresh agents with no skill took an item to done, through a round of changes, on `status` alone; a harness's own subagent is its parent's thread.
- **F-usage**: real failures named in E-base from the 2026-10-30 usage review: vacuous tests approved by reading, Done when criteria rewritten, and gates masked by a pipe.
- **F-t34c**: astromech-t34c, named in E-verdict: an orchestrator fixed deferred and no-action findings while editing the same file for a fix-now one.
- **F-prose**: atelier-t6o6 point 4: 93 ready items whose Done when is prose with no checkbox, which the new tool counts as ticked.
- **F-close**: E-shape's real-world baseline: one mission closed with its criterion blocked.

| Live source | Where it went in the new skill, or why it is left out |
| --- | --- |
| constitution: Plausibility is not correctness | Its reviewer half: `roles/reviewer.md`, What to Judge, item 2 (F-usage). Its author half left out: E-base shows no-skill agents ran fresh proof as often. |
| constitution: Execution evidence is impartial; self-judgment is not | `SKILL.md`, Independent Review (Sumeet's rule). The author answering a finding with evidence: `roles/implementer.md`, Findings, and `roles/reviewer.md`, Rebuttals. Planning reviewed before its work: `roles/planner.md`, Readiness and Review. Human review after merge is Sumeet's `CLAUDE.md`, not the skill's. |
| constitution: Drift is the default | Left out: framing with no rule an agent acts on; no eval. |
| constitution: Process is a moving part | Left out. Skipping a rule on purpose is printed by the tool: every refusal of a rule ends with the `ATELIER_SKIP="<why>"` line (capture 11 below). |
| constitution: Judgment over checklists | Left out: framing; no eval. |
| constitution: Bounded knowledge | Left out: no eval. |
| constitution: Simplicity | Left out: no eval measured it. Candidate for step B, as a reviewer's moving-part question. |
| constitution: Code is information | Left out: no eval; spikes and competing candidates are named by no specification. |
| constitution: Failures are system signals | Carried only inside `procedures/garden.md`, Fixing at the Right Level, as copied. |
| constitution: Durable state | Left out: the specifications removed what it governs (strategy files, decision records, notes); the tool owns the records. |
| constitution: Push and pull context | `SKILL.md`, Starting Another Thread: a child gets only the task line, and pulls the rest from the tool's brief or `show`. |
| constitution: Humans own intent | `SKILL.md`, Who Works With Sumeet; `roles/decider.md`, last paragraph. |
| constitution: Strategy changes deliberately | Left out: strategy files are removed by the specifications, and no eval or named failure covers a lead changing `mission.md`. Candidate for step B. |
| constitution: Evidence and scope are separate | `roles/implementer.md`, Building, first bullet (F-t34c). The reviewer's side, that a recommendation sets no priority, is the decider's call in `roles/decider.md`. |
| constitution: Changing Agent Factory | Left out of the skill: it is the maintainers' rule, and step B applies it ("a procedure stays only if an eval shows it matters"). |
| constitution: Each role reads only what it needs | The layout: one file per role, `SKILL.md` routes, no role file points at another (atelier-8ep4's intent). E-split is the evidence that instructions for another role pull a reader into it. |
| `SKILL.md`: run `status` first, step aside outside Atelier | `SKILL.md`, first paragraph (E-aside). The install lines are left out: `init` is not in the binary, and installing elsewhere is out of the specifications' scope. |
| `SKILL.md`: small work, issues, missions | Left out: the specifications removed the light path; `start --new` and `merge --trivial` cover it, and `atelier --help` lists both. |
| `SKILL.md`: Core Rules | Prove claims: as the constitution row above. Diagnose before repairing: left out (no eval; diagnose candidate). Simplest thing: left out (candidate). Default branch green: the tool enforces it (`merge` runs the merge checks; the pre-push hook refuses non-merge code on the base); worktree tidiness left out (no eval). Separate evidence from scope: `roles/implementer.md`, Building. Respect authority: `SKILL.md`, Who Works With Sumeet, and `roles/decider.md`. Start children fresh, one way: `SKILL.md`, Starting Another Thread (E-loop, atelier-1oel's Report). |
| `SKILL.md`: Independent Review | The invariant and what is trivial: `SKILL.md`, Independent Review. What the reviewer gets: the tool prints it (`atelier review <id>`'s brief; the handoff's task line, capture 5). Who arranges it: the tool prints it (`submit`'s handoff). Findings: `roles/implementer.md`, Findings; `roles/reviewer.md`, Rebuttals; `roles/decider.md`. `WON'T FIX` is removed by the specifications (fix now, defer or no action). No reviewer: left out; a knowingly directed merge without review is `ATELIER_SKIP`, which the tool prints. |
| `SKILL.md`: Handoff | Left out: no eval shows the five-part handoff changed an outcome, and `submit` checks the Report is written. Candidate. |
| `SKILL.md`: Subskills | Replaced by `SKILL.md`, Your Role. |
| `procedures/plan.md` | Success criteria optional and work waiting on calendar time or adoption kept out: `roles/planner.md`, A Mission (E-shape). Done when lines as checkboxes that could fail: `roles/planner.md`, An Item (F-prose, F-usage). Planning Review: `roles/planner.md`, Readiness and Review (the product specification's Review section, Plan reviews). Brief and Handoff: removed by the specifications (`brief` is gone); Sumeet's go-ahead is `ready --by`, `roles/planner.md`. One theme: left out, both arms of E-shape grouped alike. Sizing, Design, Derisking, Later Increments: left out, no eval. Strategy File: removed by the specifications (a mission's documents live in its directory, and `new --mission` writes `mission.md`'s sections). Loose ends: removed by the specifications. |
| `procedures/orchestrate.md` | Delegate or Do It Yourself: `roles/lead.md`, first bullet (E-abort, E-split; the binary lets a lead `start` its own mission's item, see Surprises). Results and Escalations: `roles/lead.md`, second and third bullets (E-split); the plan-level choices are left out, no eval. Closing a Mission: `roles/lead.md`, last bullet (F-close); its debrief step is removed by the specifications, and Sumeet reads `atelier show <mission>`. Before Starting (complete or abort): left out, E-abort shows both arms alike; candidate. Assignments: the tool prints the task lines, and the item's body holds Goal and Scope (capture 3). Keeping the Plan Current, cleanup of branches and worktrees: left out, no eval. |
| `procedures/implement.md` | A Done when line that cannot hold is escalated, not rewritten: `roles/implementer.md`, Building (F-usage, E-split). The Findings rule and the escalation the Review section requires: `roles/implementer.md`, Findings. Review steps 1 to 4: the tool prints them (`submit`'s handoff, the changes-requested `status` with its Responses line, capture 7). Before Editing (expected size), Building and Proving, the other escalation limits (twice the expected size, an approach found wrong) and Handoff: left out, no eval (E-base for proof); candidates. |
| `procedures/review.md` | Verdict and sizing: `roles/reviewer.md`, Size and Verdict (E-verdict). What to Look For, contract and proof: `roles/reviewer.md`, What to Judge (F-usage). Fitness, moving parts, the rule for findings on a low finding's fix: left out, no eval (rare-edge-chain is inconclusive); candidates. Stance and Output: the tool's brief says what to read, where to write and to change no other file. The recommendations' meanings: the brief prints them. `WON'T FIX`: removed by the specifications. |
| `procedures/validate.md` | Left out: no eval exercised it. Candidate. Its receipts are removed by the specifications. |
| `procedures/decide.md` | Its method (frame, options, recommend, advocates and a judge) is left out: no fresh agent has met `decide` in any eval (strategy, What mission 2 showed). Candidate. `roles/decider.md` holds only the Review section's settle rules. |
| `procedures/diagnose.md` | Left out: no eval. Candidate. |
| `procedures/migrate.md` | Left out: no eval. Candidate. |
| `procedures/install.md` | Left out: the binary has no `init`, and a sixth repository is out of the specifications' scope (atelier-t6o6 point 6). Candidate once `init` exists. |
| `procedures/garden.md` | Carried as `procedures/garden.md`, with the mapping (below). |
| `procedures/groom.md` | Carried as `procedures/groom.md`, with the mapping (below). |
| `procedures/audit.md` | Carried as `procedures/audit.md`, with the mapping (below). |
| `references/atelier.md` | Words: the product specification's Words and each command's `--help`. One Issue, in Order: `status` prints each step (captures 4 to 7). Publishing (`merge --push`, `land`): removed by the specifications. What Makes an Approval Stale: `status` and `merge` say to run `submit`. Planning on the Base: the planning commands refuse off the base and name the checkout. Records (the record test, notes): removed by the specifications. `needs-human`: `SKILL.md` and `roles/decider.md`. Loose ends and validation items: removed by the specifications. |
| `references/sizing.md` | `roles/reviewer.md`, Size and Verdict (E-verdict). |
| `references/transcripts.md` | Left out: no eval. Its links in garden and audit are removed. Candidate for mission 4's gardener. |
| `references/verify-skill.md` | Left out: no eval. Its link in garden is removed. Candidate. |
| `agents/openai.yaml` | Not carried: its text names diagnosing, which the new skill leaves out, so it is no longer true. |

No `constitution.md` is written: `ls skills/agent-factory-next` shows `SKILL.md`, `procedures` and `roles`.

**Candidates step B may restore** if a scenario shows no-skill agents fail without them: `diagnose`, `validate`, `migrate`, `install`, the old `decide`'s method, the author's half of fresh proof, Simplicity and the reviewer's moving-part question, the fitness checks, complete-or-abort, the Handoff form, `mission.md` changing only at Sumeet's direction, the extra escalation limits, one theme and sizing for a mission, and the transcripts and verify-skill references.

### Rules stated once

Each rule that is Sumeet's or that the Review section leaves to the skill, with the one file that states it (`grep -rn` over `skills/agent-factory-next/`, run after the last edit):

| Rule | File and heading | `grep` |
| --- | --- | --- |
| Every non-trivial change reviewed before merge by a thread that did not write it; no thread approves or decides a finding on its own work | `SKILL.md`, Independent Review | `grep -rn 'own work'`: `SKILL.md:45` only |
| The three exits of a fix-now finding | `roles/implementer.md`, Findings | `grep -rln 'three exits'`: `roles/implementer.md` only |
| Rebut once, with a fact or a test | `roles/implementer.md`, Findings | `grep -rn 'Rebut it once'`: `roles/implementer.md:19` only |
| Escalation required: three rounds, kept rebuttal not fixed, fix leaves Scope | `roles/implementer.md`, Findings | `grep -rn 'three rounds'`: `roles/implementer.md:26` only |
| A rebuttal is judged by a fresh reviewer, who withdraws or keeps it | `roles/reviewer.md`, Rebuttals | `grep -rn 'fresh reviewer'`: `roles/reviewer.md:38` only |
| A rebuttal gets one try | `roles/reviewer.md`, Rebuttals | `grep -rn 'one try'`: `roles/reviewer.md:43` only |
| The mission's lead decides | `roles/lead.md` | `grep -rn 'You decide the questions'`: `roles/lead.md:12` only |
| A fresh thread decides where there is no lead | `roles/decider.md` | `grep -rn 'no lead'`: `roles/decider.md:4` only |
| Settle a finding as deferred or no action | `roles/decider.md` | `grep -rn 'no-action'`: `roles/decider.md:14` only |
| Product direction goes to Sumeet as a `needs-human` item | `roles/decider.md` | `grep -rn 'product direction'`: `roles/decider.md:16` only |
| The implementer cannot dismiss a finding | `roles/implementer.md`, Findings | `grep -rn 'dismiss'`: `roles/implementer.md:16` only |
| Plan reviews: two blind, different labs, own checkouts, cherry-picked | `roles/planner.md`, Readiness and Review | `grep -rn 'blind'`: `roles/planner.md:28` only |
| Reviewers and deciders start as fresh threads with the task line only (atelier-t6o6 point 5) | `SKILL.md`, Starting Another Thread | `grep -rn 'fork'`: `SKILL.md:53,58` only |
| `ready` on a mission only on Sumeet's go | `roles/planner.md`, Readiness and Review | `grep -rn 'only once Sumeet'`: `roles/planner.md:26` only |

### Removed names

The search, run in `skills/agent-factory-next/` after the last edit, and its output:

```text
$ grep -rnE 'atelier (land|issue|work|evidence|brief|debrief|doctor|bundle|migrate|prune|serve|init)|atelier review (request|approve|finding|defer|disposition|trivial|list|show|withdraw|reopen)|review request|merge --push|--wait|--sync|--check|--timeout|--json|workflow\.yaml|\.atelier/(issues|reviews|evidence)|docs/strategies|docs/adr|loose end|validation item|decision record|light path|debrief|/site|site page|\bsupersede\b|\bdecline\b|\bland\b' .
$ echo $?
1
```

A wider search for the old record words alone, `grep -rniwE 'issues?|notes?|strateg(y|ies)|receipts?|parent|site' .`, finds nothing either.

### Commands the skill names

Each distinct command, run as `atelier <command> --help` with the prebuilt binary under the name `atelier` on a scratch `PATH`:

| Command | `--help` exit | Flags the skill names, each found in that help |
| --- | --- | --- |
| `atelier status` | 0 | |
| `atelier check` | 0 | |
| `atelier show` | 0 | |
| `atelier list` | 0 | `--deferred`, `--done`, `--label` |
| `atelier new` | 0 | |
| `atelier set` | 0 | `--labels` |
| `atelier ready` | 0 | `--by` |
| `atelier drop` | 0 | |
| `atelier close` | 0 | |
| `atelier submit` | 0 | |
| `atelier escalate` | 0 | |
| `atelier review` | 0 | |
| `atelier decide` | 0 | `--defer`, `--no-action` |
| `atelier lead` | 0 | |

### What `status` prints to each role

From `~/.cache/atelier-qr5s/orb2/capture.sh`, which builds next-loop's fixture twice in scratch repositories under `~/.cache/atelier-qr5s/orb2/` and drives each role with its own `T3CODE_THREAD_ID`. The mission and second item are made with `new`, filled in, committed and readied. The binary predates atelier-1oel and atelier-psjw, so its handoff line is the older one and `lead` still prints a commit line. Captures 9 and 10 of the script (the lead before the question was committed, and `show`) are left out.

```text
=== 1. A thread holding nothing, on the base
$ T3CODE_THREAD_ID=idle atelier status
You are thread idle, on the base branch main.
Ready to start:
  slug-k3fa (high): Make slugify collapse runs of punctuation into one hyphen
Then: atelier start slug-k3fa
[exit 0]

=== 2. A planner on the base, with a draft mission and item not yet committed
$ T3CODE_THREAD_ID=planner atelier status
You are thread planner, on the base branch main.
Ready to start:
  slug-k3fa (high): Make slugify collapse runs of punctuation into one hyphen
Planning not committed yet, which `atelier start` does not read until it is: .atelier/items/slug-5h2p/item.md, .atelier/items/slug-k3fa/item.md, .atelier/missions/slug-v460/mission.md
Commit it with: git add -- .atelier/items/slug-5h2p/item.md .atelier/items/slug-k3fa/item.md .atelier/missions/slug-v460/mission.md && git commit -m "Plan" -- .atelier/items/slug-5h2p/item.md .atelier/items/slug-k3fa/item.md .atelier/missions/slug-v460/mission.md
Then: atelier start slug-k3fa
[exit 0]

=== 3. A lead, before any item starts
$ T3CODE_THREAD_ID=lead atelier lead slug-v460
You now lead mission slug-v460: Make slugs safe for every title
Nobody led it before.
Its items:
  Ready:
    slug-k3fa (high, can start now): Make slugify collapse runs of punctuation into one hyphen
    slug-5h2p (medium, can start now): Trim hyphens at both ends
Each implementer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one for each task with your harness's tool for a new agent thread, and give it its task:
  Run atelier start slug-k3fa and follow what it prints.
  Run atelier start slug-5h2p and follow what it prints.
Then wait for each to finish, and run `atelier status`.
Wrote, and did not commit:
  .atelier/missions/slug-v460/mission.md
Planning counts once it is committed on the base.
Then: git add -- .atelier/missions/slug-v460/mission.md && git commit -m "Lead slug-v460" -- .atelier/missions/slug-v460/mission.md
[exit 0]
$ T3CODE_THREAD_ID=lead atelier status
You are thread lead, on the base branch main.
Ready to start:
  slug-k3fa (high): Make slugify collapse runs of punctuation into one hyphen
  slug-5h2p (medium): Trim hyphens at both ends
You lead mission slug-v460: Make slugs safe for every title
  Ready:
    slug-k3fa (high, can start now): Make slugify collapse runs of punctuation into one hyphen
    slug-5h2p (medium, can start now): Trim hyphens at both ends
Each implementer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one for each task with your harness's tool for a new agent thread, and give it its task:
  Run atelier start slug-k3fa and follow what it prints.
  Run atelier start slug-5h2p and follow what it prints.
Then wait for each to finish, and run `atelier status`.
[exit 0]

=== 4. An implementer, building
$ T3CODE_THREAD_ID=impl atelier start slug-k3fa
Started slug-k3fa on branch slug-k3fa.

You are the implementer of slug-k3fa (thread impl).
Item: .atelier/items/slug-k3fa/item.md
Title: Make slugify collapse runs of punctuation into one hyphen
Status: building
To submit it for review:
  [ ] Done when: `slugify("Hello,  World!")` returns `hello-world`, and
  [ ] Done when: `test_slug.py` has a test for each of those two cases, and
  [ ] Report: written under `## Report`
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
[exit 0]
$ T3CODE_THREAD_ID=impl atelier status
You are the implementer of slug-k3fa (thread impl).
Item: .atelier/items/slug-k3fa/item.md
Title: Make slugify collapse runs of punctuation into one hyphen
Status: building
To submit it for review:
  [ ] Done when: `slugify("Hello,  World!")` returns `hello-world`, and
  [ ] Done when: `test_slug.py` has a test for each of those two cases, and
  [ ] Report: written under `## Report`
`atelier submit` commits only the item's directory, so commit your change first.
Not committed: slug.py
Then: atelier submit
[exit 0]

=== 5. submit's handoff
$ T3CODE_THREAD_ID=impl atelier submit
Submitted slug-k3fa for review.
A reviewer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
  Run atelier review slug-k3fa and follow what it prints.
Then wait for its verdict, and run `atelier status`.
[exit 0]

=== 6. A reviewer
$ T3CODE_THREAD_ID=rev atelier status
You are the reviewer of slug-k3fa (thread rev), round 1.
Review file: .atelier/items/slug-k3fa/reviews/1-gtnp.md
`atelier review slug-k3fa` prints the brief again.
Title: Make slugify collapse runs of punctuation into one hyphen
Status: in review
Then: atelier verdict approve (or `atelier verdict changes`, once your file holds a fix-now finding)
[exit 0]
$ T3CODE_THREAD_ID=rev atelier verdict changes --model test
Recorded your verdict on slug-k3fa, round 1: changes requested. Committed .atelier/items/slug-k3fa/reviews/1-gtnp.md with it.
slug-k3fa is back with its implementer, impl, to answer your open findings. Your review is done: say so to whoever started you, and stop.
[exit 0]

=== 7. An implementer, changes requested
$ T3CODE_THREAD_ID=impl atelier status
You are the implementer of slug-k3fa (thread impl).
Item: .atelier/items/slug-k3fa/item.md
Title: Make slugify collapse runs of punctuation into one hyphen
Status: changes requested
To return it to review:
  [ ] Finding 1-gtnp.1 (fix now): slugify keeps leading hyphens
Answer each finding with one line under `### Responses` in the Report, and nothing else there, such as `1-gtnp.1: fixed in <commit>` or `1-gtnp.1: rebutted: <fact>`. Or, when it is right but not worth fixing now, or its fix leaves the item's scope, escalate it: atelier escalate --finding <finding> "<question>"
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
[exit 0]

=== 8. escalate and its handoff
$ T3CODE_THREAD_ID=impl atelier escalate --finding 1-gtnp.1 Leading hyphens: fix now, or defer to the mission's other item?
Asked under `## Decisions` in .atelier/items/slug-k3fa/item.md, and did not commit it:
  2026-11-11: Leading hyphens: fix now, or defer to the mission's other item?
It is about finding 1-gtnp.1.
`atelier submit` and `atelier merge` wait until it is answered, and `atelier submit` commits it with the item.
Thread lead, the lead of mission slug-v460, decides it: lead's `atelier status` lists the question.
lead's `atelier status` reads this branch's last commit, so commit the question first: git add -- .atelier/items/slug-k3fa/item.md && git commit -m "Ask about slug-k3fa" -- .atelier/items/slug-k3fa/item.md
Tell whoever started you that it waits, and run `atelier status` once it is answered.
[exit 0]

=== 11. The implementer trying to decide its own question
$ T3CODE_THREAD_ID=impl atelier decide slug-k3fa fix it
`atelier decide slug-k3fa` refused; nothing changed.
You (thread impl) implemented slug-k3fa, and none of its implementers, past or present, decides its questions.
Thread lead, the lead of mission slug-v460, decides it: lead's `atelier status` lists the question.
lead's `atelier status` reads this branch's last commit, so commit the question first: git add -- .atelier/items/slug-k3fa/item.md && git commit -m "Ask about slug-k3fa" -- .atelier/items/slug-k3fa/item.md
Tell whoever started you that it waits, and run `atelier status` once it is answered.
If the rule is wrong for this case, go past it on purpose: run the same command again with ATELIER_SKIP="<why>" in front of it.
[exit 1]

=== 12. The lead, once the question is committed
$ T3CODE_THREAD_ID=lead atelier status
You are thread lead, on the base branch main.
Ready to start:
  slug-5h2p (medium): Trim hyphens at both ends
Held by other threads:
  slug-k3fa (changes requested, held by impl): Make slugify collapse runs of punctuation into one hyphen
Missions in progress:
  slug-v460 (ready, in progress, led by lead): Make slugs safe for every title
You lead mission slug-v460: Make slugs safe for every title
  Ready:
    slug-5h2p (medium, can start now): Trim hyphens at both ends
  Changes requested:
    slug-k3fa (high, held by impl): Make slugify collapse runs of punctuation into one hyphen
Questions waiting on a decider, oldest first on each item:
  slug-k3fa: 2026-11-11: Leading hyphens: fix now, or defer to the mission's other item?
    Answer it in the checkout on branch slug-k3fa: atelier decide slug-k3fa "<answer>"
An implementer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
  Run atelier start slug-5h2p and follow what it prints.
Then wait for it to finish, and run `atelier status`.
[exit 0]

=== 13. An item in no mission: escalate prints the decider's handoff
$ T3CODE_THREAD_ID=impl atelier escalate Should slugs keep accented letters?
Asked under `## Decisions` in .atelier/items/slug-k3fa/item.md, and did not commit it:
  2026-11-11: Should slugs keep accented letters?
`atelier submit` and `atelier merge` wait until it is answered, and `atelier submit` commits it with the item.
A decider must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
  Run atelier show slug-k3fa, and read the question waiting under its Decisions. Answer it with atelier decide slug-k3fa '<your answer>'.
Then wait for its answer, and run `atelier status`.
[exit 0]

=== 14. The decider that handoff starts, before running anything else
$ T3CODE_THREAD_ID=decider atelier status
You are thread decider, on the branch of slug-k3fa, which is held by impl.
Title: Make slugify collapse runs of punctuation into one hyphen
Status: building
Only if you mean to take it over from its holder: atelier start slug-k3fa
[exit 0]

=== 15. The implementer, while the question waits
$ T3CODE_THREAD_ID=impl atelier status
You are the implementer of slug-k3fa (thread impl).
Item: .atelier/items/slug-k3fa/item.md
Title: Make slugify collapse runs of punctuation into one hyphen
Status: building
To submit it for review:
  [ ] Done when: `slugify("Hello,  World!")` returns `hello-world`, and
  [ ] Done when: `test_slug.py` has a test for each of those two cases, and
  [ ] Report: written under `## Report`
  [ ] Question answered: 2026-11-11: Should slugs keep accented letters?
A decider must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
  Run atelier show slug-k3fa, and read the question waiting under its Decisions. Answer it with atelier decide slug-k3fa '<your answer>'.
Then wait for its answer, and run `atelier status`.
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
[exit 0]
```

How the routing reads against these:

- Captures 4, 5 and 7 open "You are the implementer of", capture 6 "You are the reviewer of", and captures 3 and 12 "You lead mission": `SKILL.md` sends each to that role's file.
- Captures 1 and 2 name no role ("You are thread idle, on the base branch"): `SKILL.md` routes by task, planning to `roles/planner.md` and a task to start an item to `roles/implementer.md`.
- Capture 14, the decider started from capture 13's task line, is told only that it may take the item over: `SKILL.md` routes it by its task line to `roles/decider.md`, whatever `status` suggests.
- What the role files leave to the tool, because these captures print it: the steps to submit (4), how to answer a finding under Responses and how to escalate it (7), that the reviewer is another thread (5), that the lead starts implementers from task lines (3, 12), that a question waits and who decides it (8, 13, 15), and that an author may not decide (11).

### Line counts

`wc -l`, after the last edit:

| File | Lines |
| --- | --- |
| `SKILL.md` | 61 |
| `roles/decider.md` | 20 |
| `roles/implementer.md` | 32 |
| `roles/lead.md` | 17 |
| `roles/planner.md` | 32 |
| `roles/reviewer.md` | 44 |
| `procedures/audit.md` | 33 |
| `procedures/garden.md` | 63 |
| `procedures/groom.md` | 85 |
| **Total** | **387** |
| **Without the three carried procedures** | **206** |

The live skill's `SKILL.md`, `procedures/` and `references/` total 1,223 lines at this branch's base (`wc -l skills/agent-factory/SKILL.md skills/agent-factory/procedures/*.md skills/agent-factory/references/*.md`), not the 1,219 the strategy gives. The new skill is 32% of it, and 17% without the carried procedures. The carried procedures are 181 lines against 176 live: the mapping added five, mostly where a single `atelier issue create` became `atelier new` and `atelier set --labels`.

### The carried procedures

`diff` of each against the live copy shows only these changes:

- **Digest** (garden and groom): `atelier issue create ... --issue-type decision -l needs-human -l <label>` became `atelier new "<title>"` then `atelier set <id> --labels needs-human,<label>`; the digest's Context, Question and Options became its Goal; "answers each entry in a note" became "in conversation or in the digest's body, and the agent writes the answer under its entry"; "`decide` the digest" and "write the outcome in the digest's Decision and `decide` it" became `atelier close <id> "<outcome>"`.
- **Applying answers** (groom): `atelier issue transition`, `issue update` and `issue link` or `unlink` became `atelier set`, `atelier drop` or an edit to the record's body; `supersede` and `decline` became `atelier drop` with a reason; "decision item" became a `needs-human` item.
- **Listing** (groom): `atelier work missions --all` and `issue show` became `atelier list` and `atelier show`; the findings `review defer` filed (label `deferred-findings`) became `atelier list --deferred`; earlier digests add `atelier list --done --label groom`; "their strategies" became "their missions' documents".
- **Links**: `plan` and Planning on the Base became `roles/planner.md`; groom's `review` became `roles/reviewer.md`; the three scales became `roles/reviewer.md#size-and-verdict`; garden's "issue" for a product question became "item".
- **Removed carried text**, each because it named a removed record or a file the new skill lacks: garden's "notes its window on the latest digest, or" (notes are gone, and a closed digest's body no longer changes); groom's "(`plan` runs it)" (the planner no longer runs groom); audit's "and record architecture decisions per the record test" (as this item directs); the links to `references/transcripts.md` (garden, audit) and `references/verify-skill.md` (garden), whose words stay; audit's "read it as Transcripts says" became "read its transcripts".

**What no longer works, or works less well, on the new tool** (the item's Unknown):

- A gardener runs as the implementer of an item labelled `garden`, on its branch, and `new` runs only on the base. Filing a digest or an accepted entry means going to the checkout of the base. The procedures do not say so; `new` refuses off the base and names the checkout.
- The transcripts and verify-skill guidance the procedures leaned on is gone with the references, so garden's transcript reading and audit's retrospectives rest on the agent's own method.
- Groom's "by `roles/planner.md`'s tests" points at a thinner test than before: one theme and sizing are not carried, only the success-criteria and Done when rules.
- Groom's "a new item body is planning: write it and have it reviewed as `roles/planner.md` says" finds a review rule only for a mission's plan; nothing in the new skill says a single item's body is reviewed.
- Nothing stops working outright: every command the procedures name exists (table above), and a digest made with `new` is a draft item, which `close` ends with a result and `status` on the base lists as waiting on a human. No escalation was needed.

### The two items that change the live skill

- **atelier-8ep4** (each role reads only its own instructions): absorbed by the layout. Each role reads `SKILL.md` and one role file; the rules each role acts on are in its own file; no role file names another; the only shared text is the routing, who works with Sumeet, the review invariant and how to start a thread.
- **atelier-y0qe** (which roles work with the human): absorbed in `SKILL.md`, Who Works With Sumeet: the planner works with him, and every other role runs unattended and takes a question to `escalate`, the lead or a `needs-human` item. Its `spike` procedure and its split of garden and groom are not written, as the plan settles.

### Decisions

- **A lead that answers a question reads `roles/decider.md`.** The Review section's who-decides rule has four parts and two readers. Stating the "how" (fix, defer, no action, product direction to Sumeet) in both `roles/lead.md` and `roles/decider.md` would state it twice, and pointing `lead.md` at `decider.md` is forbidden. `SKILL.md` therefore routes by the act, as the specification's Words do (a thread's role comes from the command it enters with, and `decide` binds a decider): whoever answers a question with `atelier decide` reads `roles/decider.md`. `roles/lead.md` says only that the lead decides its items' questions.
- **How to start another thread is in `SKILL.md`.** Three roles start threads: the implementer (reviewers, deciders), the lead (implementers) and the planner (plan reviewers). `SKILL.md` is the one file they share, so the harness facts are stated there once.
- **Tools named for starting a thread**, each observed: T3 Code's `delegate_task` (2026-11-09-implementer-owns-review, and this item's own reviewers); Codex's `spawn_agent` with `fork_turns: "none"` reviewing as its own thread (atelier-1oel's Report, one `codex exec` run; it also found that `spawn_agent` forks by default); and Claude Code's `Agent` subagent carrying its parent's identity and being refused (2026-11-10-next-loop). No other tool is named.
- **Sumeet's six answers** are not written in atelier-t6o6's Report, which says only that they are there (see Surprises). The skill follows them as atelier-1oel's Goal, atelier-yudj and this item's Context state them: points 1 and 5 in the role files and `SKILL.md`, point 2 in `roles/lead.md` and `roles/decider.md`.

### Surprises

- **atelier-t6o6's Report does not hold the six answers.** It reads "Answered by Sumeet on 2026-11-11: the six answers are in this item's Report", and nothing more, at master and on this branch. The answers can be inferred from the items that implement them, but the record the strategy and this item cite as their home is empty.
- **The live skill is 1,223 lines, not 1,219.**
- **`status` outside an Atelier repository does not say so.** In a Git checkout with no `.atelier/`, `atelier status` exits 2 with "atelier: cannot read this repository's Atelier config: .../.atelier/config.toml: No such file or directory (os error 2)"; outside Git, "this is not a Git checkout". The old binary's "Not an Atelier repository" is what non-atelier-repo was written against. `SKILL.md` names both of the new failures; a clearer message is the binary's to give.
- **A decider started from the handoff is told it may take the item over.** Its `status` (capture 14) calls it a visitor, names `atelier start` "only if you mean to take it over", and does not mention the question it was started for. A decider that trusts `status` over its task line could take the item. `SKILL.md` routes it by its task line, but the binary could say "A question waits on this item: atelier show <id>".
- **A lead may `start` an item of its own mission** and becomes its implementer, which then bars it from deciding that item's questions. Nothing refuses it. `roles/lead.md` says not to.
- **The binary has no `log`**, though the product specification lists it among the commands for anyone; atelier-8b0a builds it. The skill does not name it.
- **This binary predates atelier-psjw and atelier-1oel.** `ready`, `drop`, `close` and `lead` still print a commit line, and the handoff line is the older one. The skill is written for the tool as those items leave it: it tells no agent to commit after those four commands, and it relies on the handoff line only for what both versions say.

### Open items

- Step B (atelier-t747) decides whether each kept passage earns its place; the table below names a scenario for each, or none.
- No eval was run on this skill, as the brief directs.
- The `status` captures came from the binary at master (`3ba4a62f`), not one built with atelier-1oel and atelier-psjw merged; after they merge, capture 5's handoff line and capture 3's commit line change.

### For step B

| Rule or passage | Behaviour it is there to change | Existing scenario that could show it |
| --- | --- | --- |
| `SKILL.md`: run `status` first, step aside outside Atelier | Using the skill's process in a repository without Atelier | non-atelier-repo |
| `SKILL.md`: Your Role, routing by what `status` names | Reading another role's instructions and acting on them | next-loop, next-loop-changes |
| `SKILL.md`: routing a decider by its task line | A started decider taking the item over, as `status` offers | none |
| `SKILL.md`: Who Works With Sumeet | An unattended role stopping to ask the human | settle-within-authority, abort-unreachable |
| `SKILL.md`: Independent Review, and what is trivial | Merging unreviewed work, or reviewing a typo | refused-land-push, trivial-typo |
| `SKILL.md`: Starting Another Thread | A reviewer started as a subagent that is refused, or as a fork that carries the author's reasoning | next-loop, next-loop-changes |
| `roles/implementer.md`: fix beyond the item only if it blocks | Fixing deferred findings in passing and starting a chain of rounds | rare-edge-chain (inconclusive fixture, atelier-pju0) |
| `roles/implementer.md`: a Done when line that cannot hold is escalated | Rewording or ticking a criterion that does not hold | done-when-rewrite |
| `roles/implementer.md`: the three exits, rebut once | Dismissing a finding, or rebutting it again | next-loop-changes (the fix exit only) |
| `roles/implementer.md`: escalation required | A fourth round, or grinding on a kept rebuttal | none |
| `roles/implementer.md`: write the question with both positions | A decider judging on one side | none |
| `roles/reviewer.md`: What to Judge, contract and Done when | Approving a change that misses its requirement | plan-defect, cross-repo-regression |
| `roles/reviewer.md`: run new tests against the old code | Approving a vacuous test by reading it | vacuous-test-review |
| `roles/reviewer.md`: Size and Verdict | Blocking on line findings in a change not worth merging | costly-hardening |
| `roles/reviewer.md`: Rebuttals, fresh reviewer and one try | Re-judging one's own finding, or reopening a kept rebuttal | none |
| `roles/lead.md`: you write no code | A lead editing code instead of starting an implementer | settle-within-authority |
| `roles/lead.md`: an implementer owns its review loop | A lead relaying findings round by round | none |
| `roles/lead.md`: you decide your items' questions | Questions left for Sumeet or a new thread inside a mission | none |
| `roles/lead.md`: close against the Outcome | Closing a mission with a criterion unmet | settle-within-authority (closes a mission), abort-unreachable (an unreachable criterion) |
| `roles/decider.md` | Deciding without both positions; deciding product direction | none |
| `roles/planner.md`: success criteria optional, calendar work outside | Criteria that restate items or wait on adoption | none (the micro-test input is `results/2026-11-04-mission-shaping/input.md`, not a scenario) |
| `roles/planner.md`: Done when as checkboxes | Prose Done when counted as ticked | none |
| `roles/planner.md`: `ready` only on Sumeet's go | Readying a mission without his go-ahead | none |
| `roles/planner.md`: two blind plan reviews | One plan review, or two that read each other | none |
| `procedures/garden.md`, `groom.md`, `audit.md` | Carried until mission 4 rewrites them | none |
