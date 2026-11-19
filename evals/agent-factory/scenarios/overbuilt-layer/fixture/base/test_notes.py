import contextlib
import io
import tempfile
import unittest
from pathlib import Path

import cli


class CliCase(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.file = str(Path(self.dir.name) / "notes.json")

    def tearDown(self):
        self.dir.cleanup()

    def run_cli(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = cli.main(["--file", self.file, *argv])
        return code, out.getvalue(), err.getvalue()


class NotesTest(CliCase):
    def test_add_then_list(self):
        self.assertEqual(self.run_cli("add", "buy milk")[1], "1\n")
        self.assertEqual(self.run_cli("add", "call Ana")[1], "2\n")
        self.assertEqual(self.run_cli("list")[1], "1: buy milk\n2: call Ana\n")


if __name__ == "__main__":
    unittest.main()
