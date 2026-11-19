
Round 1 found that the week took in the next Monday, and that a bad DATE
gave a traceback. `week_of` now returns the Sunday, and `parse_day` refuses
a DATE that is not YYYY-MM-DD, so `week` prints `bad date: <DATE>` and
exits 2. `WeekTest` gained a test of the week's edges, with rows on the
Sunday before and the Monday after, and one of three bad dates.
