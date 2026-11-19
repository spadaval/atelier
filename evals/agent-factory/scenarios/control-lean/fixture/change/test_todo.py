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


class DoneTest(TodoCase):
    def setUp(self):
        super().setUp()
        for title in ("Fix the gate", "Paint the shed", "Oil the hinge"):
            self.run_cli("add", title)

    def test_done_hides_the_task_and_again_changes_nothing(self):
        self.assertEqual(self.run_cli("done", "2"), (0, "", ""))
        self.assertEqual(self.run_cli("done", "2"), (0, "", ""))
        self.assertEqual(self.run_cli("list")[1], "1. Fix the gate\n3. Oil the hinge\n")

    def test_done_on_a_missing_task(self):
        self.assertEqual(self.run_cli("done", "9"), (1, "", "no task 9\n"))


if __name__ == "__main__":
    unittest.main()
