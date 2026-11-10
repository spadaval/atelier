"""Git reader: rework, record-only commits and line counts, from a repository's history up to its manifest commit."""
import collections
import datetime
import re
import subprocess
import sys

from snapshot import REPO_ROOT, git, parse_time

REWORK_DAYS = 14
# Not code: tracker records, and lock files a package manager writes.
NOT_CODE = (":(exclude).atelier", ":(exclude,glob)**/*.lock", ":(exclude,glob)**/pnpm-lock.yaml",
            ":(exclude,glob)**/package-lock.json")
LANGUAGES = {"rs": "Rust", "ts": "TypeScript", "tsx": "TypeScript", "kt": "Kotlin", "kts": "Kotlin", "py": "Python",
             "js": "JavaScript", "mjs": "JavaScript", "cjs": "JavaScript", "jsx": "JavaScript", "svelte": "Svelte",
             "go": "Go", "java": "Java", "swift": "Swift"}
HIDDEN = re.compile(r"(?:^|/)\.")  # agent tooling and vendored skills live under dot-directories
TEST_FILE = re.compile(r"(?:^|/)(?:tests?|__tests__|androidTest|testFixtures|e2e|fixtures?)/|[._-](?:test|spec)\.\w+$|Tests?\.kt$")
TEST_RULE = ("a path with a `test`, `tests`, `__tests__`, `androidTest`, `testFixtures`, `e2e`, `fixture` or `fixtures` "
             "directory, a file named `*.test.*`, `*.spec.*`, `*_test.*` or `*-test.*`, or a Kotlin file ending `Test.kt` or `Tests.kt`")
RUST_RULE = "as `scripts/count_non_test_rust.py` counts them: files whose path contains `/tests/`, `src/test_records.rs`, and `#[cfg(test)]` items"
HUNK = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")


def record_only(repo, commit, window):
    """Non-merge commits in the window on the commit's history, and those that change only `.atelier/`."""
    commits = only = 0
    for record in git(repo, "log", "--no-merges", "--no-renames", "--name-only", "--format=%x1e%cI", commit).split("\x1e")[1:]:
        when, _, files = record.partition("\n")
        if when not in window:
            continue
        files = [f for f in files.splitlines() if f]
        commits += 1
        only += bool(files) and all(f.startswith(".atelier/") for f in files)
    return {"commits": commits, "record_only": only, "share": _share(only, commits)}


def rework(repo, commit, window):
    """Lines that reached the base and were deleted from it within REWORK_DAYS, among lines written early enough to watch that long.

    Replays the first-parent history line by line, each line carrying the time its commit reached the base.
    """
    last_written = window.cutoff - datetime.timedelta(days=REWORK_DAYS)
    files, eligible = {}, []
    written = deleted = 0
    path = when = None
    offset = 0  # how far this file's earlier hunks in this commit moved later lines
    pending = 0  # hunk body lines still to skip, so a body line is never read as a header
    proc = subprocess.Popen(["git", "-C", str(repo), "log", "--first-parent", "--reverse", "--diff-merges=first-parent",
                             "-p", "-U0", "--no-renames", "--no-color", "--format=%x1e%H %ct", commit, "--", ".", *NOT_CODE],
                            stdout=subprocess.PIPE, text=True, errors="replace")
    for line in proc.stdout:
        if pending:
            pending -= not line.startswith("\\")
            continue
        if line.startswith("\x1e"):
            sha, seconds = line[1:].split()
            when = parse_time(int(seconds))
            if window.start <= when <= last_written:
                eligible.append(sha)
        elif line.startswith("diff --git "):
            path, offset = None, 0
        elif line.startswith("--- ") and path is None:
            path = line[4:].rstrip("\n").removesuffix("\t")  # Git ends a name holding a space with a tab
            path = path[2:] if path.startswith("a/") else None
        elif line.startswith("+++ "):
            new = line[4:].rstrip("\n").removesuffix("\t")
            path = new[2:] if new.startswith("b/") else path
        elif (hunk := HUNK.match(line)) and path is not None:
            old_start, old_len, _, new_len = (int(g) if g is not None else 1 for g in hunk.groups())
            births = files.setdefault(path, [])
            at = (old_start - 1 if old_len else old_start) + offset
            for born in births[at : at + old_len]:
                if window.start <= born <= last_written and when - born <= datetime.timedelta(days=REWORK_DAYS):
                    deleted += 1
            births[at : at + old_len] = [when] * new_len
            offset += new_len - old_len
            if window.start <= when <= last_written:
                written += new_len
            pending = old_len + new_len
    proc.stdout.close()
    if proc.wait():
        raise subprocess.CalledProcessError(proc.returncode, "git log")
    return {"written_from": window.start.strftime("%Y-%m-%dT%H:%M:%SZ"), "written_until": last_written.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "days": REWORK_DAYS, "eligible_commits": len(eligible), "first_eligible": eligible[0] if eligible else None,
            "last_eligible": eligible[-1] if eligible else None, "lines_written": written,
            "deleted_within_days": deleted, "share": _share(deleted, written)}


def line_count(repo, commit):
    """Non-test lines of the repository's main language: the one with the most non-test bytes outside dot-directories."""
    by_language = collections.Counter()
    for entry in git(repo, "ls-tree", "-r", "-l", commit).splitlines():
        meta, path = entry.split("\t", 1)
        language = LANGUAGES.get(path.rsplit(".", 1)[-1]) if "." in path else None
        if language and meta.split()[1] == "blob" and not HIDDEN.search(path) and not TEST_FILE.search(path):
            by_language[language] += int(meta.split()[3])
    language = min(by_language, key=lambda name: (-by_language[name], name))
    bytes_by_language = {name: by_language[name] for name in sorted(by_language)}
    if language == "Rust":
        out = subprocess.run([sys.executable, str(REPO_ROOT / "scripts/count_non_test_rust.py"), commit],
                             cwd=repo, check=True, capture_output=True, text=True).stdout
        lines = int(re.search(r"^TOTAL: (\d+)$", out, re.M).group(1))
        return {"language": language, "non_test_lines": lines, "tests": RUST_RULE, "non_test_bytes_by_language": bytes_by_language}
    extensions = sorted(ext for ext, name in LANGUAGES.items() if name == language)
    lines = 0
    for entry in git(repo, "grep", "-I", "-c", "-e", "", commit, "--", *(f"*.{e}" for e in extensions)).splitlines():
        path, count = entry[len(commit) + 1 :].rsplit(":", 1)
        if not HIDDEN.search(path) and not TEST_FILE.search(path):
            lines += int(count)
    return {"language": language, "extensions": extensions, "non_test_lines": lines, "tests": TEST_RULE,
            "non_test_bytes_by_language": bytes_by_language}


def _share(part, whole):
    return round(part / whole, 4) if whole else None
