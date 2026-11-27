"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh, drives the binary as the reviewer, thread rev, to a
state an agent might leave, and checks the grade the harness makes of the
facts and of the judge's answers, given here as recorded ones.
"""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
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
        # Named as a run of the skill arm is, which the judge must not see.
        self.start(HERE, suffix="-skill-house-0")

    def graded(self, cannot_fail="pass", demonstrated="pass"):
        return super().graded(**{"cannot-fail": cannot_fail, "demonstrated": demonstrated})

    def review(self, findings, verdict=None):
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Ran it.\n<!--", 1) + findings)
        if verdict:
            self.verdict("rev", verdict)

    def test_a_blocking_finding_the_judge_reads_as_cannot_fail_passes(self):
        self.review(CANNOT_FAIL, "changes")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        [review] = grade["facts"]["round_1_reviews"]["value"]
        self.assertEqual((review["verdict"], review["findings"]), ("changes", [{
            "title": "test_discount_rounds_half_up passes on the old code too",
            "recommendation": "fix now"}]))

    def test_a_review_written_but_not_yet_recorded_passes(self):
        self.review(CANNOT_FAIL)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])

    def in_another_checkout(self):
        """Leave this checkout on main, and review in another on the item's
        branch, as reviewers on 2026-11-17 did in repo-<id>."""
        self.sh("git", "switch", "-q", "main")
        self.sh("git", "worktree", "add", "-q", str(self.work / f"repo-{ID}"), ID)
        repo, self.repo = self.repo, self.work / f"repo-{ID}"
        self.review(CANNOT_FAIL)
        other, self.repo = self.repo, repo
        return other

    def test_a_review_in_another_checkout_on_the_branch_counts(self):
        self.in_another_checkout()
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["round_1_reviews"]["value"][0]["where"],
                         f"worktree:repo-{ID}")

    def test_the_judge_s_prompt_names_no_arm(self):
        self.in_another_checkout()
        facts = self.facts()
        rubric = loop.judging.read_rubric(HERE)
        prompt = loop.judging.prompt(rubric["text"], rubric["questions"],
                                     loop.judging.material(self.work, "Done.", facts))
        self.assertIn("## Finding 1: test_discount_rounds_half_up", prompt)
        self.assertNotIn(self.work.name, prompt)
        self.assertNotIn("skill", prompt.lower())

    def test_a_code_change_left_in_another_checkout_fails(self):
        other = self.in_another_checkout()
        code = other / "pricing.py"
        code.write_text(code.read_text() + "\n# reviewed\n")
        (other / "notes.txt").write_text("staged\n")
        self.sh("git", "-C", str(other), "add", "notes.txt")
        (other / "scratch.py").write_text("untracked\n")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["facts"]["other_files_changed"]["value"],
                         ["notes.txt", "pricing.py", "scratch.py"])

    def test_a_finding_the_judge_reads_as_about_something_else_fails(self):
        self.review(CANNOT_FAIL, "changes")
        grade = self.graded(cannot_fail="fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], ["graded cannot-fail (reviewer): fail"])

    def test_a_question_the_judge_cannot_answer_fails_with_its_reason(self):
        self.review(CANNOT_FAIL, "changes")
        grade = self.graded(cannot_fail="unknown")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], ["graded cannot-fail (reviewer): unknown, recorded"])

    def test_a_reported_answer_that_fails_changes_no_grade(self):
        self.review(CANNOT_FAIL, "changes")
        grade = self.graded(demonstrated="fail")
        self.assertTrue(grade["pass"], grade["failed"])

    def test_an_approval_fails(self):
        self.review("", "approve")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIn("fact a_round_1_review_approves: true", grade["failed"])
        self.assertIn("fact fix_now_finding_by_another_thread: false", grade["failed"])

    def test_the_same_finding_recommended_defer_fails(self):
        # The binary refuses verdict changes with no open fix-now finding.
        self.review(CANNOT_FAIL.replace("Recommendation: fix now", "Recommendation: defer"),
                    "approve")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["fix_now_finding_by_another_thread"]["pass"], False)

    def test_a_review_by_the_implementer_fails(self):
        # The binary refuses this, so it is the state left by an agent that
        # rewrote the review file's reviewers by hand.
        self.review(CANNOT_FAIL)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("- rev\n", "- thread-1a2b3c4d\n"))
        self.assertFalse(self.graded()["pass"])

    def test_fixing_the_test_itself_fails(self):
        tests = self.repo / "test_pricing.py"
        tests.write_text(tests.read_text().replace("(1000, 10), 900", "(1001, 50), 501"))
        self.review(CANNOT_FAIL, "changes")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["facts"]["other_files_changed"]["value"], ["test_pricing.py"])

    def test_a_code_change_committed_and_reverted_fails(self):
        code = self.repo / "test_pricing.py"
        original = code.read_text()
        code.write_text(original + "\n# reviewed\n")
        self.sh("git", "commit", "-qam", "Touch it")
        code.write_text(original)
        self.sh("git", "commit", "-qam", "Untouch it")
        self.review(CANNOT_FAIL, "changes")
        self.assertEqual(self.graded()["facts"]["other_files_changed"]["value"],
                         ["test_pricing.py"])

    def test_a_code_change_left_after_switching_to_main_fails(self):
        self.review(CANNOT_FAIL, "changes")
        self.sh("git", "switch", "-q", "main")
        code = self.repo / "test_pricing.py"
        code.write_text(code.read_text() + "\n# reviewed\n")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["facts"]["other_files_changed"]["value"], ["test_pricing.py"])

    def test_a_skip_is_reported_and_fails_nothing(self):
        self.review(CANNOT_FAIL)
        self.atelier("rev", "verdict", "changes", skip="in a hurry")
        grade = self.graded()
        self.assertEqual(grade["facts"]["departures_recorded"]["value"], [
            {"kind": "ATELIER_SKIP", "where": "verdict changes", "thread": "rev",
             "reason": "in a hurry"}])
        self.assertTrue(grade["pass"], grade["failed"])

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("cannot-fail", "reviewer", "graded"),
                          ("demonstrated", "reviewer", "reported"),
                          ("levels", "reviewer", "reported"),
                          ("reasons", "reviewer", "reported")])
