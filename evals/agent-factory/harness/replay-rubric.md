# A change from atelier's history, reviewed again

The agent is the reviewer of an item in a fixture made from this
repository's own history: atelier's tree as it stood before a change, now
tracked by today's binary, and the change on the item's branch in review,
with the Goal, Done when and Report its implementer gave. The change passed
review and merged then, and a defect in it was found after the merge. The
known defect, the change and the later fix are given after this rubric. The
reviewer was told none of it, and could not read the fix.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line; "fix now" blocks the merge. The facts give each
round-1 review's verdict and findings as the records hold them.

## found

Role: reviewer. Graded.

Does a round-1 review hold a finding recommended fix now that names the
known defect?

Pass: a fix-now finding describes the same failure as the known defect: the
same case (the kind of input, commit or sequence of steps) and the same
wrong outcome, in any words. It need not propose the later fix's remedy, and
it may name more than the defect. A fail: no fix-now finding makes that
point; the point is made only in a finding recommended anything but fix now;
or a finding names the code the defect lives in but a different failure.

## others-supported

Role: reviewer. Reported.

Take each fix-now finding of the round-1 reviews that does not name the
known defect. Does the code support each one: is the failure it claims real
in the change as the diff shows it, or a gap the item's Done when asks to be
closed? The item and the lines of each file the reviews cite are given
after the diffs, as the branch under review held them, so a finding may
rest on a rule the change leaves untouched. A finding that rests on
material you are not given is not unsupported for that: if it decides the
answer, answer unknown and name it.

Pass: every such finding is supported, or there is none. A fail: at least
one is not, because the code does not do what it claims, the case it names
cannot happen, or it asks for something no Done when line or the code's own
contract needs. In the reason, name each unsupported finding by its number
and give the count as "<unsupported> of <others> unsupported".

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
