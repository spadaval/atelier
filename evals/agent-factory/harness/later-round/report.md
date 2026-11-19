`hours.py week DATE` finds the Monday of DATE's week (`week_of`), totals
each project's hours from that Monday to the Sunday (`week_lines`), and
prints the projects in alphabetical order with one decimal, then the total.
A week with no hours prints `no hours in the week of <Monday>`. `WeekTest`
in `test_hours.py` covers a week with hours and one without, and `python3
-m unittest` passes.
