# Mission Reviews

A mission review turns a mission's records and Git history into one page, so
a human can make post-mission calls without asking an agent to run more
commands, and an agent can answer "why is it like this?" from the same facts
(strategy `docs/strategies/review-and-introspection.md`). `atelier debrief
<mission>` reviews a mission's outcome; `atelier brief <mission>` reviews
its plan before work starts.

## Debrief

`atelier debrief <mission>` reviews what a mission did. Both forms follow the
same order:

- **Needs your review:** open decision records and open `needs-human` work in
  scope, failing criteria, and criteria without passing proof that no such
  item already names.
- **Why:** the strategy's Why section, or the mission's Background.
- **Success criteria:** each criterion's mark, the done work that delivers
  it, and its deciding receipt with its age (code commits up to the mission's
  close, or since the receipt while the mission is open). An item delivers a
  criterion when its Goal's first sentence names it outside parentheses, when
  the criterion's text names the item, or when it holds the deciding receipt
  and that receipt passes. A one-word slug such as `fast` counts as named only
  when written as code.
- **Work:** the done items linked to the mission, in the order they closed,
  each with its children, its outcome, what changed from the plan (the
  report's Deviations and derived facts: a done claim later withdrawn, failing
  newest proof, a trivial exemption, joining after the start), what surprised
  the implementer, the decisions made in it (one-way first), how review went,
  and the follow-ups its review filed.
- **Open work:** planned work not done, follow-ups with the item (or the
  mission review) whose review filed them, and dropped items with why. A done
  child of a parent that is not done is listed under Work on its own, naming
  its parent.
- **Decisions:** the mission's decision log, in `work decisions` order.
- **Mission reviews:** review events on the mission itself, such as plan
  approval.

Planned work, which the sidebar's progress counts, is the work linked to the
mission by its start and not filed by review.

The Markdown form keeps record IDs, for agents. `--html` writes the same review
to `.atelier/runtime/reviews/<mission>-debrief.html` and prints its path. The
page has a persistent sidebar with progress and navigation, and a time rail
beside the work. It names records by title, not ID: hovering or focusing a
reference shows the record's title, summary, type, status and ID, and IDs that
record text mentions become the same references. It is one self-contained
file: inline CSS with light and dark palettes, no scripts, forms or external
resources, and every record string escaped. It is a document, not a second
UI: the human reads it in T3 Code's file panel or any browser and gives
direction in conversation, and the agent records that direction as ordinary
review events, decisions, notes, and issues. The page is never committed;
regenerate it when the records change.

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
