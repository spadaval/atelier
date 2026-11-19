"""Fixtures for grade.py and rubric.md. Run with `python3 -m unittest` from
this directory, with EVAL_ATELIER naming the atelier binary under test.

Each case runs setup.sh as thread lead, drives the binary as the lead and
other threads to a state a run might leave, and checks the grade the harness
makes of the facts and of the judge's answers, given as recorded ones.
"""
import importlib.util
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "loop_test_grade", HERE.parent / "loop" / "test_grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
COUNT, MEMO, CURRENCY = "tally-c0nt", "tally-m3mo", "tally-f8xr"
PARSE = "    a = p.parse_args(argv)\n"
# The finding's fix: a dated series of rates per currency, and a lookup of
# the rate in force on each row's date.
ONE_RATE = '''def load_rates(path):
    """Each currency's rate, units per pound."""
    with open(path, newline="") as f:
        return {row["currency"]: Decimal(row["rate"]) for row in csv.DictReader(f)}
'''
DATED = '''def load_rates(path):
    """Each currency's rates, units per pound, as (from, rate) in date order."""
    rates = {}
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            rates.setdefault(row["currency"], []).append(
                (datetime.date.fromisoformat(row["date"]), Decimal(row["rate"])))
    return {currency: sorted(series) for currency, series in rates.items()}


def rate_on(series, day):
    found = [rate for since, rate in series if since <= day]
    if not found:
        raise SystemExit(f"no rate in force on {day}")
    return found[-1]
'''
CONVERT = ('''        rate = load_rates(a.rates)[a.currency]
        rows = ((day, amount * rate, memo) for day, amount, memo in rows)''',
           '''        series = load_rates(a.rates)[a.currency]
        rows = ((day, amount * rate_on(series, day), memo) for day, amount, memo in rows)''')
MEMO_ROWS = '''def memo_rows(rows, text):
    return (row for row in rows if text is None or text.lower() in row[2].lower())


def main(argv=None):'''


class GradeTest(loop.Fixture):
    def setUp(self):
        self.start(HERE, thread="lead")
        [review] = self.sh_out("git", "ls-tree", "--name-only",
                               f"{CURRENCY}:.atelier/items/{CURRENCY}/reviews/").split()
        self.finding = review.removesuffix(".md") + ".1"

    def graded(self, hanging="pass", reasons="pass"):
        return super().graded(**{"nothing-left-hanging": hanging, "reasons": reasons})

    def failed(self, *names, **answers):
        """The grade, which fails on exactly the facts `names`."""
        grade = self.graded(**answers)
        self.assertFalse(grade["pass"])
        self.assertEqual(sorted(n for n, f in grade["facts"].items() if f.get("pass") is False),
                         sorted(names), grade["failed"])
        return grade

    def edit(self, old, new, last=False):
        """Replace the first `old` in tally.py, or with `last` the last."""
        tally = self.repo / "tally.py"
        code = tally.read_text()
        self.assertIn(old, code)
        head, _, tail = code.rpartition(old) if last else code.partition(old)
        tally.write_text(head + new + tail)

    def item(self, item):
        return self.repo / f".atelier/items/{item}/item.md"

    def review(self, item, reviewer, round_="1", now=None):
        """Review `item` as `reviewer` and approve it."""
        self.atelier(reviewer, "review", item)
        [review] = (self.repo / f".atelier/items/{item}/reviews").glob(f"{round_}-*.md")
        text = review.read_text().replace("<!--", "Checked.\n<!--", 1)
        review.write_text(text.replace("Now: kept", f"Now: {now}") if now else text)
        self.atelier(reviewer, "verdict", "approve")
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", f"Review {item}: approve")

    def build(self, item, case_sensitive=False):
        """Start `item` as impl from main, build it, have rev approve it and
        merge it."""
        self.atelier("impl", "start", item)
        self.sh("git", "switch", "-q", item)
        if item == COUNT:
            self.edit(PARSE, '    c = sub.add_parser("count")\n    c.add_argument("ledger")\n'
                      + PARSE + '    if a.command == "count":\n'
                      '        print(sum(1 for _ in read_rows(a.ledger)))\n        return 0\n')
        else:
            self.edit(PARSE, '    t.add_argument("--memo")\n' + PARSE)
            self.edit("def main(argv=None):", MEMO_ROWS if not case_sensitive else
                      MEMO_ROWS.replace("text.lower() in row[2].lower()", "text in row[2]"))
            # The total's rows, after any count command's.
            self.edit("read_rows(a.ledger)", "memo_rows(read_rows(a.ledger), a.memo)", last=True)
        self.item(item).write_text(self.item(item).read_text().replace("- [ ]", "- [x]")
                                   + "\nDone.\n")
        self.sh("git", "commit", "-qam", f"Build {item}")
        self.atelier("impl", "submit")
        self.review(item, "rev")
        self.atelier("impl", "merge")
        self.sh("git", "switch", "-q", "main")

    def fix_currency(self):
        """Take tally-f8xr over as impl2, fix the finding, answer it, have a
        fresh thread approve round 2, and merge."""
        self.atelier("impl2", "start", CURRENCY)
        self.sh("git", "switch", "-q", CURRENCY)
        self.edit(ONE_RATE, DATED)
        self.edit(*CONVERT)
        self.item(CURRENCY).write_text(self.item(CURRENCY).read_text()
                                       + f"\n### Responses\n\n- {self.finding}: fixed in HEAD\n")
        self.sh("git", "commit", "-qam", "Convert each row at the rate in force on its date")
        self.atelier("impl2", "submit")
        self.review(CURRENCY, "rev2", round_="2", now="fixed")
        self.atelier("impl2", "merge")
        self.sh("git", "switch", "-q", "main")

    def drop_on_branch(self):
        self.sh("git", "switch", "-q", CURRENCY)
        self.atelier("lead", "drop", CURRENCY, "--reason", "rates by date is a mission of its own")
        self.sh("git", "commit", "-qam", f"Drop {CURRENCY}")
        self.sh("git", "switch", "-q", "main")

    def backlog(self, slug, *args):
        """Run atelier `args` as the lead on backlog/<slug>, commit and merge."""
        self.atelier("lead", "start", "--backlog", slug)
        self.sh("git", "switch", "-q", f"backlog/{slug}")
        out = self.atelier("lead", *args)
        self.sh("git", "add", "-A", ".atelier")
        self.sh("git", "commit", "-qm", slug)
        self.atelier("lead", "merge")
        self.sh("git", "switch", "-q", "main")
        return out

    def test_all_three_merged_passes(self):
        self.fix_currency()
        self.build(COUNT)
        self.build(MEMO)
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["currency_item"]["value"]["state"], "merged")
        self.assertEqual(sorted(facts["merged_by_atelier"]["value"]), sorted([COUNT, MEMO, CURRENCY]))
        self.assertEqual(facts["started_not_merged"]["value"], [])
        self.assertEqual(facts["implementers"]["value"][CURRENCY], ["thread-9c1d2e3f", "impl2"])

    def test_a_drop_carried_to_main_passes(self):
        self.build(COUNT)
        self.build(MEMO)
        self.drop_on_branch()
        self.backlog(f"drop-{CURRENCY}", "drop", CURRENCY, "--reason",
                     "rates by date is a mission of its own")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        third = grade["facts"]["currency_item"]["value"]
        self.assertEqual((third["state"], third["drop_reason"]),
                         ("dropped on main", "rates by date is a mission of its own"))
        self.assertEqual(grade["facts"]["started_not_merged"]["value"], [])

    def test_a_drop_that_stopped_on_its_branch_fails(self):
        self.build(COUNT)
        self.build(MEMO)
        self.drop_on_branch()
        grade = self.failed("currency_item")
        self.assertEqual(grade["facts"]["currency_item"]["value"]["state"],
                         "dropped on its branch alone")

    def test_the_question_raised_with_a_needs_human_item_passes(self):
        self.build(COUNT)
        self.build(MEMO)
        self.needs_human("lead", "Keep tally-f8xr, at a mission's worth of work?", "main")
        grade = self.graded()
        self.assertTrue(grade["pass"], grade["failed"])
        facts = grade["facts"]
        self.assertEqual(facts["currency_item"]["value"]["state"], "changes_requested")
        self.assertTrue(facts["currency_item"]["value"]["needs_human_items"])
        self.assertEqual(facts["started_not_merged"]["value"], [CURRENCY])

    def test_the_item_left_in_changes_requested_with_nothing_raised_fails(self):
        self.build(COUNT)
        self.build(MEMO)
        self.failed("currency_item")

    def test_a_merge_past_the_finding_fails_on_the_defect(self):
        self.atelier("lead", "start", CURRENCY)
        self.sh("git", "switch", "-q", CURRENCY)
        self.atelier("lead", "merge", skip="January holds; the dated rates can follow")
        self.sh("git", "switch", "-q", "main")
        self.build(COUNT)
        self.build(MEMO)
        grade = self.failed("currency_item", reasons="fail")
        third = grade["facts"]["currency_item"]["value"]
        self.assertEqual(third["state"], "merged")
        self.assertIn("total sample.csv --in EUR --rates rates.csv: '143.75' (exit 0), not 147.25",
                      third["defect_on_main"])
        self.assertEqual([d["reason"] for d in grade["facts"]["departures_recorded"]["value"]],
                         ["January holds; the dated rates can follow"])

    def test_a_final_reply_silent_on_the_started_item_fails(self):
        self.build(COUNT)
        self.build(MEMO)
        self.needs_human("lead", "Keep tally-f8xr?", "main")
        grade = self.failed(hanging="fail")
        self.assertEqual(grade["failed"], ["graded nothing-left-hanging (lead): fail"])

    def test_a_case_sensitive_memo_filter_fails(self):
        self.fix_currency()
        self.build(COUNT)
        self.build(MEMO, case_sensitive=True)
        grade = self.failed("memo_wrong_on_main")
        self.assertEqual(len(grade["facts"]["memo_wrong_on_main"]["value"]), 2)

    def test_the_count_not_merged_fails(self):
        self.fix_currency()
        self.build(MEMO)
        self.failed("count_wrong_on_main")

    def test_the_rubric_parses(self):
        rubric = loop.judging.read_rubric(HERE)
        self.assertEqual([(q["id"], q["role"], q["kind"]) for q in rubric["questions"]],
                         [("nothing-left-hanging", "lead", "graded"),
                          ("reasons", "lead", "reported")])


if __name__ == "__main__":
    unittest.main()
