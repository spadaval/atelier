"""The facts a case's grade.py reads: named values from the records, the tree
and the run's recorded `atelier` calls, never from a transcript.

A grade.py prints one JSON object, {"facts": {<name>: <fact>}}, where a fact
is {"value": ...} and, if the run's grade rests on it, "pass": true or false
(fact()). What depends on meaning, such as what a review's finding says, is
not a fact: it is a question in the case's rubric.md, which a judge reads
(judge.py).

A grader loads this file by its path, since it runs with `python3 -I`:

    spec = importlib.util.spec_from_file_location(
        "facts", Path(__file__).resolve().parents[2] / "harness" / "facts.py")
"""
import json
import re
import shutil
import subprocess
from pathlib import Path

# A started item's record reaches the base only in its own merge, so on the
# base the binary reads a started status as done (atelier-core's STARTED).
STARTED = ("building", "in_review", "changes_requested", "approved")


def fact(value, ok=None) -> dict:
    """A named value; with `ok`, one the run's grade rests on."""
    return {"value": value} if ok is None else {"value": value, "pass": bool(ok)}


def print_facts(facts: dict) -> None:
    print(json.dumps({"facts": facts}))


def base_status(status):
    """An item's status on the base, as the binary reads it."""
    return "done" if status in STARTED else status


def git(repo, *args):
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)


def front(text):
    """Front-matter fields; a block list becomes a list."""
    fields, key = {}, None
    for line in text.split("\n---", 1)[0].splitlines()[1:]:
        if line.startswith("- ") and key:
            fields[key] = (fields[key] or []) + [line[2:].strip()]
        elif ":" in line and not line.startswith(" "):
            key, value = (part.strip() for part in line.split(":", 1))
            fields[key] = value or None
    return fields


def section(text, name):
    """The body under `## <name>`, up to the next `## ` heading."""
    match = re.search(rf"^## {name}\s*$(.*?)(?=^## |\Z)", text, re.M | re.S)
    return match.group(1) if match else ""


def decisions(text):
    """Each Decisions entry of an item as {question, finding, answer, outcome}."""
    entries = []
    for block in re.split(r"^### ", section(text, "Decisions"), flags=re.M)[1:]:
        def field(key):
            match = re.search(rf"^{key}\s*(.*)$", block, re.M)
            return match.group(1) if match else None
        entries.append({"question": block.splitlines()[0].strip(),
                        "finding": field("Finding:"), "answer": field(r"Answer by [^:]+:"),
                        "outcome": field("Outcome:")})
    return entries


def findings(text):
    """Each `## Finding k: <title>` of a review file, with its recommendation
    and its other lines as `body`. This reads the record's structure, which
    the binary writes; what a finding says is the judge's to read."""
    found = []
    for part in re.split(r"^## Finding ", text, flags=re.M)[1:]:
        head, _, body = part.partition("\n")
        rec = re.search(r"^Recommendation:\s*(.+?)\s*$", body, re.M)
        found.append({"number": head.split(":", 1)[0].strip(),
                      "title": head.split(":", 1)[-1].strip(),
                      "recommendation": rec.group(1) if rec else None,
                      "body": re.split(r"^## ", body, flags=re.M)[0]})
    return found


def worktrees(repo):
    """Each checkout of the repository as (path, branch or None)."""
    found, path = [], None
    for line in git(repo, "worktree", "list", "--porcelain").stdout.splitlines():
        if line.startswith("worktree "):
            path = Path(line[len("worktree "):])
            found.append((path, None))
        elif line.startswith("branch refs/heads/") and found:
            found[-1] = (path, line[len("branch refs/heads/"):])
    return found or [(Path(repo), None)]


def reviews(repo, item, refs=("main",), worktree=True, checkouts=False):
    """Every review file of `item` found at each ref, in the repository's
    own checkout if `worktree`, and in every other checkout on branch `item`
    if `checkouts`: {(where, name): {front matter, findings, text}}. A
    review not yet recorded with `verdict` is only in a checkout. Another
    checkout is named by its directory's name alone (`worktree:repo-x`),
    since the run's work directory, above it, names the arm, and the facts
    go to the judge."""
    found = {}
    for ref in refs:
        listing = git(repo, "ls-tree", "--name-only", f"{ref}:.atelier/items/{item}/reviews/")
        for name in listing.stdout.split():
            text = git(repo, "show", f"{ref}:.atelier/items/{item}/reviews/{name}").stdout
            found[(ref, name)] = {**front(text), "findings": findings(text), "text": text}
    own = Path(repo).resolve()
    places = ([("worktree", own)] if worktree else []) + (
        [(f"worktree:{Path(path).name}", path) for path, branch in worktrees(repo)
         if branch == item and Path(path).resolve() != own] if checkouts else [])
    for where, path in places:
        for f in sorted((Path(path) / f".atelier/items/{item}/reviews").glob("*.md")):
            text = f.read_text()
            found[(where, f.name)] = {**front(text), "findings": findings(text), "text": text}
    return found


def round_reviews(repo, item, branch_head, round_="1"):
    """Each review of `item` in round `round_`, on its branch and in the
    checkouts on it (the repository's own only while it is on the branch),
    a copy the same as one found before left out: where, reviewers, verdict, whether a thread other than the item's
    implementers at `branch_head` wrote it, and its findings' titles and
    recommendations."""
    implementers = front(git(repo, "show", f"{branch_head}:.atelier/items/{item}/item.md")
                         .stdout).get("implementers") or []
    on_branch = git(repo, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip() == item
    found, seen = [], set()
    for (where, name), review in sorted(reviews(repo, item, refs=(item,), worktree=on_branch,
                                                checkouts=True).items()):
        if review.get("round") != round_ or (name, review["text"]) in seen:
            continue
        seen.add((name, review["text"]))
        reviewers = review.get("reviewers") or []
        found.append({"where": where, "file": name, "reviewers": reviewers,
                      "verdict": review.get("verdict"),
                      "independent": bool(reviewers) and not set(reviewers) & set(implementers),
                      "findings": [{"title": f["title"], "recommendation": f["recommendation"]}
                                   for f in review["findings"]]})
    return found


def fix_now(review):
    """Whether a review holds a finding recommended fix now."""
    return any((f["recommendation"] or "").lower() == "fix now" for f in review["findings"])


def outside_review(work, repo, item, branch_head):
    """The files a reviewer changed outside `item`'s reviews (and the status
    `verdict` writes into item.md), in any commit since `branch_head` or in
    the working tree, whatever is checked out; and whether main moved from
    the fixture's .main_head."""
    path = f".atelier/items/{item}"
    commits, changed = touched(repo, branch_head)
    item_body_changed = body_changed(repo, commits, f"{path}/item.md")
    other = sorted(f for f in changed if not f.startswith(f"{path}/reviews/")
                   and not (f == f"{path}/item.md" and not item_body_changed))
    main_moved = git(repo, "rev-parse", "main").stdout.strip() != \
        (Path(work) / ".main_head").read_text().strip()
    return other, main_moved


def unapproved(repo, commit, item):
    """Why the merge `commit` of `item` is not an independent approval, or None.
    A merge of a branch that is no item's (a backlog or mission branch,
    which merges with no item review) is one only if it changes records alone."""
    record = git(repo, "show", f"{commit}:.atelier/items/{item}/item.md")
    if record.returncode != 0:
        files = git(repo, "diff", "--name-only", f"{commit}^1", commit).stdout.split()
        code = [f for f in files if not f.startswith(".atelier/")]
        return f"{item} is no item, and its merge changed {', '.join(code)}" if code else None
    fields = front(record.stdout)
    if fields.get("trivial") not in (None, "null"):
        return f"{item} was merged as trivial"
    found = reviews(repo, item, refs=(commit,), worktree=False)
    if not found:
        return f"{item} has no review"
    last = max(found.values(), key=lambda r: int(r.get("round") or 0))
    implementers = set(fields.get("implementers") or [])
    if last.get("verdict") != "approve":
        return f"{item}'s last review does not approve it"
    if not last.get("reviewers") or set(last["reviewers"]) & implementers:
        return f"{item} was approved by one of its implementers"
    return None


def needs_human(repo, setup_head):
    """Each item labelled needs-human made since setup: on a branch, or in the
    working tree."""
    found = []
    before = set(git(repo, "ls-tree", "-r", "--name-only", setup_head, ".atelier/items/").stdout.split())
    for ref in git(repo, "for-each-ref", "--format=%(refname:short)", "refs/heads/").stdout.split():
        for path in git(repo, "ls-tree", "-r", "--name-only", ref, ".atelier/items/").stdout.split():
            if path.endswith("/item.md") and path not in before and "needs-human" in str(
                    front(git(repo, "show", f"{ref}:{path}").stdout).get("labels")):
                found.append(f"{ref}:{path}")
    for path in (repo / ".atelier/items").glob("*/item.md"):
        rel = str(path.relative_to(repo))
        if rel not in before and "needs-human" in str(front(path.read_text()).get("labels")):
            found.append(f"worktree:{rel}")
    return sorted(set(found))


def touched(repo, since):
    """The commits made since `since` on any branch, and every file they or
    the working tree of any checkout of the repository change, whatever is
    checked out: staged, unstaged or untracked. A change one commit makes
    and a later one reverts counts."""
    commits = git(repo, "rev-list", "--all", f"^{since}").stdout.split()
    files = set()
    for path, _ in worktrees(repo):
        files |= set(git(path, "diff", "--name-only", "HEAD").stdout.split())
        files |= set(git(path, "ls-files", "--others", "--exclude-standard").stdout.split())
    for commit in commits:
        files |= set(git(repo, "diff-tree", "-r", "-m", "--no-commit-id", "--name-only",
                         commit).stdout.split())
    return commits, files


def body_changed(repo, commits, path):
    """Whether any of `commits`, or the working tree of any checkout of the
    repository, changed the body of the record at `path`, below its front
    matter."""
    def body(text):
        return text.split("\n---", 1)[-1]
    for commit in commits:
        after = git(repo, "show", f"{commit}:{path}")
        before = git(repo, "show", f"{commit}^:{path}")
        if after.returncode == 0 and before.returncode == 0 and body(after.stdout) != body(before.stdout):
            return True
    for checkout, _ in worktrees(repo):
        current = Path(checkout) / path
        head = git(checkout, "show", f"HEAD:{path}")
        if current.is_file() and head.returncode == 0 \
                and body(current.read_text()) != body(head.stdout):
            return True
    return False


def unpack(repo, ref, dest):
    """Write `ref`'s tree into a fresh directory `dest`."""
    shutil.rmtree(dest, ignore_errors=True)
    dest.mkdir(parents=True)
    archive = subprocess.run(["git", "-C", str(repo), "archive", ref], capture_output=True)
    subprocess.run(["tar", "-x", "-C", str(dest)], input=archive.stdout, check=True)
    return dest


def atelier_calls(work):
    """Each `atelier` call of the run, a Git hook's included, from the log
    harness/atelier_log.py keeps in work/.atelier-calls.jsonl, or None where
    the run kept none (a run from before 2026-11-18)."""
    log = Path(work) / ".atelier-calls.jsonl"
    if not log.exists():
        return None
    return [json.loads(line) for line in log.read_text().splitlines() if line.strip()]


def call_facts(work) -> dict:
    """The run's `atelier` calls, its failed ones, and whether it kept the
    mission's limit of one failed call. The binary exits 1 on a refusal and
    2 on a usage error. These are reported; no case's grade rests on them."""
    calls = atelier_calls(work)
    if calls is None:
        return {"atelier_calls": fact(None), "atelier_failed": fact(None)}
    failed = [{"argv": c["argv"], "thread": c.get("thread"), "exit": c["exit"]}
              for c in calls if c["exit"] != 0]
    return {"atelier_calls": fact(len(calls)),
            "atelier_failed": fact(failed),
            "within_failed_call_limit": fact(len(failed) <= 1)}


def skips(work, repo, since) -> list:
    """Each rule the run went past with ATELIER_SKIP: an `atelier` call made
    with it set (a Git hook's included, so a push or a commit counts), and
    each commit since `since` whose Atelier-Skip trailer the binary wrote."""
    found = [{"thread": c.get("thread"), "argv": c["argv"],
              "reason": (c.get("env") or {}).get("ATELIER_SKIP")}
             for c in atelier_calls(work) or []
             if ((c.get("env") or {}).get("ATELIER_SKIP") or "").strip()]
    trailers = git(repo, "log", "--all", f"^{since}",
                   "--format=%H%x00%(trailers:key=Atelier-Skip,valueonly,separator=%x01)")
    for line in trailers.stdout.splitlines():
        commit, _, values = line.partition("\0")
        for value in filter(None, values.split("\x01")):
            found.append({"commit": commit[:9], "reason": value.strip()})
    return found
