---
created_at: "2026-11-11T03:21:50.136807966+00:00"
id: "atelier-psjw"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Move the acts and repository access into atelier-app as functions that return data, make ready commit its own write, and name the human caller"
updated_at: "2026-11-11T03:21:50.136807966+00:00"
---

## Goal

Let a caller that links the core, such as the Desk, perform an act by calling a function and get the result as data. Today the new tool's acts live in the CLI crate (`next/crates/atelier-cli/src/plan.rs`, `questions.rs`, `repo.rs`), so anything but the command line has to run the binary and read text. `ready` also writes its record and prints a `git commit` line for its caller to run, and `start` carries a refusal (`not_committed`) for the case where that commit was never made.

## Done when

- [ ] Provisional: `atelier-app` has functions for `ready` and `decide` and for opening a repository at a given path. Each returns a typed result: what was written, or a refusal with its rule, its text and its next step. The CLI's `ready` and `decide` call them and print the same text as before, shown by the existing tests passing unchanged except where the next line says behaviour changes.
- [ ] Provisional: `ready` commits the paths it wrote, with the message it used to print, and leaves every other staged or changed file in that checkout as it was; it no longer prints a commit line. A test stages an unrelated file, runs `ready`, and finds one new commit holding only the mission's record and the unrelated file still staged. `start`'s `not_committed` refusal is removed, or kept with a test that shows a case it still catches.
- [ ] Provisional: a caller can name the person acting. A record written by an act run for a named human says that name where it now says `human`, and `ready --by` keeps working.
- [ ] Provisional: the product specification's `ready` row and "Where records change" say that `ready` commits, and the new skill tells no agent to run a commit after `ready`.

## Scope

`next/crates/atelier-app`, `next/crates/atelier-cli`, their tests, and the two specification passages. Not `merge`, `submit`, `review` or `verdict`. Whether `drop` and `close` should commit the same way is decided when this item is planned, with the redesign's lead.

## Context

Preliminary. Asked for by the Desk mission (atelier-z4cr, `docs/strategies/desk.md`, Design, Acts) at Sumeet's direction on 2026-11-11: with the rewrite nearly done, the Desk should not work around the tracker's internals. It touches code the cutover switches on, so it is done either by the cutover's orchestrator before the switch (atelier-u1ko) or after this host's cutover (atelier-wd1i), never while those two run. Which, is being settled with the thread that runs the redesign.

## Unknowns

- Who implements it and when; see Context.
- Whether a planning act could commit straight onto the base branch through Git objects, from any directory, as `start` builds its commit. That would remove the base-checkout rule for `ready`. It is a separate question and not this item's to answer.
