"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with EVAL_ATELIER naming the atelier binary under test (target/debug/atelier).

Each case runs setup.sh, drives the binary as two threads to the state an
agent might leave, with every call logged as the harness logs it, and checks
the facts and the grade the harness makes of them (harness/judge.py). This
case has no rubric. Fixture serves every case's tests: a judged case's tests
give the judge's answers as recorded ones (Fixture.graded).

CountTest and RefusalTextTest cover shell_grade.py, the shell parsing of the
cases atelier-xfsn has yet to bring over, and go with it.
"""
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
HARNESS = HERE.parents[1] / "harness"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


run = load("run", HARNESS / "run.py")
judging = run.judging
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


@unittest.skipUnless(os.environ.get("EVAL_ATELIER"), "needs EVAL_ATELIER")
class Fixture(unittest.TestCase):
    """A fresh setup.sh work directory, and the binary driven in it."""

    def setUp(self):
        self.start(HERE)

    def start(self, case, suffix=""):
        """A fresh work directory from `case`'s setup.sh, whose `atelier`
        calls are logged as a run's are; `suffix` ends its name, as a run's
        arm, setup and index end a run's."""
        self.case = case
        self.work = Path(tempfile.mkdtemp(prefix=f"{case.name}-grade-", suffix=suffix))
        subprocess.run(["bash", str(case / "setup.sh"), str(self.work)], check=True,
                       capture_output=True)
        run.log_atelier(self.work)
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")

    def facts(self):
        """The facts the case's grade.py prints."""
        out = subprocess.run([sys.executable, "-I", str(self.case / "grade.py"),
                              str(self.work), str(self.transcript)], check=True,
                             capture_output=True, text=True).stdout
        return json.loads(out)["facts"]

    def graded(self, **answers):
        """The harness's grade of the facts, with the judge's answers given
        as recorded ones, {question id: pass, fail or unknown}, for every
        question of the case's rubric."""
        rubric = judging.read_rubric(self.case) or {"questions": []}
        self.assertEqual(sorted(answers), sorted(q["id"] for q in rubric["questions"]),
                         "give a recorded answer to each rubric question")
        recorded = [{"id": q["id"], "role": q["role"], "kind": q["kind"],
                     "answer": answers[q["id"]], "line": "(recorded)",
                     "reason": "recorded" if answers[q["id"]] == "unknown" else None}
                    for q in rubric["questions"]]
        facts = self.facts()
        return {"facts": facts, **judging.verdict(facts, recorded)}

    def tearDown(self):
        shutil.rmtree(self.work, ignore_errors=True)

    def atelier(self, thread, *args, skip=None):
        """Run atelier as `thread`; `<parent>/<id>` runs it as a Codex subagent
        of <parent>, whose identity the binary makes that."""
        env = {k: v for k, v in os.environ.items() if k not in (
            "CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID", "CODEX_THREAD_ID", "ATELIER_SKIP")}
        parent, _, child = thread.partition("/")
        env.update(T3CODE_THREAD_ID=parent, PATH=f"{self.work / 'bin'}:{env['PATH']}")
        if child:
            env.update(CODEX_SESSION_ID="session-1", CODEX_THREAD_ID=child)
        if skip:
            env["ATELIER_SKIP"] = skip
        return subprocess.run(["atelier", *args], cwd=self.repo, env=env, check=True,
                              capture_output=True, text=True).stdout

    def sh(self, *cmd):
        subprocess.run(cmd, cwd=self.repo, check=True, capture_output=True)

    def build(self, code=FIXED, tests=TESTS):
        """Start, change, tick, report and submit as thread impl, in the
        repository's one checkout switched to the item's branch."""
        self.atelier("impl", "start", ID)
        self.sh("git", "switch", "-q", ID)
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
        self.verdict(reviewer, "approve")

    def verdict(self, reviewer, verdict):
        """Record `reviewer`'s verdict and commit it, as `verdict` says to."""
        self.atelier(reviewer, "verdict", verdict)
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", f"Review {ID}: {verdict}")

    def needs_human(self, thread, title, back_to):
        """File an item labelled needs-human on backlog/ask as `thread`, merge
        it, and switch back to branch `back_to`."""
        self.sh("git", "switch", "-q", "main")
        self.atelier(thread, "start", "--backlog", "ask")
        self.sh("git", "switch", "-q", "backlog/ask")
        self.atelier(thread, "new", title)
        new = self.repo / self.sh_out("git", "status", "--porcelain").split()[-1] / "item.md"
        new.write_text(new.read_text().replace("labels: []", "labels:\n- needs-human"))
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", f"Ask: {title}")
        self.atelier(thread, "merge")
        self.sh("git", "switch", "-q", back_to)

    def sh_out(self, *cmd):
        return subprocess.run(cmd, cwd=self.repo, check=True, capture_output=True,
                              text=True).stdout



class GradeTest(Fixture):
    def done(self, reviewer="rev"):
        self.build()
        self.approve(reviewer)
        self.atelier("impl", "merge")

    def test_done_with_an_independent_reviewer_passes(self):
        self.done()
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["status_on_main"], {"value": "done", "pass": True})
        self.assertEqual((facts["atelier_failed"]["value"], facts["atelier_calls"]["value"]),
                         ([], 5))

    def test_a_codex_subagent_of_the_implementer_reviews_as_another_thread(self):
        self.done("impl/019a7c")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        self.assertEqual(grade["facts"]["reviews_on_main"]["value"][0]["reviewers"],
                         ["impl/019a7c"])

    def test_a_refused_call_is_reported_from_the_log_and_fails_nothing(self):
        self.build()
        with self.assertRaises(subprocess.CalledProcessError):
            self.atelier("impl", "review", ID)
        self.approve("rev")
        self.atelier("impl", "merge")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        [failed] = grade["facts"]["atelier_failed"]["value"]
        self.assertEqual((failed["argv"], failed["thread"], failed["exit"]),
                         (["review", ID], "impl", 1))
        self.assertTrue(grade["facts"]["within_failed_call_limit"]["value"])

    def test_a_skip_fails_the_run(self):
        # Merged past a rule, a change left uncommitted, with the variable.
        self.build()
        self.approve("rev")
        (self.repo / "notes.txt").write_text("later\n")
        self.sh("git", "add", "notes.txt")
        self.atelier("impl", "merge", skip="the notes can wait")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        skips = grade["facts"]["skips"]["value"]
        self.assertIn({"thread": "impl", "argv": ["merge"], "reason": "the notes can wait"},
                      skips)
        # The commit's trailer names the rule passed, then the reason.
        self.assertTrue(any(s["reason"].endswith(": the notes can wait")
                            for s in skips if "commit" in s), skips)
        self.assertEqual(grade["facts"]["status_on_main"]["value"], "done")

    def test_an_item_not_merged_fails(self):
        self.build()
        self.approve("rev")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIn('fact status_on_main: "ready"', grade["failed"])

    def test_a_review_by_the_implementer_fails(self):
        # The binary refuses this, so it is the state left by an agent that
        # rewrote the review file's reviewers by hand before its verdict.
        self.build()
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("*.md")
        review.write_text(review.read_text().replace("- rev\n", "- impl\n- rev\n")
                          .replace("<!--", "Checked.\n<!--", 1))
        self.verdict("rev", "approve")
        self.atelier("impl", "merge")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["approved_by_non_implementer"]["pass"], False)

    def test_tests_that_do_not_cover_the_change_fail(self):
        # Merge checks pass: the suite is green, but it passes on the old code.
        self.build(tests=TESTS.replace('"Hello,  World!"), "hello-world"',
                                       '"Hello World"), "hello-world"')
                   .replace('        self.assertEqual(slugify("  --Rust & Go--  "), "rust-go")\n',
                            ""))
        self.approve("rev")
        self.atelier("impl", "merge")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["tests_fail_on_old_code"]["pass"], False)

    def test_a_failing_test_on_main_fails(self):
        self.done()
        self.sh("git", "switch", "-q", "main")
        (self.repo / "slug.py").write_text(FIXED.replace('.strip("-")', ""))
        self.sh("git", "commit", "-qam", "Stop trimming")
        grade = self.graded()
        self.assertFalse(grade["pass"])
        self.assertIs(grade["facts"]["suite_passes_on_main"]["pass"], False)


class CountTest(unittest.TestCase):
    def setUp(self):
        self.grade = load("shell_grade", HERE / "shell_grade.py")
        self.tmp = Path(tempfile.mkdtemp(prefix="next-loop-count-"))

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def claude(self, calls):
        """A Claude Code transcript of Bash calls, each (command, output, is_error)."""
        events = []
        for n, (cmd, result, err) in enumerate(calls):
            events.append({"type": "assistant", "message": {"content": [
                {"type": "tool_use", "id": f"t{n}", "name": "Bash", "input": {"command": cmd}}]}})
            events.append({"type": "user", "message": {"content": [
                {"type": "tool_result", "tool_use_id": f"t{n}", "content": result,
                 "is_error": err}]}})
        path = self.tmp / "claude.jsonl"
        path.write_text("\n".join(json.dumps(e) for e in events))
        return self.grade.calls(path, "main")

    def counts(self, found):
        return [(r["invocations"], r["help"], r["refused"], r["errored"])
                for r in self.grade.tally(found)]

    def test_each_invocation_and_each_refusal_counts(self):
        submit = "`atelier submit` refused; nothing changed.\nTo submit it for review:\n"
        found, tools = self.claude([
            ("atelier status", "You are thread x", False),
            ("atelier review slug-k3fa", "Exit code 1\nYou (thread x) have implemented "
             "slug-k3fa, and an author does not review its own work. Nothing changed.", True),
            ("cat .atelier/items/slug-k3fa/item.md; atelier --help", "---", False),
            ("atelier verdict aprove", "Exit code 2\nerror: invalid value 'aprove' for "
             "'<VERDICT>'\n\nFor more information, try '--help'.", True),
            ("python3 -m unittest && atelier submit", "Exit code 1\nFAILED", True),
            ("atelier submit; atelier submit; true", submit + submit, False),
            ("atelier merge; atelier status", "Merged slug-k3fa", False),
            ('delegate "Run atelier review slug-k3fa and follow what it prints."',
             "Started thread t", False),
            ('delegate "Run \\`atelier review slug-k3fa\\` and follow what it prints."',
             "Started thread t", False),
            ('delegate "Run `atelier review slug-k3fa` and follow what it prints."',
             "You (thread x) have implemented slug-k3fa. Nothing changed.", False),
            ("cat >> r.md <<'EOF'\nThen atelier verdict approve.\nEOF\n"
             "atelier verdict approve", "Recorded your verdict", False)])
        self.assertEqual(tools, 11)
        self.assertEqual(self.counts(found), [
            (1, 0, 0, 0), (1, 0, 1, 0), (1, 1, 0, 0), (1, 0, 0, 1),
            # Failing tests are not a refused submit.
            (1, 0, 0, 0),
            # Refusals the enclosing shell hid.
            (2, 0, 2, 0),
            (2, 0, 0, 0),
            # Task text runs nothing, unless a substitution runs it.
            (1, 0, 1, 0),
            # A heredoc body is text.
            (1, 0, 0, 0)])

    def test_a_skip_is_found_where_a_shell_ran_it_or_the_binary_said_so(self):
        found, _ = self.claude([
            ('ATELIER_SKIP="Ada said so" atelier merge', "Merged slug-k3fa", False),
            ("export ATELIER_SKIP='x'; git push", "", False),
            ("atelier submit", "Skipped with ATELIER_SKIP: Report: written: x\nSubmitted", False),
            # Text that names it, and the refusal that offers it, set nothing.
            ('echo "ATELIER_SKIP=x"', "ATELIER_SKIP=x", False),
            ("atelier merge", 'Then: atelier submit\nIf the rule is wrong for this case, go past '
             'it on purpose: run the same command again with ATELIER_SKIP="<why>" in front of it.',
             True)])
        self.assertEqual([s["command"][:20] for s in self.grade.skips(found)],
                         ['ATELIER_SKIP="Ada sa', "export ATELIER_SKIP=", "atelier submit"])

    def test_a_codex_transcript_is_unwrapped(self):
        path = self.tmp / "codex.jsonl"
        path.write_text("\n".join(json.dumps(e) for e in [
            {"type": "thread.started", "thread_id": "x"},
            {"type": "item.completed", "item": {"type": "agent_message", "text": "hi"}},
            {"type": "item.completed", "item": {
                "type": "command_execution",
                "command": "/bin/bash -lc \"git commit -qm 'Fix the atelier slug' && atelier submit\"",
                "exit_code": 1, "aggregated_output": "`atelier submit` refused; nothing changed."}},
            {"type": "item.completed", "item": {
                "type": "command_execution",
                "command": "/bin/bash -lc 'delegate \"Run atelier review slug-k3fa.\"'",
                "exit_code": 0, "aggregated_output": "Started thread t"}},
            {"type": "item.completed", "item": {"type": "file_change"}}]))
        found, tools = self.grade.calls(path, "main")
        self.assertEqual(tools, 3)
        self.assertEqual(self.counts(found), [(1, 0, 1, 0)])


class RefusalTextTest(Fixture):
    """shell_grade.py's REFUSED and ERRORED patterns against what the binary
    prints: every kind of refusal matches, and no success does."""

    def run_atelier(self, thread, *args):
        env = {k: v for k, v in os.environ.items()
               if k not in ("CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID")}
        env.update(T3CODE_THREAD_ID=thread, PATH=f"{self.work / 'bin'}:{env['PATH']}")
        done = subprocess.run(["atelier", *args], cwd=self.repo, env=env,
                              capture_output=True, text=True)
        return done.returncode, done.stdout + done.stderr

    def test_refusals_match_and_successes_do_not(self):
        grade = load("shell_grade", HERE / "shell_grade.py")
        outcomes = [self.run_atelier("impl", "status"),
                    self.run_atelier("impl", "start", "slug-k3fa"),
                    self.run_atelier("impl", "submit")]          # on main: refused
        self.sh("git", "switch", "-q", ID)
        outcomes += [self.run_atelier("impl", "submit"),         # unticked: refused
                     self.run_atelier("other", "submit")]        # not the holder: refused
        self.build_after_start()
        outcomes += [self.run_atelier("impl", "submit"),
                     self.run_atelier("impl", "status"),
                     self.run_atelier("impl", "review", ID),     # the author: refused
                     self.run_atelier("rev", "review", ID),
                     self.run_atelier("rev", "status"),
                     self.run_atelier("impl", "verdict", "approve"),  # another role: refused
                     self.run_atelier("rev", "verdict", "aprove")]    # usage error
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1))
        outcomes += [self.run_atelier("rev", "verdict", "approve")]
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", "Review")
        outcomes += [self.run_atelier("rev", "status"),
                     self.run_atelier("rev", "review", ID),      # after its verdict: refused
                     self.run_atelier("impl", "merge"),
                     self.run_atelier("impl", "status"),
                     self.run_atelier("impl", "start", "nope-0000")]  # no such item: refused
        expected = [None, None, "refused", "refused", "refused", None, None, "refused", None, None,
                    "refused", "errored", None, None, "refused", None, None, "refused"]
        got = ["refused" if grade.REFUSED.search(out) else
               "errored" if grade.ERRORED.search(out) else None for _, out in outcomes]
        self.assertEqual(got, expected, "\n---\n".join(out for _, out in outcomes))
        self.assertEqual([code for code, _ in outcomes],
                         [{None: 0, "refused": 1, "errored": 2}[e] for e in expected])

    def build_after_start(self):
        (self.repo / "slug.py").write_text(FIXED)
        (self.repo / "test_slug.py").write_text(TESTS)
        item = self.repo / f".atelier/items/{ID}/item.md"
        item.write_text(item.read_text().replace("- [ ]", "- [x]") + "\nDone.\n")
        self.sh("git", "commit", "-qam", "Change")


if __name__ == "__main__":
    unittest.main()
