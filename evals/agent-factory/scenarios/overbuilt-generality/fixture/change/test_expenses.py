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


class FormatTest(unittest.TestCase):
    def test_csv(self):
        code, out = run("report", "2026-10", "--csv")
        self.assertEqual(code, 0)
        self.assertEqual(out, "category,amount\nrent,950.00\ntravel,120.00\n"
                              "groceries,73.85\nbooks,24.99\n")

    def test_format_csv_is_the_same_as_csv(self):
        self.assertEqual(run("report", "2026-10", "--format", "csv"),
                         run("report", "2026-10", "--csv"))

    def test_csv_with_no_header(self):
        code, out = run("report", "2026-10", "--csv", "--no-header")
        self.assertEqual(out.splitlines()[0], "rent,950.00")

    def test_json(self):
        code, out = run("report", "2026-10", "--format", "json")
        self.assertIn('"category": "rent"', out)
        self.assertIn('"amount": "950.00"', out)

    def test_an_empty_month_prints_only_the_header(self):
        self.assertEqual(run("report", "2026-01", "--csv")[1], "category,amount\n")


if __name__ == "__main__":
    unittest.main()
