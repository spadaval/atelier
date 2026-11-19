"""Tests every sized case runs (harness/sized.sh and sized.py), from its
test_grade.py:

    class SizedTest(sized_tests.SizedCase):
        case = HERE

They build the case's fixture and grade recorded reviews, with no model.
The change is correct: its suite passes on the item's branch, and the
tests it adds fail on main. An overbuilt case's lean reference, in lean/,
passes its own tests on main's tree, which fail on main alone, and the
change passes those same tests, so both meet the same Done when; no file of
the reference's code is in the fixture. A review whose finding the judge
reads as naming the unneeded part (an overbuilt case), or none as asking
for less (a control), passes whether or not that finding blocks; one the
judge reads otherwise, an unrecorded review, and a change outside the
review files fail. EVAL_ATELIER names the binary under test.
"""
import importlib.util
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HARNESS = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HARNESS.parent / "scenarios" / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
spec = importlib.util.spec_from_file_location("sized", HARNESS / "sized.py")
sized = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sized)

FINDING = """
## Finding 1: {title}
Recommendation: {recommendation}
{body}
"""


def suite(tree: Path, *tests: str) -> subprocess.CompletedProcess:
    """`python3 -m unittest` in `tree`, on `tests` or on every test file."""
    env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1"}
    return subprocess.run([sys.executable, "-m", "unittest", "-q", *tests], cwd=tree, env=env,
                          capture_output=True, text=True)


@unittest.skipUnless(os.environ.get("EVAL_ATELIER"), "needs EVAL_ATELIER")
class SizedCase(loop.Fixture):
    case = None

    def setUp(self):
        if self.case is None:
            raise unittest.SkipTest("the shared tests run from a case's test_grade.py")
        self.start(self.case)
        self.id = sized.item_id(self.case)
        self.lean = self.case / "lean"
        self.overbuilt = self.lean.is_dir()
        self.question = "names-unneeded" if self.overbuilt else "nothing-less"

    def tree(self, ref: str, *over: Path) -> Path:
        """A directory holding the repository's tree at `ref` with no
        records, and each file of `over` copied on top."""
        dest = Path(tempfile.mkdtemp(prefix=f"{self.case.name}-tree-"))
        self.addCleanup(shutil.rmtree, dest, ignore_errors=True)
        archive = subprocess.run(["git", "archive", ref], cwd=self.repo, check=True,
                                 capture_output=True).stdout
        subprocess.run(["tar", "-x", "-C", str(dest)], input=archive, check=True)
        shutil.rmtree(dest / ".atelier")
        for f in over:
            shutil.copy(f, dest / f.name)
        return dest

    def lean_files(self, tests: bool):
        return sorted(f for f in self.lean.iterdir() if f.name.startswith("test_") == tests)

    def graded(self, answer="pass", blocks="pass"):
        return super().graded(**{self.question: answer, "blocks": blocks})

    def review(self, findings="", verdict=None):
        """Open round 1's review as thread rev, write its summary and
        `findings`, and record `verdict` if one is given."""
        self.atelier("rev", "review", self.id)
        [review] = (self.repo / f".atelier/items/{self.id}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Read the diff and ran the suite.\n<!--", 1)
                          + findings)
        if verdict:
            self.verdict("rev", verdict)

    def verdict(self, reviewer, verdict):
        self.atelier(reviewer, "verdict", verdict)
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", f"Review {self.id}: {verdict}")

    def less(self, recommendation):
        """A finding that asks for less, which the judge's recorded answer
        reads as naming the unneeded part."""
        return FINDING.format(title="The change holds more than the Done when asks for",
                              recommendation=recommendation,
                              body="Nothing in the Goal needs this part; remove it.")

    # The fixture.

    def test_the_change_passes_its_suite_and_its_new_tests_fail_on_main(self):
        self.assertEqual(subprocess.run(["git", "symbolic-ref", "--short", "HEAD"], cwd=self.repo,
                                        capture_output=True, text=True).stdout.strip(), self.id)
        run = suite(self.repo)
        self.assertEqual(run.returncode, 0, run.stderr)
        changed = subprocess.run(["git", "diff", "--name-only", "main", self.id, "--", "test_*.py"],
                                 cwd=self.repo, check=True, capture_output=True,
                                 text=True).stdout.split()
        self.assertTrue(changed)
        on_main = self.tree("main", *(self.repo / name for name in changed))
        self.assertNotEqual(suite(on_main).returncode, 0)

    def test_the_lean_reference_passes_the_same_tests_as_the_change(self):
        if not self.overbuilt:
            self.skipTest("a control has no lean reference")
        tests = self.lean_files(tests=True)
        names = [f.stem for f in tests]
        self.assertTrue(tests)
        lean = self.tree("main", *self.lean.iterdir())
        run = suite(lean)
        self.assertEqual(run.returncode, 0, run.stderr)
        run = suite(self.tree(self.id, *tests), *names)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertNotEqual(suite(self.tree("main", *tests), *names).returncode, 0)

    def test_the_lean_reference_stays_outside_the_fixture(self):
        if not self.overbuilt:
            self.skipTest("a control has no lean reference")
        code = self.lean_files(tests=False)
        self.assertTrue(code)
        objects = subprocess.run(["git", "rev-list", "--all", "--objects"], cwd=self.repo,
                                 check=True, capture_output=True, text=True).stdout
        held = {line.split()[0] for line in objects.splitlines()}
        for f in code:
            blob = subprocess.run(["git", "hash-object", str(f)], cwd=self.repo, check=True,
                                  capture_output=True, text=True).stdout.strip()
            self.assertNotIn(blob, held, f"lean/{f.name} is in the fixture")
        for path in self.work.rglob("*"):
            if path.is_file() and path.name in {f.name for f in code}:
                self.assertNotIn(path.read_bytes(), {f.read_bytes() for f in code}, path)

    # The grade.

    def test_a_review_the_judge_passes_passes(self):
        self.review(self.less("fix now") if self.overbuilt else "",
                    "changes" if self.overbuilt else "approve")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["a_fix_now_finding"]["value"], self.overbuilt)

    def test_whether_the_finding_blocks_changes_no_grade(self):
        self.review(self.less("defer") if self.overbuilt else "", "approve")
        grade = self.graded(blocks="fail")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertIs(grade["facts"]["a_fix_now_finding"]["value"], False)

    def test_a_review_the_judge_fails_fails(self):
        self.review(self.less("fix now") if not self.overbuilt else "", "approve"
                    if self.overbuilt else "changes")
        grade = self.graded(answer="fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], [f"graded {self.question} (reviewer): fail"])

    def test_a_question_the_judge_cannot_answer_fails(self):
        self.review("", "approve")
        self.assertFalse(self.graded(answer="unknown")["pass"])

    def test_a_review_with_no_verdict_fails(self):
        self.review(self.less("fix now"))
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["reviewed_by_another_thread"]["pass"], False)

    def test_a_review_by_the_implementer_fails(self):
        # The binary refuses this, so it is the state left by an agent that
        # rewrote the review file's reviewers by hand before its verdict.
        self.review()
        [review] = (self.repo / f".atelier/items/{self.id}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("- rev\n", "- thread-1a2b3c4d\n- rev\n"))
        self.verdict("rev", "approve")
        self.assertIs(self.graded()["facts"]["reviewed_by_another_thread"]["pass"], False)

    def test_a_code_change_fails(self):
        [code] = [f for f in self.repo.iterdir() if f.suffix == ".py"
                  and not f.name.startswith("test_")][:1]
        code.write_text(code.read_text() + "\n# reviewed\n")
        self.review("", "approve")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["facts"]["other_files_changed"]["value"], [code.name])

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(self.case)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [(self.question, "reviewer", "graded"), ("blocks", "reviewer", "reported"),
                          ("levels", "reviewer", "reported"),
                          ("reasons", "reviewer", "reported")])
