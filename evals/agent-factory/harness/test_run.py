"""Tests for the isolation canary's verdict, with Claude Code replaced by a
fixed reply. Run with `python3 -m unittest` from this directory."""
import importlib.util
import json
import shutil
import tempfile
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location("run", Path(__file__).with_name("run.py"))
run = importlib.util.module_from_spec(spec)
spec.loader.exec_module(run)


class CanaryTest(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-canary-test-"))
        self.real_run_claude = run.run_claude

    def tearDown(self):
        run.run_claude = self.real_run_claude
        shutil.rmtree(self.tmp, ignore_errors=True)

    def canary(self, reply):
        """Run check_isolation with a Claude Code that answers reply(repo, heading).
        Returns None if the canary passed, else the abort message."""
        def fake(cmd, cwd, env, transcript, timeout):
            heading = (Path(cwd) / "AGENTS.md").read_text().splitlines()[0].lstrip("# ")
            transcript.write_text(json.dumps(
                {"type": "result", "subtype": "success",
                 "result": reply(Path(cwd), heading)}) + "\n")
            return 0
        run.run_claude = fake
        try:
            run.check_isolation(self.tmp / "root", "high", self.tmp)
        except SystemExit as exc:
            return str(exc.code)
        return None

    def test_reports_its_own_agents_md(self):
        self.assertIsNone(self.canary(
            lambda repo, heading: f"{repo}/AGENTS.md | # {heading}\nOther files: N/A"))

    def test_a_path_in_the_repository_is_not_its_contents(self):
        message = self.canary(lambda repo, heading: (
            f"{repo}/AGENTS.md shows as untracked in the git status snapshot, "
            "but its contents were not loaded"))
        self.assertIsNotNone(message)
        self.assertIn("did not report its repository's own AGENTS.md", message)

    def test_a_bold_foreign_path_is_foreign(self):
        message = self.canary(lambda repo, heading: (
            f"{repo}/AGENTS.md | # {heading}\n**/home/u/.claude/CLAUDE.md** | Working for U"))
        self.assertIsNotNone(message)
        self.assertIn("/home/u/.claude/CLAUDE.md.", message)

    def test_a_home_relative_path_is_foreign(self):
        message = self.canary(lambda repo, heading: (
            f"{repo}/AGENTS.md | # {heading}\n~/.claude/CLAUDE.md | Working for U"))
        self.assertIsNotNone(message)
        self.assertIn(str(Path.home() / ".claude" / "CLAUDE.md"), message)


class AgentEnvTest(unittest.TestCase):
    def test_atelier_update_check_is_off(self):
        # Its notice names the host's own skill checkout, not the staged one.
        tmp = Path(tempfile.mkdtemp(prefix="af-env-test-"))
        try:
            self.assertEqual(run.agent_env(tmp).get("ATELIER_NO_UPDATE_CHECK"), "1")
        finally:
            shutil.rmtree(tmp, ignore_errors=True)


class CodexTest(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-codex-test-"))
        self.real_auth = run.CODEX_AUTH

    def tearDown(self):
        run.CODEX_AUTH = self.real_auth
        shutil.rmtree(self.tmp, ignore_errors=True)

    def test_a_codex_transcript_gives_its_last_reply_once_its_turn_completed(self):
        transcript = self.tmp / "t.jsonl"
        transcript.write_text("Reading additional input from stdin...\n" + "\n".join(
            json.dumps(e) for e in [
                {"type": "thread.started", "thread_id": "x"},
                {"type": "item.completed", "item": {"type": "agent_message", "text": "a"}},
                {"type": "item.completed", "item": {"type": "agent_message", "text": "b"}},
                {"type": "turn.completed", "usage": {}}]))
        info = run.read_transcript(transcript)
        self.assertEqual(info["result"]["result"], "b")
        self.assertIsNone(run.invalid_reason("none", 0, info))
        transcript.write_text(json.dumps({"type": "thread.started", "thread_id": "x"}))
        self.assertEqual(run.invalid_reason("none", 0, run.read_transcript(transcript)),
                         "no successful result event")

    def test_the_sign_in_is_linked_into_a_scratch_home_and_removed(self):
        run.CODEX_AUTH = self.tmp / "host-auth.json"
        run.CODEX_AUTH.write_text("{}")
        env = run.agent_env(self.tmp / "work", "codex")
        link = Path(env["CODEX_HOME"]) / "auth.json"
        self.assertEqual(env["HOME"], str(self.tmp / "work" / "home"))
        self.assertTrue(link.is_symlink())
        run.unlink_auth(self.tmp / "work")
        self.assertFalse(link.exists() or link.is_symlink())
        self.assertTrue(run.CODEX_AUTH.exists())


if __name__ == "__main__":
    unittest.main()
