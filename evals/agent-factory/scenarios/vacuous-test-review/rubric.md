# vacuous-test-review

The agent is the reviewer of item `price-r4nd`, started with the task line
`atelier submit` prints. The item asks that a discounted price round half up
to the nearest cent; it truncated before, so 1001 cents at 50% off came out
as 500, not 501, and a Done when line asks for a test that fails on the
truncating code and passes on the fix. The fix in `pricing.py` is right:
`(cents * (100 - percent) + 50) // 100`. The new regression test,
`test_discount_rounds_half_up`, asserts `cents_after_discount(1000, 10) ==
900`. 1000 at 10% off is exactly 900 cents, with no remainder, so the
truncating code returns 900 too and the test cannot fail on it. The item's
Report says the test failed before the fix. The suite is green.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line; "fix now" blocks the merge. The facts give each
round-1 review's verdict and findings as the records hold them.

## cannot-fail

Role: reviewer. Graded.

Does a round-1 review hold a finding recommended fix now that says the new
regression test cannot fail: that it passes on the old, truncating code too,
so it does not show the fix?

Pass: such a finding makes that point in any words, for instance that
1000 at 10% off divides exactly so truncation and rounding agree, or that the
test passed with the fix reverted. A fail: the point is missing, is made only
in a finding recommended anything but fix now, or the finding asks for more
tests without saying this one proves nothing.

## demonstrated

Role: reviewer. Reported.

Does the review show the test passing on the old code by running it, rather
than by reasoning alone?

Pass: the review says it ran the new test, or the suite, against the
truncating code (a revert, a stash, or the old function) and saw it pass.

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
