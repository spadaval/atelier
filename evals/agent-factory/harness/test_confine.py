"""Tests of a confined run (confine.py), run where the confinement is: a host
with bubblewrap, as CI's evals job installs it. Each command runs in a
replay run's environment, under the sandbox run.py puts its agent in, and
starts no agent and calls no model.

It shows that reading another checkout of this repository, reading another
run's work directory or a case file, and fetching this repository from
GitHub fail; that reading the run's own repository works, so the failures
are the sandbox's; and that the proxy passes a connection to an allowed host,
here a local one, so the model's API stays reachable.

Run with `python3 -m unittest test_confine` from this directory.
"""
import json
import os
import shutil
import socket
import subprocess
import sys
import tempfile
import threading
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import confine  # noqa: E402
import run  # noqa: E402

REPO = Path(__file__).resolve().parents[3]
CASE = REPO / "evals/agent-factory/scenarios/approval-follow-up/case.toml"
UNAVAILABLE = confine.available()


@unittest.skipIf(UNAVAILABLE, f"no sandbox on this host: {UNAVAILABLE}")
class ConfinedRunTest(unittest.TestCase):
    def setUp(self):
        self.root = Path(tempfile.mkdtemp(prefix="confine-test-"))
        self.work = self.root / "run-a"
        (self.work / "repo").mkdir(parents=True)
        (self.work / "repo" / "own.txt").write_text("own\n")
        other = self.root / "run-b" / "repo"
        other.mkdir(parents=True)
        (other / "fix.txt").write_text("another run's fixture\n")
        self.other = other / "fix.txt"
        self.server = None

    def tearDown(self):
        if self.server:
            self.server.close()
        shutil.rmtree(self.root, ignore_errors=True)

    def run_in(self, command: str, agent="codex", hosts=None):
        """`command` in bash in a replay run's environment and sandbox."""
        env = run.agent_env(self.work, agent, confined=True)
        with confine.Sandbox(self.work, agent, hosts=hosts) as box:
            argv, inner = box.wrap(["bash", "-c", command], env)
            out = subprocess.run(argv, env=inner, capture_output=True, text=True, timeout=120)
        run.unlink_auth(self.work)
        return out

    def assertFails(self, command: str, **kw):
        out = self.run_in(command, **kw)
        self.assertNotEqual(out.returncode, 0, f"{command} worked: {out.stdout}{out.stderr}")
        return out

    def test_the_run_reads_its_own_repository(self):
        out = self.run_in("cat own.txt && git --version && python3 -c 'print(1)'")
        self.assertEqual(out.returncode, 0, out.stderr)
        self.assertTrue(out.stdout.startswith("own\n"), out.stdout)

    def test_reading_another_checkout_fails(self):
        self.assertTrue((REPO / "AGENTS.md").is_file())
        self.assertFails(f"cat {REPO / 'AGENTS.md'}")
        self.assertFails(f"git -C {REPO} log -1")
        self.assertFails(f"ls {REPO.parent}/*/AGENTS.md")
        self.assertFails(f"cat {self.other}")

    def test_reading_a_case_file_fails(self):
        self.assertTrue(CASE.is_file())
        self.assertFails(f"cat {CASE}")
        self.assertFails(f"grep -r defect {CASE.parents[1]}")

    def test_fetching_this_repository_fails(self):
        self.assertFails("git ls-remote https://github.com/spadaval/atelier.git")
        self.assertFails("git clone -q --depth 1 https://github.com/spadaval/atelier.git x")
        self.assertFails("git ls-remote git@github.com:spadaval/atelier.git",)
        self.assertFails("curl -sSf -m 20 https://raw.githubusercontent.com/spadaval/atelier/"
                         "master/README.md")
        self.assertFails("curl -sSf -m 20 --noproxy '*' https://github.com/")
        refused = confine.refused(self.work)
        self.assertIn("github.com:443", refused)
        self.assertIn("raw.githubusercontent.com:443", refused)

    def test_an_allowed_host_is_reached_through_the_proxy(self):
        self.server = socket.create_server(("127.0.0.1", 0))
        port = self.server.getsockname()[1]

        def answer():
            conn, _ = self.server.accept()
            conn.sendall(b"hello from outside\n")
            conn.close()

        threading.Thread(target=answer, daemon=True).start()
        probe = (
            "import os, socket\n"
            f"s = socket.create_connection(('127.0.0.1', {confine.PORT}))\n"
            f"s.sendall(b'CONNECT 127.0.0.1:{port} HTTP/1.1\\r\\n\\r\\n')\n"
            "data = b''\n"
            "while b'hello' not in data:\n"
            "    chunk = s.recv(4096)\n"
            "    if not chunk: break\n"
            "    data += chunk\n"
            "print(data.decode())\n")
        (self.work / "probe.py").write_text(probe)
        out = self.run_in(f"python3 {self.work / 'probe.py'}", hosts=(f"127.0.0.1:{port}",))
        self.assertEqual(out.returncode, 0, out.stderr)
        self.assertIn("200 Connection established", out.stdout)
        self.assertIn("hello from outside", out.stdout)
        # The same host is unreachable directly: the sandbox has no network.
        self.assertFails(f"python3 -c \"import socket; socket.create_connection(("
                         f"'127.0.0.1', {port}), timeout=5)\"", hosts=(f"127.0.0.1:{port}",))

    def test_claude_s_home_holds_only_its_sign_in(self):
        home = confine.CLAUDE_CREDENTIALS.parent
        if not home.is_dir():
            self.skipTest("no ~/.claude on this host")
        out = self.run_in(f"find {home} -mindepth 2 -type f | head -5; "
                          f"cat {home}/history.jsonl 2>/dev/null | head -c 100", agent="claude")
        self.assertEqual(out.stdout.strip(), "", out.stdout)


class ProgramBindsTest(unittest.TestCase):
    """The agent's program is bound by its own directory, never wider."""

    def test_each_layout_binds_only_the_program_s_own_directory(self):
        root = Path(tempfile.mkdtemp(prefix="binds-"))
        self.addCleanup(shutil.rmtree, root, ignore_errors=True)
        layouts = {
            "claude": (root / "share/claude/versions/2.1.325", root / "share/claude/versions"),
            "codex": (root / "codex/releases/0.169.0/bin/codex", root / "codex/releases/0.169.0"),
            "tool": (root / "local/bin/tool", root / "local/bin"),
        }
        for name, (program, bound) in layouts.items():
            program.parent.mkdir(parents=True, exist_ok=True)
            program.write_text("#!/bin/sh\n")
            program.chmod(0o755)
            link = root / "path" / name
            link.parent.mkdir(exist_ok=True)
            link.symlink_to(program)
            self.assertEqual(confine.program_binds([name], {"PATH": str(link.parent)}),
                             [bound], name)


class WebToolsTest(unittest.TestCase):
    """A confined run is offered no web search or fetch of its agent's own."""

    def test_claude_code_is_not_offered_them(self):
        cmd = run.agent_command("claude", "x", "haiku", "low", 1, Path("/r"), confined=True)
        disallowed = cmd[cmd.index("--disallowedTools") + 1].split(",")
        for tool in confine.WEB_TOOLS:
            self.assertIn(tool, disallowed)
        plain = run.agent_command("claude", "x", "haiku", "low", 1, Path("/r"))
        self.assertNotIn("WebSearch", plain[plain.index("--disallowedTools") + 1])

    def test_codex_has_web_search_disabled(self):
        self.assertIn('web_search = "disabled"', run.codex_config(False, confined=True))
        self.assertNotIn("web_search", run.codex_config(False))


class MatrixTest(unittest.TestCase):
    def test_a_replay_builds_rust_and_a_confined_one_needs_no_threads(self):
        cases = [p.name for p in run.SCENARIOS.iterdir()
                 if (p / "case.toml").is_file()]
        self.assertGreaterEqual(len(cases), 3)
        for name in cases:
            meta = json.loads((run.SCENARIOS / name / "scenario.json").read_text())
            self.assertTrue(meta.get("confine") and not meta.get("threads"), name)
            self.assertTrue(run.builds_rust(name), name)
        if UNAVAILABLE:
            self.assertIn("must run confined", run.check_confined(cases))
        else:
            self.assertIsNone(run.check_confined(cases))


if __name__ == "__main__":
    unittest.main()
