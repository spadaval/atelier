## Goal

We type every transaction into `ledger.json` by hand from two banks'
statements. Importing the files the banks export would end that. Each
month we download overlapping date ranges from both, so an import must
not add a transaction twice, and the ledger is the only record we keep,
so a bad file must not leave half of it imported.

## Done when

- [ ] `python3 budget.py import FILE...` adds the transactions of each file
  to the ledger, telling the formats apart by extension. A `.csv` file is
  Nordbank's export: `;`-separated, the header `Date;Payee;Amount`, dates
  as `31.10.2026` and amounts as `-1.234,56`. A `.qif` file is Kestrel
  Bank's: a first line `!Type:Bank`, then records of a `D` line
  (`10/31/2026`), a `T` line (`-1,234.56`) and a `P` line (the payee), in
  any order, each ended by a line `^`. Its other lines, such as `M` (memo)
  and `N` (number), are ignored. Any other extension is refused.
- [ ] A transaction is added once: one already in the ledger, with the same
  date, amount and payee, is skipped. Repeats within a file are distinct
  transactions, so a statement with two equal coffees adds two, and
  importing it again adds none. Importing files one at a time or together
  gives the same ledger.
- [ ] A line that cannot be read stops the import with `FILE:LINE: reason`
  on stderr and exit status 1, and leaves the ledger as it was, whichever
  of the files it is in.
- [ ] The import prints `imported N, skipped M already in the ledger`.
- [ ] Tests cover each line above, and `python3 -m unittest` passes.

## Scope

`budget.py`, its tests, and any new module they need.
