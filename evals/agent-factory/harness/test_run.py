"""Tests for the harness: the isolation canary's verdict, with Claude Code
replaced by a fixed reply; staging; validity; and the limit of two agents,
from the command lines and config it builds and `delegate` with a stand-in
agent. Run with `python3 -m unittest` from this directory."""
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent

spec = importlib.util.spec_from_file_location("run", Path(__file__).with_name("run.py"))
run = importlib.util.module_from_spec(spec)
spec.loader.exec_module(run)


class CanaryTest(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-canary-test-"))
        self.real_run_claude = run.run_claude
        self.init = {"type": "system", "subtype": "init", "tools": ["Bash", "Read"]}

    def tearDown(self):
        run.run_claude = self.real_run_claude
        shutil.rmtree(self.tmp, ignore_errors=True)

    def canary(self, reply):
        """Run check_isolation with a Claude Code that answers reply(repo, heading).
        Returns None if the canary passed, else the abort message."""
        def fake(cmd, cwd, env, transcript, timeout):
            heading = (Path(cwd) / "AGENTS.md").read_text().splitlines()[0].lstrip("# ")
            transcript.write_text(json.dumps(self.init) + "\n" + json.dumps(
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

    def test_a_run_offered_an_agent_tool_fails_the_canary(self):
        self.init["tools"].append("Workflow")
        message = self.canary(lambda repo, heading: f"{repo}/AGENTS.md | # {heading}")
        self.assertIn("agent-starting tools offered: ['Workflow']", message)

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



class StageTest(unittest.TestCase):
    """The skill directory named is staged as agent-factory where each agent
    loads a user's skills, without maintainers/."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-stage-test-"))
        subprocess.run(["git", "init", "-q", str(self.tmp / "repo")], check=True)

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def test_the_new_skill_is_staged_for_claude_code_under_the_live_name(self):
        run.stage_for("claude", "HEAD", self.tmp, "skills/agent-factory-next")
        skill = self.tmp / "repo" / ".claude" / "skills" / "agent-factory"
        self.assertIn("name: agent-factory", (skill / "SKILL.md").read_text())
        self.assertTrue((skill / "roles" / "reviewer.md").is_file())
        self.assertIn(".claude/", (self.tmp / "repo/.git/info/exclude").read_text())

    def test_the_skill_is_staged_for_codex_in_its_scratch_home(self):
        run.stage_for("codex", "HEAD", self.tmp, "skills/agent-factory-next")
        skill = self.tmp / "home" / ".codex" / "skills" / "agent-factory"
        self.assertTrue((skill / "roles" / "decider.md").is_file())
        self.assertFalse((self.tmp / "repo" / ".claude").exists())

    def test_maintainers_is_left_out(self):
        run.stage_for("claude", "HEAD", self.tmp, "skills/agent-factory")
        skill = self.tmp / "repo" / ".claude" / "skills" / "agent-factory"
        self.assertTrue((skill / "SKILL.md").is_file())
        self.assertFalse((skill / "maintainers").exists())


def skills_message(home, names):
    """A model input item listing skills, as Codex 0.167 writes it."""
    text = ("<skills_instructions>\n## Skills\n### Skill roots\n"
            f"- `r0` = `{home}/skills`\n- `r1` = `{home}/skills/.system`\n"
            "- `r2` = `/x/plugins/cache/pages`\n### Available skills\n"
            + "".join(f"- {n}: does things. (file: {f})\n" for n, f in names)
            + "</skills_instructions>")
    return {"type": "message", "role": "developer",
            "content": [{"type": "input_text", "text": text}]}


def tools_item(*names):
    return {"type": "additional_tools", "tools": [
        {"type": "namespace", "name": "collaboration",
         "tools": [{"type": "function", "name": n} for n in names]}]}


class ValidityTest(unittest.TestCase):
    """A run whose arm and loaded skills disagree, or that was offered an
    unbounded way to start an agent, is invalid."""
    home = Path("/w/home/.codex")
    ok = {"type": "result", "subtype": "success"}

    def info(self, **init):
        return {"init": {"mcp_servers": [], **init}, "result": self.ok}

    def test_codex_skills_are_read_from_the_session_record(self):
        facts = run.codex_given([skills_message(self.home, [
            ("imagegen", "r1/imagegen/SKILL.md"),
            ("agent-factory", "r0/agent-factory/SKILL.md")])], self.home)
        self.assertTrue(facts["agent_factory"])
        self.assertEqual(facts["foreign_skills"], [])
        self.assertIsNone(run.invalid_reason("skill", 0, self.info(**facts)))
        self.assertEqual(run.invalid_reason("none", 0, self.info(**facts)),
                         "agent-factory present=True in arm none")
        bare = run.codex_given([skills_message(self.home, [
            ("imagegen", "r1/imagegen/SKILL.md")])], self.home)
        self.assertEqual(run.invalid_reason("skill", 0, self.info(**bare)),
                         "agent-factory present=False in arm skill")

    def test_a_plugin_skill_is_foreign(self):
        facts = run.codex_given([skills_message(self.home, [
            ("pages:write-page", "r2/write-page/SKILL.md")])], self.home)
        self.assertEqual(facts["foreign_skills"], ["pages:write-page (/x/plugins/cache/pages)"])
        self.assertIn("outside the scratch home", run.invalid_reason("none", 0, self.info(**facts)))

    def test_spawn_agent_is_invalid_only_without_threads(self):
        facts = run.codex_given([tools_item("spawn_agent", "wait_agent")], self.home)
        self.assertTrue(facts["spawn_agent"])
        self.assertEqual(run.invalid_reason("none", 0, self.info(**facts)),
                         "spawn_agent was offered in a scenario without threads")
        self.assertIsNone(run.invalid_reason("none", 0, self.info(**facts), threads=True))

    def test_a_claude_run_offered_the_agent_tool_is_invalid(self):
        transcript = Path(tempfile.mkdtemp(prefix="af-valid-")) / "t.jsonl"
        transcript.write_text("\n".join(json.dumps(e) for e in [
            {"type": "system", "subtype": "init", "tools": ["Bash", "Agent", "Read"],
             "skills": []}, self.ok]))
        info = run.read_transcript(transcript)
        shutil.rmtree(transcript.parent)
        self.assertEqual(run.invalid_reason("none", 0, info),
                         "agent-starting tools offered: ['Agent']")


class LimitTest(unittest.TestCase):
    """Each rule of the limit of two agents, from the command lines and
    config the harness builds, without starting an agent."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-limit-test-"))
        (self.tmp / "bin").mkdir()
        (self.tmp / "repo").mkdir()
        self.real_auth = run.CODEX_AUTH
        run.CODEX_AUTH = self.tmp / "host-auth.json"
        run.CODEX_AUTH.write_text("{}")

    def tearDown(self):
        run.CODEX_AUTH = self.real_auth
        shutil.rmtree(self.tmp, ignore_errors=True)

    def disallowed(self, argv):
        return argv[argv.index("--disallowedTools") + 1].split(",")

    def test_claude_code_is_denied_its_agent_tools_in_every_agent(self):
        first = run.agent_command("claude", "x", "haiku", "low", 1, self.tmp / "repo")
        self.assertEqual(set(self.disallowed(first)) & {"Agent", "Task", "Workflow"},
                         {"Agent", "Task", "Workflow"})
        env, note = run.threads(self.tmp, self.tmp / "run", run.agent_env(self.tmp), "claude",
                                "haiku", "low", {}, "thread-1")
        child = json.loads((self.tmp / ".threads.json").read_text())["argv"]
        self.assertEqual(self.disallowed(child), list(run.AGENT_TOOLS))
        self.assertEqual(env["T3CODE_THREAD_ID"], "thread-1")
        self.assertTrue((self.tmp / "bin" / "delegate").is_file())
        self.assertIn("delegate", note)

    def test_codex_has_no_delegate_and_spawn_agent_only_with_threads(self):
        env, note = run.threads(self.tmp, self.tmp / "run", run.agent_env(self.tmp, "codex", True),
                                "codex", "gpt-6-luna", "low", {}, "thread-1")
        self.assertFalse((self.tmp / "bin" / "delegate").exists())
        self.assertNotIn("delegate", note)
        config = (Path(env["CODEX_HOME"]) / "config.toml").read_text()
        self.assertIn("disable_native_tools = false", config)
        self.assertIn("max_threads = 1", config)
        run.agent_env(self.tmp, "codex", False)
        config = (Path(env["CODEX_HOME"]) / "config.toml").read_text()
        self.assertIn("disable_native_tools = true", config)
        self.assertIn("plugins = false", config)

    def test_jobs_above_two_and_threads_above_one_are_refused(self):
        self.assertIn("above 2", run.check_jobs(3, ["vacuous-test-review"]))
        self.assertIsNone(run.check_jobs(2, ["vacuous-test-review", "next-decide"]))
        self.assertIn("--jobs 1", run.check_jobs(2, ["vacuous-test-review", "next-loop"]))
        self.assertIsNone(run.check_jobs(1, ["next-loop"]))
        refused = subprocess.run([sys.executable, str(HERE / "run.py"), "next-loop",
                                  "--jobs", "2", "--out", str(self.tmp / "out")],
                                 capture_output=True, text=True)
        self.assertEqual(refused.returncode, 2)
        self.assertIn("--jobs 1", refused.stderr)
        self.assertFalse((self.tmp / "out").exists())

    def test_the_peak_counts_overlapping_agents(self):
        self.assertEqual(run.peak([{"start": 0, "end": 10}, {"start": 1, "end": 4},
                                   {"start": 4, "end": 9}]), 2)
        self.assertEqual(run.peak([{"start": 0, "end": 1}, {"start": 2, "end": 3}]), 1)


@unittest.skipUnless(shutil.which("codex"), "needs codex on PATH")
class CodexPromptTest(unittest.TestCase):
    """Codex's own rendering of what a session is given, from the scratch
    home the harness builds (`codex debug prompt-input` calls no model)."""
    setUp, tearDown = LimitTest.setUp, LimitTest.tearDown

    def given(self, threads):
        env = run.agent_env(self.tmp, "codex", threads)
        run.stage_for("codex", "HEAD", self.tmp, "skills/agent-factory-next")
        subprocess.run(["git", "init", "-q", str(self.tmp / "repo")], check=True)
        out = subprocess.run(["codex", "debug", "prompt-input", "-c", 'model="gpt-6-luna"',
                              "hi"], cwd=self.tmp / "repo", env=env, stdin=subprocess.DEVNULL,
                             capture_output=True, text=True, timeout=120)
        return run.codex_given(json.loads(out.stdout), Path(env["CODEX_HOME"]))

    def test_without_threads_spawn_agent_is_gone_and_the_skill_loads(self):
        facts = self.given(False)
        self.assertFalse(facts["spawn_agent"])
        self.assertTrue(facts["agent_factory"])
        self.assertEqual(facts["foreign_skills"], [])

    def test_with_threads_spawn_agent_is_offered(self):
        self.assertTrue(self.given(True)["spawn_agent"])


class DelegateTest(unittest.TestCase):
    """delegate.py with the agent replaced by a script that sleeps."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-delegate-test-"))
        (self.tmp / "bin").mkdir()
        shutil.copy(HERE / "delegate.py", self.tmp / "bin" / "delegate")
        fake = self.tmp / "fake_agent.py"
        fake.write_text("import json, sys, time\ntime.sleep(float(sys.argv[1]))\n"
                        "print(json.dumps({'type': 'result', 'result': 'did ' + sys.argv[1]}))\n")
        (self.tmp / ".threads.json").write_text(json.dumps({
            "agent": "claude", "env": {"PATH": os.environ["PATH"]},
            "out": str(self.tmp / "threads"), "log": str(self.tmp / ".agents.jsonl"),
            "lock": str(self.tmp / ".threads.lock"), "timeout": 60,
            "argv": [sys.executable, str(fake), "{task}"]}))

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def delegate(self, task, env=None):
        return subprocess.Popen([sys.executable, str(self.tmp / "bin" / "delegate"), task],
                                cwd=self.tmp, env={**os.environ, **(env or {})},
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    def test_a_child_cannot_delegate(self):
        out, err = self.delegate("0.1", {"AF_EVAL_DELEGATED": "1"}).communicate()
        self.assertEqual(err.strip().count("\n"), 0)
        self.assertIn("a child starts no thread in this harness", err)
        self.assertFalse((self.tmp / ".agents.jsonl").exists())

    def test_a_second_delegate_waits_for_the_first_child(self):
        first, second = self.delegate("1.5"), None
        for _ in range(100):
            if (self.tmp / ".agents.jsonl").exists():
                break
            time.sleep(0.05)
        second = self.delegate("0.2")
        out1, _ = first.communicate()
        out2, _ = second.communicate()
        self.assertIn("did 1.5", out1)
        self.assertIn("waiting for it to finish first", out2)
        agents = run.delegated_threads(self.tmp)
        self.assertEqual(len(agents), 2)
        self.assertEqual(run.peak(agents), 1)
        self.assertEqual(run.peak(agents + [{"start": 0, "end": time.time()}]), 2)

    def test_a_child_left_running_is_killed_when_the_run_ends(self):
        child = self.delegate("30")
        for _ in range(100):
            if (self.tmp / ".agents.jsonl").exists():
                break
            time.sleep(0.05)
        entry = json.loads((self.tmp / ".agents.jsonl").read_text().splitlines()[0])
        [agent] = run.delegated_threads(self.tmp)
        child.communicate(timeout=10)
        with self.assertRaises(ProcessLookupError):
            os.killpg(entry["pgid"], 0)
        self.assertLess(agent["end"] - agent["start"], 10)


class CodexThreadsTest(unittest.TestCase):
    def test_a_subagent_record_becomes_a_transcript_the_graders_read(self):
        tmp = Path(tempfile.mkdtemp(prefix="af-codex-threads-"))
        try:
            day = tmp / "home" / "sessions" / "2026" / "11" / "11"
            day.mkdir(parents=True)
            (day / "rollout-a-root.jsonl").write_text(json.dumps(
                {"timestamp": "2026-11-11T05:00:00Z", "type": "session_meta",
                 "payload": {"id": "root"}}) + "\n")
            (day / "rollout-b-kid.jsonl").write_text("\n".join(json.dumps(e) for e in [
                {"timestamp": "2026-11-11T05:00:05Z", "type": "session_meta",
                 "payload": {"id": "kid", "parent_thread_id": "root"}},
                {"timestamp": "2026-11-11T05:00:07Z", "type": "event_msg", "payload": {
                    "type": "item_completed", "item": {
                        "type": "CommandExecution", "exit_code": 1,
                        "command": ["/bin/bash", "-lc", "atelier review slug-k3fa"],
                        "aggregated_output": "Nothing changed."}}},
                {"timestamp": "2026-11-11T05:00:09Z", "type": "event_msg", "payload": {
                    "type": "item_completed", "item": {
                        "type": "AgentMessage", "content": [{"type": "Text", "text": "Done"}]}}},
            ]) + "\n")
            (tmp / "run").mkdir()
            [agent] = run.codex_threads(tmp / "home", tmp / "run")
            self.assertEqual((agent["thread"], agent["end"] - agent["start"]), ("codex-kid", 4))
            spec = importlib.util.spec_from_file_location(
                "loop", HERE.parent / "scenarios" / "next-loop" / "grade.py")
            loop = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(loop)
            found, tools = loop.calls(tmp / "run" / "threads" / "codex-kid.jsonl", "kid")
            self.assertEqual(found, [("kid", "/bin/bash -lc 'atelier review slug-k3fa'", 1,
                                      "Nothing changed.")])
            self.assertEqual([r["refused"] for r in loop.tally(found)], [1])
            self.assertTrue((tmp / "run" / "sessions" / "rollout-b-kid.jsonl").is_file())
        finally:
            shutil.rmtree(tmp, ignore_errors=True)


if __name__ == "__main__":
    unittest.main()
