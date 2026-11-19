import contextlib
import io
import tempfile
import unittest
from pathlib import Path

import budget
from budget import Transaction


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


class SummaryTest(LedgerCase):
    def test_summary(self):
        budget.save_ledger(self.ledger, [Transaction("2026-10-01", 250000, "Employer"),
                                         Transaction("2026-10-02", -4250, "Grocer"),
                                         Transaction("2026-10-03", -105, "Bakery")])
        self.assertEqual(self.run_cli("summary"),
                         (0, "3 transactions, in 2500.00, out -43.55\n", ""))

    def test_an_empty_ledger(self):
        self.assertEqual(self.run_cli("summary")[1], "0 transactions, in 0.00, out 0.00\n")


if __name__ == "__main__":
    unittest.main()
