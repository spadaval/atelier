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


class TagCliTest(CliCase):
    def setUp(self):
        super().setUp()
        for text in ("buy milk", "call Ana", "draft the plan"):
            self.run_cli("add", text)

    def test_tag_once(self):
        self.assertEqual(self.run_cli("tag", "3", "work")[0], 0)
        self.assertEqual(self.run_cli("tag", "3", "work")[0], 0)
        self.assertEqual(self.run_cli("list")[1],
                         "1: buy milk\n2: call Ana\n3: draft the plan #work\n")

    def test_list_with_a_tag(self):
        self.run_cli("tag", "2", "work")
        self.run_cli("tag", "3", "work")
        self.run_cli("tag", "1", "home")
        self.assertEqual(self.run_cli("list", "--tag", "work")[1],
                         "2: call Ana #work\n3: draft the plan #work\n")

    def test_tag_a_missing_note(self):
        before = Path(self.file).read_text()
        self.assertEqual(self.run_cli("tag", "7", "work"), (1, "", "no note 7\n"))
        self.assertEqual(Path(self.file).read_text(), before)
