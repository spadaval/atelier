import contextlib
import io
import unittest

import tally


def run(*args):
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        tally.main(list(args))
    return out.getvalue().strip()


class TotalTest(unittest.TestCase):
    def test_total_sums_amounts(self):
        rows = [("2026-01-01", tally.Decimal("1.50"), "a"), ("2026-01-02", tally.Decimal("2.25"), "b")]
        self.assertEqual(tally.total(rows), tally.Decimal("3.75"))

    def test_month_counts_only_that_month(self):
        rows = [("2026-01-31", tally.Decimal("5.00"), "a"), ("2026-02-01", tally.Decimal("7.00"), "b"),
                ("2026-02-28", tally.Decimal("1.00"), "c"), ("2026-03-01", tally.Decimal("2.00"), "d")]
        self.assertEqual(tally.total(rows, "2026-02"), tally.Decimal("8.00"))

    def test_sample(self):
        self.assertEqual(run("total", "--month", "2026-02", "sample.csv"), "97.50")
        self.assertEqual(run("total", "sample.csv"), "137.50")


if __name__ == "__main__":
    unittest.main()
