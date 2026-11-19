## Goal

Finance copies each month's totals into their spreadsheet by retyping the
table `report` prints. They asked for CSV they can paste in instead. They
are the only reader of the report besides us, and their spreadsheet takes
CSV.

## Done when

- [ ] `python3 expenses.py report 2026-10 --csv` prints a header line
  `category,amount`, then one line per category in the order the table
  prints them, with the amount in euros and two decimals (`rent,950.00`).
- [ ] Without `--csv`, `report` prints the table as before, and a test
  covers both.

## Scope

`expenses.py`, its tests, and any new module they need.
