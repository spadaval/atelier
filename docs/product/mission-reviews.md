# Mission Reviews

A mission review turns a mission's records and Git history into one page, so
a human can make post-mission calls without asking an agent to run more
commands, and an agent can answer "why is it like this?" from the same facts
(strategy `docs/strategies/review-and-introspection.md`). `atelier debrief
<mission>` reviews a mission's outcome; `atelier brief <mission>` reviews
its plan before work starts.

## Debrief

`atelier debrief <mission>` prints dense Markdown (ADR 0022):

- the mission line, its Outcome, strategy, start, close, and the commit the
  review was generated from;
- each success criterion with its mark and deciding receipt (see
  [validation](validation.md)), its proof's age counted in code commits up to
  the mission's close, or up to HEAD while it is open;
- each scoped item with its implementer report (Outcome, Deviations,
  Decisions, Surprises, Open items), or for older records the Close Reason
  alone; children are indented under their parent;
- the decision log in the order `work decisions` uses: one-way decision
  notes, current decision records, reversible notes, open decision records,
  superseded records, and malformed notes;
- derived deviations: scope added after the mission started, scope dropped
  (superseded items with their reason), findings deferred to a follow-up,
  non-pass evidence that is still the newest on an item, trivial exemptions,
  items reopened after a done claim, and proven criteria whose proof predates
  later code;
- review events on the mission itself, such as a plan approval;
- follow-ups: open scoped items and the follow-ups deferred findings name.

`--html` writes the same review to
`.atelier/runtime/reviews/<mission>-debrief.html` and prints its path. The page
leads with summary tiles, then uses tables and collapsible sections with
anchors. It is one self-contained file: inline CSS with light and dark
palettes, no scripts, forms or external resources, and every record string
escaped. It is a document, not a second UI: the human reads it in T3 Code's
file panel or any browser and gives direction in conversation, and the agent
records that direction as ordinary review events, decisions, notes, and
issues. The page is never committed; regenerate it when the records change.

Every fact is derived from records and Git: one history query walks HEAD's
first-parent history over the mission's issue records, reading a merge as
the change it made to the base, and finds the mission's start (its first
version out of `draft`), its close, when each item was first linked to it
(on either record, and for a child when its parent first listed it), and
done claims later withdrawn. Versions in the retired epic shape are read as
the parent issues they became. Nothing new is stored. Dates are local days.

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
brief and the debrief both show it; without one, the brief says no plan
approval is recorded. Atelier adds no approval gate (ADR 0018).
