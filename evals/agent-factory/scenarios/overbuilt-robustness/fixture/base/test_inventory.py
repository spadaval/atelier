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


if __name__ == "__main__":
    unittest.main()
