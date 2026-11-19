## Finding 1: --month ignores the year, so it counts the same month of every year
Recommendation: fix now
`in_month` compares only the month number (`tally.py` line 17), so
`python3 tally.py total --month 2026-02` on a ledger that holds February 2025
too adds both Februaries. `exports/2025.csv` joined to `sample.csv` shows it.
The second Done when line asks for that month of that year only, and its test
uses rows from one year, so it cannot catch this.

## Finding 2: --month adds a foreign purchase as if it were dollars
Recommendation: fix now
`docs/bank-export.md` says a purchase made abroad is exported in the currency
it was charged in, at that currency's amount, with its code in `currency`.
`read_rows` ignores `currency`, so a month with a 50.00 EUR dinner totals
50.00 dollars for it. It is rare (none of `exports/2025.csv`'s 48 rows is
foreign), but a total that mixes currencies is wrong when it happens. Convert
each row that is not USD to dollars at the exchange rate of its date before
summing, and test it.

## Finding 3: --month crashes on the bank's real export, which writes US dates
Recommendation: fix now
`docs/bank-export.md` says the bank writes `date` as MM/DD/YYYY for US
accounts, which Dana's is, so `datetime.date.fromisoformat` raises ValueError
on the first row of a real export, and `--month` cannot be used at all.
`sample.csv` was written by hand. Parse MM/DD/YYYY, and keep ISO for the
hand-written files.
