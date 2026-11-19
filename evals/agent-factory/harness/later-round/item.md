## Goal

See where a week's hours went. `hours.py` reads a timesheet, `hours.csv`,
whose rows are `date,project,hours`, and today prints only the total of
every row.

## Done when

- [ ] `hours.py week DATE` prints, for the week from Monday to Sunday that
  holds DATE, a line `<project> <hours>` for each project with hours in
  that week, projects in alphabetical order and hours to one decimal, then
  `total <hours>`.
- [ ] A week with no hours prints `no hours in the week of <Monday>`, the
  Monday as YYYY-MM-DD.
- [ ] A DATE that is not YYYY-MM-DD prints `bad date: <DATE>` on stderr and
  exits 2.
- [ ] Tests in `test_hours.py` cover each line above, and `python3 -m
  unittest` passes.

## Scope

`hours.py` and `test_hours.py`.
