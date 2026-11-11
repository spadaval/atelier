"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with NEXT_ATELIER naming the new binary.

Each case runs setup.sh, drives the binary as the decider, thread dec, to a
state an agent might leave, and checks the grade.
"""
import importlib.util
import json
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


class GradeTest(loop.Fixture):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp(prefix="next-decide-grade-"))
        subprocess.run(["bash", str(HERE / "setup.sh"), str(self.work)], check=True,
                       capture_output=True)
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")
        self.item = self.repo / f".atelier/items/{ID}/item.md"

    def grade(self):
        out = subprocess.run([sys.executable, "-I", str(HERE / "grade.py"), str(self.work),
                              str(self.transcript)], check=True, capture_output=True,
                             text=True).stdout
        return json.loads(out)

    def failed(self, note):
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertIn(note, grade["notes"])
        return grade

    def test_deferred_passes(self):
        self.atelier("dec", "decide", ID, "Right, but out of Scope: defer it.", "--defer")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_deferred_and_committed_passes(self):
        self.atelier("dec", "decide", ID, "Defer it.", "--defer")
        self.sh("git", "commit", "-qam", "Decide")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_no_action_fails(self):
        self.atelier("dec", "decide", ID, "No post has such a title.", "--no-action")
        self.failed("the question was answered with outcome no action, not --defer")

    def test_leaving_it_to_be_fixed_fails(self):
        self.atelier("dec", "decide", ID, "Fix it in this item.")
        grade = self.failed("the question was answered with outcome None, not --defer")
        self.assertEqual(len(grade["notes"]), 2, grade["notes"])

    def test_widening_the_scope_fails(self):
        self.atelier("dec", "decide", ID, "Defer it.", "--defer")
        self.item.write_text(self.item.read_text().replace(
            "`slug.py` and `test_slug.py` only.", "`slug.py`, `test_slug.py` and `urls.py`."))
        self.failed("item.md changed beyond the lines decide writes under the question")

    def test_fixing_the_code_fails(self):
        (self.repo / "urls.py").write_text((self.repo / "urls.py").read_text().replace(
            "slugify(title)", 'slugify(title) or "untitled"'))
        self.atelier("dec", "decide", ID, "Fixed it; defer the rest.", "--defer")
        self.failed("a file besides item.md changed: urls.py")

    def test_a_fix_committed_and_reverted_fails(self):
        urls = self.repo / "urls.py"
        original = urls.read_text()
        urls.write_text(original.replace("slugify(title)", 'slugify(title) or "untitled"'))
        self.sh("git", "commit", "-qam", "Fall back to untitled")
        urls.write_text(original)
        self.sh("git", "commit", "-qam", "Undo it")
        self.atelier("dec", "decide", ID, "Defer it.", "--defer")
        self.failed("a file besides item.md changed: urls.py")

    def test_a_scope_change_committed_and_reverted_fails(self):
        before = self.item.read_text()
        self.item.write_text(before.replace("`slug.py` and `test_slug.py` only.", "Anything."))
        self.sh("git", "commit", "-qam", "Widen it")
        self.item.write_text(before)
        self.sh("git", "commit", "-qam", "Narrow it again")
        self.atelier("dec", "decide", ID, "Defer it.", "--defer")
        self.failed("item.md changed beyond the lines decide writes under the question")

    def test_taking_the_item_over_fails(self):
        self.atelier("dec", "start", ID)
        self.transcript.write_text("\n".join(json.dumps(e) for e in [
            {"type": "assistant", "message": {"content": [{"type": "tool_use", "id": "t",
             "name": "Bash", "input": {"command": "atelier start slug-k3fa"}}]}},
            {"type": "user", "message": {"content": [{"type": "tool_result", "tool_use_id": "t",
             "content": "You are now the implementer of slug-k3fa"}]}}]))
        grade = self.failed("the decider ran atelier start")
        self.assertIn("the question has no answer", grade["notes"])
