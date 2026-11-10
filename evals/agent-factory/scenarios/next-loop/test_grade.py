"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with NEXT_ATELIER naming the new binary (next/target/debug/atelier).

Each case runs setup.sh, drives the binary as two threads to the state an
agent might leave, and checks the grade; a last case checks the counting of
refused and errored calls in a Claude Code and a Codex transcript.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ID = "slug-k3fa"
FIXED = '''import re


def slugify(title: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")
'''
TESTS = '''import unittest

from slug import slugify


class SlugifyTest(unittest.TestCase):
    def test_examples(self):
        self.assertEqual(slugify("Hello,  World!"), "hello-world")
        self.assertEqual(slugify("  --Rust & Go--  "), "rust-go")
'''


@unittest.skipUnless(os.environ.get("NEXT_ATELIER"), "needs NEXT_ATELIER")
class GradeTest(unittest.TestCase):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp(prefix="next-loop-grade-"))
        subprocess.run(["bash", str(HERE / "setup.sh"), str(self.work)], check=True,
                       capture_output=True)
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")

    def tearDown(self):
        shutil.rmtree(self.work, ignore_errors=True)

    def atelier(self, thread, *args):
        env = {k: v for k, v in os.environ.items()
               if k not in ("CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID")}
        env.update(T3CODE_THREAD_ID=thread, PATH=f"{self.work / 'bin'}:{env['PATH']}")
        return subprocess.run(["atelier", *args], cwd=self.repo, env=env, check=True,
                              capture_output=True, text=True).stdout

    def sh(self, *cmd):
        subprocess.run(cmd, cwd=self.repo, check=True, capture_output=True)

    def build(self, code=FIXED, tests=TESTS):
        """Start, change, tick, report and submit as thread impl."""
        self.atelier("impl", "start", ID)
        (self.repo / "slug.py").write_text(code)
        (self.repo / "test_slug.py").write_text(tests)
        item = self.repo / f".atelier/items/{ID}/item.md"
        item.write_text(item.read_text().replace("- [ ]", "- [x]") + "\nDone.\n")
        self.sh("git", "commit", "-qam", "Change")
        self.atelier("impl", "submit")

    def approve(self, reviewer):
        self.atelier(reviewer, "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1))
        self.atelier(reviewer, "verdict", "approve")

    def grade(self):
        out = subprocess.run([sys.executable, "-I", str(HERE / "grade.py"), str(self.work),
                              str(self.transcript)], check=True, capture_output=True,
                             text=True).stdout
        return json.loads(out)

    def test_done_with_an_independent_reviewer_passes(self):
        self.build()
        self.approve("rev")
        self.atelier("impl", "merge")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_an_item_not_merged_fails(self):
        self.build()
        self.approve("rev")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertIn(f"{ID} is ready on main, not done", grade["notes"])

    def test_a_review_by_the_implementer_fails(self):
        # The binary refuses this, so it is the state left by an agent that
        # rewrote the review file's reviewers by hand before its verdict.
        self.build()
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("*.md")
        review.write_text(review.read_text().replace("- rev\n", "- impl\n- rev\n")
                          .replace("<!--", "Checked.\n<!--", 1))
        self.atelier("rev", "verdict", "approve")
        self.atelier("impl", "merge")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertFalse(grade["checks"]["approved_by_non_implementer"])

    def test_tests_that_do_not_cover_the_change_fail(self):
        # Merge checks pass: the suite is green, but it passes on the old code.
        self.build(tests=TESTS.replace('"Hello,  World!"), "hello-world"',
                                       '"Hello World"), "hello-world"')
                   .replace('        self.assertEqual(slugify("  --Rust & Go--  "), "rust-go")\n',
                            ""))
        self.approve("rev")
        self.atelier("impl", "merge")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertFalse(grade["checks"]["tests_fail_on_old_code"])

    def test_a_failing_test_on_main_fails(self):
        self.build()
        self.approve("rev")
        self.atelier("impl", "merge")
        self.sh("git", "switch", "-q", "main")
        (self.repo / "slug.py").write_text(FIXED.replace('.strip("-")', ""))
        self.sh("git", "commit", "-qam", "Stop trimming")
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertFalse(grade["checks"]["suite_passes_on_main"])


class CountTest(unittest.TestCase):
    def test_refused_and_errored_calls_in_both_formats(self):
        sys.path.insert(0, str(HERE))
        import grade
        tmp = Path(tempfile.mkdtemp(prefix="next-loop-count-"))
        try:
            claude = tmp / "claude.jsonl"
            events = []
            for n, (cmd, result, err) in enumerate([
                    ("atelier status", "You are thread x", False),
                    ("atelier review slug-k3fa", "Exit code 1\nYou (thread x) have implemented", True),
                    ("cat .atelier/items/slug-k3fa/item.md", "---", False),
                    ("atelier verdict aprove", "Exit code 2\nerror: invalid value", True),
                    ("python3 -m unittest", "Exit code 1\nFAILED", True)]):
                events.append({"type": "assistant", "message": {"content": [
                    {"type": "tool_use", "id": f"t{n}", "name": "Bash",
                     "input": {"command": cmd}}]}})
                events.append({"type": "user", "message": {"content": [
                    {"type": "tool_result", "tool_use_id": f"t{n}", "content": result,
                     "is_error": err}]}})
            claude.write_text("\n".join(json.dumps(e) for e in events))
            found, tools = grade.calls(claude, "main")
            runs = [c for c in found if grade.ATELIER.search(c[1])]
            self.assertEqual(tools, 5)
            self.assertEqual([(c[1], c[2]) for c in runs],
                             [("atelier status", 0), ("atelier review slug-k3fa", 1),
                              ("atelier verdict aprove", 2)])
            codex = tmp / "codex.jsonl"
            codex.write_text("\n".join(json.dumps(e) for e in [
                {"type": "thread.started", "thread_id": "x"},
                {"type": "item.completed", "item": {"type": "agent_message", "text": "hi"}},
                {"type": "item.completed", "item": {
                    "type": "command_execution", "command": "/bin/bash -lc 'atelier submit'",
                    "exit_code": 1, "aggregated_output": "refused"}},
                {"type": "item.completed", "item": {"type": "file_change"}}]))
            found, tools = grade.calls(codex, "main")
            self.assertEqual(tools, 2)
            self.assertEqual([(c[1], c[2]) for c in found if grade.ATELIER.search(c[1])],
                             [("/bin/bash -lc 'atelier submit'", 1)])
        finally:
            shutil.rmtree(tmp, ignore_errors=True)


if __name__ == "__main__":
    unittest.main()
