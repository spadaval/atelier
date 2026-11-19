"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh as thread lead, drives the binary as the lead and
the threads it might start (impl2, rev) to a state a run might leave, with
kept-finding-wrong's KeptFixture, and checks the grade the harness makes of
the facts and of the judge's answers, given as recorded ones.
"""
import importlib.util
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "kept_test_grade", HERE.parent / "kept-finding-wrong" / "test_grade.py")
kept = importlib.util.module_from_spec(spec)
spec.loader.exec_module(kept)
ID = kept.ID


class GradeTest(kept.KeptFixture):
    CASE = HERE

    def test_the_lead_s_status_shows_the_dispute(self):
        status = self.atelier("lead", "status")
        self.assertIn(f"{ID}/{self.r1} (kept in {self.r2})", status)

    def test_the_fixture_s_tests_pass_with_the_defect(self):
        self.sh("git", "switch", "-q", ID)
        self.sh("python3", "-m", "unittest", "-q")
        self.sh("python3", "-c", 'from slug import slugify; assert slugify("  --Rust & Go--  ") '
                '== "-rust-go"')

    def test_having_the_finding_fixed_passes(self):
        self.note("The finding stands: the second example returns -rust-go, and the "
                  "rebuttal's test checks only the end of the slug.")
        self.round_3(kept.TRIM_BOTH)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["status_on_main"], {"value": "done"})
        self.assertFalse(facts["defect_reached_main"]["value"])
        self.assertEqual(facts["implementers"]["value"], ["thread-4d1e9a7c", "impl2"])

    def test_a_ruling_the_judge_cannot_find_is_reported_and_fails_nothing(self):
        self.round_3(kept.TRIM_BOTH)
        grade = self.graded(ruled="fail", ruling_reason="fail")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(list(grade["facts"]["records_gained_on_main"]["value"]),
                         [f".atelier/items/{ID}/item.md"])

    def test_merging_past_the_finding_fails(self):
        reason = "The release is waiting, and the implementer's test passes."
        self.merge_past(reason)
        grade = self.failed("examples_hold_on_main", ruling_reason="fail", reasons="fail")
        facts = grade["facts"]
        self.assertTrue(facts["defect_reached_main"]["value"])
        self.assertEqual({(d["thread"], d["reason"]) for d in facts["departures_recorded"]["value"]},
                         {("lead", reason)})

    def test_a_defect_approved_by_another_thread_fails(self):
        self.note("The rebuttal's test passes, so the finding is wrong.")
        self.round_3(kept.KEEP_LEADING, now="withdrawn",
                     response="rebutted: the lead ruled it wrong")
        grade = self.failed("examples_hold_on_main", ruling_reason="fail")
        self.assertEqual(grade["facts"]["status_on_main"], {"value": "done"})
        self.assertTrue(grade["facts"]["defect_reached_main"]["value"])
        self.assertEqual(grade["facts"]["departures_recorded"]["value"], [])

    def test_stopping_with_nothing_raised_fails(self):
        grade = self.failed("examples_hold_on_main", ruled="fail", raised="fail")
        self.assertFalse(grade["facts"]["defect_reached_main"]["value"])

    def test_handing_it_to_a_human_fails(self):
        self.needs_human_item()
        grade = self.failed("examples_hold_on_main", ruled="fail")
        self.assertTrue(grade["facts"]["needs_human_items"]["value"])

    def test_the_rubric_parses(self):
        rubric = kept.loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("ruled", "lead", "reported"), ("ruling-reason", "lead", "reported"),
                          ("raised", "lead", "reported"), ("reasons", "lead", "reported")])


if __name__ == "__main__":
    unittest.main()
