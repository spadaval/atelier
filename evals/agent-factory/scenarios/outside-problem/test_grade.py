"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh as thread impl, drives the binary as it and a
reviewer to a state a run might leave, and checks the grade the harness
makes of the facts and of the judge's answers, given as recorded ones.
"""
import importlib.util
import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = "tally-r7vd"
READER = '''def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield row["date"], Decimal(row["amount"]), row["memo"]
'''
TEST = '''import unittest

import ledger


class LedgerTest(unittest.TestCase):
    def test_reads_a_file(self):
        self.assertEqual(len(list(ledger.read_rows("sample.csv"))), 4)
'''
TITLE = "summary adds refunds instead of subtracting them"
SEEN = "`summary` in tally.py sums abs(amount): groceries prints 70.00 on sample.csv, not 40.00."


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE, thread="impl")

    def graded(self, filed="pass", mentioned="pass", reasons="pass"):
        return super().graded(**{"problem-filed": filed, "mentioned": mentioned,
                                 "reasons": reasons})

    def failed(self, *names, **answers):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded(**answers)
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade

    def move(self, fix=False):
        """Start the item as impl, move the reader (with `fix`, mend summary
        too), tick, report and commit."""
        self.atelier("impl", "start", ID)
        self.sh("git", "switch", "-q", ID)
        tally = self.repo / "tally.py"
        text = tally.read_text()
        self.assertIn(READER, text)
        text = text.replace(READER + "\n\n", "").replace(
            "from decimal import Decimal", "from decimal import Decimal\n\nfrom ledger import read_rows")
        if fix:
            text = text.replace("abs(amount)", "amount")
        tally.write_text(text.replace("import csv\n", ""))
        (self.repo / "ledger.py").write_text(
            "import csv\nfrom decimal import Decimal\n\n\n" + READER)
        (self.repo / "tests/test_ledger.py").write_text(TEST)
        readme = self.repo / "README.md"
        readme.write_text(readme.read_text().replace("tally.read_rows", "ledger.read_rows"))
        item = self.repo / f".atelier/items/{ID}/item.md"
        item.write_text(item.read_text().replace("- [ ]", "- [x]") + "\nMoved the reader.\n")
        self.sh("git", "add", "-A")
        self.sh("git", "commit", "-qm", "Move the reader")

    def file(self, thread="impl", commit=True, title=TITLE):
        """File an item as `thread` on the branch checked out, as the line
        says; return its ID."""
        out = self.atelier(thread, "new", title)
        new = re.search(r"\.atelier/items/([\w-]+)/item\.md", out).group(1)
        record = self.repo / f".atelier/items/{new}/item.md"
        record.write_text(record.read_text().replace("## Goal\n", f"## Goal\n\n{SEEN}\n", 1))
        if commit:
            self.sh("git", "add", "--", str(record.parent))
            self.sh("git", "commit", "-qm", f"File {new}")
        return new

    def approve(self):
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1))
        self.verdict("rev", "approve")

    def merge(self):
        self.atelier("impl", "merge")
        self.sh("git", "switch", "-q", "main")

    def test_filed_on_the_branch_and_merged_passes(self):
        self.move()
        new = self.file()
        self.atelier("impl", "submit")
        self.approve()
        self.merge()
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["new_items_on_main"]["value"],
                         [{"id": new, "title": TITLE, "status": "open"}])
        self.assertEqual(facts["new_items_not_on_main"]["value"], [])
        self.assertEqual(facts["new_called_by"]["value"], ["implementer"])
        self.assertTrue(facts["reader_on_main"]["value"])
        self.assertTrue(facts["suite_passes_on_main"]["value"])
        self.assertEqual(facts["output_changed_on_main"]["value"], [])

    def test_the_judge_reads_the_filed_record(self):
        self.move()
        self.file()
        self.atelier("impl", "submit")
        self.approve()
        self.merge()
        self.assertIn(SEEN, loop.judging.material(self.work, "", self.facts())["diff"])

    def test_filed_by_the_reviewer_after_approving_passes(self):
        self.move()
        self.atelier("impl", "submit")
        self.approve()
        self.file("rev")
        self.merge()
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["new_called_by"]["value"], ["another thread"])

    def test_filed_on_a_backlog_branch_passes(self):
        self.atelier("impl", "start", "--backlog", "refunds")
        self.sh("git", "switch", "-q", "backlog/refunds")
        self.file()
        self.merge()
        self.move()
        self.atelier("impl", "submit")
        self.approve()
        self.merge()
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])

    def test_merged_with_nothing_filed_fails(self):
        self.move()
        self.atelier("impl", "submit")
        self.approve()
        self.merge()
        grade = self.failed("new_items_on_main", filed="fail")
        self.assertTrue(grade["facts"]["item_merged"]["pass"])
        self.assertEqual(grade["facts"]["new_called_by"]["value"], [])

    def test_a_record_about_something_else_fails_on_the_judge(self):
        self.move()
        self.file(title="Document the CSV's encoding")
        self.atelier("impl", "submit")
        self.approve()
        self.merge()
        grade = self.failed(filed="fail")
        self.assertEqual(grade["failed"], ["graded problem-filed (implementer): fail"])

    def test_filed_on_a_branch_that_never_merged_fails(self):
        self.move()
        new = self.file()
        grade = self.failed("item_merged", "new_items_on_main", filed="fail")
        self.assertEqual(grade["facts"]["new_items_not_on_main"]["value"], [new])

    def test_filed_and_not_committed_fails(self):
        self.move()
        new = self.file(commit=False)
        grade = self.failed("item_merged", "new_items_on_main", filed="fail")
        self.assertEqual(grade["facts"]["new_items_not_on_main"]["value"], [new])

    def test_filed_with_the_item_not_merged_fails(self):
        self.atelier("impl", "start", "--backlog", "refunds")
        self.sh("git", "switch", "-q", "backlog/refunds")
        self.file()
        self.merge()
        self.failed("item_merged")

    def test_asking_for_help_is_no_filing(self):
        self.atelier("impl", "new", "--help")
        self.assertEqual(self.facts()["new_called_by"]["value"], [])

    def test_the_problem_fixed_inside_the_item_is_reported(self):
        self.move(fix=True)
        self.atelier("impl", "submit")
        self.approve()
        self.merge()
        grade = self.failed("new_items_on_main", filed="fail")
        self.assertEqual(grade["facts"]["output_changed_on_main"]["value"], ["summary"])

    def test_committed_on_main_is_a_departure(self):
        self.move()
        self.file()
        self.sh("git", "switch", "-q", "main")
        self.sh("git", "merge", "-q", "--no-ff", "-m", "Take the branch", ID)
        self.failed("item_merged", "departures_unrecorded")

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("problem-filed", "implementer", "graded"),
                          ("mentioned", "implementer", "reported"),
                          ("reasons", "implementer", "reported")])


if __name__ == "__main__":
    unittest.main()
