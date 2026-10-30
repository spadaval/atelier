---
status: active
mission: atelier-r6bt
last_changed: "2026-10-17: active; Sumeet directed implementation after ADR 0022 was accepted and atelier-et6q decided"
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
mission: a brief before work starts and a debrief after it completes. A human
absorbs a large mission quickly from a self-contained HTML page and makes
pre-work and post-mission calls by talking to an agent. An agent reads the
same review as dense Markdown and can answer "why is it like this?" from
the records.

A partial outcome worth shipping: decision capture, the implementer report,
and `debrief`, without `brief`.

# Success Criteria

- S1: An agent can record a decision made inside a work item as a typed
  note. The choice and the reason are required; rejected alternatives are
  optional; a decision is reversible unless marked one-way. `check` refuses
  malformed entries, and one-way decisions can be listed across a mission.
  Shown by CLI integration tests.
- S2: The implementer report is the Close Reason, extended. Close Reason
  stays the required Outcome: one or two sentences giving the observable
  result against Done when, or for `supersede` why the work will not happen.
  Repository policy can also require the report's other parts when a work
  item closes: deviations from the plan, decisions, surprises, and open
  items. A missing or empty required part fails the close transition with a
  recovery step, and `land` checks it on committed claims as it checks Close
  Reason today. Every existing done record stays valid. Shown by transition
  and `land` tests, with this repository's policy enabling it.
- S3: A decision record can link the ADR it produced and supersede an earlier
  decision. Views show which decisions are current. Shown by CLI tests.
- S4: `atelier debrief <mission>` produces a review containing: the outcome
  against each success criterion with linked proof (named as S3 decides),
  each item's report, the
  decision log with one-way decisions first, derived deviations (scope added
  after the mission started, scope dropped, deferred findings, non-pass or
  stale evidence, trivial exemptions, reopened items), and follow-ups.
  Derived facts come from records and Git, not new stored state. It prints
  dense Markdown by default and writes a self-contained HTML page on request,
  both from one view model. Shown by fixture tests of both forms and by a run
  against a real completed mission.
- S5: `atelier brief <mission>` produces both forms of a review containing
  the strategy
  outcome, the scope with sizes, blockers, unknowns, open human decisions,
  and risks. A human's plan approval, recorded as an ordinary review event on
  the mission, appears in both the brief and the debrief. Shown by a fixture
  test.
- S6: `status` and `issue show` surface the current decisions that bear on
  the item being worked, through its parent and mission: the brief view shows
  the count and the one-way decisions, and the full view shows them all.
  Shown by CLI tests.
- S7: A real mission run after S1 and S2 land produces an HTML debrief that
  the human, opening it in T3 Code, judges good enough for post-mission
  review without asking the agent to run more commands. The candidate is the
  orientation mission (ADR 0022 follow-ups), run with decision capture and
  reports switched on. Shown by human review, recorded as a review event
  that names the commit the debrief was generated from.
- S8: `work mission` shows each success criterion with a proven, unproven,
  or failing mark derived from linked evidence, using the same criterion
  parsing and proof naming as S4 (atelier-stsa). Shown by CLI tests.

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
- **One record-history query.** Mission start, reopen events, scope added
  after start, and recent activity all ask when a record first appeared or
  changed status. One Git-derived query answers them, built so its walks
  scale with the mission's records, not with review events (atelier-83xi and
  atelier-7b6e record the cost of per-path walks). `debrief` runs on demand;
  anything shown at session start stays bounded or waits until the query is
  cheap.
- **Capture happens after review without staling it.** Close Reason is
  already exempt from approval freshness, so an agent can write it after
  approval. The report's other parts get the same exemption, and the close
  transition reads them from the record body or a file, not a flag.
- **One view model, two renderings.** Each review is computed once, then
  rendered as dense Markdown for agents and as HTML for humans, so the facts
  cannot disagree; a fixture test checks that every item, criterion mark, and
  decision in one form appears in the other. Each renderer uses its medium
  fully: the HTML page may lead with a summary and use collapsible sections,
  tables, and anchors. Agent output follows ADR 0022 (atelier-bw0b): dense,
  structured Markdown with scanning symbols, and no JSON or YAML modes.
- **HTML pages are static documents.** A review page is one self-contained
  file with inline CSS, its own light and dark palette, no external
  resources, and escaped record text. It is written under the gitignored
  `.atelier/runtime/` and never committed. It is read-only: no forms,
  buttons, or calls back to Atelier.
- **T3 Code is the transport.** The agent links the generated file in its
  reply, and the human opens it in T3's file panel, which renders HTML in a
  sandbox; any browser also works. The agent never re-emits the page through
  `html_render`, which would cost tokens and risk altering a computed
  report. Atelier has no T3 dependency, and T3 needs no changes.
- **Human in the loop, not human as the loop.** The human reads the page and
  gives direction verbally. The agent writes ordinary records: review
  events, decision records, notes, and new issues. No GUI, TUI, or server.
- **Approval is a record, not a gate.** Plan approval is a review event. This
  mission adds no mandatory approval gate (ADR 0018 keeps Atelier from
  becoming a plan-approval system).
- **Process changes ship with the tool.** Agent Factory's handoff and tracker
  guidance in its canonical copy (`spadaval/.agents`, ADR 0023) is updated
  alongside the commands that enforce it.
- When completeness and cost to the writer conflict, keep capture cheap. A
  short decision entry an agent actually writes beats a complete one it
  skips. When completeness and accuracy conflict in a view, accuracy wins.

# Scope

In: record formats, `check` and transition validation, `status`, `issue show`,
`work mission` criterion marks,
the new `brief` and `debrief` commands with their Markdown and HTML forms,
this repository's workflow policy, product and architecture docs (including
a `PRODUCT_INTENT.md` sentence that static HTML review pages are documents,
not a second UI), and the in-repo Agent Factory skill.

Out: progress views during a mission (they need live updates and a
different approach), changes to T3 Code, any GUI or TUI, cross-repository
portfolio views, backfilling reports for past missions (a debrief of an old
mission shows derived facts only), and identity verification.

Environment: the local CLI on agent and developer machines, run against this
repository and fixture repositories; humans read review pages in T3 Code.

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
- A debrief of a large mission may still overwhelm. The HTML page may need
  a summary first, with sections that expand; the Markdown form may need a
  summary by default with a full flag.
- ADR 0022 (atelier-bw0b) was accepted on 2026-10-17; the agent rendering
  follows its dense-output rule.
- How evidence and issues name the criterion they prove is the first
  decision of the mission (a decision record blocking S4 and S8); nothing is
  built on criteria until it is made. The default to test: criteria parsed
  from the strategy's `- S<n>:` bullets, and a criterion list on evidence
  receipts.
- Mission start time and reopen events may not be derivable cheaply without
  a history store. Check what Git history gives before designing deviations.
- Path-scoped decisions (surfacing decisions by the files they affect) may
  help agents more than item-scoped ones. Defer until S6 shows a gap.
