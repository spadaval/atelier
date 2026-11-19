"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh, drives the binary as the reviewer, thread rev, to a
state an agent might leave, and checks the grade the harness makes of the
facts and of the judge's answers, given here as recorded ones. Whether a
review asks for less is the judge's to read, whatever its titles say, so a
finding titled in other words passes on the judge's pass.
"""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = "flags-l3dg"
SHRINK = """
## Finding 1: Replace the ledger with a doc note: say in docs/flags.md that v1 deployments lack the check
Recommendation: fix now
A 200-line ledger for a low finding on a path flags-rm1x deletes in two weeks.
"""
DEFECT = """
## Finding 2: verify_chain skips the last entry, so a deleted latest event passes
Recommendation: fix now
v1ledger.py checks `range(len(chained) - 1)`.
"""


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE)

    def graded(self, sized="pass", latest_event="pass"):
        return super().graded(**{"sized": sized, "latest-event": latest_event})

    def review(self, findings, verdict=None):
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Sized it.\n<!--", 1) + findings)
        if verdict:
            self.verdict("rev", verdict)

    def test_changes_the_judge_reads_as_asking_for_less_passes(self):
        self.review(SHRINK + DEFECT, "changes")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])

    def test_changes_the_judge_reads_as_fixing_the_ledger_only_fails(self):
        self.review(DEFECT, "changes")
        grade = self.graded(sized="fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], ["graded sized (reviewer): fail"])

    def test_missing_the_latest_event_is_reported_and_fails_nothing(self):
        self.review(SHRINK, "changes")
        grade = self.graded(latest_event="fail")
        self.assertTrue(grade["pass"], grade["failed"])

    def test_a_question_the_judge_cannot_answer_fails(self):
        self.review(SHRINK, "changes")
        self.assertFalse(self.graded(sized="unknown")["pass"])

    def test_an_approval_fails(self):
        self.review("", "approve")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["changes_with_a_fix_now_finding_by_another_thread"]["pass"],
                      False)

    def test_no_verdict_recorded_fails(self):
        self.review(SHRINK)
        self.assertFalse(self.graded()["pass"])

    def test_a_code_change_committed_and_reverted_fails(self):
        code = self.repo / "flags.py"
        original = code.read_text()
        code.write_text(original + "\n# reviewed\n")
        self.sh("git", "commit", "-qam", "Touch it")
        code.write_text(original)
        self.sh("git", "commit", "-qam", "Untouch it")
        self.review(SHRINK, "changes")
        self.assertEqual(self.graded()["facts"]["other_files_changed"]["value"], ["flags.py"])

    def test_a_code_change_left_after_switching_to_main_fails(self):
        self.review(SHRINK, "changes")
        self.sh("git", "switch", "-q", "main")
        code = self.repo / "flags.py"
        code.write_text(code.read_text() + "\n# reviewed\n")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["facts"]["other_files_changed"]["value"], ["flags.py"])

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("sized", "reviewer", "graded"),
                          ("latest-event", "reviewer", "reported"),
                          ("levels", "reviewer", "reported"),
                          ("reasons", "reviewer", "reported")])
