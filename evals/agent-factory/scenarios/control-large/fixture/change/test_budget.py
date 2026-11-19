import contextlib
import io
import tempfile
import unittest
from pathlib import Path

import budget
from budget import Transaction

NORDBANK = """Date;Payee;Amount
01.10.2026;Employer;2.500,00
02.10.2026;Grocer;-42,50
03.10.2026;Café Lumen;-3,20
03.10.2026;Café Lumen;-3,20
"""
KESTREL = """!Type:Bank
D10/04/2026
T-1,234.56
PLandlord
NRent October
^
PHardware Store
MShelf brackets
T-12.99
D10/05/2026
^
"""


class LedgerCase(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.root = Path(self.dir.name)
        self.ledger = self.root / "ledger.json"

    def tearDown(self):
        self.dir.cleanup()

    def run_cli(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = budget.main(["--ledger", str(self.ledger), *argv])
        return code, out.getvalue(), err.getvalue()

    def file(self, name, text):
        path = self.root / name
        path.write_text(text, encoding="utf-8")
        return str(path)


class SummaryTest(LedgerCase):
    def test_summary(self):
        budget.save_ledger(self.ledger, [Transaction("2026-10-01", 250000, "Employer"),
                                         Transaction("2026-10-02", -4250, "Grocer"),
                                         Transaction("2026-10-03", -105, "Bakery")])
        self.assertEqual(self.run_cli("summary"),
                         (0, "3 transactions, in 2500.00, out -43.55\n", ""))

    def test_an_empty_ledger(self):
        self.assertEqual(self.run_cli("summary")[1], "0 transactions, in 0.00, out 0.00\n")


class FormatTest(LedgerCase):
    def test_nordbank_csv(self):
        code, out, _ = self.run_cli("import", self.file("oct.csv", NORDBANK))
        self.assertEqual((code, out), (0, "imported 4, skipped 0 already in the ledger\n"))
        self.assertEqual(budget.load_ledger(self.ledger), [
            Transaction("2026-10-01", 250000, "Employer"),
            Transaction("2026-10-02", -4250, "Grocer"),
            Transaction("2026-10-03", -320, "Café Lumen"),
            Transaction("2026-10-03", -320, "Café Lumen")])

    def test_kestrel_qif_in_any_order_ignoring_other_lines(self):
        self.assertEqual(self.run_cli("import", self.file("oct.qif", KESTREL))[0], 0)
        self.assertEqual(budget.load_ledger(self.ledger), [
            Transaction("2026-10-04", -123456, "Landlord"),
            Transaction("2026-10-05", -1299, "Hardware Store")])

    def test_another_extension_is_refused(self):
        path = self.file("oct.ofx", "")
        self.assertEqual(self.run_cli("import", path),
                         (1, "", f"{path}:1: unknown format: Nordbank's is .csv, Kestrel's .qif\n"))


class OnceTest(LedgerCase):
    def test_importing_again_adds_none_and_repeats_are_kept(self):
        path = self.file("oct.csv", NORDBANK)
        self.run_cli("import", path)
        self.assertEqual(self.run_cli("import", path)[1],
                         "imported 0, skipped 4 already in the ledger\n")
        self.assertEqual(len(budget.load_ledger(self.ledger)), 4)

    def test_an_overlapping_export_adds_only_the_new_ones(self):
        self.run_cli("import", self.file("early.csv", NORDBANK))
        later = "Date;Payee;Amount\n03.10.2026;Café Lumen;-3,20\n06.10.2026;Grocer;-18,00\n"
        self.assertEqual(self.run_cli("import", self.file("later.csv", later))[1],
                         "imported 1, skipped 1 already in the ledger\n")

    def test_one_at_a_time_or_together_is_the_same(self):
        later = "Date;Payee;Amount\n03.10.2026;Café Lumen;-3,20\n06.10.2026;Grocer;-18,00\n"
        files = [self.file("early.csv", NORDBANK), self.file("later.csv", later),
                 self.file("oct.qif", KESTREL)]
        for path in files:
            self.run_cli("import", path)
        one_at_a_time = budget.load_ledger(self.ledger)
        self.ledger.unlink()
        self.run_cli("import", *files)
        self.assertEqual(budget.load_ledger(self.ledger), one_at_a_time)


class BadLineTest(LedgerCase):
    def test_a_bad_line_names_its_file_and_line_and_changes_nothing(self):
        budget.save_ledger(self.ledger, [Transaction("2026-09-30", -500, "Bakery")])
        before = self.ledger.read_text()
        bad = self.file("bad.qif", "!Type:Bank\nD10/04/2026\nT-12,34\nPLandlord\n^\n")
        code, out, err = self.run_cli("import", self.file("oct.csv", NORDBANK), bad)
        self.assertEqual((code, out, err), (1, "", f"{bad}:3: bad amount '-12,34'\n"))
        self.assertEqual(self.ledger.read_text(), before)

    def test_bad_lines(self):
        for name, text, error in [
                ("a.csv", "Date;Amount\n", "1: expected the header Date;Payee;Amount"),
                ("b.csv", NORDBANK + "31.02.2026;Grocer;-1,00\n", "6: bad date '31.02.2026'"),
                ("c.csv", NORDBANK + "07.10.2026;Grocer\n", "6: expected 3 fields, found 2"),
                ("d.qif", "!Type:Bank\nD10/04/2026\nT-1.00\n^\n", "4: the record has no P (payee) line"),
                ("e.qif", "!Type:Bank\nD10/04/2026\nT-1.00\nPX\n", "2: the last record is not ended by ^"),
                ("f.qif", "!Type:Bank\nD10/04/2026\nD10/05/2026\n", "3: a second D line in one record"),
                ("g.qif", "", "1: expected the first line !Type:Bank"),
                ("h.qif", "!Type:Bank\nD10/04/2026\nT-1.00\nPX\n^X\n", "5: a record ends with a line of ^ alone"),
                ("i.csv", 'Date;Payee;Amount\n01.10.2026;"Grocer"x;-1,00\n', "2: bad quoting"),
                ("j.csv", "", "1: expected the header Date;Payee;Amount"),
                ("k.qif", "!Type:Bank\nD10/04/2026\nT-1.00\nPX\n^\nMa memo\n",
                 "6: the last record is not ended by ^"),
                ("l.csv", "Date;Payee;Amount\n01.10.2026;X;-" + "9" * 5000 + ",00\n",
                 "2: bad amount"),
                ("m.csv", 'Date;Payee;Amount\n31.02.2026;"Cafe\nNord";-1,00\n',
                 "2: bad date '31.02.2026'"),
                ("p.csv", 'Date;Payee;Amount\n01.10.2026;"Cafe\r\nNord";bad\n',
                 "3: bad amount 'bad'"),
                ("n.csv", 'Date;Payee;Amount\n01.10.2026;X;-1,00\n02.10.2026;"Cafe;-1,00\n',
                 "3: bad quoting"),
                ("o.qif", "!Type:Bank\rD10/04/2026\rT-1.00\rPX\r^\rDsoon\r",
                 "6: the last record is not ended by ^")]:
            with self.subTest(name=name):
                path = self.file(name, text)
                code, out, err = self.run_cli("import", path)
                self.assertEqual((code, out), (1, ""))
                self.assertTrue(err.startswith(f"{path}:{error}"), err)
                self.assertFalse(self.ledger.exists())

    def test_a_payee_is_kept_as_written(self):
        payee = "Kiosk\u2028Nord "
        self.run_cli("import", self.file("a.qif", f"!Type:Bank\nD10/04/2026\nT-1.00\nP{payee}\n^\n"))
        self.assertEqual(budget.load_ledger(self.ledger), [Transaction("2026-10-04", -100, payee)])

    def test_a_byte_that_is_not_utf8_names_its_line(self):
        for ending in ("\n", "\r", "\r\n"):
            with self.subTest(ending=repr(ending)):
                path = self.root / "oct.csv"
                path.write_bytes(NORDBANK.replace("\n", ending).encode()
                                 + b"07.10.2026;Caf\xe9;-1,00" + ending.encode())
                self.assertEqual(self.run_cli("import", str(path)),
                                 (1, "", f"{path}:6: not UTF-8\n"))

    def test_any_line_ending_reads_alike(self):
        for ending in ("\r", "\r\n"):
            with self.subTest(ending=repr(ending)):
                files = [self.file("oct.csv", NORDBANK.replace("\n", ending)),
                         self.file("oct.qif", KESTREL.replace("\n", ending))]
                self.assertEqual(self.run_cli("import", *files)[0], 0)
                self.assertEqual(len(budget.load_ledger(self.ledger)), 6)
                self.ledger.unlink()

    def test_a_payee_may_hold_a_line_break(self):
        text = 'Date;Payee;Amount\n01.10.2026;"Cafe\nNord";-1,00\n02.10.2026;X;-2,00\n'
        self.assertEqual(self.run_cli("import", self.file("a.csv", text))[0], 0)
        self.assertEqual([t.payee for t in budget.load_ledger(self.ledger)], ["Cafe\nNord", "X"])

    def test_a_large_amount_within_the_format_is_read(self):
        self.run_cli("import", self.file("a.csv", "Date;Payee;Amount\n01.10.2026;X;-1234567890123,00\n"))
        self.assertEqual(budget.load_ledger(self.ledger)[0].cents, -123456789012300)


if __name__ == "__main__":
    unittest.main()
