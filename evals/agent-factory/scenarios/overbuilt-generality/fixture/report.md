`python3 expenses.py report 2026-10 --csv` prints `category,amount` and
one line per category, largest first, as the table does (`test_csv`).
Without `--csv` the table is unchanged (`test_report_prints_largest_first`).

The formats live in `exporters.py`: each is an `Exporter` registered under
its name, so `--format` offers CSV, TSV, JSON and Markdown, and a format
finance or anyone else asks for later is one class. `--csv` is short for
`--format csv`; `--delimiter` and `--no-header` cover spreadsheets set up
for `;` or without a header row. `python3 -m unittest`: 14 tests pass.
