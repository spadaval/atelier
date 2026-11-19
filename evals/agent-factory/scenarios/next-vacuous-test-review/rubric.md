# next-vacuous-test-review

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
