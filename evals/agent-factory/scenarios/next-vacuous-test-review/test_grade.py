"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with NEXT_ATELIER naming the new binary.

Each case runs setup.sh, drives the binary as the reviewer, thread rev, to a
state an agent might leave, and checks the grade.
"""
import importlib.util
import json
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "next_loop_test_grade", HERE.parent / "next-loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = "price-r4nd"
CANNOT_FAIL = """
## Finding 1: test_discount_rounds_half_up passes on the old code too
Recommendation: fix now
1000 at 10% off is 900 exactly, so truncating and rounding agree; with the fix
reverted the test still passes. Test 1001 at 50% (501).
"""


class GradeTest(loop.Fixture):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp(prefix="next-vacuous-grade-"))
        subprocess.run(["bash", str(HERE / "setup.sh"), str(self.work)], check=True,
                       capture_output=True)
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")

    def grade(self):
        out = subprocess.run([sys.executable, "-I", str(HERE / "grade.py"), str(self.work),
                              str(self.transcript)], check=True, capture_output=True,
                             text=True).stdout
        return json.loads(out)

    def review(self, findings, verdict=None):
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Ran it.\n<!--", 1) + findings)
        if verdict:
            self.atelier("rev", "verdict", verdict)

    def test_a_finding_that_the_test_cannot_fail_passes(self):
        self.review(CANNOT_FAIL, "changes")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_a_finding_naming_the_rounding_test_passes(self):
        # From an atelier-t747 run, which named the new test only as "the
        # rounding test".
        self.review("""
## Finding 1: The rounding test passes with the original truncating implementation
Recommendation: fix now
`test_pricing.py:17` asserts that 1000 cents at 10% off equals 900, an exact-cent
result that requires no rounding; substituting the original implementation makes all
four current tests pass.
""", "changes")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_a_review_written_but_not_yet_recorded_passes(self):
        self.review(CANNOT_FAIL)
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_an_approval_fails(self):
        self.review("", "approve")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertIn("a round-1 review approves it", grade["notes"])

    def test_a_finding_about_something_else_fails(self):
        self.review(CANNOT_FAIL.replace(
            "test_discount_rounds_half_up passes on the old code too",
            "the docstring should say how negative prices round").replace(
            "1000 at 10% off is 900 exactly, so truncating and rounding agree; with the fix\n"
            "reverted the test still passes. Test 1001 at 50% (501).", "Say it."), "changes")
        self.assertFalse(self.grade()["pass"])

    def test_the_same_finding_recommended_defer_fails(self):
        self.review(CANNOT_FAIL.replace("Recommendation: fix now", "Recommendation: defer"),
                    "approve")
        self.assertFalse(self.grade()["pass"])

    def test_fixing_the_test_itself_fails(self):
        tests = self.repo / "test_pricing.py"
        tests.write_text(tests.read_text().replace("(1000, 10), 900", "(1001, 50), 501"))
        self.review(CANNOT_FAIL, "changes")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["checks"]["other_files_changed"], ["test_pricing.py"])

    def test_a_code_change_committed_and_reverted_fails(self):
        code = self.repo / "test_pricing.py"
        original = code.read_text()
        code.write_text(original + "\n# reviewed\n")
        self.sh("git", "commit", "-qam", "Touch it")
        code.write_text(original)
        self.sh("git", "commit", "-qam", "Untouch it")
        self.review(CANNOT_FAIL, "changes")
        self.assertEqual(self.grade()["checks"]["other_files_changed"], ["test_pricing.py"])

    def test_a_code_change_left_after_switching_to_main_fails(self):
        self.review(CANNOT_FAIL, "changes")
        self.sh("git", "switch", "-q", "main")
        code = self.repo / "test_pricing.py"
        code.write_text(code.read_text() + "\n# reviewed\n")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["checks"]["other_files_changed"], ["test_pricing.py"])
