"""Log reader: the acts in a repository's frozen `atelier log`, the window's merged items, and each T3 thread's item.

`freeze.py copy` writes the log to the snapshot as `logs/<repository>.txt`. Its line layout is in
`docs/design.md`, The log: time, commit, record, act and thread separated by single spaces, then
what changed. The scripts never run `atelier`, so a base that moved on, a deleted branch or an
item moved to another mission after the freeze changes nothing they read.
"""
from collections import namedtuple

from snapshot import InputError, check_not_live, git

LOGS = "logs"
Line = namedtuple("Line", "time commit record act thread changed")  # thread is None where the line shows `-`


def read(snapshot, repository, before_cutoff):
    """The log's lines committed by the cutoff, in the log's order."""
    path = check_not_live(snapshot) / LOGS / f"{repository}.txt"
    if not path.is_file():
        raise InputError(f"no frozen log at {path}")
    found = []
    for text in path.read_text().splitlines():
        time, commit, record, act, thread, *changed = text.split(" ", 5)
        if before_cutoff(time):
            found.append(Line(time, commit, record, act, None if thread == "-" else thread, "".join(changed)))
    return found


def t3_thread(identity):
    """The T3 thread an identity names: a Codex subagent's `<T3 thread>/<child>` names its T3 thread."""
    return identity.split("/", 1)[0]


def ties(lines):
    """Each T3 thread's item and role, from the first line that gives it one.

    `start` and an implementer's take-over (`from <former>`) make a thread an implementer of the
    line's record, and `review` and a reviewer's take-over (`<review>; from <former>`) a reviewer.
    """
    found = {}
    for line in lines:
        role = {"start": "implementer", "review": "reviewer",
                "take-over": "implementer" if line.changed.startswith("from ") else "reviewer"}.get(line.act)
        if role and line.thread:
            found.setdefault(t3_thread(line.thread), (line.record, role))
    return found


def merged(lines, repo, window):
    """Items with a `merge` line in the window whose merge commit changes a file outside `.atelier/` from its first parent."""
    return {line.record for line in lines if line.act == "merge" and line.time in window and _changes_code(repo, line.commit)}


def done(lines, missions, window):
    """Items, not missions, made done in the window, by a merge or a close."""
    return {line.record for line in lines if line.act in ("merge", "close") and line.time in window} - missions


def missions(repo, commit):
    """The IDs of the missions recorded at the commit."""
    return {path.rsplit("/", 1)[-1] for path in git(repo, "ls-tree", "--name-only", commit, ".atelier/missions/").split()}


def _changes_code(repo, commit):
    changed = git(repo, "diff", "--no-renames", "--name-only", f"{commit}^1", commit).split("\n")
    return any(path and not path.startswith(".atelier/") for path in changed)
