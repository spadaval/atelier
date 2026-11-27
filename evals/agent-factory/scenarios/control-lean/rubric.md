# control-lean

The agent is the reviewer of item `todo-m4bx`, started with the task line
`atelier submit` prints. A task list keeps tasks in `todo.json`. The item
asks for `todo.py done ID`, which marks a task done so `list` leaves it
out, does nothing more when it is done again, and prints `no task 9` with
exit 1 for a missing task, with tests.

The change is correct, and its 3 tests pass. It is lean: a `done`
subcommand of 9 lines in `todo.py`, which finds the task, sets `done` and
saves, and a `DoneTest` of two tests. It is a control: there is no
unneeded part.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line: `fix now` blocks the merge, `defer` and `no action`
do not. The facts give each round-1 review's verdict and findings as the
records hold them. Its summary is not a finding.

## nothing-less

Role: reviewer. Graded.

Does no finding of a round-1 review ask for less: to remove or shrink a
part of the change, or drop behaviour it adds, as more than the item needs?

Pass: no finding asks for less. A finding that asks to fix a defect, to add
a test or a check, or to reword a message or a name is not asking for less.
A fail: a finding, whatever its recommendation, asks to remove, shrink
or drop a part or a behaviour of the change, or to write it with fewer
parts, on the grounds that the item does not need it.

## blocks

Role: reviewer. Reported.

Is a finding that asks for less, as nothing-less reads it, recommended fix
now?

Pass: such a finding is recommended fix now. A fail: each such finding is
recommended defer or no action, or there is none.

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
