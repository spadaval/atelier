"""A household ledger kept in ledger.json.

    python3 budget.py summary
    python3 budget.py import FILE...

`summary` prints how many transactions the ledger holds, and the money in
and out. `import` adds the transactions of Nordbank's CSV and Kestrel Bank's
QIF statements, skipping those already in the ledger.
"""
import argparse
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
# A line ends at CRLF, CR or LF.
LINE_END = re.compile(r"\r\n|\r|\n")
# A Nordbank field: quoted, with "" for a quote inside, or holding no quote.
FIELD = re.compile(r'"((?:[^"]|"")*)"|[^;"\r\n]*')


def cents(text, pattern):
    """The amount `text` in cents, or None if it does not match `pattern`,
    which allows exactly two decimals."""
    if not pattern.fullmatch(text):
        return None
    try:
        return int(text.replace(",", "").replace(".", ""))
    except ValueError:  # more digits than Python converts
        return None


def iso_date(text, form):
    """`text` in the date form `form` as YYYY-MM-DD, or None."""
    try:
        return datetime.strptime(text, form).date().isoformat()
    except ValueError:
        return None


def read_text(path):
    """The file at `path` as text; a byte that is not UTF-8 is an error on
    its line."""
    data = Path(path).read_bytes()
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError as e:
        line = len(LINE_END.findall(data[:e.start].decode("utf-8"))) + 1
        raise StatementError(path, line, "not UTF-8") from None


def nordbank_records(path, text):
    """Each record of a Nordbank export, as a list of (field, line) with the
    line each field starts on; a blank line is none. A quote anywhere but
    around a whole field is an error on the line of the field it is in."""
    pos, line = 0, 1
    while pos < len(text):
        record = []
        while True:
            field = FIELD.match(text, pos)
            quoted = field.group(1)
            record.append((quoted.replace('""', '"') if quoted is not None else field.group(0), line))
            line += len(LINE_END.findall(field.group(0)))
            pos = field.end()
            if pos == len(text):
                break
            if text[pos] == ";":
                pos += 1
                continue
            end = LINE_END.match(text, pos)
            if end is None:
                raise StatementError(path, record[-1][1], "bad quoting")
            pos, line = end.end(), line + 1
            break
        if field.group(0) or len(record) > 1:
            yield record


def read_nordbank(path):
    """The transactions in a Nordbank CSV export."""
    found = []
    records = nordbank_records(path, read_text(path))
    header = next(records, None)
    if header is None or [value for value, _ in header] != ["Date", "Payee", "Amount"]:
        raise StatementError(path, header[0][1] if header else 1,
                             "expected the header Date;Payee;Amount")
    for record in records:
        if len(record) != 3:
            raise StatementError(path, record[0][1], f"expected 3 fields, found {len(record)}")
        (date_text, date_line), (payee, payee_line), (amount_text, amount_line) = record
        date = iso_date(date_text, "%d.%m.%Y")
        if date is None:
            raise StatementError(path, date_line, f"bad date {date_text!r}")
        amount = cents(amount_text, NORDBANK_AMOUNT)
        if amount is None:
            raise StatementError(path, amount_line, f"bad amount {amount_text!r}")
        if not payee.strip():
            raise StatementError(path, payee_line, "no payee")
        found.append(Transaction(date, amount, payee))
    return found


def read_kestrel(path):
    """The transactions in a Kestrel Bank QIF export."""
    found, record, opened = [], {}, None
    lines = LINE_END.split(read_text(path))
    if lines[0] != "!Type:Bank":
        raise StatementError(path, 1, "expected the first line !Type:Bank")
    for line, text in enumerate(lines[1:], start=2):
        if not text:
            continue
        code, value = text[0], text[1:]
        if code == "^":
            if value:
                raise StatementError(path, line, "a record ends with a line of ^ alone")
            found.append(kestrel_transaction(path, line, record))
            record, opened = {}, None
            continue
        if code in "DTP":
            if code in record:
                raise StatementError(path, line, f"a second {code} line in one record")
            record[code] = (value, line)
            opened = opened or line
    if opened:
        raise StatementError(path, opened, "the last record is not ended by ^")
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
    return Transaction(date, amount, payee)


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
