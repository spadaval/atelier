# Mission Reviews

A mission review turns a mission's records and Git history into one page, so
a human can make post-mission calls without asking an agent to run more
commands, and an agent can answer "why is it like this?" from the same facts
(strategy `docs/strategies/review-and-introspection.md`). `atelier debrief
<mission>` reviews a mission's outcome; `atelier brief <mission>` reviews
its plan before work starts.

## Debrief

`atelier debrief <mission>` reviews what a mission did as an after-action
report: the problem, what the mission set out to do, what it did, and where
it stands. Both forms follow the same order:

- **Status:** a one-line verdict: criteria proven, how many calls need the
  human, how much is left and how much was found along the way.
- **The problem:** the strategy's Why section, or the mission's Background.
- **What we set out to do:** the mission's Outcome, its success criteria, and
  the planned work: items linked to the mission by its start and not filed by
  review, in filing order, children under their parent.
- **What we did:** one timeline, oldest first. Each done item appears when it
  closed, with its outcome, children, what changed from the plan (the
  report's Deviations and derived facts: a done claim later withdrawn,
  failing newest proof, a trivial exemption), what surprised the
  implementer, the decisions made in it (one-way first), any decision record
  cues (its ADR, the records it replaces), how review went and who approved
  it, and the follow-ups its review filed. Beside them, at their times: the
  mission's start and close, work added to the scope, work filed other than
  by an item's review, dropped work with why, each validation run, and review
  events on the mission, such as plan approval.
- **Results:** each criterion's mark, the done work that delivers it, and its
  deciding receipt with its age (code commits up to the mission's close, or
  since the receipt while the mission is open), or the open item that will
  prove it. An item delivers a criterion when its Goal's first sentence names
  it outside parentheses, when the criterion's text names the item, or when
  it holds the deciding receipt and that receipt passes. A one-word slug such
  as `fast` counts as named only when written as code.
- **Validation:** how the mission was checked. Item review: how many
  delivered items an independent reviewer approved, how many were waived as
  trivial, the findings by their latest disposition, and the items reviewed
  without approval or not reviewed at all. Each validation run: the receipts
  one producer recorded against one commit, with each receipt's criteria,
  results and summary, the code commits since, and the work delivered after
  it. Then the criteria still awaiting proof.
- **Plan vs actual:** planned work done, the items that changed course, work
  added after the start, dropped work with why, and how much was found.
- **Needs your review:** open decision records and open `needs-human` work
  in scope, failing criteria, and criteria without passing proof that no such
  item already names.
- **Left to do:** planned work not done, and added work someone has taken on,
  other than what the human is asked for. A done child of a parent that is
  not done appears in the timeline on its own, naming its parent.
- **Found along the way:** open follow-ups that review filed, with the item
  (or the mission review) that filed them, and work added after the start
  that is parked in `backlog` or `draft`. Until follow-ups record their
  source (atelier-tq1j), parked added work is the only sign of discovered
  work outside review.

Decisions appear only where they were made. The page's sidebar indexes the
one-way decisions; `atelier work decisions <mission>` lists the whole log.

The Markdown form keeps record IDs, for agents. `--html` writes the same
review to `.atelier/runtime/reviews/<mission>-debrief.html` and prints its
path. The page has a persistent sidebar with progress, navigation, the
delivered items and the one-way decisions, and a time rail beside the
timeline. Every issue it names, including IDs mentioned in record text, is a
chip with its ID, title and status; hovering or focusing it opens a card with
its type, summary, outcome and parent. Review events and receipts are chips
with their ID and kind, and a card with what they say. It is one
self-contained file: inline CSS with light and dark palettes, no scripts,
forms or external resources, and every record string escaped. It is a
document, not a second UI: the human reads it in T3 Code's file panel or any
browser and gives direction in conversation, and the agent records that
direction as ordinary review events, decisions, notes, and issues. The page
is never committed; regenerate it when the records change.

Every fact is derived from records and Git: one history query walks HEAD's
first-parent history over the mission's issue records, reading a merge as
the change it made to the base, and finds the mission's start (its first
version out of `draft`), its close, when each item was first linked to it
(on either record, and for a child when its parent first listed it), and
done claims later withdrawn. Versions in the retired epic shape are read as
the parent issues they became. Nothing new is stored. Dates are local.

The Markdown and HTML forms come from one view model, and a test checks that
every item, criterion mark, and decision in one appears in the other.

## Brief

`atelier brief <mission>` reviews the plan, so the human can approve or
change it while changes are cheap. It shows the mission line, Outcome and
strategy; each success criterion; the scope, one item line each with its
size (`| size M`, from the `size-*` label), open blockers and needs-human
cue, children under their parent; the blocked items; the Unknowns of each
open item; open human decisions (open decision records and needs-human work
in scope); the strategy's Risks and Unknowns section; and review events on
the mission. `--html` writes `.atelier/runtime/reviews/<mission>-brief.html`
with the same facts, under the same rules as the debrief page.

A plan approval is an ordinary review event on the mission:
`atelier review approve <mission> --identity "<human>" "<summary>"`. The
brief and the debrief both show it; without one, both forms of the brief say
no plan approval is recorded. Any approval event on the mission counts, so a
later approval of the debrief also reads as approval in a brief generated
after it. Atelier adds no approval gate (ADR 0018).
