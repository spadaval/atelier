"""Each counted measure's arithmetic, and each tie rule, on a small T3 database, transcripts, frozen log and scratch Git repository.

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
import atelier_log  # noqa: E402
import commands  # noqa: E402
import counted  # noqa: E402
import draw_samples  # noqa: E402
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
    return git(repo, "rev-parse", "--short", "HEAD")


def merge(repo, branch, files, when):
    """Commit the files on a new branch and merge it into main with a merge commit."""
    git(repo, "checkout", "-q", "-b", branch)
    commit(repo, files, f"Work on {branch}", when)
    git(repo, "checkout", "-q", "main")
    git(repo, "merge", "-q", "--no-ff", "-m", f"Merge {branch}", branch, when=when)
    return git(repo, "rev-parse", "--short", "HEAD")


def item_record(item_id, status, mission="null"):
    return f"---\nschema: atelier.item\nid: {item_id}\nstatus: {status}\nmission: {mission}\n---\n\n## Goal\n"


def review(created, *findings):
    return (f"---\nschema: atelier.review\nround: 1\ncommit: abc123\nverdict: changes\ncreated_at: {created}\n---\n\n"
            "## Summary\n\nText.\n\n" + "".join(f"## Finding {n}: Title {n}\n{text}\n\n" for n, text in enumerate(findings, 1)))


def lines(n, word="line"):
    return "".join(f"{word} {i}\n" for i in range(n))


def build_repo(repo):
    """Commits by day: merge_a brings a.ts's 10 lines for demo-aaaa; merge_b brings only demo-bbbb's record; c4 deletes 4
    of a.ts's lines 3 days later; c6 deletes 2 more 21 days later. Returns the commits by name, short, as the log names them."""
    repo.mkdir(parents=True)
    git(repo, "init", "-q", "-b", "main")
    c = {"start": commit(repo, {
        ".atelier/missions/demo-cccc/mission.md": "---\nid: demo-cccc\nstatus: done\n---\n",
        ".atelier/items/demo-aaaa/item.md": item_record("demo-aaaa", "done", "demo-cccc"),
        ".atelier/items/demo-aaaa/reviews/1-ab12.md": review("2026-10-15T00:00:00.123456789Z", "Recommendation: fix now",
                                                             "Recommendation: defer"),
        ".atelier/items/demo-aaaa/reviews/2-ij90.md": review("2026-10-16T00:00:00Z", "Recommendation: fix now\nWas: 1-ab12.1",
                                                             "Recommendation: fix now\nText that is the reviewer's."),
        ".atelier/items/demo-bbbb/item.md": item_record("demo-bbbb", "done", "demo-cccc"),
        ".atelier/items/demo-bbbb/reviews/1-zz00.md": review("2026-10-01T00:00:00Z", "Recommendation: fix now"),  # before the window
        ".atelier/items/demo-dddd/item.md": item_record("demo-dddd", "ready"),
        ".atelier/items/demo-eeee/item.md": item_record("demo-eeee", "done"),
        ".atelier/items/demo-ffff/item.md": item_record("demo-ffff", "done"),
        "src/old.ts": lines(5), ".agents/vendored.js": lines(200, "vendored javascript")},
        "Start", "2026-10-01T00:00:00Z")}
    c["merge_a"] = merge(repo, "demo-aaaa", {"src/a.ts": lines(10)}, "2026-10-15T00:00:00Z")
    c["merge_b"] = merge(repo, "demo-bbbb", {".atelier/items/demo-bbbb/item.md": item_record("demo-bbbb", "done", "demo-cccc") + "x\n"},
                         "2026-10-16T00:00:00Z")
    commit(repo, {"src/a.ts": "".join(lines(10).splitlines(True)[4:])}, "Trim a.ts", "2026-10-18T00:00:00Z")
    c["c5"] = commit(repo, {"src/b.ts": lines(2), "src/b.test.ts": lines(6)}, "Add b.ts (demo-eeee, demo-ffff)", "2026-10-20T00:00:00Z")
    commit(repo, {"src/a.ts": "".join(lines(10).splitlines(True)[6:])}, "Trim a.ts again", "2026-11-05T00:00:00Z")
    c["manifest"] = commit(repo, {".atelier/items/demo-dddd/item.md": item_record("demo-dddd", "ready") + "y\n"}, "Record", "2026-11-06T00:00:00Z")
    return c


def build_log(c):
    """The frozen log: one line per act, in the layout of docs/design.md, The log."""
    return "".join(line + "\n" for line in (
        f"2026-10-13T00:00:00Z {c['merge_a']} demo-gggg merge t1 approved -> done",  # before the window
        f"2026-10-14T01:00:00Z {c['start']} demo-cccc ready - draft -> ready; by b",
        f"2026-10-14T02:00:00Z {c['start']} demo-cccc lead t4",  # a lead line ties no thread
        f"2026-10-15T00:00:00Z {c['start']} demo-aaaa start t1 ready -> building",
        f"2026-10-15T00:00:00Z {c['start']} demo-aaaa review t3/019a-child 1-ab12",  # a Codex subagent of t3
        f"2026-10-15T00:00:00Z {c['merge_a']} demo-aaaa merge t1 approved -> done",
        f"2026-10-16T00:00:00Z {c['start']} demo-bbbb review t5 1-ef56",
        f"2026-10-16T00:00:00Z {c['merge_b']} demo-bbbb merge t9 approved -> done",  # changes records only
        f"2026-10-19T00:00:00Z {c['start']} demo-ffff start t9 ready -> building",
        f"2026-10-19T01:00:00Z {c['start']} demo-ffff take-over t8 from t9",
        f"2026-10-19T02:00:00Z {c['start']} demo-eeee take-over t6 1-cd34; from t5",
        f"2026-10-20T00:00:00Z {c['c5']} demo-eeee merge human approved -> done",
        f"2026-10-20T00:00:00Z {c['c5']} demo-ffff merge t8 approved -> done",
        f"2026-10-21T00:00:00Z {c['start']} demo-dddd start t9 ready -> building",  # t9's second item
        f"2026-10-22T00:00:00Z {c['start']} demo-cccc close - ready -> done",
        f"{LATE} {c['start']} demo-dddd take-over t4 from t9",  # after the cutoff
        f"{LATE} {c['c5']} demo-dddd merge t4 approved -> done",
    ))


def item(thread, ordinal, kind, when, **payload):
    return (thread, f"{thread}-{ordinal}", ordinal, kind, "completed", json.dumps({"startedAt": when, **payload}))


def build_db(path):
    db = sqlite3.connect(path)
    db.executescript("""
        CREATE TABLE projection_projects (project_id TEXT, workspace_root TEXT);
        CREATE TABLE orchestration_v2_projection_threads (thread_id TEXT, project_id TEXT, created_at TEXT, payload_json TEXT);
        CREATE TABLE orchestration_v2_projection_turn_items (thread_id TEXT, turn_item_id TEXT, ordinal INTEGER, type TEXT, status TEXT, payload_json TEXT);
        CREATE TABLE orchestration_v2_projection_provider_threads (thread_id TEXT, payload_json TEXT);
        CREATE TABLE orchestration_v2_projection_messages (thread_id TEXT, message_id TEXT, role TEXT, created_at TEXT, payload_json TEXT);
    """)
    db.execute("INSERT INTO projection_projects VALUES ('p1', '/x/projects/demo'), ('p2', '/x/projects/other')")
    t0 = "2026-11-01T00:00:00Z"
    for tid, project, parent in (("t0", "p2", None), ("t1", "p1", "t0"), ("t2", "p1", "t1"), ("t3", "p1", None),
                                 ("t4", "p1", None), ("t5", "p1", None), ("t6", "p1", None), ("t7", "p2", None),
                                 ("t8", "p1", None), ("t9", "p1", None)):
        db.execute("INSERT INTO orchestration_v2_projection_threads VALUES (?, ?, ?, ?)",
                   (tid, project, t0, json.dumps({"lineage": {"parentThreadId": parent}})))
    db.execute("INSERT INTO orchestration_v2_projection_provider_threads VALUES ('t1', ?), ('t3', ?), ('t6', ?)",
               (json.dumps({"nativeThreadRef": {"driver": "claudeAgent", "nativeId": "s1"}}),
                json.dumps({"nativeThreadRef": {"driver": "codex", "nativeId": "01a1bd55-0000-0000-0000-000000000c01"}}),
                json.dumps({"nativeThreadRef": {"driver": "claudeAgent", "nativeId": "s6"}})))
    db.executemany("INSERT INTO orchestration_v2_projection_messages VALUES (?, ?, 'user', ?, ?)", [
        ("t0", "m1", "2026-10-17T00:00:00.000Z", json.dumps({"createdBy": "user"})),  # t0 started t1, so it is in the mission
        ("t1", "m2", "2026-10-17T00:00:00.000Z", json.dumps({"createdBy": "agent"})),
        ("t4", "m3", "2026-10-14T03:00:00.000Z", json.dumps({"createdBy": "user"})),
        ("t4", "m4", "2026-10-23T00:00:00.000Z", json.dumps({"createdBy": "user"})),  # after the mission closed
        ("t6", "m5", "2026-10-17T00:00:00.000Z", json.dumps({"createdBy": "user"})),  # demo-eeee is in no mission
    ])
    rows = [
        item("t1", 1, "command_execution", t0, input="atelier start demo-aaaa", output="Branch: demo-aaaa",
             nativeItemRef={"nativeId": "u1"}),
        item("t1", 2, "command_execution", t0, input="cat .atelier/items/demo-aaaa/item.md", output=""),
        item("t1", 3, "dynamic_tool", t0, toolName="Read"),
        item("t1", 4, "file_change", t0, fileName="/w/demo/.atelier/items/demo-aaaa/item.md"),
        item("t1", 5, "file_change", t0, fileName="/w/demo/src/a.ts"),
        item("t1", 6, "command_execution", t0, input="atelier merge --help", output="Usage"),
        item("t1", 7, "command_execution", t0, input="cargo test", output="error: test run failed"),
        item("t1", 8, "command_execution", t0, input="atelier land", output="land: refused — 1 check (x)"),
        item("t1", 9, "subagent", t0, childThreadId="t2", nativeItemRef={"driver": "claudeAgent", "nativeId": "agentx"}),
        item("t1", 10, "command_execution", LATE, input="atelier land", output="Error: late"),
        item("t2", 1, "command_execution", t0, input="git status", output=""),
        item("t3", 1, "command_execution", t0, input="/bin/bash -lc 'atelier review list demo-aaaa'", output=""),
        item("t4", 1, "command_execution", t0, input="atelier status", output="ok"),
        item("t5", 1, "command_execution", t0, input="ls", output=""),
        item("t6", 1, "command_execution", t0, input="ls", output=""),
        item("t7", 1, "command_execution", t0, input="atelier check", output="Error: elsewhere"),
        item("t8", 1, "command_execution", t0, input="ls", output=""),  # a shell-only implementer
        item("t8", 2, "dynamic_tool", t0, toolName="Read"),
        item("t8", 3, "command_execution", t0, input="git status", output=""),
        item("t8", 4, "command_execution", t0, output="",  # prints, writes nothing (R6, R8, R10)
             input="echo \"open('src/x.ts', 'w')\"; echo '>' src/y.ts; python3 -c \"print('open(\\\"src/z.ts\\\", \\\"w\\\")')\""),
        item("t8", 5, "command_execution", t0, input="python3 -c \"p = 'src/f.ts'; open(p, 'w').write('x')\"", output=""),
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
        [{"type": "tool_use", "id": "u3", "name": "Bash", "input": {"command": "git status"}}]))
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
        cls.commits = build_repo(cls.repo)
        cls.commit = cls.commits["manifest"]
        (cls.snap / atelier_log.LOGS).mkdir()
        (cls.snap / atelier_log.LOGS / "demo.txt").write_text(build_log(cls.commits))
        build_db(cls.snap / threads.DB_PATH)
        build_transcripts(cls.snap)
        cls.window = snapshot.Window(snapshot.WINDOW_START, CUTOFF)
        cls.manifest = root / "manifest.json"
        cls.manifest.write_text(json.dumps({"cutoff": CUTOFF, "repositories": {"demo": {"commit": cls.commit}}}))
        cls.lines = atelier_log.read(cls.snap, "demo", cls.window.before_cutoff)
        db = threads.open_db(cls.snap)
        cls.result = counted.measure(db, counted.transcripts.index(cls.snap), cls.lines, cls.repo, "demo", cls.commit, cls.window)
        db.close()

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()


class Measures(Fixture):
    def test_items_and_ties(self):
        # demo-bbbb's merge changes only its record; demo-gggg's is before the window and demo-dddd's after the cutoff;
        # demo-cccc is a mission
        self.assertEqual(sorted(atelier_log.merged(self.lines, self.repo, self.window)), ["demo-aaaa", "demo-eeee", "demo-ffff"])
        self.assertEqual(self.result["items"], {"done_in_window": 4, "merged": 3, "merged_with_threads": 3})
        self.assertEqual(self.result["threads"], {"in_repository": 8, "tied_to_merged_item": 6, "untied": 1,
                                                  "tied_to_other_item": 1, "tied_by": {"log": 5, "parent": 1}})

    def test_lines_after_the_cutoff_are_dropped(self):
        self.assertEqual(len(self.lines), 15)
        self.assertEqual(self.lines[3], atelier_log.Line("2026-10-15T00:00:00Z", self.commits["start"], "demo-aaaa", "start",
                                                         "t1", "ready -> building"))
        self.assertEqual(self.lines[2].changed, "")

    def test_each_tie_rule(self):
        ties = atelier_log.ties(self.lines)
        db = threads.open_db(self.snap)
        found = {t.id: counted.thread_stats(t, ties, [], self.window) for t in threads.threads(db, "demo", self.window.before_cutoff)}
        db.close()
        self.assertEqual({tid: (s.item, s.role, s.tied_by) for tid, s in found.items()}, {
            "t1": ("demo-aaaa", "implementer", "log"),  # start
            "t2": ("demo-aaaa", "implementer", "parent"),  # Claude's Agent tool started it in t1, so it acts as t1
            "t3": ("demo-aaaa", "reviewer", "log"),  # review, by its Codex subagent t3/019a-child
            "t4": (None, None, None),  # named on a lead line, and on a take-over after the cutoff
            "t5": ("demo-bbbb", "reviewer", "log"),  # review
            "t6": ("demo-eeee", "reviewer", "log"),  # a reviewer's take-over
            "t8": ("demo-ffff", "implementer", "log"),  # an implementer's take-over
            "t9": ("demo-ffff", "implementer", "log"),  # start; its later start of demo-dddd leaves it on its first item
        })

    def test_process_share(self):
        share = self.result["process_share"]
        self.assertEqual((share["shell_commands"], share["process_commands"], share["share"]), (14, 5, 0.3571))
        # (opus 24.0 * 1 of 2 calls + sonnet 15.3 * 0 of 1 + sol 4.4 * 1 of 2 calls) / 43.7
        self.assertEqual(share["token_cost_share"], 0.3249)
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
        self.assertEqual((rework["eligible_commits"], rework["written_until"]), (3, "2026-10-27T05:14:58Z"))  # merge_b changed no code

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


class Samples(Fixture):
    def setUp(self):
        for patch in (mock.patch.object(draw_samples, "REPOS", self.repo.parent),
                      mock.patch.object(draw_samples, "PROJECTS", {"p1": "demo"})):
            patch.start()
            self.addCleanup(patch.stop)
        self.manifest_data = json.loads(self.manifest.read_text())
        self.cutoff = draw_samples.utc(CUTOFF)

    def missions(self):
        return draw_samples.closed_missions(self.manifest_data, self.snap, self.cutoff)

    def test_closed_missions(self):
        # its items name it at the manifest commit; its threads are on its lines and its items' lines
        self.assertEqual(self.missions(), {"demo-cccc": dict(
            repository="demo", items=["demo-aaaa", "demo-bbbb"], start="2026-10-14T01:00:00.000Z",
            close="2026-10-22T00:00:00.000Z", threads={"t1", "t3", "t4", "t5", "t9"})})

    def test_interventions(self):
        db = sqlite3.connect(f"{(self.snap / threads.DB_PATH).as_uri()}?mode=ro&immutable=1", uri=True)
        entries, stats = draw_samples.draw_interventions(db, self.missions())
        db.close()
        # t0 started t1; t1's message is the agent's; t4's second message is after the close; t6 is in no mission
        self.assertEqual(sorted((e["thread"], e["message"]) for e in entries), [("t0", "m1"), ("t4", "m3")])
        self.assertEqual(stats, dict(threads_in_projects=8, threads_tied_directly=5, threads_tied_through_lineage=1,
                                     threads_tied_to_several=0, agent_written_messages_left_out=1))

    def test_findings(self):
        # left out: a deferred finding, one carried from an earlier round, and one raised before the window
        findings, sizes = draw_samples.draw_findings(self.manifest_data, self.cutoff)
        self.assertEqual(sizes, {"demo": 2})
        self.assertEqual(sorted(f["finding"] for f in findings), ["demo-aaaa/1-ab12.1", "demo-aaaa/2-ij90.2"])

    def test_a_rerun_after_the_freeze_gives_the_same_output(self):
        """The base moves on, an item moves to another mission and branches are deleted: nothing read changes."""
        def run(out):
            counted.main(["--snapshot", str(self.snap), "--projects", str(self.repo.parent), "--manifest",
                          str(self.manifest), "--out", str(out)])
            return ([(out / name).read_bytes() for name in ("counted.json", "counted.md")],
                    self.missions(), draw_samples.draw_findings(self.manifest_data, self.cutoff))

        before = run(Path(self.tmp.name) / "before")
        commit(self.repo, {"src/later.ts": lines(3), ".atelier/items/demo-aaaa/item.md": item_record("demo-aaaa", "done", "demo-zzzz"),
                           ".atelier/items/demo-dddd/reviews/1-yy00.md": review("2026-10-20T00:00:00Z", "Recommendation: fix now")},
               "Later", "2026-11-12T00:00:00Z")
        git(self.repo, "branch", "-D", "demo-aaaa", "demo-bbbb")
        self.assertEqual(run(Path(self.tmp.name) / "after"), before)


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
            log = atelier_log.read(self.snap, "demo", self.window.before_cutoff)
            counted.measure(db, counted.transcripts.index(self.snap), log, self.repo, "demo", self.commit, self.window)
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
    """One plain case per recognised form, and the plain forms that are not calls or edits."""

    def test_atelier_calls(self):
        for command, calls in (("cd x && atelier -q issue show a | head", [["-q", "issue", "show", "a"]]),
                               ("/bin/bash -lc 'cd x && atelier check'", [["check"]]),
                               ("FOO=1 nice -n 19 ionice -c3 timeout 60s atelier land", [["land"]]),
                               ("for c in a b; do atelier $c --help; done", [["$c", "--help"]]),
                               ("if atelier check; then atelier land; fi", [["check"], ["land"]]),
                               ("atelier frobnicate", [["frobnicate"]]),
                               ("cat <<'EOF' > notes.md\natelier status\nEOF\natelier work ready", [["work", "ready"]])):
            self.assertEqual(commands.atelier_calls(command), calls, command)
        for not_a_call in ("target/debug/atelier check", "grep atelier src", "cargo install --path crates/atelier-cli",
                           'echo "atelier status"', "printf '%s\\n' ';' atelier status", "command -v atelier"):
            self.assertEqual(commands.atelier_calls(not_a_call), [], not_a_call)
            self.assertFalse(commands.is_process(not_a_call), not_a_call)
        self.assertTrue(commands.is_process("cat .atelier/issues/a.md"))

    def test_shell_writes(self):
        self.assertEqual(commands.shell_writes("cargo test 2>&1 > out/r.md; echo '>' x.md"), ["out/r.md"])
        self.assertEqual(commands.shell_writes("sed -i 's/a/b/' src/lib.rs && perl -pi -e s/x/y/ a.rs"), ["src/lib.rs", "a.rs"])
        self.assertEqual(commands.shell_writes("python3 -c \"open('src/e.rs', 'w')\""), ["src/e.rs"])
        self.assertEqual(commands.shell_writes("python3 - <<'EOF'\nq = Path('src/c.ts')\nwith q.open('w') as f: pass\n"
                                               "Path('docs/a.md').write_text(x)\nprint(\"open('src/no.rs', 'w')\")\nEOF"),
                         ["src/c.ts", "docs/a.md"])
        self.assertEqual(commands.shell_writes("python3 -c \"p = 'src/a.rs'; p = open(p, 'w')\""), ["src/a.rs"])  # R13
        self.assertEqual(commands.shell_writes("bash -lc 'python3 - <<EOF\nopen(\"d.rs\", \"w\")\nEOF'"), ["d.rs"])
        self.assertEqual(commands.shell_writes("echo \"open('src/lib.rs', 'w')\""), [])
        self.assertEqual(commands.shell_writes("cat > notes.md <<'EOF'\nopen('src/lib.rs', 'w')\nEOF"), ["notes.md"])
        # a relative write is taken from the cd before it, with .. reduced (R14)
        self.assertEqual(commands.shell_writes("cd /tmp/x && echo a > src.rs; cd ../../w/repo && echo b > src/a.rs"),
                         ["/tmp/x/src.rs", "/w/repo/src/a.rs"])
        self.assertTrue(commands.writes_product("cd /w/repo && sed -i s/a/b/ src/lib.rs"))
        for not_a_product_write in ("cargo test > /dev/null", "echo x > .atelier/issues/a.md", "cargo test > out.txt",
                                    "echo x > ~/scratch/a.py", "cd /tmp/x && echo a > src.rs"):
            self.assertFalse(commands.writes_product(not_a_product_write), not_a_product_write)

    def test_help_and_refusal(self):
        self.assertTrue(commands.is_help("atelier review finding --help"))
        self.assertTrue(commands.is_help("atelier help"))
        self.assertFalse(commands.is_help("atelier status && grep --help"))
        self.assertTrue(commands.refused("merge: refused — 2 checks (a, b)"))
        self.assertTrue(commands.refused("x\nerror: unrecognized subcommand 'y'"))
        self.assertFalse(commands.refused("Error: Exit code 1\nok"))


if __name__ == "__main__":
    unittest.main()
