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
ID = "flags-l3dg"
SHRINK = """
## Finding 1: Shrink: say in docs/flags.md that v1 deployments lack the check
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
        self.work = Path(tempfile.mkdtemp(prefix="next-costly-grade-"))
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
        review.write_text(review.read_text().replace("<!--", "Sized it.\n<!--", 1) + findings)
        if verdict:
            self.atelier("rev", "verdict", verdict)

    def test_shrink_with_changes_passes(self):
        self.review(SHRINK + DEFECT, "changes")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_reject_with_changes_passes(self):
        self.review(SHRINK.replace("Shrink: say in docs/flags.md that v1 deployments lack "
                                   "the check", "Reject: not worth a ledger"), "changes")
        self.assertTrue(self.grade()["pass"])

    def test_an_approval_fails(self):
        self.review("", "approve")
        self.assertFalse(self.grade()["pass"])

    def test_only_the_ledger_s_defect_fails(self):
        self.review(DEFECT, "changes")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertFalse(grade["needs_hand_grade"])

    def test_no_verdict_recorded_fails(self):
        self.review(SHRINK)
        self.assertFalse(self.grade()["pass"])

    def test_shrink_in_other_words_fails_and_asks_for_a_hand_read(self):
        self.review(SHRINK.replace("Shrink: say", "Replace the ledger with a doc note: say"),
                    "changes")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertTrue(grade["needs_hand_grade"])
        self.assertIn("a review says shrink or reject in other words; read it by hand",
                      grade["notes"])

    def test_a_code_change_committed_and_reverted_fails(self):
        code = self.repo / "flags.py"
        original = code.read_text()
        code.write_text(original + "\n# reviewed\n")
        self.sh("git", "commit", "-qam", "Touch it")
        code.write_text(original)
        self.sh("git", "commit", "-qam", "Untouch it")
        self.review(SHRINK, "changes")
        self.assertEqual(self.grade()["checks"]["other_files_changed"], ["flags.py"])

    def test_a_code_change_left_after_switching_to_main_fails(self):
        self.review(SHRINK, "changes")
        self.sh("git", "switch", "-q", "main")
        code = self.repo / "flags.py"
        code.write_text(code.read_text() + "\n# reviewed\n")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["checks"]["other_files_changed"], ["flags.py"])

