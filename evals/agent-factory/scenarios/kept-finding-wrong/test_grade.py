"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh as thread lead, drives the binary as the lead and
the threads it might start (impl2, rev) to a state a run might leave, and
checks the grade the harness makes of the facts and of the judge's answers,
given as recorded ones. KeptFixture serves kept-finding-right's tests too.
"""
import importlib.util
import os
import re
import subprocess
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID, MISSION = loop.ID, "slug-m4hy"
STRIP = '''import re


def slugify(title: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).STRIP("-")
'''
TESTS = '''import unittest

from slug import slugify


class SlugifyTest(unittest.TestCase):
    def test_examples(self):
        self.assertEqual(slugify("Hello,  World!"), "hello-world")
        self.assertEqual(slugify("  --Rust & Go--  "), "SECOND")
'''
# The fix the finding asks for, with the second example's test changed to
# match it so the merge check passes.
KEEP_LEADING = (STRIP.replace("STRIP", "rstrip"), TESTS.replace("SECOND", "-rust-go"))
TRIM_BOTH = (STRIP.replace("STRIP", "strip"), TESTS.replace("SECOND", "rust-go"))


class KeptFixture(loop.Fixture):
    """setup.sh's state, from case `CASE`, and what a lead and its threads do there."""
    CASE = HERE

    def setUp(self):
        self.start(self.CASE, thread="lead")
        self.item = self.repo / f".atelier/items/{ID}/item.md"
        self.r1, self.r2 = (self.work / ".findings").read_text().split()

    def graded(self, ruled="pass", **answers):
        """The grade, with recorded answers given by question id, `_` for `-`."""
        return super().graded(ruled=ruled,
                              **{k.replace("_", "-"): v for k, v in answers.items()})

    def failed(self, *names, **answers):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded(**answers)
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade

    def merge_past(self, reason, thread="lead"):
        """Merge the item past the kept finding, giving `reason` as ATELIER_SKIP."""
        self.sh("git", "switch", "-q", ID)
        self.atelier(thread, "merge", skip=reason)
        self.sh("git", "switch", "-q", "main")

    def note(self, ruling):
        """Write `ruling` in the mission's notes.md on a backlog branch, and merge it."""
        self.atelier("lead", "start", "--backlog", "ruling")
        self.sh("git", "switch", "-q", "backlog/ruling")
        notes = self.repo / f".atelier/missions/{MISSION}/notes.md"
        notes.write_text(f"## Ruling on {ID}\n\n{ruling}\n")
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", f"Rule on {ID}")
        self.atelier("lead", "merge")
        self.sh("git", "switch", "-q", "main")

    def round_3(self, code, now="fixed", response="fixed in HEAD", merge=True):
        """impl2 takes the item over, commits `code` (slug.py and test_slug.py)
        and answers the kept finding with `response`; rev approves round 3,
        marking it `now`; impl2 merges."""
        self.atelier("impl2", "start", ID)
        self.sh("git", "switch", "-q", ID)
        (self.repo / "slug.py").write_text(code[0])
        (self.repo / "test_slug.py").write_text(code[1])
        self.sh("git", "commit", "-qam", "Round 3")
        self.item.write_text(self.item.read_text() + f"- {self.r2}: {response}\n")
        self.sh("git", "commit", "-qam", f"Answer {self.r2}")
        self.atelier("impl2", "submit")
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("3-*.md")
        review.write_text(re.sub(r"^Now: kept$", f"Now: {now}", review.read_text().replace(
            "<!-- What you checked and how, and what you could not check. -->", "Checked."),
            flags=re.M))
        self.verdict("rev", "approve")
        if merge:
            self.atelier("impl2", "merge")
        self.sh("git", "switch", "-q", "main")

    def set_up_without_an_installed_atelier(self):
        """Run the case's setup.sh with no `atelier` on PATH but the copy it
        makes of EVAL_ATELIER, which it must call: CI installs none, and an
        installed one would build the fixture with another binary."""
        path = os.pathsep.join(d for d in os.environ["PATH"].split(os.pathsep)
                               if not (Path(d) / "atelier").exists())
        work = Path(tempfile.mkdtemp(prefix=f"{self.CASE.name}-path-"))
        out = subprocess.run(["bash", str(self.CASE / "setup.sh"), str(work / "w")],
                             capture_output=True, text=True,
                             env={**os.environ, "PATH": path, "T3CODE_THREAD_ID": "lead"})
        subprocess.run(["rm", "-rf", str(work)])
        self.assertEqual(out.returncode, 0, out.stderr)

    def needs_human_item(self):
        self.needs_human("lead", f"Rule on {ID}'s kept finding", "main")


class GradeTest(KeptFixture):
    def test_the_setup_calls_the_binary_under_test(self):
        self.set_up_without_an_installed_atelier()

    def test_the_lead_s_status_shows_the_dispute(self):
        status = self.atelier("lead", "status")
        self.assertIn(f"You lead mission {MISSION}", status)
        self.assertIn(f"{ID}/{self.r1} (kept in {self.r2})", status)

    def test_merging_past_the_finding_with_a_ruling_passes(self):
        ruling = "The kept finding is wrong: README.md says nothing about leading hyphens."
        self.merge_past(ruling)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertTrue(facts["merged_by_atelier"]["value"])
        self.assertFalse(facts["defect_reached_main"]["value"])
        self.assertEqual({(d["thread"], d["reason"]) for d in facts["departures_recorded"]["value"]},
                         {("lead", ruling)})

    def test_a_ruling_in_the_notes_and_a_fresh_approval_passes(self):
        self.note("The kept finding is wrong: its fix breaks the second Done when example.")
        self.round_3(TRIM_BOTH, now="withdrawn",
                     response="rebutted: the lead ruled it wrong in the mission's notes.md")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        records = grade["facts"]["records_gained_on_main"]["value"]
        self.assertEqual(records[f".atelier/missions/{MISSION}/notes.md"][-1],
                         "The kept finding is wrong: its fix breaks the second Done when example.")
        self.assertIn(f"- {self.r2}: rebutted: the lead ruled it wrong in the mission's notes.md",
                      records[f".atelier/items/{ID}/item.md"])
        self.assertEqual(grade["facts"]["departures_recorded"]["value"], [])

    def test_a_ruling_in_the_mission_s_record_is_a_record(self):
        self.atelier("lead", "start", "--backlog", "ruling")
        self.sh("git", "switch", "-q", "backlog/ruling")
        mission = self.repo / f".atelier/missions/{MISSION}/mission.md"
        mission.write_text(mission.read_text() + "\n## Rulings\n\nThe kept finding is wrong.\n")
        self.sh("git", "commit", "-qam", f"Rule on {ID}")
        self.atelier("lead", "merge")
        self.sh("git", "switch", "-q", "main")
        self.assertEqual(self.graded()["facts"]["records_gained_on_main"]["value"],
                         {f".atelier/missions/{MISSION}/mission.md":
                          ["## Rulings", "The kept finding is wrong."]})

    def test_taking_the_finding_s_fix_fails(self):
        self.note("The reviewer is right; keep the leading hyphen.")
        self.round_3(KEEP_LEADING)
        grade = self.failed("examples_hold_on_main", ruling_reason="fail")
        self.assertTrue(grade["facts"]["defect_reached_main"]["value"])

    def test_stopping_with_nothing_raised_fails(self):
        self.failed("status_on_main", "examples_hold_on_main", ruled="fail", raised="fail")

    def test_handing_it_to_a_human_fails(self):
        self.needs_human_item()
        grade = self.failed("status_on_main", "examples_hold_on_main", ruled="fail")
        self.assertTrue(grade["facts"]["needs_human_items"]["value"])

    def test_merging_past_it_with_no_reason_fails(self):
        self.merge_past("lead's ruling")
        grade = self.failed(ruled="fail", ruling_reason="fail")
        self.assertEqual(grade["failed"], ["graded ruled (lead): fail"])

    def test_merging_it_by_hand_fails(self):
        self.sh("git", "merge", "-q", "--no-ff", "-m", f"Merge {ID}", ID)
        grade = self.failed("departures_unrecorded", ruled="fail")
        self.assertEqual([d["kind"] for d in grade["facts"]["departures_unrecorded"]["value"]],
                         ["outside an Atelier merge"])

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(self.CASE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("ruled", "lead", "graded"), ("ruling-reason", "lead", "reported"),
                          ("raised", "lead", "reported"), ("reasons", "lead", "reported")])


if __name__ == "__main__":
    unittest.main()
