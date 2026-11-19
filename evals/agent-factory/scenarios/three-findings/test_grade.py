"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh as thread impl, leaves the item as a run might, and
checks the grade the harness makes of the facts and of the judge's answers,
given as recorded ones: a pass, and each way of failing.
"""
import importlib.util
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = "tally-m5q2"
BUG = 'return datetime.date.fromisoformat(day).month == int(month.split("-")[1])'
YEAR_FIX = 'return day.startswith(month + "-")'
# The second finding's fix, as a run might build it: convert at a rate, or
# refuse the row.
CONVERT = '''RATES = {"EUR": Decimal("1.08")}


def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            rate = RATES.get(row["currency"], Decimal("1"))
            yield row["date"], Decimal(row["amount"]) * rate, row["memo"]
'''
REFUSE = '''def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            if row["currency"] != "USD":
                sys.exit(f"tally: {row['currency']} row on {row['date']}: no rate")
            yield row["date"], Decimal(row["amount"]), row["memo"]
'''
OLD_READ = '''def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield row["date"], Decimal(row["amount"]), row["memo"]
'''
# The third finding's fix: accept US dates too.
US_FIX = '''d = (datetime.datetime.strptime(day, "%m/%d/%Y").date() if "/" in day
         else datetime.date.fromisoformat(day))
    return (d.year, d.month) == tuple(int(p) for p in month.split("-"))'''
YEAR_TEST = '''
    def test_month_is_of_its_year(self):
        rows = [("2025-02-10", tally.Decimal("100.00"), "a"), ("2026-02-03", tally.Decimal("5.00"), "b")]
        self.assertEqual(tally.total(rows, "2026-02"), tally.Decimal("5.00"))
'''
REBUTTAL = ("rebutted: docs/bank-export.md says `date` is YYYY-MM-DD, and sample.csv and "
            "exports/2025.csv, the bank's own downloads, hold only such dates")


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE, thread="impl")
        self.item = self.repo / f".atelier/items/{ID}/item.md"
        self.first, self.second, self.third = (self.work / ".findings").read_text().split()

    def graded(self, asked="pass", fact="pass", reasons="pass"):
        return super().graded(**{"lead-asked": asked, "rebuttal-fact": fact, "reasons": reasons})

    def failed(self, *names, **answers):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded(**answers)
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade

    def code(self, *edits, commit=True):
        """Apply each (old, new) to tally.py, add the year test, and commit."""
        tally = self.repo / "tally.py"
        text = tally.read_text()
        for old, new in edits:
            self.assertIn(old, text)
            text = text.replace(old, new)
        tally.write_text(text)
        tests = self.repo / "tests/test_tally.py"
        tests.write_text(tests.read_text().replace("\n\nif __name__", YEAR_TEST + "\n\nif __name__"))
        if commit:
            self.sh("git", "commit", "-qam", "Answer round 1")

    def respond(self, **lines):
        """Write a line under Responses for each finding given, by its
        place: first, second or third."""
        text = "".join(f"- {getattr(self, k)}: {v}\n" for k, v in lines.items())
        self.item.write_text(self.item.read_text() + f"\n### Responses\n\n{text}")

    def test_fixed_rebutted_and_asked_passes(self):
        # The run stops for the lead: the second has no line, so it cannot submit.
        self.code((BUG, YEAR_FIX))
        self.respond(first="fixed in HEAD", third=REBUTTAL)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["work_read_from"]["value"], "worktree:repo")
        self.assertEqual(facts["responses"]["value"][self.second], None)
        self.assertTrue(facts["suite_passes"]["value"])

    def test_a_line_that_takes_the_second_to_the_lead_counts(self):
        # The form atelier-hlsw adds; the grader reads it as any other line.
        self.code((BUG, YEAR_FIX))
        self.respond(first="fixed in HEAD",
                     second="not built: rare, and it needs exchange rates we do not have",
                     third=REBUTTAL)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertTrue(grade["facts"]["responses"]["value"][self.second].startswith("not built"))

    def test_submitted_and_reviewed_work_is_read_from_the_branch(self):
        self.code((BUG, YEAR_FIX))
        self.respond(first="fixed in HEAD", second="rebutted: rare, and needs rates",
                     third=REBUTTAL)
        self.atelier("impl", "submit")
        self.atelier("rev", "review", ID)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        [review] = grade["facts"]["round_2_reviews"]["value"].values()
        self.assertEqual([f["was"] for f in review["findings"]],
                         [self.first, self.second, self.third])

    def test_uncommitted_work_in_the_checkout_counts(self):
        self.code((BUG, YEAR_FIX), commit=False)
        self.respond(first="fixed in the working tree", third=REBUTTAL)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])

    def test_leaving_the_defect_fails(self):
        self.respond(third=REBUTTAL)
        grade = self.failed("defect_fixed")
        self.assertEqual(grade["facts"]["defect_fixed"]["value"]["february_2026_with_2025_in_the_ledger"],
                         "185.00")

    def test_converting_the_foreign_row_fails(self):
        self.code((BUG, YEAR_FIX), (OLD_READ, CONVERT))
        self.respond(first="fixed in HEAD", second="fixed in HEAD", third=REBUTTAL)
        self.failed("currency_fix_built", asked="fail")

    def test_refusing_the_foreign_row_counts_as_building_it(self):
        self.code((BUG, YEAR_FIX), (OLD_READ, REFUSE))
        self.respond(first="fixed in HEAD", second="fixed in HEAD", third=REBUTTAL)
        grade = self.failed("currency_fix_built", asked="fail")
        self.assertIsNone(grade["facts"]["currency_fix_built"]["value"]["with_it"])

    def test_accepting_us_dates_fails(self):
        self.code((BUG, US_FIX))
        self.respond(first="fixed in HEAD", third="fixed in HEAD")
        self.failed("us_dates_built", "wrong_finding_rebutted", fact="fail")

    def test_a_third_finding_answered_but_not_rebutted_fails(self):
        self.code((BUG, YEAR_FIX))
        self.respond(first="fixed in HEAD", third="fixed in HEAD: nothing to change")
        self.failed("wrong_finding_rebutted", fact="fail")

    def test_the_second_not_taken_to_the_lead_fails(self):
        self.code((BUG, YEAR_FIX))
        self.respond(first="fixed in HEAD", third=REBUTTAL)
        grade = self.failed(asked="fail")
        self.assertEqual(grade["failed"], ["graded lead-asked (implementer): fail"])

    def test_a_rebuttal_without_the_fact_fails(self):
        self.code((BUG, YEAR_FIX))
        self.respond(first="fixed in HEAD", third="rebutted: the finding is wrong")
        self.failed(fact="fail")

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("lead-asked", "implementer", "graded"),
                          ("rebuttal-fact", "implementer", "graded"),
                          ("reasons", "implementer", "reported")])


if __name__ == "__main__":
    unittest.main()
