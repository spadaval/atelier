import contextlib
import io
import tempfile
import unittest
from pathlib import Path

import inventory
from inventory import InventoryError, Item, load, low_stock

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


class LowStockTest(unittest.TestCase):
    def test_none_is_no_items(self):
        self.assertEqual(low_stock(None, 5), [])

    def test_bad_quantities_are_skipped(self):
        items = [Item("AAA-0001", "a", None), Item("AAA-0002", "b", -1),
                 Item("AAA-0003", "c", "lots"), Item("AAA-0004", "d", float("nan")),
                 Item("AAA-0005", "e", True), Item("AAA-0006", "f", 2)]
        with self.assertLogs("inventory", "WARNING") as logs:
            self.assertEqual(low_stock(items, 5), [("AAA-0006", 2)])
        self.assertEqual(len(logs.output), 5)

    def test_a_quantity_given_as_text_is_read(self):
        self.assertEqual(low_stock([Item("AAA-0001", "a", " 3 ")], 5), [("AAA-0001", 3)])

    def test_a_whole_float_is_read(self):
        self.assertEqual(low_stock([Item("AAA-0001", "a", 3.0)], 5), [("AAA-0001", 3)])

    def test_a_repeated_sku_keeps_its_lowest_quantity(self):
        items = [Item("AAA-0001", "a", 9), Item("AAA-0001", "a", 1)]
        with self.assertLogs("inventory", "WARNING"):
            self.assertEqual(low_stock(items, 5), [("AAA-0001", 1)])


if __name__ == "__main__":
    unittest.main()
