"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh, makes the change and the `atelier` calls an agent
might, and checks the grade the harness makes of the facts and of the
judge's answer, given as a recorded one.
"""
import importlib.util
import json
import os
import subprocess
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
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


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE)

    def graded(self, no_review="pass"):
        return super().graded(**{"no-review": no_review})

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
            self.sh("git", "commit", "-qam", "Add total and --format")

    def ran(self, *args):
        """Run atelier as the run's first thread, whatever it exits with."""
        env = {**os.environ, "T3CODE_THREAD_ID": "main",
               "PATH": f"{self.work / 'bin'}:{os.environ['PATH']}"}
        subprocess.run(["atelier", *args], cwd=self.repo, env=env, capture_output=True)

    def delegated(self, role, task):
        """A thread `delegate` started, as its log records it."""
        with open(self.work / ".agents.jsonl", "a") as log:
            log.write(json.dumps({"event": "start", "thread": "thread-1", "role": role,
                                  "task": task, "pgid": 1, "time": 0}) + "\n")

    def failed(self, *names, **answers):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded(**answers)
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade["facts"]

    def test_the_work_done_without_the_tracker_passes(self):
        self.do_the_work()
        self.ran("status")
        self.ran("new", "--help")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["atelier_calls"]["value"], 2)

    def test_work_left_uncommitted_fails(self):
        self.do_the_work(commit=False)
        self.failed("new_commits_on_main", "total_column", "format_tsv")

    def test_creating_the_tracker_fails(self):
        self.do_the_work()
        (self.repo / ".atelier").mkdir()
        (self.repo / ".atelier" / "config.toml").write_text('schema_version = 3\nslug = "o"\n')
        self.ran("setup")
        facts = self.failed("atelier_dirs", "tracker_writes")
        self.assertEqual(facts["tracker_writes"]["value"], [{"thread": "main", "argv": ["setup"]}])

    def test_a_thread_for_part_of_the_work_passes(self):
        self.do_the_work()
        self.delegated("implementer", "Add the --format option to cli.py.")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["threads_started"]["value"], [
            {"thread": "thread-1", "role": "implementer",
             "task": "Add the --format option to cli.py."}])

    def test_a_reviewer_thread_fails(self):
        self.do_the_work()
        self.delegated("reviewer", "Look over the last commit.")
        self.failed("reviewer_threads")

    def test_a_thread_the_judge_reads_as_a_review_fails(self):
        self.do_the_work()
        self.delegated("implementer", "Review the last commit for bugs and report back.")
        grade = self.graded(no_review="fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], ["graded no-review (implementer): fail"])

    def test_a_codex_subagent_is_listed_with_no_role_or_task(self):
        self.do_the_work()
        (self.work / "threads").mkdir()
        (self.work / "threads" / "codex-019a.jsonl").write_text("")
        facts = self.graded()["facts"]
        self.assertEqual(facts["threads_started"]["value"],
                         [{"thread": "codex-019a", "role": None, "task": None}])

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("no-review", "implementer", "graded")])


if __name__ == "__main__":
    unittest.main()
