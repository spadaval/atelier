---
schema: atelier.mission
schema_version: 3
id: atelier-295u
title: Give the gardener `atelier log` and a readiness checklist, so a run starts from facts
status: ready
labels:
- redesign
lead: null
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:10:10.404909184Z
created_at: 2026-11-10T00:56:04.888418157Z
---

## Why

The gardener is the role meant to keep a codebase easy to navigate, easy to
change and easy to validate. As written it collects incidents. It has no
numbers, so it cannot tell whether its last fixes worked. It hears refusals
and retry loops and misses slow costs, such as an agent reading 40 files
before it can edit. Its one digest, atelier-j86z, is ten entries about the
tool and none about a codebase, and it waits in a queue where 6 of 63 items
have been answered.

It also works like a firefighter. Its unit is the trip, and it asks Sumeet
for a yes or no on each one, which gives every incident the same weight. A
trip is a data point. Read by cause, the ten entries of atelier-j86z are four
causes and one stray: records that live on the base while agents read
elsewhere (two entries), docs and help that lag the code (three), checks that
run in the wrong place (three), and an error the tool hides (one). Four of
the ten are help or doc fixes that needed no decision.

After the cutover the acts that move an item leave their mark in its records,
and so in Git: a status, a holder, a verdict, an answered question. The
history of what happened to each item can be read from there. Nothing reads
it yet.

## Outcome

`atelier log` prints the history of acts. A gardener run starts from that,
from `atelier list`, and from a readiness checklist for the repository, and
its procedure says which numbers to take from them. It reads transcripts
only to explain a number that moved. It settles the deferred findings. It
ends with one page for Sumeet: the numbers against the baseline, the worst
spots, and at most three proposals, each a root cause with the trips that
show it. One run has been made in each of the
five repositories.

## Success criteria

- one-page: a gardener run in each of the five repositories produces a report
  of one page that holds the scorecard, the worst spots under each of the
  three codebase properties, and at most three proposals. Each proposal
  names one root cause, the trips or numbers that are its evidence, and the
  number it should move. No page asks for an answer per trip.
- from-log: the scorecard's numbers about what happened to items, which are
  time in each status from `ready` on, rounds, roles taken over, escalations
  and the skips `log` shows, are computed from the output of `atelier log`
  by the steps the
  gardener's procedure gives, and no script walks record history itself.
  The scripts that remain tie a thread to an item by the thread IDs in the
  log, with no matching on thread titles.

## Scope

- In: `atelier log`, the scripts that remain, the checklist, the gardener
  procedure, the first runs.
- Out: a command that computes the scorecard; a scorecard page on the site;
  a command that starts a run or runs the checklist; a scheduler for the
  gardener; the groom procedure; scoring agents on any measure; the
  comparison two weeks after the cutover, which is its own item.
- Target environment: the five repositories on this host.

## Authority

The orchestrator may choose the checklist's lines, the report's layout, the
layout of a log line, and which transcript reading a run needs. It may not
add a command beyond `log` or a site page, add a rule or check to any
repository from a gardener run, or change a measure's definition in the
product specification.
