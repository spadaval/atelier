---
created_at: "2026-11-10T00:56:09.122058912+00:00"
id: "atelier-orb2"
issue_type: "task"
labels:
- "redesign"
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
status: "todo"
title: "Write the new Agent Factory beside the live one, one file per role"
updated_at: "2026-11-11T02:46:32.897266145+00:00"
---

## Goal

Write the candidate new Agent Factory in `skills/agent-factory-next/`, against the commands of the new binary in `next/`. It says how to decide. It leaves what to run to the tool: `atelier status` already tells each thread its role, what it holds and the next command, and fresh agents took an item to done on that alone (mission atelier-fcb7's notes). So the skill starts from nothing and holds only what the tool cannot print.

The live skill, `skills/agent-factory/`, is not touched: every agent on this host loads it through a link, and the cutover moves the new directory into its place. Step B (atelier-t747) then runs this candidate against no skill and cuts whatever no scenario supports, so a small skill is an acceptable result and nothing is added here "to be safe".

## Done when

- [ ] `skills/agent-factory-next/` holds `SKILL.md` (front matter `name: agent-factory`, and a description that names no removed command) and `roles/lead.md`, `roles/implementer.md`, `roles/reviewer.md`, `roles/decider.md` and `roles/planner.md`. `SKILL.md` is the only file every role reads. It says to run `atelier status` first and to stop using the skill where that reports no Atelier repository, and it sends a thread to the one role file for the role `status` names. No role file tells its reader to read another role's file.
- [ ] `SKILL.md` carries, in the words of the product specification's Review section as it stands (`docs/strategies/redesign-product.md`), the rules that are Sumeet's and that the binary leaves to the skill: every non-trivial change is reviewed before merge by a thread that did not write it; a fix-now finding has three exits (fix, rebut once with a fact or a test, escalate); a rebuttal goes to a fresh reviewer and gets one try; escalation is required after three rounds without an approval, after a rejected rebuttal the implementer still will not fix, and when a fix would leave the item's scope; the mission's lead decides when the item's mission has one. Each rule is stated once, in the file whose reader acts on it, and `grep` shows no second statement of it.
- [ ] The skill says how a reviewer and a decider are started, as Sumeet decided on 2026-11-11 (atelier-t6o6, point 5; atelier-1oel builds the rule): each is another thread, started with fresh context and given only the task line the tool prints, never a fork of its starter's context. Under Claude Code a subagent runs as its parent's thread and is refused, so the reviewer is a new thread, which under T3 Code is `delegate_task`. Under Codex a subagent has an identity of its own and may review. The skill names no harness tool that was not observed to work (`docs/strategies/redesign-architecture.md`, Threads and roles, and atelier-1oel's Report once it exists).
- [ ] The Report has a table with one row per section of `skills/agent-factory/constitution.md` and per procedure and reference of the live skill (`SKILL.md`, the twelve files under `procedures/`, the four under `references/`): where its rule went in the new skill (file and heading), or why it is left out. The allowed reasons to leave something out are: the new tool prints or enforces it (with the command that shows it), the specifications removed what it governs, or no recorded eval result and no named real failure shows it changed an agent's behaviour. `evals/agent-factory/results/` holds the recorded results. No `constitution.md` is written in the new directory.
- [ ] `diagnose`, `validate`, `migrate`, `install` and the old `decide` are carried only where that table gives evidence for them, and then as a few lines in the role file whose reader does that work, not as files of their own. Each one left out is named in the Report as a candidate step B may restore if a scenario shows no-skill agents fail without it.
- [ ] `procedures/garden.md`, `procedures/groom.md` and `procedures/audit.md` are copied from the live skill with only command names, record paths and links changed to the new tool's, since mission 4 rewrites them. Where one of them relies on a command or record kind that has no replacement, the line is kept, the gap is listed in the Report, and nothing is invented to fill it.
- [ ] The two open items that change the live skill are answered as this plan settles them (Context, below), and the Report says where each one's intent landed.
- [ ] No file in `skills/agent-factory-next/` names a removed command or record path. The Report gives the search and its output: each name in the product specification's "What is removed" table and "Today / Replaced by" table (`atelier land`, `atelier issue`, `atelier work`, `atelier review request` and the other `review` subcommands, `atelier evidence`, `atelier brief`, `atelier debrief`, `atelier doctor`, `atelier bundle`, `atelier migrate`, `atelier prune`, `merge --push`, `--wait`, `--sync`, `--check`, `workflow.yaml`, `.atelier/issues`, `.atelier/reviews`, `.atelier/evidence`, `docs/strategies`, `docs/adr`, loose end, validation item, decision record, light path), and `atelier serve`, `atelier init` and any site page, which the new binary does not have. Naming one as history is not allowed in the skill.
- [ ] Every `atelier` command the new skill names exists in the new binary: the Report lists each distinct command with the exit status of its `--help`, run with a binary built from this branch's `next/`.
- [ ] No sentence in a role file restates what `atelier status`, a command's refusal or its `--help` prints for that role. The Report shows the `status` output for each role (implementer building, implementer with changes requested, reviewer, lead, a thread holding nothing on the base), taken from a scratch repository, and the reviewer of this item can check the role files against it.
- [ ] The Report gives `wc -l` for each agent-facing file and the total, against the live skill's 1,219 (its `SKILL.md`, `procedures/` and `references/`), and separately the total without the three carried procedures.
- [ ] The Report ends with a table for step B: each rule or passage of the new skill, the behaviour it is there to change, and the existing scenario under `evals/agent-factory/scenarios/` that could show it, or "none".

## Scope

Only `skills/agent-factory-next/` and this item's record. Not `skills/agent-factory/`, `evals/` (step B ports the scenarios and changes the harness), `next/`, `AGENTS.md` or any doc. A gap or wrong wording in what the new binary prints is reported in the Report, not fixed and not papered over in the skill. A reviewer's finding that asks for a procedure or rule the specifications and this item do not name is escalated to the orchestrator, not written.

## Context

Mission atelier-qr5s, step A of `docs/strategies/redesign-3-cutover.md`; read its Shape, the row for step A and "What mission 2 showed". The new tool is specified in `docs/strategies/redesign-product.md` (Words, Lifecycle, Commands, Review, Hooks and the bypass) and `docs/strategies/redesign-architecture.md` (Status, Threads and roles). Build the new binary with `cargo build -j 4` in `next/` and run it from `next/target/debug/atelier` in scratch repositories only; `scripts/cutover/rehearse.sh` and `evals/agent-factory/scenarios/next-loop/setup.sh` show how to make one. It is installed nowhere.

Settled in this plan:

- **Layout.** `SKILL.md`, `roles/<role>.md` for the five roles the product specification's Words name, and `procedures/` for the three carried files. Nothing else unless the table above gives evidence for it. `agents/openai.yaml` is carried if its text is still true.
- **Who works with the human.** `SKILL.md` says in one or two sentences that the planner works with Sumeet, and that every other role runs unattended: it never waits on him, and takes a question to `escalate`, the lead, or a `needs-human` item as the Review section says.
- **atelier-8ep4** (each role reads only its own instructions) is not merged and is not started here. The new skill absorbs its intent through the layout: one file per role, and `SKILL.md` holding only what every role uses.
- **atelier-y0qe** (which roles work with the human) is not merged and is not started here. The new skill absorbs the split between the planner and the unattended roles, above. Its new `spike` procedure and its rewrite of `garden` and `groom` as two halves are not written: the first is behaviour no specification names, and the strategy carries the other two unchanged until mission 4.
- **No site.** The new binary has no `serve`, `brief` or `debrief`. Where the lead ends a mission, the skill says Sumeet reads it with `atelier show <mission>`, and no passage sends anyone a page.
- **Sumeet's six answers.** atelier-t6o6's Report holds them. Those that touch the skill: the one-try rule, the three-round rule and the fresh reviewer for a rebuttal stay with the skill (point 1); any independent thread may decide, and the lead is the one the skill sends a question to when the mission has one (point 2); and a Codex subagent may review (point 5). atelier-1oel rewords the Review section for point 1 on another branch; write the rules as that item's Goal states them.

Expected size: 150 to 300 lines of new text, plus about 175 carried lines in the three procedures. That is a guess to judge surprise against, not a target.

## Unknowns

- Whether the garden, groom and audit procedures can work at all on the new records before mission 4, since they lean on digests, labels and views the new tool renames. The Report says what breaks.
