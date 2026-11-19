"""Tests for the harness: the isolation canary's verdict, with Claude Code
replaced by a fixed reply; setups; staging; validity; tokens; the log of
`atelier` calls; the limit of two agents, from the command lines and config
it builds and `delegate` with a stand-in agent; and a run in a setup that
mixes Claude Code and Codex, with stand-ins for both. Run with `python3 -m
unittest` from this directory."""
import contextlib
import importlib.util
import io
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
table = run.table


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
            run.check_isolation(self.tmp / "root", self.tmp)
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


class CodexCanaryTest(CanaryTest):
    """Under Codex the canary reads what the session record gave the model."""

    def canary(self, reply, extra_dirs=()):
        def fake(cmd, cwd, env, transcript, timeout):
            agents_md = (Path(cwd) / "AGENTS.md").read_text()
            blocks = [(Path(cwd), agents_md)] + [(Path(d), "# Elsewhere\n") for d in extra_dirs]
            day = Path(env["CODEX_HOME"]) / "sessions" / "2026" / "11" / "17"
            day.mkdir(parents=True)
            (day / "rollout-root.jsonl").write_text("\n".join(json.dumps(e) for e in [
                {"type": "session_meta", "payload": {"id": "root"}}] + [
                {"type": "response_item", "payload": {"type": "message", "role": "user",
                 "content": [{"type": "input_text", "text":
                              f"# AGENTS.md instructions for {d}\n\n<INSTRUCTIONS>\n{text}"
                              "\n</INSTRUCTIONS>"}]}} for d, text in blocks]))
            transcript.write_text("\n".join(json.dumps(e) for e in [
                {"type": "thread.started", "thread_id": "root"},
                {"type": "item.completed", "item": {"type": "agent_message",
                                                    "text": reply(Path(cwd), "")}},
                {"type": "turn.completed"}]) + "\n")
            return 0
        run.run_claude = fake
        try:
            run.check_isolation(self.tmp / "root", self.tmp, "codex")
        except SystemExit as exc:
            return str(exc.code)
        return None

    def test_a_reply_that_drops_the_heading_passes_on_the_record(self):
        # gpt-6-luna's reply of 2026-11-17, with the file in its input.
        self.assertIsNone(self.canary(lambda repo, _: f"{repo}/AGENTS.md | NONE"))

    def test_another_instruction_file_in_the_record_fails(self):
        why = self.canary(lambda repo, _: f"{repo}/AGENTS.md | NONE", extra_dirs=["/srv"])
        self.assertIn("/srv/AGENTS.md", why)

    def test_a_foreign_path_in_the_reply_still_fails(self):
        why = self.canary(lambda repo, _: f"{repo}/AGENTS.md | x\n/etc/AGENTS.md | y")
        self.assertIn("/etc/AGENTS.md", why)

    # CanaryTest's own cases fake a Claude Code transcript.
    test_reports_its_own_agents_md = None
    test_a_path_in_the_repository_is_not_its_contents = None
    test_a_bold_foreign_path_is_foreign = None
    test_a_run_offered_an_agent_tool_fails_the_canary = None
    test_a_home_relative_path_is_foreign = None


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

    def test_the_skill_is_staged_for_claude_code_in_the_workspace(self):
        run.stage_for("claude", "HEAD", self.tmp, "skills/agent-factory")
        skill = self.tmp / "repo" / ".claude" / "skills" / "agent-factory"
        self.assertIn("name: agent-factory", (skill / "SKILL.md").read_text())
        self.assertTrue((skill / "roles" / "reviewer.md").is_file())
        self.assertIn(".claude/", (self.tmp / "repo/.git/info/exclude").read_text())

    def test_the_skill_is_staged_for_codex_in_its_scratch_home(self):
        run.stage_for("codex", "HEAD", self.tmp, "skills/agent-factory")
        skill = self.tmp / "home" / ".codex" / "skills" / "agent-factory"
        self.assertTrue((skill / "roles" / "implementer.md").is_file())
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
        self.assertIn("spawn_agent was offered", run.invalid_reason("none", 0, self.info(**facts)))
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
        env, note = run.threads(self.tmp, self.tmp / "run", run.agent_env(self.tmp),
                                run.one_model_setup("claude", "haiku", "low"), {}, "thread-1")
        roles = json.loads((self.tmp / ".threads.json").read_text())["roles"]
        for role in run.ROLES:
            self.assertEqual(self.disallowed(roles[role]["argv"]), list(run.AGENT_TOOLS))
        self.assertEqual(env["T3CODE_THREAD_ID"], "thread-1")
        self.assertTrue((self.tmp / "bin" / "delegate").is_file())
        self.assertIn("delegate", note)

    def test_codex_has_no_delegate_and_spawn_agent_only_with_threads(self):
        env, note = run.threads(self.tmp, self.tmp / "run", run.agent_env(self.tmp, "codex", True),
                                run.one_model_setup("codex", "gpt-6-luna", "low"), {},
                                "thread-1")
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
        self.assertIsNone(run.check_jobs(2, ["vacuous-test-review",
                                             "costly-hardening"]))
        self.assertIn("--jobs 1", run.check_jobs(2, ["vacuous-test-review", "loop"]))
        self.assertIsNone(run.check_jobs(1, ["loop"]))
        refused = subprocess.run([sys.executable, str(HERE / "run.py"), "loop",
                                  "--jobs", "2", "--out", str(self.tmp / "out")],
                                 capture_output=True, text=True)
        self.assertEqual(refused.returncode, 2)
        self.assertIn("--jobs 1", refused.stderr)
        self.assertFalse((self.tmp / "out").exists())

    def test_only_a_pass_needing_no_hand_read_loses_its_work_directory(self):
        self.assertTrue(run.discard_work({"pass": True, "work_dir": "w"}, keep=False))
        self.assertFalse(run.discard_work({"pass": True, "work_dir": "w"}, keep=True))
        self.assertFalse(run.discard_work({"pass": True, "needs_hand_grade": True,
                                           "work_dir": "w"}, keep=False))
        self.assertFalse(run.discard_work({"pass": None, "work_dir": "w"}, keep=False))
        self.assertFalse(run.discard_work({"pass": False, "work_dir": "w"}, keep=False))

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
        run.stage_for("codex", "HEAD", self.tmp, "skills/agent-factory")
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
        role = {"env": {"PATH": os.environ["PATH"]},
                "argv": [sys.executable, str(fake), "{task}"]}
        (self.tmp / ".threads.json").write_text(json.dumps({
            "roles": {"implementer": role, "reviewer": role},
            "out": str(self.tmp / "threads"), "log": str(self.tmp / ".agents.jsonl"),
            "lock": str(self.tmp / ".threads.lock"), "timeout": 60}))

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def delegate(self, task, env=None, role="implementer"):
        return subprocess.Popen([sys.executable, str(self.tmp / "bin" / "delegate"), role, task],
                                cwd=self.tmp, env={**os.environ, **(env or {})},
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

    def test_a_role_the_setup_lacks_is_refused(self):
        _, err = self.delegate("0.1", role="lead").communicate()
        self.assertIn("'lead' is not a role here", err)
        self.assertIn("implementer, reviewer", err)
        self.assertFalse((self.tmp / ".agents.jsonl").exists())

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
        second = self.delegate("0.2", role="reviewer")
        out1, _ = first.communicate()
        out2, _ = second.communicate()
        self.assertIn("did 1.5", out1)
        self.assertIn("waiting for it to finish first", out2)
        agents = run.delegated_threads(self.tmp)
        self.assertEqual([(a["role"], a["exit"]) for a in agents],
                         [("implementer", 0), ("reviewer", 0)])
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
    def turn(self, kind, second):
        return {"timestamp": f"2026-11-17T23:{second // 60:02d}:{second % 60:02d}Z",
                "type": "event_msg", "payload": {"type": kind}}

    def test_a_subagent_idle_between_turns_is_not_alive_then(self):
        # Run ed72 of 2026-11-17: the implementer's first turn ended at 88 s,
        # the reviewer ran 91 to 128 s, and a follow-up gave the implementer
        # a second turn at 131 s. Its record spans 13 to 142 s.
        implementer = [{"type": "session_meta", "timestamp": "2026-11-17T23:00:13Z"},
                       self.turn("task_started", 13), self.turn("task_complete", 88),
                       self.turn("task_started", 131), self.turn("task_complete", 142)]
        reviewer = [self.turn("task_started", 91), self.turn("task_complete", 128)]
        start = run.epoch("2026-11-17T23:00:00Z")
        turns = [{"start": a, "end": b} for a, b in run.codex_turns(implementer, start + 150)
                 + run.codex_turns(reviewer, start + 150)]
        self.assertEqual([(t["start"] - start, t["end"] - start) for t in turns],
                         [(13, 88), (131, 142), (91, 128)])
        self.assertEqual(run.peak([{"start": start, "end": start + 150}] + turns), 2)

    def test_a_turn_with_no_end_runs_until_the_first_agent_ended(self):
        start = run.epoch("2026-11-17T23:00:00Z")
        self.assertEqual(run.codex_turns([self.turn("task_started", 10)], start + 60),
                         [(start + 10, start + 60)])

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
                {"timestamp": "2026-11-11T05:00:05Z", "type": "event_msg",
                 "payload": {"type": "task_started"}},
                {"timestamp": "2026-11-11T05:00:07Z", "type": "event_msg", "payload": {
                    "type": "item_completed", "item": {
                        "type": "CommandExecution", "exit_code": 1,
                        "command": ["/bin/bash", "-lc", "atelier review slug-k3fa"],
                        "aggregated_output": "Nothing changed."}}},
                {"timestamp": "2026-11-11T05:00:09Z", "type": "event_msg", "payload": {
                    "type": "item_completed", "item": {
                        "type": "AgentMessage", "content": [{"type": "Text", "text": "Done"}]}}},
                {"timestamp": "2026-11-11T05:00:09Z", "type": "event_msg",
                 "payload": {"type": "task_complete"}},
            ]) + "\n")
            (tmp / "run").mkdir()
            [agent] = run.codex_threads(tmp / "home", tmp / "run", 0)
            self.assertEqual((agent["thread"], agent["end"] - agent["start"]), ("codex-kid", 4))
            written = [json.loads(line) for line in
                       (tmp / "run" / "threads" / "codex-kid.jsonl").read_text().splitlines()]
            self.assertIn({"type": "item.completed", "item": {
                "type": "command_execution", "command": "/bin/bash -lc 'atelier review slug-k3fa'",
                "exit_code": 1, "aggregated_output": "Nothing changed."}}, written)
            self.assertTrue((tmp / "run" / "sessions" / "rollout-b-kid.jsonl").is_file())
        finally:
            shutil.rmtree(tmp, ignore_errors=True)


class SetupTest(unittest.TestCase):
    """The checked-in setups, their judges, and a one-model setup."""

    def test_sol_is_one_codex_model_at_high_with_one_judge(self):
        sol = run.load_setup("sol")
        self.assertEqual({r: (s["agent"], s["model"], s["effort"])
                          for r, s in sol["roles"].items()},
                         {r: ("codex", "gpt-6.1-sol", "high") for r in run.ROLES})
        self.assertEqual(list(sol["judges"]), ["openai"])
        self.assertEqual(sol["judges"]["openai"]["agent"], "claude")
        self.assertTrue(run.spawns(sol))

    def test_house_mixes_labs_and_has_a_judge_for_each(self):
        house = run.load_setup("house")
        self.assertEqual({r: (s["agent"], s["model"], s["effort"])
                          for r, s in house["roles"].items()},
                         {"lead": ("claude", "claude-fable-5-2", "high"),
                          "implementer": ("claude", "claude-opus-5-5", "high"),
                          "reviewer": ("codex", "gpt-6.1-sol", "xhigh")})
        self.assertEqual({lab: run.LABS[j["agent"]] for lab, j in house["judges"].items()},
                         {"anthropic": "openai", "openai": "anthropic"})
        self.assertFalse(run.spawns(house))

    def test_a_judge_from_its_threads_lab_or_a_missing_one_is_refused(self):
        house = run.load_setup("house")
        same = {**house, "judges": {**house["judges"],
                                    "anthropic": {"agent": "claude", "model": "m", "effort": "e"}}}
        self.assertIn("must be from another lab", run.setup_problem(same))
        missing = {**house, "judges": {"openai": house["judges"]["openai"]}}
        self.assertIn("one judge for each", run.setup_problem(missing))
        no_effort = {**house, "roles": {**house["roles"], "lead": {"agent": "claude",
                                                                    "model": "m"}}}
        self.assertIn("an effort", run.setup_problem(no_effort))
        with self.assertRaises(ValueError):
            run.load_setup("nonesuch")

    def test_a_one_model_setup_puts_it_in_every_role_with_a_judge_from_another_lab(self):
        luna = run.one_model_setup("codex", "gpt-6-luna", "medium")
        self.assertEqual(luna["name"], "gpt-6-luna-medium")
        self.assertIsNone(run.setup_problem(luna))
        self.assertEqual({s["model"] for s in luna["roles"].values()}, {"gpt-6-luna"})
        self.assertTrue(run.spawns(luna))
        self.assertIsNone(run.setup_problem(run.one_model_setup("claude", "opus", "high")))
        self.assertFalse(run.spawns(run.one_model_setup("claude", "opus", "high")))

    def test_every_case_names_its_first_agents_role(self):
        for meta in sorted(run.SCENARIOS.glob("*/scenario.json")):
            self.assertIn(json.loads(meta.read_text()).get("role"), run.ROLES, meta)

    def test_codex_runs_at_its_roles_effort(self):
        argv = run.agent_command("codex", "x", "gpt-6.1-sol", "xhigh", 1, Path("/r"))
        self.assertIn('model_reasoning_effort="xhigh"', argv)


class MixedSetupTest(unittest.TestCase):
    """In a setup that mixes Claude Code and Codex every thread starts with
    `delegate <role>`, as its role's agent, and the limit of two holds."""

    def setUp(self):
        LimitTest.setUp(self)
        subprocess.run(["git", "init", "-q", str(self.tmp / "repo")], check=True)
        self.house = run.load_setup("house")

    tearDown = LimitTest.tearDown

    def threads(self, setup, stage=None):
        first = setup["roles"]["implementer"]
        env = run.agent_env(self.tmp, first["agent"], run.spawns(setup))
        return run.threads(self.tmp, self.tmp / "run", env, setup, {}, "thread-1", stage)

    def test_each_role_runs_its_own_agent_model_and_effort(self):
        _, note = self.threads(self.house)
        self.assertIn("delegate <role>", note)
        roles = json.loads((self.tmp / ".threads.json").read_text())["roles"]
        lead, implementer, reviewer = (roles[r]["argv"] for r in run.ROLES)
        self.assertEqual((lead[0], lead[lead.index("--model") + 1],
                          lead[lead.index("--effort") + 1]), ("claude", "claude-fable-5-2", "high"))
        self.assertEqual(implementer[implementer.index("--model") + 1], "claude-opus-5-5")
        self.assertEqual((reviewer[:2], reviewer[reviewer.index("-m") + 1]),
                         (["codex", "exec"], "gpt-6.1-sol"))
        self.assertIn('model_reasoning_effort="xhigh"', reviewer)
        home = Path(roles["reviewer"]["env"]["CODEX_HOME"])
        self.assertEqual(home, run.role_home(self.tmp, "reviewer"))
        self.assertIn("disable_native_tools = true", (home / "config.toml").read_text())
        self.assertTrue((home / "auth.json").is_symlink())
        run.unlink_auth(self.tmp)
        self.assertFalse((home / "auth.json").is_symlink())
        self.assertNotIn("CODEX_HOME", roles["lead"]["env"])

    def test_the_skill_is_staged_for_each_agent_of_the_setup(self):
        self.threads(self.house, lambda kind, home: run.stage_for(
            kind, "HEAD", self.tmp, "skills/agent-factory", home))
        self.assertTrue((self.tmp / "repo/.claude/skills/agent-factory/SKILL.md").is_file())
        self.assertTrue((run.role_home(self.tmp, "reviewer") /
                         "skills/agent-factory/SKILL.md").is_file())
        self.assertEqual((self.tmp / "repo/.git/info/exclude").read_text().count(".claude/"), 1)

    def test_a_codex_first_agent_in_a_mixed_setup_delegates_and_cannot_spawn(self):
        setup = {**self.house, "roles": {**self.house["roles"],
                                          "implementer": self.house["roles"]["reviewer"]}}
        env, note = self.threads(setup)
        self.assertTrue((self.tmp / "bin" / "delegate").is_file())
        self.assertIn("delegate <role>", note)
        config = (Path(env["CODEX_HOME"]) / "config.toml").read_text()
        self.assertIn("disable_native_tools = true", config)

    def test_a_codex_thread_at_another_effort_than_its_roles_is_invalid(self):
        spec = self.house["roles"]["reviewer"]
        thread = {"thread": "t", "role": "reviewer", "init": {
            "mcp_servers": [], "agent_factory": False, "model": "gpt-6.1-sol",
            "effort": "low"}}
        self.assertIn("ran gpt-6.1-sol at low", run.thread_problem("none", thread, spec))
        thread["init"]["effort"] = "xhigh"
        self.assertIsNone(run.thread_problem("none", thread, spec))
        thread["init"]["spawn_agent"] = True
        self.assertIn("spawn_agent", run.thread_problem("none", thread, spec))


class TokensTest(unittest.TestCase):
    def test_claude_and_codex_usage_count_alike(self):
        claude = {"input_tokens": 14, "cache_creation_input_tokens": 100,
                  "cache_read_input_tokens": 1000, "output_tokens": 50}
        codex = {"input_tokens": 1114, "cached_input_tokens": 1000, "output_tokens": 50}
        self.assertEqual(run.token_counts([claude]), run.token_counts([codex]))
        self.assertEqual(run.token_counts([codex, codex])["in"], 2228)
        self.assertEqual(run.token_counts([]), {"in": 0, "cached_in": 0, "out": 0})


class AtelierLogTest(unittest.TestCase):
    def test_every_call_is_logged_with_its_thread_and_exit(self):
        tmp = Path(tempfile.mkdtemp(prefix="af-atelier-log-"))
        try:
            (tmp / "bin").mkdir()
            (tmp / "bin" / "atelier").write_text(
                "#!/bin/sh\necho \"real $*\"\n[ \"$1\" = status ]\n")
            (tmp / "bin" / "atelier").chmod(0o755)
            run.log_atelier(tmp)
            env = {"PATH": f"{tmp / 'bin'}:{os.environ['PATH']}", "T3CODE_THREAD_ID": "t-1"}
            ok = subprocess.run(["atelier", "status"], env=env, capture_output=True, text=True)
            bad = subprocess.run(["atelier", "merge", "x"], env=env, capture_output=True, text=True)
            self.assertEqual((ok.returncode, ok.stdout), (0, "real status\n"))
            self.assertEqual(bad.returncode, 1)
            calls = run.atelier_calls(tmp)
            self.assertEqual([(c["argv"], c["thread"], c["exit"], c["by"]) for c in calls],
                             [(["status"], "t-1", 0, "t-1"), (["merge", "x"], "t-1", 1, "t-1")])
        finally:
            shutil.rmtree(tmp, ignore_errors=True)

    def test_a_codex_subagent_is_told_from_its_parent(self):
        # Review 1-jgy4: in sol a spawn_agent child inherits its parent's
        # T3CODE_THREAD_ID and session, and has a CODEX_THREAD_ID of its own.
        tmp = Path(tempfile.mkdtemp(prefix="af-atelier-log-"))
        try:
            (tmp / "bin").mkdir()
            (tmp / "bin" / "atelier").write_text("#!/bin/sh\nexit 1\n")
            (tmp / "bin" / "atelier").chmod(0o755)
            run.log_atelier(tmp)
            path = f"{tmp / 'bin'}:{os.environ['PATH']}"
            shared = {"PATH": path, "T3CODE_THREAD_ID": "thread-parent",
                      "CODEX_SESSION_ID": "root-session"}
            for thread in ("root-session", "reviewer-session"):
                subprocess.run(["atelier", "review", "x"],
                               env={**shared, "CODEX_THREAD_ID": thread})
            subprocess.run(["atelier", "status"], env={"PATH": path,
                                                       "CLAUDE_CODE_SESSION_ID": "s-1"})
            calls = run.atelier_calls(tmp, first="main")
            self.assertEqual([(c["thread"], c["by"]) for c in calls], [
                ("thread-parent", "thread-parent"),
                ("thread-parent/reviewer-session", "codex-reviewer-session"),
                ("s-1", "main")])
        finally:
            shutil.rmtree(tmp, ignore_errors=True)

    def test_the_identity_is_atelier_s(self):
        # The cases of crates/atelier-core/src/identity.rs's tests.
        spec = importlib.util.spec_from_file_location("atelier_log",
                                                      HERE / "atelier_log.py")
        log = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(log)
        everything = {"T3CODE_THREAD_ID": "t3", "CLAUDE_CODE_SESSION_ID": "claude",
                      "CODEX_SESSION_ID": "codex", "GROK_SESSION_ID": "grok"}
        self.assertEqual(log.identity(everything), "t3")
        self.assertEqual(log.identity({}), "human")
        self.assertEqual(log.identity({"T3CODE_THREAD_ID": "", "GROK_SESSION_ID": "g"}), "g")
        self.assertEqual(log.identity({"CODEX_SESSION_ID": " "}), "human")
        root = {"T3CODE_THREAD_ID": "t3", "CODEX_SESSION_ID": "s", "CODEX_THREAD_ID": "s"}
        self.assertEqual(log.identity(root), "t3")
        self.assertEqual(log.identity({**root, "CODEX_THREAD_ID": "kid"}), "t3/kid")


FAKE_AGENT = r'''
import json, os, subprocess, sys, uuid
from pathlib import Path
agent, model, effort, prompt = sys.argv[1:5]
ran_effort = Path(__file__).with_name("codex-effort").read_text() if agent == "codex" else effort
if agent == "claude":
    print(json.dumps({"type": "system", "subtype": "init", "model": model, "tools": ["Bash"],
                      "skills": [], "mcp_servers": []}), flush=True)
else:
    sid = str(uuid.uuid4())
    print(json.dumps({"type": "thread.started", "thread_id": sid}), flush=True)
    day = Path(os.environ["CODEX_HOME"]) / "sessions" / "2026" / "11" / "18"
    day.mkdir(parents=True, exist_ok=True)
    usage = {"input_tokens": 300, "cached_input_tokens": 200, "output_tokens": 30}
    (day / f"rollout-x-{sid}.jsonl").write_text("\n".join(json.dumps(e) for e in [
        {"type": "session_meta", "payload": {"id": sid}},
        {"type": "turn_context", "payload": {"model": model, "effort": ran_effort}},
        {"type": "event_msg", "payload": {"type": "token_count",
                                          "info": {"total_token_usage": usage}}}]) + "\n")
if prompt.startswith("FIRST"):
    subprocess.run(["atelier", "submit"], stdout=sys.stderr)
    subprocess.run(["delegate", "reviewer", "review it"], stdout=sys.stderr)
final = "ok"
if Path(__file__).with_name("final-link").exists():
    # A final reply that links a review file, as agents' replies do.
    final = f"Wrote [the review]({Path.cwd()}/.atelier/items/x-1/reviews/1-ab.md)."
if agent == "claude":
    print(json.dumps({"type": "result", "subtype": "success", "result": final,
                      "usage": {"input_tokens": 10, "cache_read_input_tokens": 90,
                                "output_tokens": 5}}))
else:
    print(json.dumps({"type": "item.completed", "item": {"type": "agent_message", "text": final}}))
    if Path(__file__).with_name("codex-fails").exists():
        # Usage recorded, then the turn fails: exec prints no usage for it.
        print(json.dumps({"type": "turn.failed", "error": {"message": "stopped"}}))
        sys.exit(1)
    print(json.dumps({"type": "turn.completed", "usage": {
        "input_tokens": 300, "cached_input_tokens": 200, "output_tokens": 30}}))
'''


class RunOneTest(unittest.TestCase):
    """run_one in house, end to end, with stand-ins for Claude Code and Codex:
    a Claude Code implementer starts a Codex reviewer with `delegate`."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-run-one-"))
        self.saved = (run.SCENARIOS, run.agent_command, run.CODEX_AUTH)
        run.CODEX_AUTH = self.tmp / "host-auth.json"
        run.CODEX_AUTH.write_text("{}")
        case = self.tmp / "scenarios" / "case"
        case.mkdir(parents=True)
        (case / "scenario.json").write_text(json.dumps(
            {"description": "x", "role": "implementer", "threads": True}))
        (case / "prompt.md").write_text("FIRST {{skill}}go")
        (case / "setup.sh").write_text(
            'set -e\nmkdir -p "$1/repo" "$1/bin"\ngit init -q "$1/repo"\n'
            'printf "#!/bin/sh\\nexit 1\\n" > "$1/bin/atelier"\nchmod +x "$1/bin/atelier"\n')
        (case / "grade.py").write_text('print(\'{"pass": true}\')')
        fake = self.tmp / "fake_agent.py"
        fake.write_text(FAKE_AGENT)
        (self.tmp / "codex-effort").write_text("xhigh")
        run.SCENARIOS = self.tmp / "scenarios"
        run.agent_command = lambda agent, prompt, model, effort, budget, cwd, system=None, confined=False: [
            sys.executable, str(fake), agent, model, effort, prompt]

    def tearDown(self):
        run.SCENARIOS, run.agent_command, run.CODEX_AUTH = self.saved
        shutil.rmtree(self.tmp, ignore_errors=True)

    def run_house(self):
        return run.run_one("case", "none", "HEAD", run.load_setup("house"), 0,
                           self.tmp / "out", self.tmp / "work")

    def test_each_thread_runs_as_its_role_and_reports_its_tokens(self):
        r = self.run_house()
        self.assertIs(r["pass"], True, r.get("invalid"))
        self.assertEqual((r["setup"], r["role"], r["model"]), ("house", "implementer",
                                                               "claude-opus-5-5"))
        first, reviewer = r["threads"]
        self.assertEqual((first["agent"], first["tokens"]), ("claude", {
            "in": 100, "cached_in": 90, "out": 5}))
        self.assertEqual((reviewer["role"], reviewer["agent"], reviewer["model"],
                          reviewer["effort"], reviewer["exit"]),
                         ("reviewer", "codex", "gpt-6.1-sol", "xhigh", 0))
        self.assertEqual(reviewer["tokens"], {"in": 300, "cached_in": 200, "out": 30})
        self.assertEqual(r["tokens"], {"in": 400, "cached_in": 290, "out": 35})
        self.assertEqual(r["peak_agents"], 2)
        [failed] = r["atelier_failed"]
        self.assertEqual((failed["argv"], failed["thread"], failed["by"], failed["exit"]),
                         (["submit"], first["thread"], first["thread"], 1))
        self.assertFalse((run.role_home(Path(r["work_dir"]), "reviewer") / "auth.json")
                         .is_symlink())
        self.assertIn("| house | case | none | 1 of 1 |", table.table([r]))

    def test_a_codex_thread_that_fails_keeps_the_tokens_its_record_counts(self):
        # Review 2's reproduction: the reviewer records usage, then its turn fails.
        (self.tmp / "codex-fails").write_text("")
        r = self.run_house()
        reviewer = r["threads"][1]
        self.assertEqual((reviewer["exit"], reviewer["tokens"]),
                         (1, {"in": 300, "cached_in": 200, "out": 30}))
        self.assertEqual(r["tokens"], {"in": 400, "cached_in": 290, "out": 35})

    def test_a_codex_first_agent_that_fails_keeps_its_tokens(self):
        (self.tmp / "codex-fails").write_text("")
        meta = self.tmp / "scenarios" / "case" / "scenario.json"
        meta.write_text(json.dumps({"description": "x", "role": "reviewer"}))
        r = self.run_house()
        self.assertEqual((r["agent"], r["exit"]), ("codex", 1))
        self.assertIsNone(r["pass"])
        self.assertEqual(r["tokens"], {"in": 300, "cached_in": 200, "out": 30})

    def test_a_thread_at_another_effort_than_its_roles_makes_the_run_invalid(self):
        (self.tmp / "codex-effort").write_text("low")
        r = self.run_house()
        self.assertIsNone(r["pass"])
        self.assertIn("(reviewer): it ran gpt-6.1-sol at low", r["invalid"])


FAKE_JUDGE = r"""
import json, re, sys
from pathlib import Path
agent, model = sys.argv[1:3]
prompt = sys.stdin.read()
here = Path(__file__).parent
(here / "judge-prompts").mkdir(exist_ok=True)
(here / "judge-prompts" / f"{model}.txt").write_text(prompt)
(here / "judge-cwds").mkdir(exist_ok=True)
(here / "judge-cwds" / model).write_text(str(Path.cwd()))
if (here / "judge-exits").exists():
    sys.exit(3)
asked = re.findall(r"^- (\S+) \(about the", prompt.split("# Your questions", 1)[1], re.M)
told = json.loads((here / "judge-answers.json").read_text())
# The line it quotes is the run's final reply, which the material holds.
final = prompt.split("# The final reply of the run's first agent\n\n", 1)[1].split("\n")[0]
answers = {"answers": [{"id": q, "answer": told.get(q, "pass"), "line": final,
                        "reason": "said so"} for q in asked]}
reply = "Here it is.\n" + json.dumps(answers)
if (here / "judge-reply").exists():
    reply = (here / "judge-reply").read_text()
if agent == "claude":
    print(json.dumps({"type": "system", "subtype": "init", "tools": [], "mcp_servers": []}))
    print(json.dumps({"type": "result", "subtype": "success", "result": reply,
                      "usage": {"input_tokens": 7, "output_tokens": 3}}))
else:
    print(json.dumps({"type": "thread.started", "thread_id": "j"}))
    print(json.dumps({"type": "item.completed", "item": {"type": "agent_message", "text": reply}}))
    print(json.dumps({"type": "turn.completed", "usage": {
        "input_tokens": 9, "cached_input_tokens": 0, "output_tokens": 4}}))
"""
RUBRIC = """# A case

The fixture, for the judge.

## found-it

Role: reviewer. Graded.

Does the review find it?

Pass: it does.

## built-it

Role: implementer. Graded.

Did the implementer build it?

Pass: it did.

## said-why

Role: reviewer. Reported.

Does the review say why?

Pass: it does.
"""
FACTS = """import json
print(json.dumps({"facts": {"merged": {"value": True, "pass": True},
                            "calls": {"value": 3}}}))
"""


class JudgeTest(unittest.TestCase):
    """A house run of a case with a rubric, end to end, with stand-ins for
    the agents and the judges: each question goes to the judge of its
    role's lab, and the run's grade is its facts and its graded answers."""
    judging = run.judging

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-judge-"))
        self.saved = (run.SCENARIOS, run.agent_command, run.judge_command, run.CODEX_AUTH)
        run.CODEX_AUTH = self.tmp / "host-auth.json"
        run.CODEX_AUTH.write_text("{}")
        case = self.tmp / "scenarios" / "case"
        case.mkdir(parents=True)
        (case / "scenario.json").write_text(json.dumps({"description": "x", "role": "reviewer"}))
        (case / "prompt.md").write_text("{{skill}}go")
        (case / "setup.sh").write_text(
            'set -e\nmkdir -p "$1/repo"\ngit init -q -b main "$1/repo"\ncd "$1/repo"\n'
            'git -c user.name=a -c user.email=a@a commit -q --allow-empty -m start\n')
        (case / "grade.py").write_text(FACTS)
        (case / "rubric.md").write_text(RUBRIC)
        (self.tmp / "judge-answers.json").write_text("{}")
        (self.tmp / "codex-effort").write_text("xhigh")
        agent, judge = self.tmp / "fake_agent.py", self.tmp / "fake_judge.py"
        agent.write_text(FAKE_AGENT)
        judge.write_text(FAKE_JUDGE)
        run.SCENARIOS = self.tmp / "scenarios"
        run.agent_command = lambda agent_, prompt, model, effort, budget, cwd, system=None, confined=False: [
            sys.executable, str(agent), agent_, model, effort, prompt]
        run.judge_command = lambda spec, cwd: [sys.executable, str(judge), spec["agent"],
                                               spec["model"]]

    def tearDown(self):
        run.SCENARIOS, run.agent_command, run.judge_command, run.CODEX_AUTH = self.saved
        shutil.rmtree(self.tmp, ignore_errors=True)

    def run_house(self, answers=None):
        (self.tmp / "judge-answers.json").write_text(json.dumps(answers or {}))
        return run.run_one("case", "none", "HEAD", run.load_setup("house"), 0,
                           self.tmp / "out", self.tmp / "work")

    def test_each_question_goes_to_the_judge_from_another_lab_than_its_role(self):
        r = self.run_house()
        self.assertIs(r["pass"], True, r.get("failed"))
        judges = {a["id"]: (a["role"], a["judge"]["lab"], a["judge"]["model"])
                  for a in r["rubric"]}
        # The reviewer is Codex's, so an Anthropic model judges it; the
        # implementer is Claude Code's, so an OpenAI one does.
        self.assertEqual(judges, {
            "found-it": ("reviewer", "openai", "claude-opus-5-5"),
            "built-it": ("implementer", "anthropic", "gpt-6.1-sol"),
            "said-why": ("reviewer", "openai", "claude-opus-5-5")})
        self.assertEqual({(j["model"], tuple(j["questions"])) for j in r["judged"]},
                         {("claude-opus-5-5", ("found-it", "said-why")),
                          ("gpt-6.1-sol", ("built-it",))})
        self.assertEqual(r["facts"]["merged"], {"value": True, "pass": True})
        found = next(a for a in r["rubric"] if a["id"] == "found-it")
        self.assertEqual((found["answer"], found["line"], found["line_found"]),
                         ("pass", "ok", True))
        self.assertTrue((Path(r["run_dir"]) / "judge-openai.jsonl").is_file())
        # Each judge ran after the agent, alone, and counts as an agent.
        self.assertEqual(r["peak_agents"], 1)
        self.assertEqual(len(r["agents"]), 3)

    def test_the_judge_is_not_told_the_arm(self):
        r = self.run_house()
        name = Path(r["run_dir"]).name
        self.assertIn("-none-", name)
        for model in ("claude-opus-5-5", "gpt-6.1-sol"):
            prompt = (self.tmp / "judge-prompts" / f"{model}.txt").read_text()
            cwd = Path((self.tmp / "judge-cwds" / model).read_text())
            self.assertNotIn(name, prompt)
            self.assertNotIn("arm", prompt.lower().split())
            self.assertTrue(cwd.name.startswith("judge-") and name not in str(cwd), cwd)
            self.assertFalse(cwd.exists(), "the judge's directory is removed")
        prompt = (self.tmp / "judge-prompts" / "claude-opus-5-5.txt").read_text()
        self.assertIn("- found-it (about the reviewer)", prompt)
        self.assertNotIn("- built-it", prompt)
        self.assertIn('"merged"', prompt)

    def test_a_final_reply_s_links_are_blinded_and_still_quotable(self):
        (self.tmp / "final-link").write_text("")
        r = self.run_house()
        name = Path(r["run_dir"]).name
        self.assertIn(name, (Path(r["run_dir"]) / "final.md").read_text())
        for model in ("claude-opus-5-5", "gpt-6.1-sol"):
            prompt = (self.tmp / "judge-prompts" / f"{model}.txt").read_text()
            self.assertNotIn(name, prompt)
            self.assertNotIn(str(self.tmp / "work"), prompt)
            self.assertIn("Wrote [the review](/work/repo/.atelier/items/x-1/reviews/1-ab.md).",
                          prompt)
        # The judge quoted the blinded line, which the material holds.
        self.assertIs(r["pass"], True, r.get("failed"))

    def test_a_reported_answer_that_fails_changes_no_grade(self):
        r = self.run_house({"said-why": "fail"})
        self.assertIs(r["pass"], True, r.get("failed"))
        self.assertEqual(next(a["answer"] for a in r["rubric"] if a["id"] == "said-why"),
                         "fail")

    def test_a_graded_answer_that_fails_fails_the_run(self):
        r = self.run_house({"built-it": "fail"})
        self.assertIs(r["pass"], False)
        self.assertEqual(r["failed"], ["graded built-it (implementer): fail, said so"])

    def test_a_judge_that_cannot_answer_fails_its_graded_questions_with_why(self):
        (self.tmp / "judge-exits").write_text("")
        r = self.run_house()
        self.assertIs(r["pass"], False)
        self.assertIn("graded found-it (reviewer): unknown, the judge (claude-opus-5-5) "
                      "exited 3 with no reply", r["failed"])
        self.assertNotIn("said-why", " ".join(r["failed"]))

    def test_a_malformed_reply_fails_the_graded_questions_and_keeps_the_result(self):
        for reply, why in (('{"answers": [', "was not the JSON asked for"),
                           ('{"answers": 7}', "held no list of answers"),
                           ('{"answers": [{"id": [], "answer": "pass", "line": "ok"}]}',
                            "held no answer to it"),
                           ('{"answers": [{"id": {}, "answer": "pass", "line": "ok"}]}',
                            "held no answer to it")):
            (self.tmp / "judge-reply").write_text(reply)
            r = self.run_house()
            self.assertIs(r["pass"], False)
            self.assertIn(f"graded found-it (reviewer): unknown, the judge's reply {why}",
                          r["failed"])
            self.assertTrue((Path(r["run_dir"]) / "result.json").is_file())

    def test_an_invalid_run_is_null_and_not_judged(self):
        (self.tmp / "codex-effort").write_text("low")
        r = self.run_house()
        self.assertIsNone(r["pass"])
        self.assertIn("it ran gpt-6.1-sol at low", r["invalid"])
        self.assertEqual(r["judged"], [])
        self.assertFalse((self.tmp / "judge-prompts").exists())

    def test_regrading_an_invalid_run_keeps_it_invalid_and_unjudged(self):
        (self.tmp / "codex-effort").write_text("low")
        r = self.run_house()
        out = self.judging.rejudge(run, Path(r["run_dir"]), None, self.tmp / "work")
        self.assertIsNone(out["pass"])
        self.assertEqual(out["invalid"], r["invalid"])
        self.assertEqual(out["judged"], [])
        self.assertFalse((self.tmp / "judge-prompts").exists())
        saved = json.loads((Path(r["run_dir"]) / "rejudge.json").read_text())
        self.assertEqual((saved["pass"], saved["invalid"]), (None, r["invalid"]))

    def test_regrading_a_valid_run_judges_it_again(self):
        r = self.run_house({"built-it": "fail"})
        (self.tmp / "judge-answers.json").write_text("{}")
        out = self.judging.rejudge(run, Path(r["run_dir"]), None, self.tmp / "work")
        self.assertEqual((r["pass"], out["previous_pass"], out["pass"]), (False, False, True))
        self.assertEqual(len(out["judged"]), 2)

    def test_regrading_a_run_saved_under_a_case_s_old_name_grades_it_as_renamed(self):
        r = self.run_house({"built-it": "fail"})
        result = Path(r["run_dir"]) / "result.json"
        result.write_text(json.dumps({**json.loads(result.read_text()), "scenario": "next-case"}))
        saved = dict(run.RENAMED)
        run.RENAMED.update({"next-case": "case", "next-gone": None})
        try:
            (self.tmp / "judge-answers.json").write_text("{}")
            out = self.judging.rejudge(run, Path(r["run_dir"]), None, self.tmp / "work")
            self.assertIs(out["pass"], True)
            result.write_text(json.dumps({**json.loads(result.read_text()),
                                          "scenario": "next-gone"}))
            out = self.judging.rejudge(run, Path(r["run_dir"]), None, self.tmp / "work")
            self.assertEqual(out["error"], "next-gone was deleted")
        finally:
            run.RENAMED.clear()
            run.RENAMED.update(saved)

    def test_every_renamed_case_is_a_case_today(self):
        for old, new in run.RENAMED.items():
            with self.subTest(old=old):
                self.assertTrue(new is None or (Path(run.__file__).resolve().parents[1]
                                                / "scenarios" / new / "grade.py").is_file())
        self.assertIsNone(run.case_dir("next-done-when-rewrite"))
        self.assertIsNone(run.case_dir("no-such-case"))

    def test_a_case_without_facts_keeps_its_own_grade(self):
        (self.tmp / "scenarios" / "case" / "grade.py").write_text(
            'print(\'{"pass": false, "checks": {}, "notes": ["old"]}\')')
        (self.tmp / "scenarios" / "case" / "rubric.md").unlink()
        r = self.run_house()
        self.assertIs(r["pass"], False)
        self.assertEqual(r["notes"], ["old"])


class JudgingTest(unittest.TestCase):
    """judge.py's parts: the rubric, the answers, the verdict and the
    material the judge reads."""
    judging = run.judging
    QUESTIONS = [{"id": "a", "role": "reviewer", "kind": "graded"},
                 {"id": "b", "role": "reviewer", "kind": "reported"}]

    def test_every_case_s_rubric_parses(self):
        for path in sorted(run.SCENARIOS.glob("*/rubric.md")):
            rubric = self.judging.read_rubric(path.parent)
            self.assertTrue(rubric["questions"], path)

    def test_a_question_without_its_role_or_pass_is_refused(self):
        with self.assertRaisesRegex(ValueError, "'x' needs a line 'Role"):
            self.judging.parse_rubric("# R\n\n## x\n\nRole: captain. Graded.\n\nPass: y\n")
        with self.assertRaisesRegex(ValueError, "'x' needs a line 'Pass"):
            self.judging.parse_rubric("# R\n\n## x\n\nRole: lead. Reported.\n\nIs it?\n")

    def test_answers_are_read_from_json_with_text_around_it(self):
        given = {"diff": "+ a line of the diff\n"}
        reply = ('Sure.\n```json\n{"answers": [{"id": "a", "answer": "PASS", "line": '
                 '"a line  of the diff", "reason": "r"}, {"id": "b", "answer": "maybe"}]}\n```')
        got = self.judging.parse_answers(reply, self.QUESTIONS, given)
        self.assertEqual(got["a"], {"answer": "pass", "line": "a line  of the diff",
                                    "reason": "r", "line_found": True})
        self.assertEqual(got["b"]["answer"], "unknown")
        self.assertIn("'maybe'", got["b"]["reason"])

    def test_an_answer_resting_on_no_line_of_the_material_is_unknown(self):
        given = {"diff": "actual evidence\n"}
        for answer, why in (({"id": "a", "answer": "pass"}, "without quoting the line"),
                            ({"id": "a", "answer": "pass", "line": "invented evidence"},
                             "quoting a line that is not in the material")):
            got = self.judging.parse_answers(json.dumps({"answers": [answer]}),
                                             self.QUESTIONS[:1], given)["a"]
            self.assertEqual(got["answer"], "unknown")
            self.assertIn(why, got["reason"])
            facts = {"ok": {"value": 1, "pass": True}}
            graded = [{**self.QUESTIONS[0], **got}]
            self.assertIs(self.judging.verdict(facts, graded)["pass"], False)
            # A reported answer resting on nothing changes no grade.
            reported = [{**self.QUESTIONS[1], **got}]
            self.assertIs(self.judging.verdict(facts, reported)["pass"], True)

    def test_a_reply_that_is_not_json_leaves_every_question_unknown(self):
        got = self.judging.parse_answers("I think it passes.", self.QUESTIONS, {})
        self.assertEqual({a["answer"] for a in got.values()}, {"unknown"})
        self.assertIn("not the JSON", got["a"]["reason"])

    def test_a_run_passes_on_its_facts_and_graded_answers(self):
        facts = {"ok": {"value": 1, "pass": True}, "shown": {"value": 2}}
        answers = [{**q, "answer": "pass" if q["kind"] == "graded" else "fail"}
                   for q in self.QUESTIONS]
        self.assertEqual(self.judging.verdict(facts, answers), {"pass": True, "failed": []})
        facts["ok"]["pass"] = False
        self.assertEqual(self.judging.verdict(facts, answers)["failed"], ["fact ok: 1"])

    def test_the_material_is_what_the_run_wrote_since_the_start(self):
        work = Path(tempfile.mkdtemp(prefix="af-material-"))
        try:
            repo = work / "repo"
            sh = lambda *a: subprocess.run(["git", "-C", str(repo), "-c", "user.name=a", "-c",
                                            "user.email=a@a", *a], check=True,
                                           capture_output=True)
            repo.mkdir()
            sh("init", "-q", "-b", "main")
            reviews = repo / ".atelier" / "items" / "x-1" / "reviews"
            reviews.mkdir(parents=True)
            (reviews / "1-old.md").write_text("an old review\n")
            (repo / "code.py").write_text("old = 1\n")
            sh("add", "-A")
            sh("commit", "-qm", "fixture")
            self.judging.snapshot_refs(work)
            sh("switch", "-qc", "x-1")
            (repo / "code.py").write_text("new = 2\n")
            (reviews / "1-new.md").write_text("## Finding 1: wrong\n")
            sh("add", "-A")
            sh("commit", "-qm", "Change it")
            (repo / "notes.txt").write_text("left here\n")
            (reviews / "2-draft.md").write_text("a draft review\n")
            given = self.judging.material(work, "Done.", {"f": {"value": 1}})
            self.assertIn("### Branch x-1 (new)", given["diff"])
            self.assertIn("+new = 2", given["diff"])
            self.assertIn("Change it", given["diff"])
            self.assertIn("+left here", given["diff"])
            self.assertNotIn("Finding 1", given["diff"])
            self.assertIn("1-new.md (at branch x-1)\n## Finding 1: wrong", given["reviews"])
            self.assertIn("2-draft.md (uncommitted, in the checkout on x-1)", given["reviews"])
            self.assertNotIn("an old review", given["reviews"])
            self.assertEqual(given["final"], "Done.")
        finally:
            shutil.rmtree(work, ignore_errors=True)


class MainTest(unittest.TestCase):
    """main runs a canary for each agent of its setup, and prints the table."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="af-main-"))
        # A work root must be outside $HOME, where TMPDIR may be.
        self.work_root = Path(tempfile.mkdtemp(prefix="af-main-", dir="/var/tmp"))
        self.saved = (run.check_isolation, run.run_one, sys.argv)
        self.canaries, self.setups = [], []
        run.check_isolation = lambda root, out, agent: self.canaries.append(agent)

        def fake_run(scenario, arm, ref, setup, index, out, root, skill_dir):
            self.setups.append(setup["name"])
            return {"scenario": scenario, "arm": arm, "setup": setup["name"], "index": index,
                    "pass": True, "seconds": 10, "tokens": {"in": 1000, "out": 10},
                    "run_dir": "", "agents": []}
        run.run_one = fake_run

    def tearDown(self):
        run.check_isolation, run.run_one, sys.argv = self.saved
        shutil.rmtree(self.tmp, ignore_errors=True)
        shutil.rmtree(self.work_root, ignore_errors=True)

    def main(self, *args):
        sys.argv = ["run.py", "vacuous-test-review", "--arms", "none",
                    "--out", str(self.tmp / "out"), "--work-root", str(self.work_root),
                    *args]
        out = io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
            run.main()
        return out.getvalue()

    def test_house_checks_both_agents_and_sol_one(self):
        printed = self.main("--setup", "house")
        self.assertEqual(self.canaries, ["claude", "codex"])
        self.assertIn("| house | vacuous-test-review | none | 1 of 1 | 0 | 10 | 1.0k | 10 |",
                      printed)
        self.canaries.clear()
        self.main("--setup", "sol")
        self.assertEqual(self.canaries, ["codex"])

    def test_one_model_on_the_command_line(self):
        self.main("--agent", "codex", "--model", "gpt-6-luna", "--effort", "low")
        self.assertEqual((self.canaries, self.setups), (["codex"], ["gpt-6-luna-low"]))

    def test_a_setup_with_a_model_is_refused(self):
        with self.assertRaises(SystemExit):
            self.main("--setup", "sol", "--model", "x")
        self.assertEqual(self.canaries, [])


class ProductMapTest(unittest.TestCase):
    """A replay fixture's AGENTS.md keeps its map and commands, whatever
    headings an era gave its process (replay.product_map)."""

    def test_process_goes_under_any_heading_and_the_map_stays(self):
        text = """# Guide

How we work here.

## Getting Going

- Load the agent-factory skill first.
- Run `atelier status`, then `atelier issue claim <id>`.
- Keep `.atelier/` committed.

## Where Things Are

- Intent: `SPEC.md`
- Notes: `docs/gone.md`
- Hooks: `.claude/settings.json`

## Handy

Use `cargo nextest run`, and say why in the evidence.

- Tests: `cargo test`, which CI runs on the pull request before the merge.

```bash
# the checks
cargo fmt -- --check
atelier lint
```
"""
        self.assertEqual(run.replay.product_map(text, lambda p: p in ("SPEC.md", ".claude")),
                         "# Guide\n\n## Where Things Are\n\n- Intent: `SPEC.md`\n\n"
                         "## Handy\n\n```bash\ncargo fmt -- --check\n```\n")


if __name__ == "__main__":
    unittest.main()
