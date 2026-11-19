import contextlib
import io
import tempfile
import unittest
from pathlib import Path

import inventory
from inventory import InventoryError, load

HERE = Path(__file__).resolve().parent


def run(*argv):
    out, err = io.StringIO(), io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        code = inventory.main(list(argv))
    return code, out.getvalue(), err.getvalue()


class LoadTest(unittest.TestCase):
    def load_text(self, text):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "stock.csv"
            path.write_text(text)
            return load(path)

    def test_loads_the_sample(self):
        self.assertEqual(len(load(HERE / "stock.csv")), 6)

    def test_refuses_bad_rows(self):
        for row, message in [("BOL-1,bolt,3", "bad SKU"),
                             ("BOL-0001,bolt,-3", "bad quantity"),
                             ("BOL-0001,bolt,three", "bad quantity"),
                             ("BOL-0001,bolt,2.5", "bad quantity"),
                             ("BOL-0001,bolt", "expected 3 fields"),
                             ("BOL-0001,bolt,3\nBOL-0001,bolt,4", "appears twice")]:
            with self.subTest(row=row), self.assertRaisesRegex(InventoryError, message):
                self.load_text("sku,name,quantity\n" + row + "\n")

    def test_summary(self):
        self.assertEqual(run(str(HERE / "stock.csv")), (0, "6 items, 1450 units\n", ""))


class LowCliTest(unittest.TestCase):
    def test_low_lists_lowest_first_then_by_sku(self):
        code, out, _ = run(str(HERE / "stock.csv"), "--low", "5")
        self.assertEqual(code, 0)
        self.assertEqual(out, "6 items, 1450 units\nHNG-0007 0\nBRK-0042 3\n"
                              "WSH-0006 3\nNUT-0001 4\n")

    def test_nothing_below(self):
        self.assertEqual(run(str(HERE / "stock.csv"), "--low", "0")[1],
                         "6 items, 1450 units\nnothing below 0\n")

    def test_low_takes_a_whole_number(self):
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            inventory.main([str(HERE / "stock.csv"), "--low", "2.5"])


if __name__ == "__main__":
    unittest.main()
