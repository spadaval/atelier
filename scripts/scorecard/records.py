"""Record reader: items, and which of them merged, from `.atelier/` and Git at a repository's manifest commit.

A merged item is one whose record says `done`, closed in the window, that is not a mission, and
whose ID is named by a commit on the manifest commit's history that changes a file outside
`.atelier/`, so its work reached the base and not only its record.
"""
import functools
import re
import subprocess
from dataclasses import dataclass

from snapshot import git

FRONT_FIELD = re.compile(r'^(id|status|issue_type|closed_at): "?([^"\n]*)"?$', re.M)


@dataclass(frozen=True)
class Item:
    id: str
    kind: str
    status: str
    closed_at: str


def items(repo, commit):
    """Every issue record at the commit, by ID, read from Git objects rather than a checkout."""
    paths = [p for p in git(repo, "ls-tree", "-r", "--name-only", commit, ".atelier/issues/").splitlines() if p.endswith(".md")]
    batch = subprocess.run(["git", "-C", str(repo), "cat-file", "--batch"], check=True, capture_output=True,
                           input="".join(f"{commit}:{p}\n" for p in paths).encode()).stdout
    found, pos = {}, 0
    for _ in paths:
        header_end = batch.index(b"\n", pos)
        size = int(batch[pos:header_end].split()[2])
        text = batch[header_end + 1 : header_end + 1 + size].decode()
        pos = header_end + 2 + size
        front = dict(FRONT_FIELD.findall(text.split("\n---", 1)[0]))
        found[front["id"]] = Item(front["id"], front.get("issue_type", ""), front.get("status", ""), front.get("closed_at", ""))
    return found


def merged(repo, commit, all_items, window):
    """Merged items, by ID, with their close time."""
    candidates = {i.id: i.closed_at for i in all_items.values()
                  if i.status == "done" and i.kind != "mission" and i.closed_at and i.closed_at in window}
    pattern = id_pattern(frozenset(candidates))
    named = set()
    log = git(repo, "log", "--diff-merges=first-parent", "--name-only", "--format=%x1e%B%x1f", commit)
    for record in log.split("\x1e")[1:]:
        message, _, files = record.partition("\x1f")
        if any(f and not f.startswith(".atelier/") for f in files.splitlines()):
            named.update(pattern.findall(message))
    return {i: candidates[i] for i in sorted(named)}


@functools.cache
def id_pattern(item_ids):
    """Matches any of the given item IDs as a whole word; the longest wins where IDs overlap."""
    if not item_ids:
        return re.compile(r"(?!)")
    alternatives = "|".join(map(re.escape, sorted(item_ids, key=lambda i: (-len(i), i))))
    return re.compile(r"(?<![A-Za-z0-9])(" + alternatives + r")(?![A-Za-z0-9])")
