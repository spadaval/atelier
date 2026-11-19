import contextlib
import io
import unittest
from pathlib import Path

import expenses
from expenses import Expense, monthly_totals

HERE = Path(__file__).resolve().parent


def run(*argv):
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        code = expenses.main([*argv, "--file", str(HERE / "expenses.json")])
    return code, out.getvalue()


class TotalsTest(unittest.TestCase):
    def test_totals_count_only_the_month(self):
        rows = [Expense("2026-10-01", "food", 100), Expense("2026-10-31", "food", 50),
                Expense("2026-11-01", "food", 999), Expense("2026-10-15", "rent", 7000)]
        self.assertEqual(monthly_totals(rows, "2026-10"), {"food": 150, "rent": 7000})

    def test_report_prints_largest_first(self):
        code, out = run("report", "2026-10")
        self.assertEqual(code, 0)
        self.assertEqual([line.split()[0] for line in out.splitlines()],
                         ["rent", "travel", "groceries", "books"])
        self.assertIn("950.00", out)


if __name__ == "__main__":
    unittest.main()
