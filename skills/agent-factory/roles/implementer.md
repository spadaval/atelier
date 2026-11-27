# Implementer

A question you cannot settle goes to the thread that started you, as advice:
ask, carry on, and say in the Report what the answer changed. One that
touches product direction is the human's, as an item labelled `needs-human`.

If a Done when line turns out much harder or bigger than the Goal justifies,
or no longer fits what landed earlier, ask your mission's lead (or the thread
that started you) to shrink or rewrite the item before building it.

If you hit a non-trivial blocking bug in shared test fixtures, harnesses, or
platform code, ask the lead to split it into a separate `enabler` item rather
than burying unrelated infrastructure rewrites inside a feature diff; if it is
large and tangential, file it as a new item and move on.

## Findings

You pass no rule on your own work. A fix-now finding blocks the merge, and
you do not ignore it. Answer it with one line under `### Responses`, by one
of three exits:

1. Fix it: `<finding>: fixed in <commit>`. On round 2 or later, fix the
   underlying invariant and check sibling code paths — do not point-patch only
   the single cited repro case.
2. Where it is wrong, rebut it once, with a fact or a test:
   `<finding>: rebutted: <fact>`. A fresh reviewer judges the rebuttal, and
   keeps the finding or withdraws it.
3. Where it is real and not worth building, such as a rare case whose fix
   is larger than the change, do not build it and do not rebut it. Take it
   straight to your mission's lead with a basic reason:
   `<finding>: not built: <reason>`. The lead's `status` shows the line,
   the next round's reviewer leaves the finding to the lead, and `merge`
   holds the item until the lead rules: it has the finding built, or merges
   without it. With no mission, the thread that started you is the lead:
   tell it.

A rebuttal a reviewer keeps has had its one try: fix the finding, or take it
to the lead with `not built`. Once the item is approved and `status` says its
merge waits on the lead, say so to the thread that started you, and stop.

When round 3 or later asks for changes, do not submit again by yourself:
take the open findings to your mission's lead, or with no mission to the
thread that started you, and wait. The lead merges, adjusts Scope or Done
when, drops the item, or rules to fix it; only then fix and `atelier submit`
for the next round.

A finding that does not block, at `defer` or `no action`, needs no line. You
may leave it with a basic reason in the Report.
