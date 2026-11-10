"""Each counted measure's arithmetic, on a small T3 database, transcripts and a scratch Git repository.

Run: python3 -m unittest discover scripts/scorecard  (with TMPDIR outside any Git repository)
"""
import builtins
import json
import os
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).parent))
import commands  # noqa: E402
import counted  # noqa: E402
import gitreader  # noqa: E402
import snapshot  # noqa: E402
import threads  # noqa: E402

CUTOFF = "2026-11-10T05:14:58Z"
LATE = "2026-11-10T05:15:30Z"  # after the cutoff, so dropped


def git(repo, *args, when="2026-10-01T00:00:00Z"):
    env = {**os.environ, "GIT_AUTHOR_DATE": when, "GIT_COMMITTER_DATE": when, "GIT_AUTHOR_NAME": "b",
           "GIT_AUTHOR_EMAIL": "b@b", "GIT_COMMITTER_NAME": "b", "GIT_COMMITTER_EMAIL": "b@b"}
    return subprocess.run(["git", "-C", str(repo), "-c", "commit.gpgsign=false", *args], env=env, check=True,
                          capture_output=True, text=True).stdout.strip()


def commit(repo, files, message, when):
    for path, text in files.items():
        (repo / path).parent.mkdir(parents=True, exist_ok=True)
        (repo / path).write_text(text)
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", message, when=when)
    return git(repo, "rev-parse", "HEAD")


def issue(item_id, status, kind="task", closed=None):
    closed_line = f'closed_at: "{closed}"\n' if closed else ""
    return f'---\nid: "{item_id}"\nissue_type: "{kind}"\nstatus: "{status}"\n{closed_line}---\n\n## Goal\n'


def lines(n, word="line"):
    return "".join(f"{word} {i}\n" for i in range(n))


def build_repo(repo):
    """c2 writes 10 lines; c4 deletes 4 of them 3 days later; c6 deletes 2 more 21 days later."""
    repo.mkdir(parents=True)
    git(repo, "init", "-q", "-b", "main")
    commit(repo, {".atelier/issues/demo-aaaa.md": issue("demo-aaaa", "done", closed="2026-10-20T00:00:00Z"),
                  ".atelier/issues/demo-bbbb.md": issue("demo-bbbb", "done", closed="2026-10-22T00:00:00Z"),
                  ".atelier/issues/demo-cccc.md": issue("demo-cccc", "done", "mission", "2026-10-22T00:00:00Z"),
                  ".atelier/issues/demo-dddd.md": issue("demo-dddd", "todo"),
                  ".atelier/issues/demo-eeee.md": issue("demo-eeee", "done", closed="2026-10-21T00:00:00Z"),
                  ".atelier/issues/demo-ffff.md": issue("demo-ffff", "done", closed="2026-10-21T00:00:00Z"),
                  "src/old.ts": lines(5), ".agents/vendored.js": lines(200, "vendored javascript")},
           "Start", "2026-10-01T00:00:00Z")
    commit(repo, {"src/a.ts": lines(10)}, "Add a.ts for demo-aaaa (mission demo-cccc)", "2026-10-15T00:00:00Z")
    commit(repo, {".atelier/issues/demo-aaaa.md": issue("demo-aaaa", "done", closed="2026-10-20T00:00:00Z") + "x\n"},
           "Record demo-bbbb", "2026-10-16T00:00:00Z")
    commit(repo, {"src/a.ts": "".join(lines(10).splitlines(True)[4:])}, "Trim a.ts", "2026-10-18T00:00:00Z")
    commit(repo, {"src/b.ts": lines(2), "src/b.test.ts": lines(6)}, "Add b.ts (demo-eeee, demo-ffff)", "2026-10-20T00:00:00Z")
    commit(repo, {"src/a.ts": "".join(lines(10).splitlines(True)[6:])}, "Trim a.ts again", "2026-11-05T00:00:00Z")
    return commit(repo, {".atelier/issues/demo-dddd.md": issue("demo-dddd", "todo") + "y\n"}, "Record", "2026-11-06T00:00:00Z")


def item(thread, ordinal, kind, when, **payload):
    return (thread, f"{thread}-{ordinal}", ordinal, kind, "completed", json.dumps({"startedAt": when, **payload}))


def build_db(path, record_commit):
    db = sqlite3.connect(path)
    db.executescript("""
        CREATE TABLE projection_projects (project_id TEXT, workspace_root TEXT);
        CREATE TABLE orchestration_v2_projection_threads (thread_id TEXT, project_id TEXT, title TEXT, created_at TEXT, payload_json TEXT);
        CREATE TABLE orchestration_v2_projection_turn_items (thread_id TEXT, turn_item_id TEXT, ordinal INTEGER, type TEXT, status TEXT, payload_json TEXT);
        CREATE TABLE orchestration_v2_projection_provider_threads (thread_id TEXT, payload_json TEXT);
    """)
    db.execute("INSERT INTO projection_projects VALUES ('p1', '/x/projects/demo'), ('p2', '/x/projects/other')")
    t0 = "2026-11-01T00:00:00Z"
    for tid, title, branch, project in (("t1", "Implement demo-aaaa", "main", "p1"), ("t2", "Look around", "main", "p1"),
                                        ("t3", "Codex review, see demo-bbbb", "task/demo-aaaa", "p1"), ("t4", "Status", "main", "p1"),
                                        ("t5", "Work on demo-bbbb", "main", "p1"), ("t6", "Build demo-eeee", "main", "p1"),
                                        ("t7", "demo-aaaa elsewhere", "main", "p2"), ("t8", "Build demo-ffff", "main", "p1"),
                                        ("t9", "Implement", "main", "p1")):
        db.execute("INSERT INTO orchestration_v2_projection_threads VALUES (?, ?, ?, ?, ?)",
                   (tid, project, title, t0, json.dumps({"branch": branch})))
    db.execute("INSERT INTO orchestration_v2_projection_provider_threads VALUES ('t1', ?), ('t3', ?), ('t6', ?)",
               (json.dumps({"nativeThreadRef": {"driver": "claudeAgent", "nativeId": "s1"}}),
                json.dumps({"nativeThreadRef": {"driver": "codex", "nativeId": "01a1bd55-0000-0000-0000-000000000c01"}}),
                json.dumps({"nativeThreadRef": {"driver": "claudeAgent", "nativeId": "s6"}})))
    rows = [
        item("t1", 0, "user_message", t0, text="Act as the implementation sub-agent for demo-aaaa"),
        item("t1", 1, "command_execution", t0, input="atelier start demo-aaaa", output="Branch: demo-aaaa",
             nativeItemRef={"nativeId": "u1"}),
        item("t1", 2, "command_execution", t0, input="cat .atelier/issues/demo-aaaa.md", output=""),
        item("t1", 3, "dynamic_tool", t0, toolName="Read"),
        item("t1", 4, "file_change", t0, fileName="/w/demo/.atelier/issues/demo-aaaa.md"),
        item("t1", 5, "file_change", t0, fileName="/w/demo/src/a.ts"),
        item("t1", 6, "command_execution", t0, input="atelier merge --help", output="Usage"),
        item("t1", 7, "command_execution", t0, input="cargo test", output="error: test run failed"),
        item("t1", 8, "command_execution", t0, input="atelier land", output="land: refused — 1 check (x)"),
        item("t1", 9, "subagent", t0, childThreadId="t2", nativeItemRef={"driver": "claudeAgent", "nativeId": "agentx"}),
        item("t1", 10, "command_execution", LATE, input="atelier land", output="Error: late"),
        item("t2", 0, "user_message", t0, text="Review demo-aaaa"),
        item("t2", 1, "command_execution", t0, input="git commit -m Record", output=f"[main {record_commit[:7]}] Record",
             nativeItemRef={"nativeId": "u3"}),
        item("t3", 1, "command_execution", t0, input="/bin/bash -lc 'atelier review list demo-aaaa'", output=""),
        item("t4", 1, "command_execution", t0, input="atelier status", output="ok"),
        item("t5", 1, "command_execution", t0, input="ls", output=""),
        item("t6", 1, "command_execution", t0, input="ls", output=""),
        item("t7", 1, "command_execution", t0, input="atelier check", output="Error: elsewhere"),
        item("t8", 1, "command_execution", t0, input="ls", output=""),  # a shell-only implementer
        item("t8", 2, "dynamic_tool", t0, toolName="Read"),
        item("t8", 3, "command_execution", t0, input="git commit -m Record", output=f"[main {record_commit[:7]}] Record"),
        item("t8", 4, "command_execution", t0, input="echo \"open('src/x.ts', 'w')\"", output=""),  # prints, writes nothing
        item("t8", 5, "command_execution", t0, input="python3 -c \"p = 'src/f.ts'; open(p, 'w').write('x')\"", output=""),
        item("t9", 0, "user_message", t0, text="Act as the implementation sub-agent for demo-ffff"),
        item("t9", 1, "command_execution", t0, output="",  # writes first
             input="python3 - <<'EOF'\nfrom pathlib import Path\np = Path('src/b.ts')\np.write_text('x')\nEOF\ncd /tmp/check"),
        item("t9", 2, "command_execution", t0, input="cargo test", output=""),
        item("t9", 3, "file_change", t0, fileName="/w/demo/src/c.ts"),
    ]
    db.executemany("INSERT INTO orchestration_v2_projection_turn_items VALUES (?, ?, ?, ?, ?, ?)", rows)
    db.commit()
    db.close()


def claude_line(message_id, model, usage, blocks, when="2026-11-01T00:00:01Z"):
    return json.dumps({"type": "assistant", "timestamp": when, "message": {"id": message_id, "model": model, "usage": usage, "content": blocks}}) + "\n"


def build_transcripts(snap):
    session = snap / "claude-projects/-x-projects-demo"
    (session / "s1/subagents").mkdir(parents=True)
    first = {"input_tokens": 1_000_000, "cache_read_input_tokens": 10_000_000, "cache_creation_input_tokens": 2_000_000, "output_tokens": 10}
    final = {**first, "output_tokens": 100_000}  # a response is written once per block; the last copy is final
    (session / "s1.jsonl").write_text(
        claude_line("m1", "claude-opus-5-5", first, [{"type": "tool_use", "id": "u1", "name": "Bash", "input": {"command": "atelier start demo-aaaa"}}])
        + claude_line("m1", "claude-opus-5-5", final, [{"type": "tool_use", "id": "u2", "name": "Read", "input": {}}])
        + claude_line("m2", "claude-opus-5-5", final, [], when=LATE)
        + claude_line("m3", "<synthetic>", final, []))
    (session / "s6.jsonl").write_text(claude_line("m5", "claude-fable-5-2", final, []))  # no list price
    (session / "s1/subagents/agent-agentx.jsonl").write_text(claude_line(
        "m4", "claude-sonnet-5-5", {"input_tokens": 100_000, "output_tokens": 1_000_000},
        [{"type": "tool_use", "id": "u3", "name": "Bash", "input": {"command": "git commit -m Record"}}]))
    codex = snap / "codex-sessions/2026/11/08"
    codex.mkdir(parents=True)

    def count(total, when="2026-11-08T00:00:02Z"):
        return {"type": "event_msg", "timestamp": when, "payload": {"type": "token_count", "info": {"total_token_usage": total}}}

    total = {"input_tokens": 5_000_000, "cached_input_tokens": 4_000_000, "output_tokens": 200_000}
    records = [{"type": "turn_context", "payload": {"model": "gpt-6.1-sol"}},
               {"type": "response_item", "payload": {"type": "custom_tool_call", "name": "exec",
                "input": 'await tools.exec_command({cmd:"atelier review list"}); await tools.view_image({path:"x"})'}},
               count(total), count(total), count({**total, "output_tokens": 900_000}, LATE)]
    (codex / "rollout-2026-11-08T00-00-00-01a1bd55-0000-0000-0000-000000000c01.jsonl").write_text(
        "".join(json.dumps(r) + "\n" for r in records))


class Fixture(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        root = Path(cls.tmp.name)
        cls.snap = root / "snapshot"
        (cls.snap / "t3").mkdir(parents=True)
        cls.repo = root / "projects/demo"
        cls.commit = build_repo(cls.repo)
        build_db(cls.snap / threads.DB_PATH, git(cls.repo, "log", "--format=%H", "--grep", "Record demo-bbbb"))
        build_transcripts(cls.snap)
        cls.window = snapshot.Window(snapshot.WINDOW_START, CUTOFF)
        cls.manifest = root / "manifest.json"
        cls.manifest.write_text(json.dumps({"cutoff": CUTOFF, "repositories": {"demo": {"commit": cls.commit}}}))
        db = threads.open_db(cls.snap)
        cls.result = counted.measure(db, counted.transcripts.index(cls.snap), cls.repo, "demo", cls.commit, cls.window)
        db.close()

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()


class Measures(Fixture):
    def test_items_and_ties(self):
        self.assertEqual(self.result["items"], {"done_in_window": 4, "merged": 3, "merged_with_threads": 3})
        self.assertEqual(self.result["threads"], {"in_repository": 8, "tied_to_merged_item": 6, "untied": 1,
                                                  "tied_to_other_item": 1,
                                                  "tied_by": {"branch": 1, "title": 3, "first_message": 2}})

    def test_process_share(self):
        share = self.result["process_share"]
        self.assertEqual((share["shell_commands"], share["process_commands"], share["share"]), (14, 7, 0.5))  # t2's and t8's commits of records count
        # (opus 24.0 * 1 of 2 calls + sonnet 15.3 * 1 of 1, judged by T3 with its output + sol 4.4 * 1 of 2 calls) / 43.7
        self.assertEqual(share["token_cost_share"], 0.6751)
        self.assertEqual(share["token_coverage"], {"threads_with_transcript": 4, "threads_counted": 6})

    def test_confusion(self):
        self.assertEqual(self.result["confusion"], {"atelier_calls": 5, "refused_or_error": 1, "help_lookups": 1,
                                                    "confused_calls": 2, "per_100_calls": 40.0})

    def test_tokens_per_merged_item(self):
        tokens = self.result["tokens_per_merged_item"]
        # demo-eeee's thread used a model with no list price; demo-ffff's threads have no transcript
        self.assertEqual(tokens["per_item_usd"], {"demo-aaaa": 43.7})
        self.assertEqual(tokens["threads_with_unpriced_model"], 1)
        self.assertEqual((tokens["items_fully_covered"], tokens["median_usd"], tokens["mean_usd"]), (1, 43.7, 43.7))

    def test_calls_before_first_edit(self):
        # t1: start, cat, Read and the record edit come first (4, 2 shell); t8 writes by shell after 4 calls (3 shell);
        # t9 writes through a Path variable first (0)
        self.assertEqual(self.result["calls_before_first_edit"],
                         {"implementer_threads": 3, "median_tool_calls": 4.0, "median_shell_commands": 2.0})

    def test_rework(self):
        rework = self.result["rework"]
        self.assertEqual((rework["lines_written"], rework["deleted_within_days"], rework["share"]), (18, 4, 0.2222))
        self.assertEqual((rework["eligible_commits"], rework["written_until"]), (3, "2026-10-27T05:14:58Z"))  # c3 changed no code

    def test_record_only_commits(self):
        self.assertEqual(self.result["record_only_commits"], {"commits": 6, "record_only": 2, "share": 0.3333})

    def test_line_count(self):
        count = self.result["line_count"]
        self.assertEqual((count["language"], count["non_test_lines"]), ("TypeScript", 11))

    def test_rust_line_count_uses_the_repository_script(self):
        repo = Path(self.tmp.name) / "rust"
        repo.mkdir()
        git(repo, "init", "-q", "-b", "main")
        head = commit(repo, {"src/lib.rs": "fn a() {}\n#[cfg(test)]\nmod t {\n}\n", "crates/x/tests/t.rs": lines(9)}, "Rust", "2026-10-15T00:00:00Z")
        self.assertEqual(gitreader.line_count(repo, head)["non_test_lines"], 1)


class Output(Fixture):
    def test_two_runs_write_identical_files_without_home_paths(self):
        outs = [Path(self.tmp.name) / f"out{n}" for n in (1, 2)]
        for out in outs:
            counted.main(["--snapshot", str(self.snap), "--projects", str(self.repo.parent), "--manifest",
                          str(self.manifest), "--out", str(out)])
        for name in ("counted.json", "counted.md"):
            first, second = ((out / name).read_bytes() for out in outs)
            self.assertEqual(first, second, name)
            self.assertNotIn(str(Path.home()).encode(), first)
            self.assertNotIn(self.tmp.name.encode(), first)
        self.assertEqual(json.loads((outs[0] / "counted.json").read_text())["repositories"]["demo"], self.result)


class Inputs(Fixture):
    def test_unknown_schema_names_the_expected_one(self):
        bad = Path(self.tmp.name) / "bad"
        (bad / "t3").mkdir(parents=True)
        db = sqlite3.connect(bad / threads.DB_PATH)
        db.execute("CREATE TABLE orchestration_v2_projection_threads (thread_id TEXT)")
        db.close()
        with self.assertRaises(SystemExit) as exit_:
            counted.main(["--snapshot", str(bad), "--manifest", str(self.manifest), "--out", str(bad / "out")])
        self.assertIn(threads.SCHEMA, str(exit_.exception.code))
        self.assertIn("orchestration_v2_projection_turn_items.payload_json", str(exit_.exception.code))

    def test_live_stores_are_refused_and_never_opened(self):
        with self.assertRaises(SystemExit), mock.patch("sys.stderr"):
            counted.main([])  # the snapshot has no default
        for live in snapshot.LIVE_STORES:
            with self.assertRaises(snapshot.InputError):
                snapshot.check_not_live(live / "sub")
        opened = []
        real_open, real_connect = builtins.open, sqlite3.connect
        with mock.patch("builtins.open", lambda f, *a, **k: opened.append(str(f)) or real_open(f, *a, **k)), \
             mock.patch("sqlite3.connect", lambda f, *a, **k: opened.append(str(f)) or real_connect(f, *a, **k)):
            db = threads.open_db(self.snap)
            counted.measure(db, counted.transcripts.index(self.snap), self.repo, "demo", self.commit, self.window)
            db.close()
        self.assertTrue(opened)
        for path in opened:
            path = path.removeprefix("file://").split("?")[0]
            self.assertTrue(Path(path).is_relative_to(self.snap), path)
            self.assertFalse(any(Path(path).is_relative_to(live) for live in snapshot.LIVE_STORES), path)

    def test_database_opened_read_only(self):
        db = threads.open_db(self.snap)
        with self.assertRaises(sqlite3.OperationalError):
            db.execute("CREATE TABLE x (y)")
        db.close()
        self.assertEqual(sorted(p.name for p in (self.snap / "t3").iterdir()), ["statev2.sqlite"])


class Commands(unittest.TestCase):
    def test_atelier_calls(self):
        for command, calls in (("cd x && atelier -q issue show a | head", [["-q", "issue", "show", "a"]]),
                               ("/bin/bash -lc 'cd x && atelier check'", [["check"]]),
                               ("FOO=1 nice -n 19 ionice -c3 timeout 60s atelier land", [["land"]]),
                               ("cargo test && atelier land; git status", [["land"]]),
                               ("atelier frobnicate", [["frobnicate"]]),
                               ('for c in a b; do atelier $c --help; done', [["$c", "--help"]]),
                               ('id="$(atelier issue create -q x)"; if atelier check; then atelier land; fi',
                                [["issue", "create", "-q", "x"], ["check"], ["land"]]),
                               ("cat <<'EOF' > notes.md\natelier status\nEOF\natelier work ready", [["work", "ready"]])):
            self.assertEqual(commands.atelier_calls(command), calls, command)
        for not_a_call in ("target/debug/atelier check", "grep atelier src", "cargo install --path crates/atelier-cli",
                           'echo "atelier status"', "grep -n 'atelier merge --help' f", 'echo "a; atelier status"',
                           "A=$(command -v atelier)"):
            self.assertFalse(commands.atelier_calls(not_a_call), not_a_call)
        self.assertFalse(commands.is_help("grep -n 'atelier merge --help' f"))
        # R7: only a substitution outside single quotes runs
        for literal in ("echo '$(atelier status)'", "echo \"\\$(atelier status)\"", "grep '$(atelier merge --help)' f"):
            self.assertEqual(commands.atelier_calls(literal), [], literal)
            self.assertFalse(commands.is_process(literal), literal)
        self.assertEqual(commands.atelier_calls('echo "id: $(atelier issue create -q x)" $(atelier status)'),
                         [["issue", "create", "-q", "x"], ["status"]])

    def test_shell_writes(self):
        self.assertEqual(commands.shell_writes("sed -i 's/a/b/' src/lib.rs && perl -pi -e s/x/y/ a.rs"), ["src/lib.rs", "a.rs"])
        self.assertEqual(commands.shell_writes("cargo test 2>&1 | tee /tmp/x.log > out/r.md"), ["out/r.md", "/tmp/x.log"])
        self.assertEqual(commands.shell_writes("python3 - <<'EOF'\nopen('src/a.py', 'w')\nEOF"), ["src/a.py"])
        self.assertEqual(commands.shell_writes("python3 - <<'EOF'\nq = Path('src/c.ts')\nwith q.open('w') as f: pass\n"
                                               "Path('docs/a.md').write_text(x)\nopen(other, 'w')\nEOF"), ["src/c.ts", "docs/a.md"])
        self.assertEqual(commands.shell_writes("apply_patch <<'EOF'\n*** Update File: src/b.rs\nEOF"), ["src/b.rs"])
        self.assertFalse(commands.writes_product("cargo test > /dev/null 2>&1; echo x > .atelier/issues/a.md"))
        for not_a_product_write in ("cargo test > out.txt", 'cp a "$f"', "cp a.rs target/debug/b.rs 2>/dev/null",
                                    "echo x > ~/scratch/a.py", "tee /home/u/tmp-review/a.rs < x"):
            self.assertFalse(commands.writes_product(not_a_product_write), not_a_product_write)
        self.assertTrue(commands.writes_product("cp /tmp/new.rs crates/x/src/lib.rs"))
        self.assertEqual(commands.shell_writes("cd /tmp/x && echo a > src.rs; cd /w/repo && echo b > src/a.rs"),
                         ["/tmp/x/src.rs", "/w/repo/src/a.rs"])
        self.assertFalse(commands.writes_product("cd ~/.cache/t && python3 -c \"open('src/a.rs', 'w')\""))
        # R5: a write is resolved in the directory it ran in, not one a later cd moves to
        self.assertEqual(commands.shell_writes("python3 -c \"open('src/lib.rs', 'w')\"; cd /tmp/check"), ["src/lib.rs"])
        self.assertEqual(commands.shell_writes("cd /tmp/x && python3 - <<'EOF'\nopen('a.rs', 'w')\nEOF\ncd /w/demo"), ["/tmp/x/a.rs"])
        self.assertEqual(commands.shell_writes("apply_patch <<'EOF'\n*** Add File: b.rs\nEOF\ncd /tmp"), ["b.rs"])
        self.assertEqual(commands.shell_writes("x=$(python3 - <<'EOF'\nopen('c.rs', 'w')\nEOF\n)"), ["c.rs"])
        self.assertEqual(commands.shell_writes("bash -lc 'python3 - <<EOF\nopen(\"d.rs\", \"w\")\nEOF'"), ["d.rs"])
        # R6: Python is read only where a Python interpreter runs it
        for not_run in ("echo \"open('src/lib.rs', 'w')\"", "grep -n \"write_text(\" src/a.py",
                        "cat > notes.md <<'EOF'\nopen('src/lib.rs', 'w')\nEOF"):
            self.assertNotIn("src/lib.rs", commands.shell_writes(not_run), not_run)

    def test_help_and_refusal(self):
        self.assertTrue(commands.is_help("atelier review finding --help"))
        self.assertTrue(commands.is_help("atelier help"))
        self.assertFalse(commands.is_help("atelier status && grep --help"))
        self.assertTrue(commands.refused("merge: refused — 2 checks (a, b)"))
        self.assertTrue(commands.refused("x\nerror: unrecognized subcommand 'y'"))
        self.assertFalse(commands.refused("Error: Exit code 1\nok"))


if __name__ == "__main__":
    unittest.main()
