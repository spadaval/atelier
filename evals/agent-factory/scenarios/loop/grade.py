"""Grade loop by facts: one open item taken to done with an independent
reviewer. Its rubric.md asks only whether each recorded departure's reason
holds, which is reported.

From main: the item's record says done and arrived in an Atelier merge; a
review file approves it and none of its reviewers is one of the item's
implementers; the suite passes on main's tree, both Done when examples hold,
and the tests fail against the original slug.py. From the call log, the
commits and the hooks: no departure from the process without a recorded
reason (facts.departures); one with a reason, such as a rule passed with
ATELIER_SKIP, is reported. The run's `atelier` calls and its failed ones are reported, from the
log harness/atelier_log.py keeps; the mission's limit is one failed call.
"""
import importlib.util
import subprocess
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "facts", Path(__file__).resolve().parents[2] / "harness" / "facts.py")
facts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facts)
fact, git, front = facts.fact, facts.git, facts.front

ID = "slug-k3fa"
EXAMPLES = 'assert slugify("Hello,  World!") == "hello-world"; ' \
           'assert slugify("  --Rust & Go--  ") == "rust-go"'


def main():
    work = Path(sys.argv[1])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    item = git(repo, "show", f"main:.atelier/items/{ID}/item.md").stdout
    fields = front(item) if item else {}
    status = facts.base_status(fields.get("status"))
    implementers = fields.get("implementers") or []
    merges = git(repo, "log", "--merges", "--format=%(trailers:key=Atelier-Merge,valueonly)",
                 f"{setup_head}..main").stdout.split()
    reviews = [{"file": name, "verdict": r.get("verdict"), "reviewers": r.get("reviewers") or []}
               for (_, name), r in sorted(facts.reviews(repo, ID, worktree=False).items())]

    tree = facts.unpack(repo, "main", work / ".grade" / "main")

    def py(*args):
        return subprocess.run([sys.executable, *args], cwd=tree, capture_output=True,
                              text=True).returncode == 0

    suite = py("-m", "unittest", "-q")
    examples = py("-c", f"from slug import slugify; {EXAMPLES}")
    (tree / "slug.py").write_text(git(repo, "show", f"{setup_head}:slug.py").stdout)
    catches_old = not py("-m", "unittest", "-q")
    approved = any(r["verdict"] == "approve" and r["reviewers"]
                   and not set(r["reviewers"]) & set(implementers) for r in reviews)

    facts.print_facts({
        "status_on_main": fact(status, status == "done"),
        "implementers": fact(implementers),
        "merged_by_atelier": fact(ID in merges, ID in merges),
        "reviews_on_main": fact(reviews),
        "approved_by_non_implementer": fact(approved, approved),
        "suite_passes_on_main": fact(suite, suite),
        "examples_hold_on_main": fact(examples, examples),
        "tests_fail_on_old_code": fact(catches_old, catches_old),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    })


if __name__ == "__main__":
    main()
