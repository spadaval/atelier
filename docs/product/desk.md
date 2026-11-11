# The Desk: product intent

Status: proposed with mission atelier-z4cr (2026-11-11). The decisions it
rests on are in [ADR 0032](../adr/0032-the-desk-is-the-humans-product.md).

## Why it exists

Atelier has two kinds of user. Agents do the work, and they use the `atelier`
command and its records. One human decides what is worth doing, keeps the
ground such that agents can do it, and judges the result. He has no tool of
his own. The site `atelier serve` showed mirrored the tracker: it listed
records, and every problem with it came from that. The human does not need
to see records. He needs to make a small number of decisions well.

The Desk is that tool. The tracker is where agents work, and the Desk is
where the human plans, tends and reads results.

## Who it is for

One person running fleets of agents across several repositories on one host,
at a desktop for planning and on a phone for checking in and answering.

## Its jobs

In order of importance:

1. **Plan.** Take a mission from an idea to a go: its why, outcome, design
   and documents on one page, a planning session with an agent started from
   that page, and "say go" when it is ready.
2. **Tend.** Keep each repository easy to navigate, change and validate:
   numbers against the last period, the worst spots, and at most three
   proposals, each a root cause with its trips as evidence. The same for the
   backlog: what to do next and what not to do.
3. **Read results.** A finished mission's debrief, what was decided on his
   behalf, and what changed since he last looked.
4. **Answer.** The short queue of things only he can settle.
5. **Watch.** Which agents are running, on what, and whether one is stuck.
   This supports the other four. The Desk is not a monitoring tool.

## Principles

- **Missions are the unit of attention.** Items show only inside the mission
  being planned or judged.
- **Decisions, not records.** A page is built around a decision the human
  has to make. A page that only lists records is a sign it is not finished.
- **Causes, not incidents.** A trip, a finding or a failed run is evidence.
  The Desk groups evidence under a claim and asks about the claim.
- **Sessions, not queues.** Planning, gardening, grooming and spikes are
  done with an agent in conversation. The Desk starts that session and shows
  its state. It does not leave a list to be answered later.
- **Everything decided becomes a record.** A go, an answer or a note is
  written by an `atelier` command under the human's identity and committed.
  The Desk holds no decision the tracker does not.
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

## How it fits

| Source | What the Desk takes from it |
| --- | --- |
| Records, through the core's read and view layer | Missions, their documents, items, reviews and the history of acts |
| T3 Code, through its MCP server | Threads, their models and state; a way to start a session |
| A scheduled feed | Numbers, worst spots and candidate causes, read from history and transcripts by an unattended run |
| A host-wide database | Trends, findings, and what the human last saw. Nothing a record holds. |

The Desk writes records only by running `atelier` commands under the
human's name. The tracker can be rebuilt without the Desk changing its
purpose, and no agent, check or merge depends on the Desk.

## What it is built from

A Rust server, `atelier-desk`, links the tracker's core and serves the
Desk's API. A web app at `desk/` is the interface. One service per host
carries both, reached over the tailnet.

## Order of building

1. The mission page with its brief and debrief, the inbox, agents and the
   human's acts, on the redesigned records (mission atelier-z4cr).
2. Health for real: the feed, the database, and causes written by a run.
3. Sessions for gardening and grooming, what changed since the human last
   looked, and notifications to a phone.
