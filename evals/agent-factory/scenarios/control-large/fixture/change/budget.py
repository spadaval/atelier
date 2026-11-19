"""A household ledger kept in ledger.json.

    python3 budget.py summary
    python3 budget.py import FILE...

`summary` prints how many transactions the ledger holds, and the money in
and out. `import` adds the transactions of Nordbank's CSV and Kestrel Bank's
QIF statements, skipping those already in the ledger.
"""
import argparse
import csv
import json
import re
import sys
from collections import Counter
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path


@dataclass(frozen=True)
class Transaction:
    date: str  # YYYY-MM-DD
    cents: int  # negative for money out
    payee: str


def load_ledger(path):
    path = Path(path)
    if not path.exists():
        return []
    return [Transaction(**t) for t in json.loads(path.read_text())]


def save_ledger(path, transactions):
    rows = [{"date": t.date, "cents": t.cents, "payee": t.payee} for t in transactions]
    Path(path).write_text(json.dumps(rows, indent=2) + "\n")


def euros(cents):
    sign = "-" if cents < 0 else ""
    return f"{sign}{abs(cents) // 100}.{abs(cents) % 100:02d}"


class StatementError(Exception):
    def __init__(self, path, line, reason):
        super().__init__(f"{path}:{line}: {reason}")


# Amounts with two decimals and optional thousands groups: Nordbank writes
# -1.234,56 and Kestrel -1,234.56.
NORDBANK_AMOUNT = re.compile(r"-?(\d{1,3}(\.\d{3})+|\d+),\d{2}")
KESTREL_AMOUNT = re.compile(r"-?(\d{1,3}(,\d{3})+|\d+)\.\d{2}")


def cents(text, pattern):
    """The amount `text` in cents, or None if it does not match `pattern`,
    which allows exactly two decimals."""
    if not pattern.fullmatch(text):
        return None
    return int(text.replace(",", "").replace(".", ""))


def iso_date(text, form):
    """`text` in the date form `form` as YYYY-MM-DD, or None."""
    try:
        return datetime.strptime(text, form).date().isoformat()
    except ValueError:
        return None


def read_nordbank(path):
    """The transactions in a Nordbank CSV export."""
    found = []
    with open(path, newline="", encoding="utf-8") as f:
        reader = csv.reader(f, delimiter=";")
        if next(reader, None) != ["Date", "Payee", "Amount"]:
            raise StatementError(path, 1, "expected the header Date;Payee;Amount")
        for row in reader:
            line = reader.line_num
            if not row:
                continue
            if len(row) != 3:
                raise StatementError(path, line, f"expected 3 fields, found {len(row)}")
            date = iso_date(row[0], "%d.%m.%Y")
            if date is None:
                raise StatementError(path, line, f"bad date {row[0]!r}")
            amount = cents(row[2], NORDBANK_AMOUNT)
            if amount is None:
                raise StatementError(path, line, f"bad amount {row[2]!r}")
            if not row[1].strip():
                raise StatementError(path, line, "no payee")
            found.append(Transaction(date, amount, row[1].strip()))
    return found


def read_kestrel(path):
    """The transactions in a Kestrel Bank QIF export."""
    found, record = [], {}
    with open(path, encoding="utf-8") as f:
        for line, text in enumerate(f, start=1):
            text = text.rstrip("\r\n")
            if line == 1:
                if text != "!Type:Bank":
                    raise StatementError(path, 1, "expected the first line !Type:Bank")
                continue
            if not text:
                continue
            code, value = text[0], text[1:]
            if code == "^":
                found.append(kestrel_transaction(path, line, record))
                record = {}
            elif code in "DTP":
                if code in record:
                    raise StatementError(path, line, f"a second {code} line in one record")
                record[code] = (value, line)
    if record:
        raise StatementError(path, min(at for _, at in record.values()),
                             "the last record is not ended by ^")
    return found


def kestrel_transaction(path, end, record):
    """The transaction of a QIF record, from its D, T and P lines, which
    end at line `end`."""
    for code, name in (("D", "date"), ("T", "amount"), ("P", "payee")):
        if code not in record:
            raise StatementError(path, end, f"the record has no {code} ({name}) line")
    (date_text, date_line), (amount_text, amount_line), (payee, payee_line) = (
        record["D"], record["T"], record["P"])
    date = iso_date(date_text, "%m/%d/%Y")
    if date is None:
        raise StatementError(path, date_line, f"bad date {date_text!r}")
    amount = cents(amount_text, KESTREL_AMOUNT)
    if amount is None:
        raise StatementError(path, amount_line, f"bad amount {amount_text!r}")
    if not payee.strip():
        raise StatementError(path, payee_line, "no payee")
    return Transaction(date, amount, payee.strip())


READERS = {".csv": read_nordbank, ".qif": read_kestrel}


def read_statement(path):
    reader = READERS.get(Path(path).suffix.lower())
    if reader is None:
        raise StatementError(path, 1, "unknown format: Nordbank's is .csv, Kestrel's .qif")
    return reader(path)


def merge(ledger, statements):
    """The ledger with each statement's new transactions added, and how many
    were added and skipped. Within one statement, equal transactions are
    distinct; each one is skipped only while the ledger, with the statements
    before it, holds an equal one not yet matched."""
    ledger = list(ledger)
    added = skipped = 0
    for statement in statements:
        unmatched = Counter(ledger)
        for t in statement:
            if unmatched[t]:
                unmatched[t] -= 1
                skipped += 1
            else:
                ledger.append(t)
                added += 1
    return ledger, added, skipped


def main(argv=None):
    parser = argparse.ArgumentParser(prog="budget.py")
    parser.add_argument("--ledger", default="ledger.json")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("summary", help="count the transactions, and the money in and out")
    imp = sub.add_parser("import", help="add the transactions of bank statements")
    imp.add_argument("files", nargs="+", metavar="FILE")
    args = parser.parse_args(argv)

    ledger = load_ledger(args.ledger)
    if args.command == "import":
        try:
            statements = [read_statement(path) for path in args.files]
        except (StatementError, OSError) as e:
            print(e, file=sys.stderr)
            return 1
        ledger, added, skipped = merge(ledger, statements)
        save_ledger(args.ledger, ledger)
        print(f"imported {added}, skipped {skipped} already in the ledger")
        return 0
    money_in = sum(t.cents for t in ledger if t.cents > 0)
    money_out = sum(t.cents for t in ledger if t.cents < 0)
    print(f"{len(ledger)} transactions, in {euros(money_in)}, out {euros(money_out)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
