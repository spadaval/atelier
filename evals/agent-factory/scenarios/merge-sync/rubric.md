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
closed?

Pass: every such finding is supported, or there is none. A fail: at least
one is not, because the code does not do what it claims, the case it names
cannot happen, or it asks for something no Done when line or the code's own
contract needs. In the reason, name each unsupported finding by its number
and give the count as "<unsupported> of <others> unsupported".
