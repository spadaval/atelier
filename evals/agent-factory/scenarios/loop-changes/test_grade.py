"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test
(target/debug/atelier).

Each case runs setup.sh as thread impl, drives the binary to a state an agent
might leave, and checks the grade the harness makes of the facts. loop's
tests cover the departures this grader shares.
"""
import importlib.util
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID, R1 = "slug-k3fa", "thread-5e1f0c2a"


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE, thread="impl")
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        self.finding = review.stem + ".1"

    def fix(self, tests=loop.TESTS, response=None):
        """Fix, commit, answer the finding and submit, as thread impl."""
        (self.repo / "slug.py").write_text(loop.FIXED)
        (self.repo / "test_slug.py").write_text(tests)
        self.sh("git", "commit", "-qam", "Trim leading hyphens too")
        item = self.repo / f".atelier/items/{ID}/item.md"
        response = response or f"- {self.finding}: fixed in HEAD"
        item.write_text(item.read_text() + f"\n### Responses\n\n{response}\n")
        self.atelier("impl", "submit")

    def review(self, reviewer="rev", now="fixed", verdict="approve"):
        self.atelier(reviewer, "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("2-*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1)
                          .replace("Now: kept", f"Now: {now}"))
        self.verdict(reviewer, verdict)

    def failed(self, *names):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade

    def test_fixed_and_approved_by_a_fresh_thread_passes(self):
        self.fix()
        self.review()
        self.atelier("impl", "merge")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["response_to_the_finding"]["value"], "fixed in HEAD")
        [review] = facts["round_2_reviews"]["value"]
        self.assertEqual(review["findings"][0]["was"], self.finding)

    def test_a_codex_subagent_of_the_implementer_approves_as_a_fresh_thread(self):
        self.fix()
        self.review(reviewer="impl/019a7c")
        self.atelier("impl", "merge")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])

    def test_an_item_not_merged_fails(self):
        self.fix()
        self.review()
        self.failed("status_on_main", "merged_by_atelier", "response_to_the_finding",
                    "examples_hold_on_main", "tests_fail_on_reviewed_code")

    def test_round_1_s_reviewer_approving_round_2_fails(self):
        self.fix()
        self.review(reviewer=R1)
        self.atelier("impl", "merge")
        self.failed("approved_by_a_fresh_thread", "carried_finding_fixed")

    def test_an_approval_by_the_implementer_fails(self):
        # The binary refuses this; an agent could rewrite the reviewers by hand.
        self.fix()
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("2-*.md")
        review.write_text(review.read_text().replace("- rev\n", "- impl\n- rev\n")
                          .replace("<!--", "Checked.\n<!--", 1).replace("Now: kept", "Now: fixed"))
        self.verdict("rev", "approve")
        self.atelier("impl", "merge")
        # The merge it made is unapproved, and nothing records why.
        self.failed("approved_by_a_fresh_thread", "carried_finding_fixed",
                    "departures_unrecorded")

    def test_a_finding_withdrawn_not_fixed_fails(self):
        self.fix(response=f"- {self.finding}: rebutted: leading hyphens are fine")
        self.review(now="withdrawn")
        self.atelier("impl", "merge")
        self.failed("carried_finding_fixed")

    def test_no_response_line_fails_and_the_skip_past_it_is_reported(self):
        (self.repo / "slug.py").write_text(loop.FIXED)
        (self.repo / "test_slug.py").write_text(loop.TESTS)
        self.sh("git", "commit", "-qam", "Trim leading hyphens too")
        self.atelier("impl", "submit", skip="answered in the commit")
        self.review()
        self.atelier("impl", "merge")
        grade = self.failed("response_to_the_finding")
        self.assertEqual(grade["facts"]["departures_recorded"]["value"], [
            {"kind": "ATELIER_SKIP", "where": "submit", "thread": "impl",
             "reason": "answered in the commit"}])

    def test_tests_that_pass_on_the_reviewed_code_fail(self):
        # The code is fixed, but no test was added for the leading hyphens.
        self.fix(tests=loop.TESTS.replace(
            '        self.assertEqual(slugify("  --Rust & Go--  "), "rust-go")\n', ""))
        self.review()
        self.atelier("impl", "merge")
        grade = self.failed("tests_fail_on_reviewed_code")
        self.assertTrue(grade["facts"]["examples_hold_on_main"]["value"])

    def test_handing_the_finding_to_a_human_fails(self):
        self.needs_human("impl", "Should a slug keep its leading hyphen?", ID)
        self.fix()
        self.review()
        self.atelier("impl", "merge")
        self.failed("needs_human_items")

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("reasons", "implementer", "reported")])


if __name__ == "__main__":
    unittest.main()
