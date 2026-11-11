---
created_at: "2026-11-11T06:17:49.741506717+00:00"
id: "atelier-ivbq"
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
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-wd1i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fix what the new skill and the eval fixtures showed status gets wrong: six small gaps in what a thread is told"
updated_at: "2026-11-11T06:17:49.741506717+00:00"
---

## Goal

Close the gaps in what the new binary tells a thread that writing the new skill (atelier-orb2), the identity change (atelier-1oel) and the eval fixtures (atelier-a4rs) each ran into. None is a new rule: each is `status` or a refusal saying less, or something other, than the specifications' own commands allow. They were found before the cutover and are fixed after it, as part of what first use shows (atelier-2mlh).

## Done when

- [ ] Provisional: in a directory that is a Git checkout with no `.atelier/`, `status` says this is not an Atelier repository, where today it fails with "cannot read this repository's Atelier config".
- [ ] Provisional: a thread that holds nothing on an item with an unanswered question is told of the question and of `atelier decide`, where today `status` calls it a visitor and offers to take the item over.
- [ ] Provisional: after three rounds without an approval, the line that names `atelier escalate` includes `--finding <name>` when a kept finding is open, and is not shown once the question is asked or answered.
- [ ] Provisional: `status` shows the whole of a Done when line that wraps, not its first line.
- [ ] Provisional: inside Codex's session-start hook, where none of the variables the identity rule reads is set, `status` names the session from what the hook is given on stdin, or says plainly that it cannot tell which thread is asking. atelier-1oel's Report says what the hook receives.
- [ ] Provisional: `atelier log`, when atelier-8b0a builds it, prints an identity of the form `<parent>/<child>` whole. This line is a note for that item's planner and is dropped here once that item carries it.
- [ ] Provisional: the handoff line a Codex thread reads tells it to start the other thread itself. In the eval runs with no skill, a Codex agent would not start a subagent unless the user had asked (three runs of atelier-t747), though the line says a Codex subagent counts as another thread.
- [ ] Provisional: a lead's `status` on its implementer's branch does not offer to take the item over as its first suggestion, since a lead that takes it over can no longer decide its questions.

## Context

Preliminary; planned with step H of mission atelier-qr5s. One more finding is not in this list because it would be a new rule, which is Sumeet's to decide: a mission's lead can `start` an item of its own mission, which makes it that item's implementer and bars it from deciding the item's questions. The new skill tells a lead not to; the binary allows it. A second finding of the same kind, from atelier-t747's runs: `atelier check` and the pre-commit hook accept an item and a mission whose status was set to `done` by hand, because that change is one the act table lists for `close`; the binary cannot tell `close` from an edit. Whether that needs a rule is Sumeet's.
