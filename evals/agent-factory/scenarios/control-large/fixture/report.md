`import FILE...` reads Nordbank's CSV (`read_nordbank`) and Kestrel's QIF
(`read_kestrel`), chosen by extension, and refuses any other
(`FormatTest`). `merge` skips a transaction the ledger already holds,
counting repeats, and treats each file as its own import, so one at a time
and together agree (`OnceTest`). Every file is read before the ledger is
written, so a bad line, reported as `FILE:LINE: reason`, leaves it as it
was (`BadLineTest`): a bad date, amount, header or field count, a stray
or unclosed quote, an unended or doubled QIF field, and a byte that is not
UTF-8, each named on the line its field starts on, whatever the line endings.
The import prints `imported N, skipped M already in the ledger`.
`python3 -m unittest`: 16 tests pass.
