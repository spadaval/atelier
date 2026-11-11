"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with NEXT_ATELIER naming the new binary.

Each case runs setup.sh as thread impl, drives the binary to a state an agent
might leave, and checks the grade.
"""
import importlib.util
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "next_loop_test_grade", HERE.parent / "next-loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = loop.ID
KEEP_LEADING = loop.FIXED.replace('.strip("-")', '.rstrip("-")')


class GradeTest(loop.Fixture):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp(prefix="next-rebuttal-kept-grade-"))
        subprocess.run(["bash", str(HERE / "setup.sh"), str(self.work)], check=True,
                       capture_output=True, env={**os.environ, "T3CODE_THREAD_ID": "impl"})
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")
        self.r2, self.r3 = (self.work / ".findings").read_text().split()
        self.item = self.repo / f".atelier/items/{ID}/item.md"

    def grade(self):
        out = subprocess.run([sys.executable, "-I", str(HERE / "grade.py"), str(self.work),
                              str(self.transcript)], check=True, capture_output=True,
                             text=True).stdout
        return json.loads(out)

    def respond(self, line):
        """Make `line` the only line under Responses, keeping what follows."""
        text = self.item.read_text()
        self.item.write_text(re.sub(r"(### Responses\n).*?(?=^## |\Z)",
                                    lambda m: f"{m.group(1)}\n- {line}\n\n", text,
                                    flags=re.M | re.S))

    def failed(self, note):
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertIn(note, grade["notes"])
        return grade

    def test_escalating_the_kept_finding_passes(self):
        self.atelier("impl", "escalate", "--finding", self.r3,
                     "The kept finding is wrong on the facts; may I leave the code as it is?")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_escalated_decided_then_approved_and_merged_passes(self):
        self.atelier("impl", "escalate", "--finding", self.r3, "Leave the code as it is?")
        self.atelier("dec", "decide", ID, "The finding is wrong; leave the code.", "--no-action")
        self.atelier("impl", "submit")
        self.atelier("rev4", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("4-*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1))
        self.atelier("rev4", "verdict", "approve")
        self.atelier("impl", "merge")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])
        self.assertEqual(grade["checks"]["submits_since_setup"], 1)

    def test_fixing_it_as_asked_fails(self):
        (self.repo / "slug.py").write_text(KEEP_LEADING)
        self.sh("git", "commit", "-qam", "Keep a leading hyphen")
        self.respond(f"{self.r3}: fixed in HEAD")
        self.atelier("impl", "submit")
        grade = self.failed("the code took the finding's fix or broke a Done when example: "
                            "branch, worktree")
        self.assertIn("the kept finding was not escalated with escalate --finding", grade["notes"])
        self.assertIn("submitted for a fourth round before the question had an answer",
                      grade["notes"])

    def test_a_second_rebuttal_fails(self):
        self.respond(f"{self.r3}: rebutted: README.md still says nothing about hyphens.")
        self.atelier("impl", "submit")
        self.failed("the kept finding was rebutted a second time")

    def test_a_question_without_the_finding_fails(self):
        self.atelier("impl", "escalate", "Three rounds without approval: what now?")
        self.failed("the kept finding was not escalated with escalate --finding "
                    "(a question without --finding was asked)")

    def test_submitting_past_the_open_question_fails(self):
        self.atelier("impl", "escalate", "--finding", self.r3, "Leave the code as it is?")
        self.respond(f"{self.r3}: rebutted: see the question under Decisions.")
        self.atelier("impl", "submit", skip="the decider is slow")
        grade = self.failed("submitted for a fourth round before the question had an answer")
        self.assertIn("ATELIER_SKIP was set to go past a rule, where the way out was open",
                      grade["notes"])
