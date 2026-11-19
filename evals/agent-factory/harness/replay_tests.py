"""Tests every replay case runs (harness/replay.py), from its test_grade.py:

    class ReplayTest(replay_tests.ReplayCase):
        case = HERE

They build the case's fixture, with no model and no cargo build, and check
it: the case names its row in defects.md; the fixture is this repository's
tree at the base, tracked by today's binary with the item in review and the
change on its branch, with no remote and no history after the change;
nothing of the fix or the case file is in it; a recorded review grades as
the rubric says; the judge is given the defect and the fix's diff and the
reviewer is not; and a transcript that shows the fix makes a run invalid.
EVAL_ATELIER names the binary under test.
"""
import importlib.util
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HARNESS = Path(__file__).resolve().parent
DEFECTS = HARNESS.parents[2] / "docs/architecture/quality/scorecard-baseline/defects.md"


def load(name):
    spec = importlib.util.spec_from_file_location(name, HARNESS / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


run = load("run")
replay, judging = run.replay, run.judging
FOUND = """
## Finding 1: {title}
Recommendation: fix now
{body}
"""


def words(text: str) -> str:
    return " ".join(text.split())


@unittest.skipUnless(os.environ.get("EVAL_ATELIER"), "needs EVAL_ATELIER")
class ReplayCase(unittest.TestCase):
    case = None

    @classmethod
    def setUpClass(cls):
        if cls.case is None:
            raise unittest.SkipTest("the shared tests run from a case's test_grade.py")
        cls.data = replay.load(cls.case)
        cls.template = Path(tempfile.mkdtemp(prefix=f"{cls.case.name}-fixture-"))
        subprocess.run(["bash", str(cls.case / "setup.sh"), str(cls.template / "work")],
                       check=True, capture_output=True)

    @classmethod
    def tearDownClass(cls):
        shutil.rmtree(cls.template, ignore_errors=True)

    def setUp(self):
        # Named as a run of the skill arm is, which the judge must not see.
        self.root = Path(tempfile.mkdtemp(prefix=f"{self.case.name}-grade-"))
        self.work = self.root / "work-skill-house-0"
        shutil.copytree(self.template / "work", self.work, symlinks=True)
        run.log_atelier(self.work)
        self.repo = self.work / "repo"
        self.id = self.data["id"]

    def tearDown(self):
        shutil.rmtree(self.root, ignore_errors=True)

    def git(self, *args, cwd=None):
        return subprocess.run(["git", "-C", str(cwd or self.repo), *args], check=True,
                              capture_output=True, text=True).stdout

    def atelier(self, thread, *args):
        env = {k: v for k, v in os.environ.items() if k not in (
            "CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID", "CODEX_THREAD_ID", "ATELIER_SKIP")}
        env.update(T3CODE_THREAD_ID=thread, PATH=f"{self.work / 'bin'}:{env['PATH']}")
        return subprocess.run(["atelier", *args], cwd=self.repo, env=env, check=True,
                              capture_output=True, text=True).stdout

    def review(self, findings, verdict=None):
        """Write round 1's review as thread rev, and record `verdict`."""
        self.atelier("rev", "review", self.id)
        [path] = (self.repo / f".atelier/items/{self.id}/reviews").glob("1-*.md")
        path.write_text(path.read_text().replace("<!--", "Read the change.\n<!--", 1)
                        + findings)
        if verdict:
            self.atelier("rev", "verdict", verdict)
            self.git("add", "-A", ".atelier")
            self.git("commit", "-qm", f"Review {self.id}: {verdict}")

    def facts(self):
        out = subprocess.run([sys.executable, "-I", str(self.case / "grade.py"), str(self.work)],
                             check=True, capture_output=True, text=True).stdout
        return json.loads(out)["facts"]

    def graded(self, found, others):
        """The harness's grade, with the judge's answers given as recorded."""
        rubric = judging.read_rubric(self.case)
        answers = {"found": found, "others-supported": others}
        self.assertEqual(sorted(answers), sorted(q["id"] for q in rubric["questions"]))
        recorded = [{"id": q["id"], "role": q["role"], "kind": q["kind"],
                     "answer": answers[q["id"]], "line": "(recorded)", "reason": "recorded"}
                    for q in rubric["questions"]]
        facts = self.facts()
        return {"facts": facts, **judging.verdict(facts, recorded)}

    def test_the_case_names_its_row_in_defects_md(self):
        d = self.data
        row = next((line for line in DEFECTS.read_text().splitlines()
                    if line.startswith(f"| {d['row']} | {d['defect_item']} |")), None)
        self.assertIsNotNone(row, f"defects.md has no row {d['row']} for {d['defect_item']}")
        cells = [c.strip() for c in row.strip("|").split("|")]
        fix, cause, item, outcome = cells[3], cells[5], cells[6], cells[7]
        self.assertIn(d["fix"][:8], fix)
        self.assertEqual(d["change"][:8], cause)
        self.assertEqual(d["item"], item)
        self.assertEqual(outcome, "After merge")
        self.assertEqual(replay.git("rev-parse", f"{d['change']}^").strip(), d["base"],
                         "the base is the change's parent")

    def test_the_fixture_is_the_tree_at_base_with_the_change_in_review(self):
        d = self.data

        def blobs(cwd, ref):
            out = subprocess.run(["git", "-C", str(cwd), "ls-tree", "-r", ref],
                                 check=True, capture_output=True, text=True).stdout
            return {line.split("\t")[1]: line.split()[2] for line in out.splitlines()
                    if line.split("\t")[1].split("/")[0] not in replay.DROPPED}

        here, then = blobs(self.repo, "main"), blobs(replay.REPO, d["base"])
        agents = here.pop("AGENTS.md", None), then.pop("AGENTS.md", None)
        self.assertEqual(here, then)
        if agents[1]:
            text = (self.repo / "AGENTS.md").read_text()
            for name in replay.PROCESS_SECTIONS:
                self.assertNotIn(f"## {name}\n", text)
        self.assertEqual(blobs(self.repo, self.id), {**blobs(replay.REPO, d["change"]),
                                                     **({"AGENTS.md": agents[0]}
                                                        if agents[0] else {})})
        tracked = self.git("ls-tree", "-r", "--name-only", self.id).split()
        self.assertFalse([p for p in tracked if p.split("/")[0] in replay.DROPPED
                          and not p.startswith(f".atelier/items/{self.id}/")
                          and p != ".atelier/config.toml"])
        self.assertEqual(self.git("remote").strip(), "")
        self.assertEqual(sorted(self.git("for-each-ref", "--format=%(refname:short)",
                                         "refs/heads/").split()), sorted(["main", self.id]))
        self.assertEqual(self.git("rev-list", "--count", "main").strip(), "1")
        log = self.git("log", "--format=%s", f"main..{self.id}").splitlines()
        subject = replay.git("log", "-1", "--format=%s", d["change"]).strip()
        self.assertEqual(log[2], subject, "the change follows start, then the report and submit")
        record = (self.repo / f".atelier/items/{self.id}/item.md").read_text()
        self.assertIn("status: in_review", record)
        self.assertIn(words(d["goal"]), words(record))
        for line in d["done_when"]:
            self.assertIn(words(f"- [x] {line}"), words(record))
        self.assertIn(f"Then: atelier review {self.id}", self.atelier("rev", "status"))

    def test_nothing_of_the_fix_or_the_case_file_is_in_the_fixture(self):
        found = replay.markers(self.data) + [words(self.data["defect"]), str(self.case)]
        texts = [self.git("log", "--all", "-p", "--format=%H %s%n%b")]
        for path in self.work.rglob("*"):
            if path.is_file() and ".git" not in path.relative_to(self.work).parts:
                texts.append(path.read_text(errors="replace"))
        text = words("\n".join(texts))
        for marker in found:
            self.assertNotIn(words(marker), text)

    def test_a_fix_now_finding_the_judge_reads_as_the_defect_passes(self):
        self.review(FOUND.format(title="The defect", body=self.data["defect"]), "changes")
        grade = self.graded("pass", "pass")
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["fix_now_findings"]["value"], 1)

    def test_a_review_without_the_defect_fails_and_an_unsupported_finding_is_reported(self):
        self.review(FOUND.format(title="Rename a variable", body="It reads badly."), "changes")
        grade = self.graded("fail", "fail")
        self.assertFalse(grade["pass"])
        self.assertEqual(grade["failed"], ["graded found (reviewer): fail, recorded"])

    def test_no_review_fails_on_the_facts(self):
        grade = self.graded("fail", "pass")
        self.assertIn("fact a_round_1_review_by_another_thread: false", grade["failed"])

    def test_the_judge_is_given_the_defect_and_the_fix_and_the_reviewer_is_not(self):
        rubric = judging.read_rubric(self.case)
        self.assertIn(words(self.data["defect"]), words(rubric["text"]))
        fix = replay.git("show", "--format=", self.data["fix"], "--", "crates").splitlines()
        hunk = next(line for line in fix if line.startswith("@@"))
        self.assertIn(hunk, rubric["text"])
        self.review(FOUND.format(title="The defect", body=self.data["defect"]))
        prompt = judging.prompt(rubric["text"], rubric["questions"],
                                judging.material(self.work, "Done.", self.facts()))
        self.assertNotIn(self.work.name, prompt)
        self.assertNotIn("skill", prompt.lower())
        task = (self.case / "prompt.md").read_text()
        for marker in replay.markers(self.data) + [self.data["fix"][:7], "defect"]:
            self.assertNotIn(marker, task)

    def test_a_transcript_that_shows_the_fix_or_a_web_search_is_invalid(self):
        path = self.root / "transcript.jsonl"
        path.write_text(json.dumps({"type": "item.completed", "item": {
            "type": "agent_message", "text": "Reviewed it."}}) + "\n")
        self.assertIsNone(replay.leak(self.case, [path]))
        for marker in replay.markers(self.data):
            path.write_text(json.dumps({"type": "item.completed", "item": {
                "type": "command_execution", "aggregated_output": f"x {marker} y"}}) + "\n")
            self.assertIn(repr(marker), replay.leak(self.case, [path]) or "")
        for event in ({"type": "item.completed", "item": {"type": "web_search", "query": "x"}},
                      {"type": "assistant", "message": {"content": [
                          {"type": "tool_use", "name": "WebFetch", "input": {}}]}}):
            path.write_text(json.dumps(event) + "\n")
            self.assertIn("web search or fetch", replay.leak(self.case, [path]) or "")
        self.assertTrue(re.fullmatch(r"[0-9a-f]{7}", replay.markers(self.data)[0]))
