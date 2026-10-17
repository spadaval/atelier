# Mission Reviews

A mission review turns a mission's records and Git history into one page, so
a human can make post-mission calls without asking an agent to run more
commands, and an agent can answer "why is it like this?" from the same facts
(strategy `docs/strategies/review-and-introspection.md`). `atelier debrief
<mission>` reviews a mission's outcome; `atelier brief <mission>` will review
its plan before work starts (atelier-acrt).

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
- the decision log: one-way decision notes, decision records, reversible
  notes;
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

Every fact is derived from records and Git: one history query over the
mission's issue records finds the mission's start (its first version out of
`draft`), its close, when each item was first linked to it (on either
record), and done claims later withdrawn. Nothing new is stored.

The Markdown and HTML forms come from one view model, and a test checks that
every item, criterion mark, and decision in one appears in the other.
