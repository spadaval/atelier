---
status: draft
mission: atelier-r6bt
last_changed: "2026-10-16: terminal styling allowed for reviews (human direction)"
---

# Why

Human review is most useful at two points: before a mission starts, when
bad decisions and priorities are cheap to change, and after it completes,
when someone needs to see what happened and decide what to revert or carry
into the next cycle. Today both happen in conversation: the human asks, an
agent runs several commands and paraphrases. That summary is lossy and has
no canonical source behind it.

The deeper gap is in the records. Atelier captures proof and review well
(425 review events and 181 evidence receipts against 79 issues at the start of
this mission) but almost none of the reasoning: one decision record, three
handoff notes, and a one-paragraph Close Reason per item. What an implementer
chose, rejected, and was surprised by is lost when the session ends, so
neither a human nor the next agent can review it.

# Outcome

Implementers record the decisions they make and a short report when they
finish. Two read-only commands turn the records into complete reviews of a
mission: a brief before work starts and a debrief after it
completes. A human can make pre-work and post-mission calls from those
documents. An agent can answer "why is it like this?" from the same records.

A partial outcome worth shipping: decision capture, the implementer report,
and `debrief`, without `brief`.

# Success Criteria

- S1: An agent can record a decision made inside a work item, with the
  choice, rejected alternatives, reason, and whether it is reversible or
  one-way. `check` refuses malformed entries, and one-way decisions can be
  listed across a mission. Shown by CLI integration tests.
- S2: Repository policy can require an implementer report when a work item
  closes: outcome, deviations from the plan, decisions, surprises, and open
  items. A missing or empty report fails the close transition with a
  recovery step. Shown by transition tests, with this repository's policy
  enabling it.
- S3: A decision record can link the ADR it produced and supersede an earlier
  decision. Views show which decisions are current. Shown by CLI tests.
- S4: `atelier debrief <mission>` prints a review containing: the outcome
  against each success criterion with linked proof, each item's report, the
  decision log with one-way decisions first, derived deviations (scope added
  after the mission started, scope dropped, deferred findings, non-pass or
  stale evidence, trivial exemptions, reopened items), and follow-ups.
  Derived facts come from records and Git, not new stored state. Shown by a
  fixture test and by a run against a real completed mission.
- S5: `atelier brief <mission>` prints a review containing the strategy
  outcome, the scope with sizes, blockers, unknowns, open human decisions,
  and risks. A human's plan approval, recorded as an ordinary review event on
  the mission, appears in both the brief and the debrief. Shown by a fixture
  test.
- S6: `prime` and `issue show` surface the current decisions that bear on the
  item being worked, through its parent and mission. Shown by CLI tests.
- S7: A real mission run after S1 and S2 land produces a debrief that the
  human judges good enough for post-mission review without asking the agent
  to run more commands. Shown by human review, recorded as a review event.

# Shape

- **Store only what cannot be derived.** Decisions, reports, and approvals
  are written down. Deviations, timelines, and proof status are computed at
  view time from records and Git history.
- **Use existing record types.** Decision entries and reports live in issue
  records (sections or typed notes), and big decisions stay `decision`
  records. No new record directory, sidecar, or cache.
- **Reports are views.** `brief` and `debrief` are deterministic, read-only
  commands. They write nothing and never present a receipt as passing
  without checking its result and freshness.
- **Rich on a terminal, Markdown everywhere else.** On a terminal, reviews use
  the full styling the existing human output policy allows: color, emphasis,
  status symbols, width-aware layout, and clickable links. Redirected or
  with an explicit format flag, they emit plain Markdown with the same
  content. Color never carries meaning on its own, and `NO_COLOR` and the
  color flag are honored.
- **The medium is the CLI plus conversation.** Humans read reviews in a
  terminal, or as Markdown in an editor or PR, and give direction through an
  agent, which writes ordinary records: review events, decision records,
  notes, and new issues.
  No GUI, interactive TUI, or server; output may be paged. A static HTML
  export is considered only if S7 shows the terminal and Markdown forms are
  hard to read.
- **Approval is a record, not a gate.** Plan approval is a review event. This
  mission adds no mandatory approval gate (ADR 0018 keeps Atelier from
  becoming a plan-approval system).
- **Process changes ship with the tool.** Agent Factory's handoff and tracker
  guidance in `.agents/skills/agent-factory/` is updated alongside the
  commands that enforce it.
- When completeness and cost to the writer conflict, keep capture cheap. A
  short decision entry an agent actually writes beats a complete one it
  skips. When completeness and accuracy conflict in a view, accuracy wins.

# Scope

In: record formats, `check` and transition validation, `prime`, `issue show`,
the new `brief` and `debrief` commands, this repository's workflow policy,
product and architecture docs, and the in-repo Agent Factory skill.

Out: progress views during a mission, any GUI or TUI, cross-repository
portfolio views, backfilling reports for past missions (a debrief of an old
mission shows derived facts only), and identity verification.

Environment: the local CLI on agent and developer machines, run against this
repository and fixture repositories.

# Authority

The orchestrator may change the increment breakdown and order, record
representation details (section names, note kinds, field names), command
names and flags, and output layout. These need the human: changes to S1–S7,
any GUI, TUI, server, or stored cache, a mandatory approval gate, a new
record type or directory, and changes to `PRODUCT_INTENT.md` principles.

# Risks and Unknowns

- Agents may still skip or water down decisions when capture is cheap.
  Validation can check form, not candor. The first increment's dogfooding
  shows whether the required fields are right before views are built on them.
- A debrief of a large mission may be too long to read. It may need a summary
  by default with detail on request.
- Mission start time and reopen events may not be derivable cheaply without
  a history store. Check what Git history gives before designing deviations.
- Whether the report replaces Close Reason or sits beside it, and how
  existing records migrate, is undecided.
- Path-scoped decisions (surfacing decisions by the files they affect) may
  help agents more than item-scoped ones. Defer until S6 shows a gap.
