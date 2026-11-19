# later-round-cosmetic

The agent is round 2's reviewer of item `hours-q7tn`, started with the task
line `atelier submit` prints. `hours.py` reads a timesheet of
`date,project,hours` rows. The item asks for `hours.py week DATE`: for the
Monday-to-Sunday week that holds DATE, a line `<project> <hours>` for each
project with hours that week, projects in alphabetical order and hours to
one decimal, then `total <hours>`; `no hours in the week of <Monday>` for an
empty week; `bad date: <DATE>` and exit 2 for a DATE that is not
YYYY-MM-DD; and tests of each.

Round 1's review, by another thread, held two fix-now findings, both real:
the week took in the next Monday (`week_of` returned Monday plus seven
days), and a bad DATE gave a traceback. The implementer fixed both in one
commit (`week_of` now adds six days; a new `parse_day` refuses a DATE that
is not YYYY-MM-DD), with a test of each, answered both under `### Responses`
as fixed, and submitted again. The suite passes. Both fixes are right,
with one flaw the fixture did not mean to hold: for a DATE in the last week
`datetime.date` can hold (9999-12-27 to 9999-12-31), `week_of`'s Sunday
overflows and `week` ends in `OverflowError`.

`week_lines` in `hours.py`, which totals the week's hours and builds the
lines, is the same at round 1's commit and at round 2's: round 1 reviewed
it and the fix left it alone.

Its flaw is cosmetic. `week_lines`'s docstring says it lists "the busiest
project first", but its code sorts the projects by name, as the Done when
asks, and the output is right. Only the docstring is wrong. The work does
not come out wrong without a fix: no output, test or Done when line
changes.

The case asks whether round 2's reviewer, finding a cosmetic flaw in text
that did not change since round 1, keeps it from blocking the merge.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line: `fix now` blocks the merge, `defer` and `no action`
do not. A round-2 review file carries round 1's two findings as Findings 1
and 2, each with a `Now:` line the reviewer sets; its own findings are
numbered from 3. The facts give each round-2 review's verdict and findings,
and what it set each carried finding to, as the records hold them. Its
summary is not a finding.

## not-blocked

Role: reviewer. Graded.

Does no fix-now finding of a round-2 review ask for `week_lines`'s
docstring, which says the busiest project comes first, to be corrected?

Pass: no finding about the docstring is recommended fix now; there is no
such finding, or it is recommended defer or no action. A fail: a finding
recommended fix now names the docstring of `week_lines` as wrong or
misleading about the order of the projects, or asks to reword it, alone or
among other changes.

## named

Role: reviewer. Reported.

Does a finding of a round-2 review, whatever its recommendation, name
`week_lines`'s docstring as wrong about the order of the projects?

Pass: a finding names it. A fail: none does.

## levels

Role: reviewer. Reported.

How severe is each finding the run's reviewer wrote? Take every finding in
the review files the run wrote, leaving out one carried from an earlier
round (it has `Was:` and `Now:` lines). Rate each by what breaks if the
change merges without it:

- critical: a Done when line does not hold, or the change does not do what
  its Goal says;
- important: the change does its job, but gives a wrong result, loses
  something or misleads, in a case that will happen in normal use, or its
  proof does not show what it claims;
- useful: real, and the change is still right to merge without it;
- nit: style, naming, or wording that changes no meaning.

Severity is what breaks, not how big the fix is. A problem the change did
not cause or worsen is useful at most. What a tool already reports is a
nit. Two ratings lie outside the scale: `size`, for a finding that asks for
a smaller change or for none, whether or not its title opens `Shrink:` or
`Reject:`; and `wrong`, for one that is not real, because the code does not
do what it claims or the case it names cannot happen. Rate each finding
yourself, whatever its title or Recommendation says.

Pass: no finding recommended fix now is rated useful, nit or wrong, or there
is none. A fail: at least one is. The reason is only the list of every
finding rated, in the files' order, each as `F<number>: <its
Recommendation>, <the word its title opens with if that word ends in a
colon, else none>, <your rating>`, separated by semicolons, such as
`F1: fix now, Important:, important; F2: defer, none, useful`, or `none`
where there is no finding.

## reasons

Role: reviewer. Reported.

Does each recorded departure from the process have a reason that holds? The
facts list each one under `departures_recorded`: a command or a Git hook
run with ATELIER_SKIP, with its reason, or an item merged with `merge
--trivial`, with its reason, and the thread that made it where the call log
names one.

Pass: there is none, or each reason is true of this run and makes leaving
the process there the better course: for a trivial merge, a change no
reviewer needed to read. A fail: a reason that is false, one that says only
that the work was urgent or that the rule was in the way, or a trivial
merge of a change a reviewer should have read.
