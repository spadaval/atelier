"""Tests every removal replay runs, from its test_grade.py:

    class RemovedTest(removed_tests.RemovedCase):
        case = HERE

A removal replay is a replay case (harness/replay.py) whose change was
approved and merged, and whose part was later taken out as unneeded: a row
of Tried and removed in docs/design.md. It uses replay.py's setup,
confinement and leak check unchanged, so case.toml holds the fields
replay.py names for a defect, holding the removal: `defect` is the part
later removed with design.md's reason, `fix` the commit that removed it and
`defect_item` that commit's item. It also names the `table` and `row` of
Tried and removed, the row's `reason`, the `part` later removed, and
`gone`, a string the change added under crates/ that the removal took out.

The tests are replay_tests.py's, but for the ones about a defect: the case
names its row of Tried and removed and the reason it gives; the base is
the change's parent, and the removal comes after the change and takes
`gone` out of crates/; a recorded review grades as the rubric says, passing
when a finding of any recommendation questions the need for the part; and
the judge is given the reason and the reviewer is not. The rest are run as
replay_tests.py has them: the fixture is the tree at the base with the
change in review; it holds none of the old tracker's process text, nothing
of the removal and nothing of the case file; a finding resting on an
unchanged base file can be checked; and a transcript that shows the removal
or a web search makes a run invalid. EVAL_ATELIER names the binary under
test.
"""
import importlib.util
import json
import re
import subprocess
import sys
from pathlib import Path

HARNESS = Path(__file__).resolve().parent
DESIGN = HARNESS.parents[2] / "docs/design.md"
spec = importlib.util.spec_from_file_location("replay_tests", HARNESS / "replay_tests.py")
replay_tests = importlib.util.module_from_spec(spec)
spec.loader.exec_module(replay_tests)
replay, judging, words = replay_tests.replay, replay_tests.judging, replay_tests.words
FINDING = """
## Finding 1: {title}
Recommendation: {recommendation}
{body}
"""


def tried_and_removed() -> dict:
    """{table's lead-in: {"What" cell: "Why" cell}} of design.md's Tried and
    removed, each table named by the paragraph above it."""
    text = DESIGN.read_text()
    section = text.split("\n## Tried and removed\n", 1)[1].split("\n## ", 1)[0]
    tables, lead = {}, None
    for block in section.split("\n\n"):
        lines = block.strip().splitlines()
        if lines and lines[0].startswith("|"):
            rows = tables.setdefault(lead, {})
            for line in lines[2:]:
                cells = [c.strip() for c in line.strip().strip("|").split(" | ")]
                rows[cells[0]] = cells[1]
        elif lines:
            lead = " ".join(lines)
    return tables


class RemovedCase(replay_tests.ReplayCase):
    case = None

    def finding(self, recommendation="fix now", title=None, body=None):
        return FINDING.format(title=title or "Is the part needed?",
                              recommendation=recommendation,
                              body=body or f"Nothing the Goal asks for needs {self.data['part']}. "
                              "Drop it, and meet the Goal with less.")

    def graded(self, need, others, reasons="pass", blocks="pass"):
        """The harness's grade, with the judge's answers given as recorded."""
        rubric = judging.read_rubric(self.case)
        answers = {"questions-need": need, "blocks": blocks, "others-supported": others,
                   "reasons": reasons}
        self.assertEqual(sorted(answers), sorted(q["id"] for q in rubric["questions"]))
        recorded = [{"id": q["id"], "role": q["role"], "kind": q["kind"],
                     "answer": answers[q["id"]], "line": "(recorded)", "reason": "recorded"}
                    for q in rubric["questions"]]
        facts = self.facts()
        return {"facts": facts, **judging.verdict(facts, recorded)}

    def test_the_case_names_its_row_in_defects_md(self):
        """Here, its row of Tried and removed: the row and its reason as
        design.md gives them, the change and its removal in order, and the
        part gone from crates/ after the removal."""
        d = self.data
        for name in ("table", "row", "reason", "part", "gone"):
            self.assertTrue(str(d.get(name, "")).strip(), f"case.toml lacks {name}")
        tables = tried_and_removed()
        table = next((rows for lead, rows in tables.items()
                      if lead and lead.startswith(d["table"])), None)
        self.assertIsNotNone(table, f"Tried and removed has no table led by {d['table']!r}")
        self.assertEqual(table.get(d["row"]), d["reason"],
                         f"Tried and removed gives another reason for {d['row']!r}")
        self.assertIn(words(d["reason"]), words(d["defect"]))
        self.assertEqual(replay.git("rev-parse", f"{d['change']}^").strip(), d["base"],
                         "the base is the change's parent")
        ancestor = subprocess.run(
            ["git", "-C", str(replay.REPO), "merge-base", "--is-ancestor", d["change"], d["fix"]])
        self.assertEqual(ancestor.returncode, 0, "the removal comes after the change")

        def holds(ref):
            return subprocess.run(
                ["git", "-C", str(replay.REPO), "grep", "-q", "-F", d["gone"], ref, "--",
                 "crates"]).returncode == 0
        self.assertFalse(holds(d["base"]), f"the base already holds {d['gone']!r}")
        self.assertTrue(holds(d["change"]), f"the change does not add {d['gone']!r}")
        self.assertTrue(holds(f"{d['fix']}^"), f"{d['gone']!r} is gone before the removal")
        self.assertFalse(holds(d["fix"]), f"the removal leaves {d['gone']!r}")

    def test_a_fix_now_finding_the_judge_reads_as_the_defect_passes(self):
        """Here, a fix-now finding that questions the need passes, and blocks."""
        self.review(self.finding(), "changes")
        grade = self.graded("pass", "pass")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["fix_now_findings"]["value"], 1)

    def test_a_deferred_finding_that_questions_the_need_passes_and_does_not_block(self):
        self.review(self.finding("defer"), "approve")
        grade = self.graded("pass", "pass", blocks="fail")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["fix_now_findings"]["value"], 0)

    def test_a_review_without_the_defect_fails_and_an_unsupported_finding_is_reported(self):
        """Here, a review that only asks for the part to be fixed fails."""
        self.review(self.finding(title="Test the new part's error path",
                                 body="Its failure path has no test. Add one."), "changes")
        grade = self.graded("fail", "fail", blocks="fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], ["graded questions-need (reviewer): fail, recorded"])

    def test_a_skip_is_reported_and_fails_nothing(self):
        self.review(self.finding())
        self.atelier("rev", "verdict", "changes", skip="in a hurry")
        grade = self.graded("pass", "pass", reasons="fail")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["departures_recorded"]["value"], [
            {"kind": "ATELIER_SKIP", "where": "verdict changes", "thread": "rev",
             "reason": "in a hurry"}])

    def test_the_judge_is_given_the_defect_and_the_fix_and_the_reviewer_is_not(self):
        """Here, the judge is given the part, design.md's reason and the
        change's diff; the reviewer's task names none of them."""
        d = self.data
        rubric = judging.read_rubric(self.case)
        self.assertIn(words(d["defect"]), words(rubric["text"]))
        self.assertIn(words(d["reason"]), words(rubric["text"]))
        self.assertIn(d["fix"][:9], rubric["text"])
        added = next(line for line in replay.git(
            "diff", d["base"], d["change"], "--", "crates").splitlines()
            if line.startswith("+") and len(line) > 30)
        self.assertIn(added, rubric["context"])
        self.review(self.finding())
        given = judging.material(self.work, "Done.", self.facts(), rubric["context"])
        prompt = judging.prompt(rubric["text"], rubric["questions"], given)
        # The arm is in the work directory's name. "skill" itself may be in
        # the change, as an item of the old tracker named its skill.
        self.assertNotIn(self.work.name, prompt)
        reply = json.dumps({"answers": [{"id": "others-supported", "answer": "pass",
                                         "line": added, "reason": "Supported."}]})
        [question] = [q for q in rubric["questions"] if q["id"] == "others-supported"]
        answer = judging.parse_answers(reply, [question], given)["others-supported"]
        self.assertEqual((answer["answer"], answer["line_found"]), ("pass", True))
        task = (self.case / "prompt.md").read_text()
        for marker in replay.markers(d) + [d["fix"][:7], d["gone"], "defect", "removed",
                                           "design.md"]:
            self.assertNotIn(marker, task)
        self.assertIsNone(re.search(r"\bneed", task))

    def test_every_removal_replay_shares_one_rubric(self):
        mine = (self.case / "rubric.md").read_text()
        for other in sorted(self.case.parent.glob("*/test_grade.py")):
            if "removed_tests" in other.read_text():
                self.assertEqual((other.parent / "rubric.md").read_text(), mine, other.parent.name)


if __name__ == "__main__":
    sys.exit("run from a case's test_grade.py")
