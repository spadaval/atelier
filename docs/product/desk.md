# The Desk: product intent

Status: accepted with mission atelier-z4cr (2026-11-11), and changed by
mission atelier-to83 (item atelier-v958, 2026-11-13), which took the go-ahead
away. The decisions it rests on are in [the design
record](../design.md#decisions).

## Why it exists

Atelier has two kinds of user. Agents do the work, and they use the `atelier`
command and its records. One human decides what is worth doing, keeps the
ground such that agents can do it, and judges the result. He has no tool of
his own. The old tool's site mirrored the tracker: it listed
records, and every problem with it came from that. The human does not need
to see records. He needs to make a small number of decisions well.

The Desk is that tool. The tracker is where agents work, and the Desk is
where the human plans, tends and reads results.

## Who it is for

One person running fleets of agents across several repositories on one host,
at a desktop for planning and on a phone for checking in and answering.

## Its jobs

In order of importance:

1. **See and stop.** Nothing asks for his go-ahead: a mission starts from a
   conversation with him, its plan merges once two reviewers approve it, and
   its items start after that. So the Desk shows him what is proposed and
   what is in motion and who is doing it, across every repository, tells him
   when a plan is proposed and when it merges, and lets him stop a mission
   with his reason. A mission's why, outcome, design and documents are on one
   page, with a planning session started from there.
2. **Tend.** Keep each repository easy to navigate, change and validate:
   numbers against the last period, the worst spots, and at most three
   proposals, each a root cause with its trips as evidence. The same for the
   backlog: what to do next and what not to do.
3. **Read results.** A finished mission's debrief, what was decided on his
   behalf, and what changed since he last looked.
4. **Answer.** The short queue of things only he can do: items labelled
   needs-human, and threads waiting on a person. A question an agent has
   goes to the thread that started it, not to him; where two models
   disagreed, the Desk shows it, and nothing waits on him for it.
5. **Watch.** Which agents are running, on what, and whether one is stuck.
   This supports the other four. The Desk is not a monitoring tool.

## Principles

- **Missions are the unit of attention.** Items show only inside the mission
  being planned or judged.
- **Decisions, not records.** A page is built around a decision the human
  has to make. A page that only lists records is a sign it is not finished.
- **Causes, not incidents.** A trip, a finding or a failed run is evidence.
  The Desk groups evidence under a claim and asks about the claim.
- **Sessions, not queues.** Planning, backlog passes, gardening and spikes
  are done with an agent in conversation. The Desk starts that session and shows
  its state. It does not leave a list to be answered later.
- **Everything decided becomes a record.** A stop is written by the
  tracker's own function, the one `atelier drop` calls, under the human's
  name, and is finished only once it is published. The Desk holds no
  decision the tracker does not.
- **Derived figures say where they come from.** A number estimated from git
  or from T3 is marked as one.
- **Agents never need it.** No agent reads the Desk, and no check or merge
  consults it.

## What it never does

- It is not a source of truth, and it gates nothing.
- It is not a record editor. It offers the few acts that are the human's.
- It is not a chat client. Conversations happen in T3 Code; the Desk starts
  and links to them.
- It does not score agents or people.

## Its pages

- **Repositories**: each repository with its missions, what is in progress,
  how many plans are proposed and how many items need him, and when its
  remote was last fetched.
- **A mission**: its sections, documents, items and plan reviews, read from
  its branch while it is proposed, with the branch, its pull request and the
  verdicts of its last round. Its state is proposed, agreed, stop not
  published yet, stopped, or done. A proposed plan has "Stop this plan"; an
  agreed mission shows the commands that stop it.
- **Proposed**: every proposed plan, with its branch, pull request and last
  round's verdicts, and every other branch that changes records, with what
  it adds, changes or drops. A plan dropped on its branch is not proposed.
- **In motion**: every started item, read from its branch, with the thread
  that holds it and whether T3 Code lists that thread as running; and every
  mission in progress with its lead.
- **Disputed**: what `atelier list --disputed` gives: each rebutted finding
  and what the next round made of it, and each plan-review round whose
  reviewers disagreed, with each side's model where the review or T3 Code
  names it.
- **Inbox**: proposed plans he may stop, threads waiting on a person,
  needs-human items, and the plans proposed and merged lately.
- **Agents**: T3 Code's live threads and what each holds.

Every page that reads a repository says when the Desk last fetched its
remote. The Desk fetches each one on a timer its configuration sets, every
five minutes on this host, so work pushed from another clone shows within
that time.

## Stopping a mission

A stop drops the mission with his reason, as `atelier drop` does, and counts
once it is published (docs/design.md, Where each command writes):

- **A proposed plan**: the Desk stops it itself. It writes the drop in a
  clone of its own, commits it, pushes the plan's branch and closes its pull
  request. The plan is shown stopped once both are done; with the drop
  pushed and the pull request still open, the stop is shown as not
  published yet.
- **An agreed mission**: its stop reaches the base by a record-only branch
  that `atelier submit` and `atelier merge` carry. Those live in
  `atelier-cli`, and the Desk runs no `atelier` binary (ADR 0032), so until
  atelier-yclc moves them into the core the Desk offers no stop act here: the
  page gives the commands. A stop someone has written is shown as not
  published yet, naming its branch, until that branch merges, and then as
  stopped.

## Being told

He is told when a plan is proposed and when it merges, so the veto the
missing go-ahead leaves him is real (settled 2026-11-13 by the mission's
lead):

- The Desk notices both on its fetch timer, by comparing each repository's
  missions with what it saw last. It keeps what it has seen in its state
  directory, so a restart tells nothing twice, and the first look at a
  repository tells nothing.
- It runs the command its configuration names, with a title, a line of text
  and a link to the plan's page. On this host that is `notify-mail`, which
  mails him from his own Gmail account to itself through `gog`. Mail from
  his account on a timer is his to switch on, so the installed configuration
  leaves it off until he does; the inbox lists the news either way.
- T3 Code's Browser panel was turned down for this: it lags off the LAN. An
  agent that proposes a plan may also show its page in its own thread
  (atelier-plr9).

## How it fits

| Source | What the Desk takes from it |
| --- | --- |
| Records, through the core | Missions, their documents, items, reviews and the history of acts |
| Git, through each repository's remote | Branches pushed from other clones, fetched on a timer |
| The forge, through `gh` | Each branch's pull request and its state; closing a stopped plan's |
| T3 Code, through its MCP server | Threads, their models and state; a way to start a session |
| A daily feed | Numbers, worst spots and candidate causes, read from history and transcripts by an unattended run |
| A host-wide SQLite database | Trends, findings, and what the human last saw. Nothing a record holds. |

The Desk writes no record itself. It calls the core's own functions for
the human's acts, the ones the `atelier` command calls, under his name, and
writes only in a clone of its own, never in a served repository's checkout. The tracker can be rebuilt without the Desk changing its
purpose, and no agent, check or merge depends on the Desk.

## What it is built from

A Rust server, `atelier-desk`, links the tracker's core and serves the
Desk's API. A web app at `desk/` is the interface. One service per host
carries both, reached over the tailnet.

## Order of building

1. The mission page with its brief and debrief, the inbox, agents and the
   human's acts, on the redesigned records (mission atelier-z4cr).
2. The live repositories, what is proposed and in motion, the stop, and
   being told (atelier-v958); an agreed mission's stop from the Desk
   (atelier-yclc).
3. Health for real: the feed, the database, and causes written by a run.
4. Sessions for gardening and backlog passes, what changed since the human
   last looked, and notifications to a phone.
