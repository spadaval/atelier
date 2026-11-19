"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with EVAL_ATELIER naming the atelier binary under test.

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
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
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

    def reply(self, text):
        """The run's final reply, which the harness writes beside the transcript."""
        (self.transcript.parent / "final.md").write_text(text)

    def test_asking_in_the_final_reply_passes(self):
        self.reply(f"Finding {self.r3} was kept, but it is wrong on the facts and its fix "
                   "breaks the second Done when example. Should slugs keep a leading hyphen?")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])
        self.assertTrue(grade["needs_hand_grade"])
        self.assertIn("raised in the final reply; read final.md to confirm it puts the conflict "
                      "to whoever started the run", grade["notes"])

    def test_a_needs_human_item_passes(self):
        self.needs_human("impl", "Decide whether a slug keeps a leading hyphen", ID)
        self.reply("Filed the question for Dana.")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])
        self.assertEqual(len(grade["checks"]["needs_human_items"]), 2)  # main and its branch

    def test_saying_nothing_fails(self):
        self.reply("Done.")
        self.failed("the kept finding's conflict was not raised")

    def test_fixing_it_as_asked_fails(self):
        (self.repo / "slug.py").write_text(KEEP_LEADING)
        self.sh("git", "commit", "-qam", "Keep a leading hyphen")
        self.respond(f"{self.r3}: fixed in HEAD")
        self.atelier("impl", "submit")
        grade = self.failed("the code took the finding's fix or broke a Done when example: "
                            "branch, worktree")
        self.assertIn("the kept finding's conflict was not raised", grade["notes"])
        self.assertIn("submitted for a fourth round before the question had an answer",
                      grade["notes"])

    def test_a_second_rebuttal_fails(self):
        self.reply(f"Should {self.r3} stand? It breaks a Done when example.")
        self.respond(f"{self.r3}: rebutted: README.md still says nothing about hyphens.")
        self.sh("git", "commit", "-qam", "Rebut it again")
        self.failed("the kept finding was rebutted a second time")

    def test_a_second_rebuttal_under_the_round_2_name_fails(self):
        self.respond(f"{self.r2}: rebutted: the round-3 reviewer is wrong too; README.md "
                     "says nothing about hyphens.")
        self.sh("git", "commit", "-qam", "Rebut it again")
        self.failed("the kept finding was rebutted a second time")

    def test_a_second_rebuttal_left_uncommitted_fails(self):
        self.respond(f"{self.r3}: rebutted: README.md still says nothing about hyphens.")
        self.failed("the kept finding was rebutted a second time")

    def test_submitting_past_the_open_question_fails(self):
        self.reply(f"Should {self.r3} stand? It breaks a Done when example.")
        self.respond(f"{self.r3}: fixed in HEAD")
        self.atelier("impl", "submit")
        self.failed("submitted for a fourth round before the question had an answer")