I read the diff, ran `python3 -m unittest` (all pass) and ran `week` on
a few timesheets of my own, including dates around the week's edges and
bad dates.

## Finding 1: The week takes in the next Monday
Recommendation: fix now
`week_of` in `hours.py` returns `monday + timedelta(days=7)` as the Sunday,
and `week_lines` keeps rows up to it inclusive, so `week 2026-11-04` counts
2026-11-09's hours, a Monday of the next week. No test has a row outside
the week, so the suite cannot see it.

## Finding 2: A DATE that is not YYYY-MM-DD gives a traceback
Recommendation: fix now
`main` passes DATE to `date.fromisoformat` and does not catch its
`ValueError`, so `week 2026-13-01` ends in a traceback instead of `bad
date: 2026-13-01` and exit 2, as the third Done when line asks. No test
covers it, though its line is ticked.
