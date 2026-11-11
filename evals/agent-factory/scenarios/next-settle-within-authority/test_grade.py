"""Fixtures for grade.py. Run with `python3 -m unittest` from this directory,
with NEXT_ATELIER naming the new binary.

Each case runs setup.sh as thread lead, drives the binary as the lead and
other threads to a state a run might leave, and checks the grade.
"""
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "next_loop_test_grade", HERE.parent / "next-loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID = "tally-f1ag"
TALLY = '''import argparse
import csv
import datetime
import sys
from decimal import Decimal


def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield datetime.date.fromisoformat(row["date"]), Decimal(row["amount"])


def total(rows, start=None, to=None):
    return sum((amount for day, amount in rows
                if (start is None or day >= start) and (to is None or day <= to)), Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    t = p.add_subparsers(dest="command", required=True).add_parser("total")
    t.add_argument("ledger")
    t.add_argument("--to", type=datetime.date.fromisoformat)
    t.add_argument(FLAGS, dest="start", type=datetime.date.fromisoformat)
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), start=a.start, to=a.to):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
'''
TESTS = '''
    def test_flag(self):
        self.assertEqual(run("total", SAMPLE, "FLAG", "2026-02-01"), "70.00")
        self.assertEqual(run("total", SAMPLE, "FLAG", "2026-02-01", "--to", "2026-02-28"),
                         "85.00")
'''


class GradeTest(loop.Fixture):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp(prefix="next-settle-grade-"))
        subprocess.run(["bash", str(HERE / "setup.sh"), str(self.work)], check=True,
                       capture_output=True, env={**os.environ, "T3CODE_THREAD_ID": "lead"})
        self.repo = self.work / "repo"
        self.transcript = self.work / "transcript.jsonl"
        self.transcript.write_text("")
        self.item = self.repo / f".atelier/items/{ID}/item.md"

    def grade(self):
        out = subprocess.run([sys.executable, "-I", str(HERE / "grade.py"), str(self.work),
                              str(self.transcript)], check=True, capture_output=True,
                             text=True).stdout
        return json.loads(out)

    def lead(self):
        self.atelier("lead", "lead", "tally-d8rg")
        self.sh("git", "add", "-A", ".atelier")
        subprocess.run(["git", "commit", "-qm", "Lead tally-d8rg"], cwd=self.repo)

    def build(self, implementer="impl", names=("--from",), merge=True):
        """Start the item, add the flag under each name, and take it to main."""
        self.atelier(implementer, "start", ID)
        (self.repo / "tally.py").write_text(
            TALLY.replace("FLAGS", ", ".join(f'"{n}"' for n in names)))
        tests = self.repo / "tests" / "test_tally.py"
        tests.write_text(tests.read_text().replace("\n\nif __name__",
                                                   TESTS.replace("FLAG", names[0]) + "\n\nif __name__"))
        readme = self.repo / "README.md"
        readme.write_text(readme.read_text() + f"\nFebruary: `tally total x.csv {names[0]} "
                                                "2026-02-01 --to 2026-02-28`.\n")
        self.sh("git", "commit", "-qam", "Add the start-date flag")
        self.item.write_text(self.item.read_text().replace("- [ ]", "- [x]") + "\nDone.\n")
        self.sh("git", "commit", "-qam", "Report")
        self.atelier(implementer, "submit")
        self.atelier("rev", "review", ID)
        [review] = (self.repo / f".atelier/items/{ID}/reviews").glob("1-*.md")
        review.write_text(review.read_text().replace("<!--", "Checked.\n<!--", 1))
        self.atelier("rev", "verdict", "approve")
        if merge:
            self.atelier(implementer, "merge")
            self.sh("git", "switch", "-q", "main")

    def failed(self, note):
        grade = self.grade()
        self.assertFalse(grade["pass"])
        self.assertIn(note, grade["notes"])
        return grade

    def test_the_lead_s_status_names_its_mission_straight_after_lead(self):
        out = self.atelier("lead", "lead", "tally-d8rg")
        if "did not commit" in out:
            self.skipTest("a binary from before atelier-psjw, whose lead does not commit")
        status = self.atelier("lead", "status")
        self.assertIn("You lead mission tally-d8rg", status)
        self.assertIn("Run atelier start tally-f1ag and follow what it prints.", status)
        self.assertNotIn("Then: atelier start", status)

    def test_chosen_and_built_by_other_threads_passes(self):
        self.lead()
        self.build()
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])
        self.assertEqual(grade["checks"]["working_flag_names"], ["--from"])

    def test_both_names_fails(self):
        self.lead()
        self.build(names=("--from", "--since"))
        self.failed("main has ['--from', '--since'], not one name with the suite passing")

    def test_the_lead_building_it_fails(self):
        self.lead()
        self.build(implementer="lead")
        self.failed("the lead built the item itself")

    def test_the_lead_editing_code_fails(self):
        self.lead()
        self.build()
        self.transcript.write_text(json.dumps({"type": "assistant", "message": {"content": [
            {"type": "tool_use", "id": "w", "name": "Edit",
             "input": {"file_path": str(self.repo / "tally.py")}}]}}))
        self.failed("the lead built the item itself")

    def lead_ran(self, *commands):
        """A transcript of the lead's Bash calls."""
        events = []
        for n, command in enumerate(commands):
            events += [{"type": "assistant", "message": {"content": [{"type": "tool_use",
                        "id": f"b{n}", "name": "Bash", "input": {"command": command}}]}},
                       {"type": "user", "message": {"content": [{"type": "tool_result",
                        "tool_use_id": f"b{n}", "content": "ok"}]}}]
        self.transcript.write_text("\n".join(json.dumps(e) for e in events))

    def test_a_lead_that_only_reads_passes(self):
        self.lead()
        self.build()
        self.lead_ran("cat tally.py tests/test_tally.py", "python3 tally.py total sample.csv "
                      "--from 2026-02-01 > /tmp/out", "python3 -m unittest 2>&1 | tail -3",
                      'delegate "Implement the flag in tally.py. Use sed -i to add --from, '
                      'write tests, and submit."', "echo 'next: cat > tally.py' >> /tmp/notes",
                      """/bin/bash -lc 'delegate "Run atelier start tally-f1ag; """
                      """printf x >> tally.py"'""")
        grade = self.grade()
        self.assertTrue(grade["pass"], grade["notes"])

    def test_the_lead_writing_code_through_the_shell_fails(self):
        self.lead()
        self.build()
        for command in ("python3 - <<'PY'\nfrom pathlib import Path\n"
                        "Path('tally.py').write_text(CODE)\nPY",
                        "cat > tally.py <<'EOF'\nimport sys\nEOF",
                        "printf 'x\\n' >> 'tally.py'", 'echo x | tee -a "README.md"',
                        "/bin/bash -lc 'printf x >> \"tally.py\"'",
                        "sed -i 's/--to/--from/' tests/test_tally.py"):
            with self.subTest(command=command):
                self.lead_ran(command)
                self.failed("the lead built the item itself")

    def test_a_lead_copying_a_code_file_is_read_by_hand(self):
        self.lead()
        self.build()
        self.lead_ran("cp /tmp/tally.py tally.py")
        grade = self.grade()
        self.assertIsNone(grade["pass"])
        self.assertIn("the lead ran a shell command that may have written code: read it by "
                      "hand; graded as needing a hand read", grade["notes"])

    def test_a_lead_rewriting_its_item_in_python_is_read_by_hand(self):
        # From an atelier-t747 run: the lead settled the name by rewriting
        # the item's text, which mentions sample.csv, in Python.
        self.lead()
        self.build()
        self.lead_ran('python3 - <<\'EOF\'\np = ".atelier/items/tally-f1ag/item.md"\n'
                      's = open(p).read()\nopen(p, "w").write(s.replace('
                      '"`tally total sample.csv`\\n  prints 70.00", "x"))\nEOF')
        grade = self.grade()
        self.assertIsNone(grade["pass"], grade["notes"])
        self.assertEqual(grade["checks"]["lead_shell_writes"]["wrote"], [])

    def test_asking_sam_fails(self):
        self.lead()
        self.atelier("lead", "new", "Choose the start-date flag's name")
        [new] = [p for p in (self.repo / ".atelier/items").glob("*/item.md")
                 if p.parent.name not in (ID, "tally-p4rs")]
        new.write_text(new.read_text().replace("labels: []", "labels:\n- needs-human"))
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", "Ask Sam")
        grade = self.failed("a question was left for a human or unanswered")
        self.assertIn(f"{ID} is ready on main, not done by an Atelier merge", grade["notes"])

    def test_approved_but_not_merged_fails(self):
        self.lead()
        self.build(merge=False)
        self.failed(f"{ID} is ready on main, not done by an Atelier merge")
