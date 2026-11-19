import io
import json
import unittest

import exporters
from exporters import Exporter, get_exporter, register

COLUMNS = ("category", "amount")
ROWS = [("rent", "950.00"), ("books", "24.99")]


def written(name, **options):
    out = io.StringIO()
    get_exporter(name, **options).write(COLUMNS, ROWS, out)
    return out.getvalue()


class ExporterTest(unittest.TestCase):
    def test_every_format_is_registered(self):
        self.assertEqual(exporters.available_formats(), ["csv", "json", "markdown", "tsv"])

    def test_csv_quotes_a_comma(self):
        out = io.StringIO()
        get_exporter("csv").write(COLUMNS, [("food, drink", "1.00")], out)
        self.assertEqual(out.getvalue(), 'category,amount\n"food, drink",1.00\n')

    def test_tsv(self):
        self.assertEqual(written("tsv"), "category\tamount\nrent\t950.00\nbooks\t24.99\n")

    def test_json(self):
        self.assertEqual(json.loads(written("json")),
                         [{"category": "rent", "amount": "950.00"},
                          {"category": "books", "amount": "24.99"}])

    def test_markdown_escapes_a_pipe(self):
        out = io.StringIO()
        get_exporter("markdown").write(COLUMNS, [("a|b", "1.00")], out)
        self.assertIn("| a\\|b | 1.00 |", out.getvalue())

    def test_an_unknown_format_names_the_known_ones(self):
        with self.assertRaisesRegex(ValueError, "csv, json, markdown, tsv"):
            get_exporter("xml")

    def test_a_format_registers_once(self):
        with self.assertRaises(ValueError):
            @register("csv")
            class Again(Exporter):
                def write(self, columns, rows, out):
                    pass


if __name__ == "__main__":
    unittest.main()
