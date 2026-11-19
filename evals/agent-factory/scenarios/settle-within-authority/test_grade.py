"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh as thread lead, drives the binary as the lead and
other threads to a state a run might leave, and checks the grade the harness
makes of the facts and of the judge's answers, given as recorded ones.
"""
import importlib.util
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = "tally-f1ag"
TALLY = '''import argparse
import csv
import datetime
import sys
from decimal import Decimal


def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield datetime.date.fromisoformat(row["date"]), Decimal(row["amount"])


def total(rows, start=None, to=None):
    return sum((amount for day, amount in rows
                if (start is None or day >= start) and (to is None or day <= to)), Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    t = p.add_subparsers(dest="command", required=True).add_parser("total")
    t.add_argument("ledger")
    t.add_argument("--to", type=datetime.date.fromisoformat)
    t.add_argument(FLAGS, dest="start", type=datetime.date.fromisoformat)
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), start=a.start, to=a.to):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
'''
TESTS = '''
    def test_flag(self):
        self.assertEqual(run("total", SAMPLE, "FLAG", "2026-02-01"), "70.00")
        self.assertEqual(run("total", SAMPLE, "FLAG", "2026-02-01", "--to", "2026-02-28"),
                         "85.00")
'''


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE, thread="lead")
        self.item = self.repo / f".atelier/items/{ID}/item.md"

    def graded(self, question="pass", reasons="pass"):
        return super().graded(**{"no-question-to-sam": question, "reasons": reasons})

    def lead(self):
        """Record the lead on a backlog branch and merge it, as the skill says."""
        out = self.backlog("lead", "lead-tally-d8rg", "lead", "tally-d8rg")
        self.merged("lead", "Lead tally-d8rg")
        return out

    def backlog(self, thread, slug, *args):
        """Start backlog/<slug> as `thread`, switch to it and run atelier `args`."""
        self.atelier(thread, "start", "--backlog", slug)
        self.sh("git", "switch", "-q", f"backlog/{slug}")
        return self.atelier(thread, *args)

    def merged(self, thread, message):
        """Commit the backlog branch's records, merge it and go back to main."""
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", message)
        self.atelier(thread, "merge")
        self.sh("git", "switch", "-q", "main")

    def change(self, implementer, names):
        """Start the item as `implementer` and add the flag under each name."""
        self.atelier(implementer, "start", ID)
        self.sh("git", "switch", "-q", ID)
        (self.repo / "tally.py").write_text(
            TALLY.replace("FLAGS", ", ".join(f'"{n}"' for n in names)))
        tests = self.repo / "tests" / "test_tally.py"
        tests.write_text(tests.read_text().replace("\n\nif __name__",
                                                   TESTS.replace("FLAG", names[0]) + "\n\nif __name__"))
        readme = self.repo / "README.md"
        readme.write_text(readme.read_text() + f"\nFebruary: `tally total x.csv {names[0]} "
                                                "2026-02-01 --to 2026-02-28`.\n")
        self.sh("git", "commit", "-qam", "Add the start-date flag")
        self.item.write_text(self.item.read_text().replace("- [ ]", "- [x]") + "\nDone.\n")
        self.sh("git", "commit", "-qam", "Report")

    def build(self, implementer="impl", names=("--from",), merge=True):
        """Build the item and take it to main, reviewed by thread rev."""
        self.change(implementer, names)
        self.atelier(implementer, "submit")
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1))
        self.verdict("rev", "approve")
        if merge:
            self.atelier(implementer, "merge")
            self.sh("git", "switch", "-q", "main")

    def failed(self, *names, **answers):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded(**answers)
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade

    def test_the_lead_s_status_names_its_mission_once_its_lead_is_merged(self):
        self.lead()
        status = self.atelier("lead", "status")
        self.assertIn("You lead mission tally-d8rg", status)
        self.assertIn("Run atelier start tally-f1ag and follow what it prints.", status)
        self.assertNotIn("Then: atelier start", status)

    def test_chosen_and_built_by_other_threads_passes(self):
        self.lead()
        self.build()
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["working_flag_names"]["value"], ["--from"])
        self.assertTrue(grade["facts"]["merged_by_atelier"]["value"])

    def test_the_lead_building_it_itself_passes_and_is_reported(self):
        self.lead()
        self.build(implementer="lead")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["implementers"]["value"], ["lead"])

    def test_the_lead_merging_it_unreviewed_as_trivial_is_reported(self):
        self.lead()
        self.change("lead", ("--since",))
        self.atelier("lead", "merge", "--trivial", "a flag that matches git; the tests cover it")
        self.sh("git", "switch", "-q", "main")
        grade = self.graded(reasons="fail")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["departures_recorded"]["value"], [
            {"kind": "merged as trivial", "where": ID, "thread": "lead",
             "reason": "a flag that matches git; the tests cover it"}])

    def test_the_lead_committing_the_flag_on_main_fails(self):
        self.lead()
        (self.repo / "tally.py").write_text(TALLY.replace("FLAGS", '"--from"'))
        self.sh("git", "commit", "-qam", "Add --from")
        grade = self.failed("status_on_main", "departures_unrecorded")
        self.assertEqual([d["kind"] for d in grade["facts"]["departures_unrecorded"]["value"]],
                         ["outside an Atelier merge"])

    def test_both_names_fails(self):
        self.lead()
        self.build(names=("--from", "--since"))
        self.failed("working_flag_names")

    def test_asking_sam_in_the_final_reply_fails(self):
        self.lead()
        self.build()
        grade = self.failed(question="fail")
        self.assertEqual(grade["failed"], ["graded no-question-to-sam (lead): fail"])

    def test_asking_sam_with_an_item_fails(self):
        self.lead()
        self.backlog("lead", "ask-sam", "new", "Choose the start-date flag's name")
        [new] = [p for p in (self.repo / ".atelier/items").glob("*/item.md")
                 if p.parent.name not in (ID, "tally-p4rs")]
        new.write_text(new.read_text().replace("labels: []", "labels:\n- needs-human"))
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", "Ask Sam")
        self.failed("status_on_main", "working_flag_names", "needs_human_items")

    def test_approved_but_not_merged_fails(self):
        self.lead()
        self.build(merge=False)
        self.failed("status_on_main", "working_flag_names")

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("no-question-to-sam", "lead", "graded"),
                          ("reasons", "lead", "reported")])


if __name__ == "__main__":
    unittest.main()
