"""convert_records.py on a small old-layout repository each test builds.

Run: TMPDIR=<a directory outside any Git repository> python3 -m unittest discover scripts/cutover
"""
import os
import posixpath
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCRIPT = HERE / "convert_records.py"
NEW_BINARY = HERE.parent.parent / "next/target/debug/atelier"
sys.path.insert(0, str(HERE))
import convert_records  # noqa: E402

UPDATED = "2026-11-02T10:00:00.250000000+00:00"
CREATED = "2026-11-01T09:00:00.123456789+00:00"


def git(repo, *args):
    env = {**os.environ, "GIT_AUTHOR_NAME": "b", "GIT_AUTHOR_EMAIL": "b@b", "GIT_COMMITTER_NAME": "b",
           "GIT_COMMITTER_EMAIL": "b@b", "GIT_AUTHOR_DATE": "2026-11-01T00:00:00Z",
           "GIT_COMMITTER_DATE": "2026-11-01T00:00:00Z"}
    return subprocess.run(["git", "-C", str(repo), "-c", "commit.gpgsign=false", *args], env=env, check=True,
                          capture_output=True, text=True).stdout.rstrip("\n")


def targets(entries, key):
    if not entries:
        return f"  {key}: []\n"
    out = f"  {key}:\n"
    for e in entries:
        out += f'  - kind: "{e[2] if len(e) > 2 else "issue"}"\n    id: "{e[0]}"\n'
        out += f'    type: "{e[1]}"\n' if e[1] else ""
    return out


def record(rid, kind, status, title, body, labels=(), relates=(), blocks=(), children=(), strategy=None):
    labels_text = "labels: []\n" if not labels else "labels:\n" + "".join(f'- "{lab}"\n' for lab in labels)
    return (f'---\ncreated_at: "{CREATED}"\nid: "{rid}"\nissue_type: "{kind}"\n{labels_text}'
            + (f"strategy:\n  path: {strategy}\n" if strategy else "")
            + 'priority: "P2"\nrelationships:\n'
            + targets([(b, None) for b in blocks], "blocks") + targets([(c, None) for c in children], "children")
            + "  attachments: []\n" + targets(relates, "relates")
            + f'schema: "atelier.issue"\nschema_version: 2\nstatus: "{status}"\ntitle: "{title}"\n'
            + f'updated_at: "{UPDATED}"\n---\n{body}')


ITEM_BODY = """
## Goal

Make the widget round.

## Context

The spec is [the plan](../../docs/strategies/plan.md); see also docs/strategies/plan.md.

```text
## Not a heading
```

## Done when

- [ ] It is round.
  - Rounder than before.

## Scope

Only the widget.

## Notes

- 2026-11-01 [note] Tried squares first.
"""

STRATEGY = """---
status: approved
---

# Why

Widgets are square. See [the architecture](architecture.md "the design"), [the shape](#shape) and [the mock][m].
[The item](../../.atelier/issues/demo-a1ta.md) does it, after [the second mission](../../.atelier/issues/demo-m2nd.md).

## Detail

More reasons.

# Outcome

Round widgets.

# Success Criteria

Nobody complains.

# Shape

Mocks are in [mocks](<mocks/a.md> "a mock"), [the mock][m] and [n][].

[m]:
  mocks/a.md#a-mock
[n]: <mocks/a.md>

# Scope

Widgets only.

# Authority

The lead decides.

# Risks and Unknowns

Corners.
"""

WORKFLOW = """schema: atelier.workflow
schema_version: 3
branch_policy:
  base_branch: main
  merge_strategy: merge_commit
statuses:
  todo:
    category: todo
workflows:
  work:
    applies_to:
    - task
    transitions:
      start:
        from: [todo]
        to: in_progress
# The forge.
forge:
  provider: github
  remote: origin
  ci_workflow: ci.yml
merge_checks:
  - shell.check:
      command: "git diff --check HEAD^ HEAD -- . ':!.atelier'"
  - ci.passed
"""

FILES = {
    ".atelier/config.toml": 'schema = "atelier.project_config"\nschema_version = 1\nproject_slug = "demo"\n',
    ".atelier/workflow.yaml": WORKFLOW,
    ".atelier/reviews/demo-r1vw.md": "a review\n",
    ".atelier/evidence/demo-e1vd.md": "evidence\n",
    ".github/workflows/ci.yml": "name: ci\n",
    "docs/strategies/plan.md": STRATEGY,
    "docs/strategies/architecture.md": "The plan is [here](plan.md) and in docs/strategies/plan.md.\n",
    "docs/other.md": "See [the plan](<strategies/plan.md>).\n",
    "docs/strategies/mocks/a.md": "# A mock\n",
    "README.md": "Demo.\n",
    ".gitignore": "/.atelier/runtime/\n",
}

RECORDS = [
    record("demo-m1ss", "mission", "todo", "Round widgets",
           "\n## Outcome\n\nThe record's outcome.\n\n## Background\n\nWhy now.\n\n## Notes\n\n- a note\n",
           labels=["shape"], relates=[("demo-f5le", "advances"), ("demo-d9dn", "advances")],
           strategy="docs/strategies/plan.md"),
    record("demo-m2nd", "mission", "draft", "Second: a mission", "\n## Why\n\nBecause.\n\n## Background\n\nOld.\n"),
    record("demo-m3cl", "mission", "done", "Closed mission", "\n## Outcome\n\nDone.\n"),
    record("demo-a1ta", "task", "todo", "Make the widget round", ITEM_BODY, labels=["widgets"],
           relates=[("demo-m2nd", "advances"), ("demo-m1ss", "advances"), ("demo-b2bu", "blocked_by"),
                    ("demo-d9dn", "blocked_by"), ("demo-r1vw", "discovered_from", "review")]),
    record("demo-b2bu", "bug", "backlog", "true", "\n## Goal\n\ng\n\n## Done when\n\nd\n",
           labels=["bug"], blocks=["demo-c3fe"], children=["demo-c3fe"]),
    record("demo-c3fe", "feature", "in_progress", "Build it", "\n## Goal\n\ng\n",
           relates=[("demo-a1ta", "related")]),
    record("demo-e4sp", "spike", "draft", "Spike", "\n## Goal\n\ng\n\n## Done when\n\nd\n",
           relates=[("demo-m2nd", "advances")], children=["demo-q5gc"]),
    record("demo-f5le", "loose_end", "todo", "Loose end", "\n## Goal\n\ng\n\n## Done when\n\nd\n"),
    record("demo-g6va", "validation", "backlog", "Validate", "\n## Goal\n\ng\n\n## Done when\n\nd\n"),
    record("demo-h7de", "decision", "todo", "Pick a shape",
           "\n## Context\n\nShapes vary.\n\n## Question\n\nRound or oval?\n\n## Options\n\n- round\n- oval\n"),
    record("demo-n3ch", "task", "todo", "Child", "\n## Goal\n\ng\n\n## Done when\n\nd\n", children=["demo-r6gc"]),
    record("demo-q5gc", "task", "todo", "Grandchild by a spike", "\n## Goal\n\ng\n\n## Done when\n\nd\n"),
    record("demo-r6gc", "task", "todo", "Grandchild", "\n## Goal\n\ng\n\n## Done when\n\nd\n"),
    record("demo-d9dn", "task", "done", "Done", "\n## Goal\n\ng\n", relates=[("demo-m1ss", "advances")],
           children=["demo-n3ch", "demo-e4sp"]),
    record("demo-j8dc", "decision", "done", "Decided", "\n## Question\n\nq\n"),
    record("demo-k1sp", "feature", "superseded", "Superseded", "\n## Goal\n\ng\n"),
    record("demo-l2dc", "task", "declined", "Declined", "\n## Goal\n\ng\n"),
]

OPEN_ITEMS = {"demo-a1ta", "demo-b2bu", "demo-c3fe", "demo-e4sp", "demo-f5le", "demo-g6va", "demo-h7de", "demo-n3ch",
              "demo-q5gc", "demo-r6gc"}


def resolves(root, file, target):
    """Whether a relative link's file exists and, when it names a fragment
    of a Markdown file, that file has a heading of that slug."""
    path, _, fragment = target.partition("#")
    found = root / posixpath.normpath(posixpath.join(posixpath.dirname(file), path))
    if not found.exists():
        return False
    headings = re.findall(r"^#+ (.*)$", found.read_text(), re.M) if found.is_file() else []
    return not fragment or fragment in {convert_records.slug(h) for h in headings}


class Converted(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.tmp)
        self.repo = Path(self.tmp) / "repo"
        self.repo.mkdir()
        git(self.repo, "init", "-q", "-b", "main")
        files = dict(FILES)
        for text in RECORDS:
            rid = re.search(r'^id: "(.*)"', text, re.M).group(1)
            files[f".atelier/issues/{rid}.md"] = text
        self.write(files)
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-qm", "old layout")
        self.old = git(self.repo, "rev-parse", "HEAD")

    def write(self, files):
        for path, text in files.items():
            (self.repo / path).parent.mkdir(parents=True, exist_ok=True)
            (self.repo / path).write_text(text)

    def run_script(self, *args):
        return subprocess.run([sys.executable, str(SCRIPT), *args], cwd=self.repo, capture_output=True, text=True)

    def convert(self):
        result = self.run_script()
        self.assertEqual(result.returncode, 0, result.stderr)
        return result.stdout

    def read(self, path):
        return (self.repo / path).read_text()

    def body(self, path):
        return self.read(path).split("\n---\n", 1)[1]

    def test_replaces_atelier_and_changes_only_the_moved_strategy_outside_it(self):
        out = self.convert()
        self.assertEqual(sorted(os.listdir(self.repo / ".atelier")), ["config.toml", "items", "missions"])
        status = git(self.repo, "status", "--porcelain", "--untracked-files=all").splitlines()
        self.assertEqual([line for line in status if ".atelier/" not in line], [" D docs/strategies/plan.md"])
        self.assertEqual(git(self.repo, "rev-parse", "HEAD"), self.old)
        self.assertIn("Nothing was committed", out)

    def test_open_records_keep_their_ids_and_closed_ones_are_left(self):
        out = self.convert()
        self.assertEqual(set(os.listdir(self.repo / ".atelier/items")), OPEN_ITEMS)
        self.assertEqual(set(os.listdir(self.repo / ".atelier/missions")), {"demo-m1ss", "demo-m2nd"})
        statuses = {rid: re.search(r"^status: (\w+)$", self.read(f".atelier/items/{rid}/item.md"), re.M).group(1)
                    for rid in OPEN_ITEMS}
        self.assertEqual(statuses["demo-e4sp"], "draft")  # draft
        self.assertEqual(statuses["demo-b2bu"], "draft")  # backlog
        self.assertEqual(statuses["demo-a1ta"], "ready")  # todo
        self.assertEqual(statuses["demo-c3fe"], "ready")  # in_progress
        self.assertIn("ready:\n  by: conversion\n  at: 2026-11-02T10:00:00.250Z\n",
                      self.read(".atelier/items/demo-c3fe/item.md"))
        self.assertIn("ready: null\n", self.read(".atelier/items/demo-e4sp/item.md"))
        self.assertIn("status: draft\n", self.read(".atelier/missions/demo-m2nd/mission.md"))
        self.assertIn("status: ready\n", self.read(".atelier/missions/demo-m1ss/mission.md"))
        self.assertRegex(out, r"Made ready from in_progress[^\n]*\n  demo-c3fe\n")

    def test_an_item_keeps_its_fields_and_text(self):
        self.convert()
        self.assertEqual(self.read(".atelier/items/demo-a1ta/item.md"), """---
schema: atelier.item
schema_version: 3
id: demo-a1ta
title: Make the widget round
status: ready
mission: demo-m1ss
labels:
- widgets
priority: medium
blocked_by:
- demo-b2bu
implementers: []
ready:
  by: conversion
  at: 2026-11-02T10:00:00.250Z
trivial: null
caused_by: null
created_at: 2026-11-01T09:00:00.123456789Z
---

## Goal

Make the widget round.

## Done when

- [ ] It is round.
  - Rounder than before.

## Scope

Only the widget.

## Context

The spec is [the plan](../../missions/demo-m1ss); see also .atelier/missions/demo-m1ss/.

```text
## Not a heading
```

## Notes

- 2026-11-01 [note] Tried squares first.

## Links

Relationships of the old record that the new format has no field for.

- advances: demo-m2nd (Second: a mission)
- discovered_from review: demo-r1vw
""")

    def test_types_become_labels_and_relationships_their_fields_or_text(self):
        out = self.convert()
        bug = self.read(".atelier/items/demo-b2bu/item.md")
        self.assertIn("title: 'true'\n", bug)
        self.assertIn("labels:\n- bug\n", bug)  # not twice
        feature = self.read(".atelier/items/demo-c3fe/item.md")
        self.assertIn("labels:\n- feature\n", feature)
        self.assertIn("blocked_by:\n- demo-b2bu\n", feature)  # held by the blocker
        self.assertIn("- parent: demo-b2bu (true)\n- related: demo-a1ta (Make the widget round)\n", feature)
        for rid, label in (("demo-e4sp", "spike"), ("demo-f5le", "loose_end"), ("demo-g6va", "validation")):
            self.assertIn(f"labels:\n- {label}\n", self.read(f".atelier/items/{rid}/item.md"))
        self.assertIn("mission: demo-m1ss\n", self.read(".atelier/items/demo-f5le/item.md"))  # held by the mission
        # An item that advances no mission takes its nearest ancestor's, and still names its parent.
        child = self.read(".atelier/items/demo-n3ch/item.md")
        self.assertIn("mission: demo-m1ss\n", child)  # through a closed parent
        self.assertIn("- parent: demo-d9dn (Done)\n", child)
        self.assertIn("labels: []\n", child)
        self.assertIn("mission: demo-m1ss\n", self.read(".atelier/items/demo-r6gc/item.md"))  # past an open parent
        self.assertIn("mission: demo-m2nd\n", self.read(".atelier/items/demo-q5gc/item.md"))  # the nearest one
        self.assertIn("mission: demo-m2nd\n", self.read(".atelier/items/demo-e4sp/item.md"))  # its own, not its parent's
        self.assertIn("Items that advance no mission, given their nearest ancestor's (the parent is in Links):\n"
                      "  demo-n3ch: demo-m1ss, through demo-d9dn\n  demo-q5gc: demo-m2nd, through demo-e4sp\n"
                      "  demo-r6gc: demo-m1ss, through demo-d9dn\n", out)
        self.assertIn("  demo-a1ta: advances demo-m2nd, discovered_from review demo-r1vw\n", out)

    def test_a_decision_becomes_a_question_and_missing_sections_are_written(self):
        self.convert()
        decision = self.read(".atelier/items/demo-h7de/item.md")
        self.assertIn("labels:\n- decision\n- needs-human\n", decision)
        self.assertEqual(decision.split("\n---\n", 1)[1], """
## Goal

Round or oval?

## Done when

- [ ] The question is answered and the answer is written in this item's Report.

## Context

Shapes vary.

## Options

- round
- oval
""")
        self.assertIn(f"## Done when\n\n{convert_records.UNSPECIFIED}\n", self.read(".atelier/items/demo-c3fe/item.md"))
        written = [str(p.relative_to(self.repo)) for p in (self.repo / ".atelier").rglob("*") if p.is_file()]
        self.assertFalse([p for p in written if "review" in p or "evidence" in p], written)

    def test_a_mission_takes_its_sections_from_its_strategy(self):
        self.convert()
        mission = ".atelier/missions/demo-m1ss/"
        self.assertEqual(sorted(os.listdir(self.repo / mission)), ["design.md", "mission.md", "notes.md"])
        self.assertTrue(self.read(mission + "mission.md").startswith(
            "---\nschema: atelier.mission\nschema_version: 3\nid: demo-m1ss\ntitle: Round widgets\nstatus: ready\n"
            "labels:\n- shape\nlead: null\nready:\n  by: conversion\n  at: 2026-11-02T10:00:00.250Z\n"
            "created_at: 2026-11-01T09:00:00.123456789Z\n---\n"))
        self.assertEqual(self.body(mission + "mission.md"), """
## Why

Widgets are square. See [the architecture](../../../docs/strategies/architecture.md "the design"), [the shape](design.md#shape) and [the mock][m].
[The item](../../items/demo-a1ta/item.md) does it, after [the second mission](../demo-m2nd/mission.md).

### Detail

More reasons.

## Outcome

Round widgets.

## Success criteria

Nobody complains.

## Scope

Widgets only.

## Authority

The lead decides.

[m]:
  ../../../docs/strategies/mocks/a.md#a-mock
""")
        self.assertEqual(self.read(mission + "design.md"), """---
status: approved
---

# Shape

Mocks are in [mocks](<../../../docs/strategies/mocks/a.md> "a mock"), [the mock][m] and [n][].

[m]:
  ../../../docs/strategies/mocks/a.md#a-mock
[n]: <../../../docs/strategies/mocks/a.md>

# Risks and Unknowns

Corners.
""")
        notes = self.read(mission + "notes.md")
        for text in ("## Outcome\n\nThe record's outcome.\n", "## Background\n\nWhy now.\n", "## Notes\n\n- a note\n",
                     "- advances: demo-d9dn (Done)\n"):
            self.assertIn(text, notes)
        self.assertNotIn("demo-f5le", notes)  # it holds the mission as its field
        self.assertFalse((self.repo / "docs/strategies/plan.md").exists())

    def test_a_mission_without_a_strategy_takes_its_sections_from_its_record(self):
        self.convert()
        mission = ".atelier/missions/demo-m2nd/"
        self.assertIn("title: 'Second: a mission'\n", self.read(mission + "mission.md"))
        self.assertEqual(self.body(mission + "mission.md"),
                         f"\n## Why\n\nBecause.\n\n## Outcome\n\n{convert_records.UNSPECIFIED}\n")
        self.assertIn("## Background\n\nOld.\n", self.read(mission + "notes.md"))
        self.assertFalse((self.repo / mission / "design.md").exists())

    def test_every_link_that_resolved_still_resolves_and_old_paths_are_listed(self):
        out = self.convert()
        links = 0
        for path in (self.repo / ".atelier").rglob("*.md"):
            rel = str(path.relative_to(self.repo))
            text = path.read_text()
            inline = re.findall(r"\]\(<?([^)\s>]+)>?(?: \"[^\"]*\")?\)", text)
            definitions = dict(re.findall(r"^\[([^\]]+)\]:\s*<?([^\s>]+)>?$", text, re.M))
            for target in inline + list(definitions.values()):
                links += 1
                self.assertTrue(resolves(self.repo, rel, target), f"{rel}: {target}")
            # Each reference a document uses is defined in that document.
            for label in re.findall(r"\]\[([^\]]+)\]", text) + re.findall(r"\[([^\]]+)\]\[\]", text):
                self.assertIn(label, definitions, f"{rel}: [{label}]")
        self.assertEqual(links, 9)
        self.assertEqual(self.read("docs/strategies/architecture.md"), FILES["docs/strategies/architecture.md"])
        self.assertIn("Tracked files that still name a moved file's old path, left unchanged:\n"
                      "  docs/other.md: docs/strategies/plan.md\n"
                      "  docs/strategies/architecture.md: docs/strategies/plan.md\n", out)

    def test_config_comes_from_the_workflow(self):
        self.convert()
        self.assertEqual(self.read(".atelier/config.toml"), """schema_version = 3
slug = "demo"
base_branch = "main"

[[merge_check]]
run = "git diff --check HEAD^ HEAD -- . ':!.atelier'"

[forge]
provider = "github"
remote = "origin"
ci_workflow = "ci.yml"
""")

    def test_config_names_no_ci_workflow_the_repository_lacks(self):
        (self.repo / ".github/workflows/ci.yml").unlink()
        self.convert()
        self.assertTrue(self.read(".atelier/config.toml").endswith('[forge]\nprovider = "github"\nremote = "origin"\n'))

    def test_the_output_counts_what_was_converted_and_left(self):
        out = self.convert()
        for line in ("  item draft: 3\n", "  item ready: 6\n", "  item from decision ready: 1\n",
                     "  mission draft: 1\n", "  mission ready: 1\n", "  mission done: 1\n", "  task done: 1\n",
                     "  task declined: 1\n", "  feature superseded: 1\n", "  decision done: 1\n",
                     "  reviews files: 1\n", "  evidence files: 1\n",
                     "  blocked_by entries naming a closed or missing record: 1\n",
                     "  docs/strategies/plan.md -> .atelier/missions/demo-m1ss/\n"):
            self.assertIn(line, out)

    def test_an_unreadable_record_stops_it_before_it_writes(self):
        for bad in ('status: "doing"', 'title: "unterminated', "relationships: {a: b}"):
            with self.subTest(bad=bad):
                text = re.sub(r'^status: "todo"$' if bad.startswith("status") else
                              r'^title: .*$' if bad.startswith("title") else r"^relationships:$", bad,
                              RECORDS[3], count=1, flags=re.M)
                self.assertNotEqual(text, RECORDS[3])
                self.write({".atelier/issues/demo-a1ta.md": text})
                result = self.run_script()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("demo-a1ta.md", result.stderr)
                self.assertEqual(git(self.repo, "status", "--porcelain"), " M .atelier/issues/demo-a1ta.md")

    def test_the_same_commit_gives_the_same_bytes_and_verify_checks_them(self):
        self.convert()
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-qm", "convert")
        converted = git(self.repo, "rev-parse", "HEAD")
        git(self.repo, "checkout", "-q", "--detach", self.old)
        self.convert()
        git(self.repo, "add", "-A")
        self.assertEqual(git(self.repo, "diff", "--cached", "--stat", converted), "")
        git(self.repo, "reset", "-q", "--hard")
        git(self.repo, "checkout", "-q", "main")

        result = self.run_script("--verify", self.old)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("match a fresh conversion", result.stdout)

        item = self.repo / ".atelier/items/demo-a1ta/item.md"
        item.write_bytes(item.read_bytes().replace(b"round.", b"round!", 1))
        (self.repo / ".atelier/items/demo-zz99").mkdir()
        (self.repo / ".atelier/items/demo-zz99/item.md").write_text("extra\n")
        (self.repo / ".atelier/missions/demo-m2nd/notes.md").unlink()
        self.write({"docs/strategies/plan.md": STRATEGY, ".atelier/runtime/leftover.json": "{}\n"})  # ignored by Git
        result = self.run_script("--verify", self.old)
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout.splitlines()[1:], [
            "  differs: .atelier/items/demo-a1ta/item.md",
            "  not in a fresh conversion: .atelier/items/demo-zz99/item.md",
            "  missing: .atelier/missions/demo-m2nd/notes.md",
            "  not in a fresh conversion: .atelier/runtime/leftover.json",
            "  still present, though moved: docs/strategies/plan.md",
        ])

    def test_a_done_when_with_no_checkbox_becomes_one_unticked_line(self):
        sketch = "```text\n  a\n    b\n\n- [ ] not a box\n```\n\nThen   more\nprose.\n"
        bodies = {
            "demo-p1pg": "One paragraph\nwrapped   twice.\n",
            "demo-p2pg": "First paragraph.\n\n\tSecond\nparagraph.\n",
            "demo-p3li": "It holds:\n\n- one\n- two\n  wrapped\n1. three\n",
            "demo-p4cb": "Prose before.\n\n- [ ] It is round.\n\nProse beside it.\n",
            "demo-p5fn": "Sketch:\n\n" + sketch,
            "demo-p6tb": "Rows:\n| a | b |\n|---|---|\n| 1 | 2 |\n",
        }
        self.write({f".atelier/issues/{rid}.md": record(rid, "task", "todo", rid, f"\n## Goal\n\ng\n\n## Done when\n\n{b}")
                    for rid, b in bodies.items()})
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-qm", "prose Done whens")
        self.old = git(self.repo, "rev-parse", "HEAD")
        self.convert()
        done_when = {rid: self.body(f".atelier/items/{rid}/item.md").split("## Done when\n\n", 1)[1]
                     for rid in bodies}
        self.assertEqual(done_when, {
            "demo-p1pg": "- [ ] One paragraph wrapped twice.\n",
            "demo-p2pg": "- [ ] First paragraph. Second paragraph.\n",
            "demo-p3li": "- [ ] It holds: - one - two wrapped 1. three\n",
            "demo-p4cb": "Prose before.\n\n- [ ] It is round.\n\nProse beside it.\n",
            "demo-p5fn": "- [ ] Sketch:\n\n" + sketch,
            "demo-p6tb": "- [ ] Rows:\n\n| a | b |\n|---|---|\n| 1 | 2 |\n",
        })
        self.assertIn("## Done when\n\n- [ ] d\n", self.read(".atelier/items/demo-b2bu/item.md"))
        # A section the converter fills in itself is unchanged.
        self.assertIn(f"## Done when\n\n{convert_records.UNSPECIFIED}\n", self.read(".atelier/items/demo-c3fe/item.md"))
        self.assertIn(f"## Done when\n\n{convert_records.DECISION_DONE_WHEN}\n",
                      self.read(".atelier/items/demo-h7de/item.md"))

        # A second run of the same commit gives the same bytes, and --verify passes.
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-qm", "convert")
        converted = git(self.repo, "rev-parse", "HEAD")
        git(self.repo, "checkout", "-q", "--detach", self.old)
        self.convert()
        git(self.repo, "add", "-A")
        self.assertEqual(git(self.repo, "diff", "--cached", "--stat", converted), "")
        git(self.repo, "reset", "-q", "--hard")
        git(self.repo, "checkout", "-q", "main")
        result = self.run_script("--verify", self.old)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_checkbox_done_when_reads_checkbox_lines_as_the_new_binary_does(self):
        same = ["- [x] Ticked.\n", "Prose.\n  * [X] Nested, ticked.\n", "+ [ ]\n", "", "\n\n",
                "```\nonly a fence\n```\n", "| only | a table |\n"]
        for content in same:
            self.assertEqual(convert_records.checkbox_done_when(content), content, content)
        for content in ["- [-] Not a box.\n", "- [ ]x glued.\n", "Prose.\n```\n- [ ] In a fence.\n```\n"]:
            self.assertTrue(convert_records.checkbox_done_when(content).startswith("- [ ] "), content)

    @unittest.skipUnless(NEW_BINARY.exists(), "the new binary is not built")
    def test_the_new_binary_reads_what_it_writes(self):
        self.convert()
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-qm", "convert")
        for command in ("status", "list"):
            result = subprocess.run([str(NEW_BINARY), command], cwd=self.repo, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
