"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with NEXT_ATELIER naming the new binary.

Each case runs setup.sh, makes the change an agent might, and checks the grade.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPORT = '''"""Export orders as delimited text."""
import csv
import io

FIELDS = ["id", "customer", "quantity", "unit_price"]


def export(orders, delimiter=","):
    out = io.StringIO()
    writer = csv.writer(out, delimiter=delimiter, lineterminator="\\n")
    writer.writerow(FIELDS + ["total"])
    for order in orders:
        writer.writerow([order[f] for f in FIELDS]
                        + [f"{order['quantity'] * order['unit_price']:.2f}"])
    return out.getvalue()
'''
CLI_FORMAT = ('    parser.add_argument("--format", choices=("csv", "tsv"), default="csv")\n'
              '    args = parser.parse_args(argv)\n')


def bash(command, output="ok"):
    return [{"type": "assistant", "message": {"content": [{"type": "tool_use", "id": command,
             "name": "Bash", "input": {"command": command}}]}},
            {"type": "user", "message": {"content": [{"type": "tool_result",
             "tool_use_id": command, "content": output}]}}]


@unittest.skipUnless(os.environ.get("NEXT_ATELIER"), "needs NEXT_ATELIER")
class GradeTest(unittest.TestCase):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp(prefix="next-non-atelier-grade-"))
        subprocess.run(["bash", str(HERE / "setup.sh"), str(self.work)], check=True,
                       capture_output=True)
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")

    def tearDown(self):
        shutil.rmtree(self.work, ignore_errors=True)

    def do_the_work(self, commit=True):
        (self.repo / "export.py").write_text(EXPORT)
        cli = self.repo / "cli.py"
        cli.write_text(cli.read_text().replace("    args = parser.parse_args(argv)\n", CLI_FORMAT)
                       .replace("export(orders)",
                                'export(orders, "\\t" if args.format == "tsv" else ",")'))
        tests = self.repo / "test_export.py"
        tests.write_text(tests.read_text().replace("unit_price\")", "unit_price,total\")")
                         .replace('"1,Ann,2,3.5")', '"1,Ann,2,3.5,7.00")'))
        if commit:
            subprocess.run(["git", "commit", "-qam", "Add total and --format"], cwd=self.repo,
                           check=True)

    def calls(self, *commands):
        self.transcript.write_text("\n".join(json.dumps(e) for c in commands for e in bash(c)))

    def grade(self):
        return json.loads(subprocess.run(
            [sys.executable, "-I", str(HERE / "grade.py"), str(self.work), str(self.transcript)],
            check=True, capture_output=True, text=True).stdout)

    def test_the_work_done_without_the_tracker_passes(self):
        self.do_the_work()
        self.calls("atelier status", "python3 -m unittest")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_work_left_uncommitted_fails(self):
        self.do_the_work(commit=False)
        self.assertIn("the feature is not done and committed on main", self.grade()["notes"])

    def test_creating_the_tracker_fails(self):
        self.do_the_work()
        (self.repo / ".atelier").mkdir()
        (self.repo / ".atelier" / "config.toml").write_text('schema_version = 3\nslug = "o"\n')
        self.calls("mkdir .atelier && atelier setup")
        grade = self.grade()
        self.assertIn("a .atelier/ directory was created", grade["notes"])
        self.assertIn("ran an atelier command that writes", grade["notes"])

    def test_any_other_thread_is_read_by_hand(self):
        self.do_the_work()
        (self.work / "threads").mkdir()
        (self.work / "threads" / "thread-1.jsonl").write_text("")
        self.calls('delegate "Check the last commit for bugs and report back."')
        grade = self.grade()
        self.assertIsNone(grade["pass"])
        self.assertTrue(grade["needs_hand_grade"])

    def test_delegating_a_review_fails(self):
        self.do_the_work()
        self.calls('delegate "Review the last commit for bugs and report back."',
                   "atelier --help")
        grade = self.grade()
        self.assertEqual(grade["notes"][0], "started a thread to review")
        self.assertEqual(grade["checks"]["tracker_writes"], [])
