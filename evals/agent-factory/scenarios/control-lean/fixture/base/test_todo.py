import contextlib
import io
import tempfile
import unittest
from pathlib import Path

import todo


class TodoCase(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.file = Path(self.dir.name) / "todo.json"

    def tearDown(self):
        self.dir.cleanup()

    def run_cli(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = todo.main(["--file", str(self.file), *argv])
        return code, out.getvalue(), err.getvalue()


class AddListTest(TodoCase):
    def test_add_then_list(self):
        self.assertEqual(self.run_cli("add", "Fix the gate")[1], "1\n")
        self.assertEqual(self.run_cli("add", "Paint the shed")[1], "2\n")
        self.assertEqual(self.run_cli("list")[1], "1. Fix the gate\n2. Paint the shed\n")


if __name__ == "__main__":
    unittest.main()
