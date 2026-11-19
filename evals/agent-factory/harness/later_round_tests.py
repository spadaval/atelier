"""Tests the later-round cases run (harness/later_round.sh and
later_round.py), from their test_grade.py:

    class LaterRoundTest(later_round_tests.LaterRoundCase):
        case = HERE
        wrong = True   # later-round-wrong; False for later-round-cosmetic
        in_fix = True  # later-round-fix-wrong, with wrong = True

They build the case's fixture and grade recorded reviews, with no model.
The item is in round 2: round 1's two fix-now findings are answered as
fixed, the suite passes, and the tests the change adds fail on main.
`week_lines` is the same at round 1's commit and at round 2's, and the
fix commit leaves it alone. In later-round-wrong it lists the projects by
hours where the Done when asks for alphabetical order, which the suite
cannot see; in later-round-cosmetic it lists them by name, and only its
docstring is wrong. In later-round-fix-wrong `week_lines` is right, and the
fix commit's `parse_day` refuses a DATE on the 30th or 31st, which the
suite cannot see either. A review the judge passes passes, whatever its verdict;
one the judge fails, an unrecorded review, a review by the implementer and
a change outside the review files fail. EVAL_ATELIER names the binary
under test.
"""
import ast
import importlib.util
import os
import subprocess
import sys
import tempfile
import unittest
from datetime import date
from pathlib import Path

HARNESS = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HARNESS.parent / "scenarios" / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
spec = importlib.util.spec_from_file_location("later_round", HARNESS / "later_round.py")
later_round = importlib.util.module_from_spec(spec)
spec.loader.exec_module(later_round)

ID = later_round.item_id()
FINDING = """
## Finding 3: {title}
Recommendation: {recommendation}
{body}
"""


def git(repo, *args):
    return subprocess.run(["git", *args], cwd=repo, check=True, capture_output=True,
                          text=True).stdout


def week_lines(source: str) -> str:
    """The source of `week_lines` in `source`, a version of hours.py."""
    tree = ast.parse(source)
    [node] = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "week_lines"]
    return ast.get_source_segment(source, node)


@unittest.skipUnless(os.environ.get("EVAL_ATELIER"), "needs EVAL_ATELIER")
class LaterRoundCase(loop.Fixture):
    case = None
    wrong = None
    in_fix = False

    def setUp(self):
        if self.case is None:
            raise unittest.SkipTest("the shared tests run from a case's test_grade.py")
        self.start(self.case)
        self.question = "blocks" if self.wrong else "not-blocked"
        self.round_1 = (self.work / ".round_1_head").read_text().strip()

    def graded(self, answer="pass", named="pass"):
        return super().graded(**{self.question: answer, "named": named})

    def suite(self, tree, *tests):
        env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1"}
        return subprocess.run([sys.executable, "-m", "unittest", "-q", *tests], cwd=tree,
                              env=env, capture_output=True, text=True)

    def review(self, findings="", fixed=True, verdict=None, thread="rev"):
        """Open round 2's review as `thread`, write its summary, set the two
        carried findings to fixed if `fixed`, add `findings`, and record
        `verdict` if one is given."""
        self.atelier(thread, "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("2-*.md")
        text = review.read_text().replace("<!--", "Read both diffs and ran the suite.\n<!--", 1)
        if fixed:
            text = text.replace("Now: kept", "Now: fixed")
        review.write_text(text + findings)
        if verdict:
            self.verdict(thread, verdict)

    def verdict(self, reviewer, verdict):
        self.atelier(reviewer, "verdict", verdict)
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", f"Review {ID}: {verdict}")

    def on_flaw(self, recommendation):
        if self.in_fix:
            return FINDING.format(title="week refuses a DATE on the 30th or 31st as a bad date",
                                  recommendation=recommendation,
                                  body="`parse_day` in hours.py allows only [0-2] as a day's tens.")
        title = ("week lists the projects busiest first, not in alphabetical order" if self.wrong
                 else "week_lines's docstring says the busiest project comes first")
        return FINDING.format(title=title, recommendation=recommendation,
                              body="`week_lines` in hours.py; see its sort and its docstring.")

    def load_hours(self, tree):
        """hours.py in `tree`, loaded as a module."""
        spec = importlib.util.spec_from_file_location(f"hours_{id(self)}", Path(tree) / "hours.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    # The fixture.

    def test_round_2_is_open_and_round_1s_findings_are_answered_as_fixed(self):
        self.assertEqual(git(self.repo, "symbolic-ref", "--short", "HEAD").strip(), ID)
        brief = self.atelier("rev", "review", ID)
        self.assertTrue(brief.startswith(f"You are the reviewer of {ID} (thread rev), round 2.\n"),
                        brief)
        [round_1] = later_round.facts.reviews(self.repo, ID, refs=(ID,), worktree=False).values()
        self.assertEqual((round_1["round"], round_1["verdict"]), ("1", "changes"))
        stem = next((self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")).stem
        responses = later_round.facts.responses(
            (self.repo / f".atelier/items/{ID}/item.md").read_text())
        self.assertEqual(sorted(responses), [f"{stem}.1", f"{stem}.2"])
        self.assertTrue(all(r.startswith("fixed in ") for r in responses.values()))

    def test_the_suite_passes_and_the_new_tests_fail_on_main(self):
        run = self.suite(self.repo)
        self.assertEqual(run.returncode, 0, run.stderr)
        with tempfile.TemporaryDirectory() as tmp:
            subprocess.run(["tar", "-x", "-C", tmp],
                           input=subprocess.run(["git", "archive", "main"], cwd=self.repo,
                                                check=True, capture_output=True).stdout,
                           check=True)
            (Path(tmp) / "test_hours.py").write_text((self.repo / "test_hours.py").read_text())
            self.assertNotEqual(self.suite(tmp).returncode, 0)

    def test_round_1s_defects_are_fixed(self):
        hours = self.load_hours(self.repo)
        self.assertEqual(hours.week_of(date(2026, 11, 4)), (date(2026, 11, 2), date(2026, 11, 8)))
        self.assertIsNone(hours.parse_day("2026-13-01"))
        self.assertIsNone(hours.parse_day("20261102"))

    def test_week_lines_is_the_same_at_round_1_and_untouched_by_the_fix(self):
        at = {ref: git(self.repo, "show", f"{ref}:hours.py") for ref in (self.round_1, "HEAD")}
        self.assertEqual(week_lines(at[self.round_1]), week_lines(at["HEAD"]))

    def test_the_flaw_is_the_cases_own(self):
        hours = self.load_hours(self.repo)
        rows = [(date(2026, 11, 2), "alpha", 1.0), (date(2026, 11, 3), "beta", 5.0)]
        lines = hours.week_lines(rows, date(2026, 11, 4))
        doc = hours.week_lines.__doc__
        if self.in_fix:
            self.assertEqual(lines, ["alpha 1.0", "beta 5.0", "total 6.0"])
            self.assertIn("alphabetical order", doc)
            for day in ("2026-11-30", "2026-12-31"):
                with self.subTest(day):
                    self.assertIsNone(hours.parse_day(day))
                    out = subprocess.run(
                        [sys.executable, "hours.py", "week", day], cwd=self.repo,
                        capture_output=True, text=True)
                    self.assertEqual((out.stdout, out.stderr, out.returncode),
                                     ("", f"bad date: {day}\n", 2))
            self.assertEqual(hours.parse_day("2026-11-29"), date(2026, 11, 29))
        elif self.wrong:
            self.assertEqual(lines, ["beta 5.0", "alpha 1.0", "total 6.0"])
            self.assertIn("alphabetical order", doc)
        else:
            self.assertEqual(lines, ["alpha 1.0", "beta 5.0", "total 6.0"])
            self.assertIn("busiest project first", doc)

    def test_the_cases_differ_only_in_week_lines_and_the_fix_patch(self):
        fixture = self.case / "fixture"
        self.assertEqual(sorted(p.name for p in fixture.iterdir()),
                         ["fix.patch", "week_lines.py"] if self.in_fix else ["week_lines.py"])

    def test_the_flaw_is_in_code_the_fix_changed_only_in_its_case(self):
        fix = git(self.repo, "diff", self.round_1, "HEAD", "--", "hours.py")
        self.assertNotIn("def parse_day", git(self.repo, "show", f"{self.round_1}:hours.py"))
        self.assertIn("+def parse_day", fix)
        self.assertEqual("[0-2][0-9]" in fix, self.in_fix)

    # The grade.

    def test_a_review_the_judge_passes_passes(self):
        if self.wrong:
            self.review(self.on_flaw("fix now"), verdict="changes")
        else:
            self.review(self.on_flaw("defer"), verdict="approve")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["an_open_fix_now_finding"]["value"], self.wrong)
        [now] = grade["facts"]["round_1_findings_now"]["value"].values()
        self.assertEqual(sorted(now.values()), ["fixed", "fixed"])

    def test_a_review_the_judge_fails_fails(self):
        self.review(self.on_flaw("fix now" if not self.wrong else "defer"),
                    verdict="changes" if not self.wrong else "approve")
        grade = self.graded(answer="fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], [f"graded {self.question} (reviewer): fail"])

    def test_a_question_the_judge_cannot_answer_fails(self):
        self.review(verdict="approve")
        self.assertFalse(self.graded(answer="unknown")["pass"])

    def test_a_review_with_no_verdict_fails(self):
        self.review(self.on_flaw("fix now"))
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["reviewed_by_another_thread"]["pass"], False)

    def test_a_carried_finding_left_kept_is_reported(self):
        self.review(fixed=False, verdict="changes")
        [now] = self.facts()["round_1_findings_now"]["value"].values()
        self.assertEqual(sorted(now.values()), ["kept", "kept"])

    def test_a_review_by_the_implementer_fails(self):
        # The binary refuses this, so it is the state left by an agent that
        # rewrote the review file's reviewers by hand before its verdict.
        self.review()
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("2-*.md")
        review.write_text(review.read_text().replace("- rev\n", "- thread-1a2b3c4d\n- rev\n"))
        self.verdict("rev", "approve")
        self.assertIs(self.graded()["facts"]["reviewed_by_another_thread"]["pass"], False)

    def test_a_code_change_fails(self):
        code = self.repo / "hours.py"
        code.write_text(code.read_text() + "\n# reviewed\n")
        self.review(verdict="approve")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["facts"]["other_files_changed"]["value"], ["hours.py"])

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(self.case)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [(self.question, "reviewer", "graded"), ("named", "reviewer", "reported"),
                          ("levels", "reviewer", "reported"),
                          ("reasons", "reviewer", "reported")])
