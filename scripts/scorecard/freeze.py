#!/usr/bin/env python3
"""Freeze the scorecard baseline's inputs into a private snapshot, once.

  freeze.py copy    take the cutoff and base commits, write each repository's
                    `atelier log`, back up T3's database, copy transcripts and
                    sessions, write the manifest's header
  freeze.py hash    add every snapshot file's size and SHA-256 to the manifest
  freeze.py verify  check the snapshot against the manifest, and its modes

Run `copy` and `hash` under `nice -n 19 ionice -c3`. `copy` refuses if the
snapshot exists, so T3's live database is backed up only once.
"""
import datetime, hashlib, json, os, re, shutil, sqlite3, stat, subprocess, sys
from pathlib import Path

HOME = Path.home()
SNAP = HOME / ".local/share/atelier-baseline/2026-11"
MANIFEST = Path(__file__).resolve().parents[2] / "docs/architecture/quality/scorecard-baseline/manifest.json"
T3_DB = HOME / ".t3/userdata/statev2.sqlite"
CLAUDE = HOME / ".claude/projects"
CODEX = HOME / ".codex/sessions"
CODEX_FROM = "2026-10-14"
PROJECTS = HOME / "projects"
REPOS = ["atelier", "astromech", "demiurge", "life-on-rails", "cosplay-studio"]
DB_COPY = "t3/statev2.sqlite"


def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def git(repo, *args):
    return subprocess.run(["git", "-C", str(repo), *args], check=True, capture_output=True, text=True).stdout.strip()


def base_commit(repo):
    """The default branch's tip; the remote-tracking ref if it is strictly ahead."""
    branch = git(repo, "symbolic-ref", "--short", "refs/remotes/origin/HEAD").split("/", 1)[1]
    local = git(repo, "rev-parse", f"refs/heads/{branch}")
    remote = git(repo, "rev-parse", f"refs/remotes/origin/{branch}")
    ahead = subprocess.run(["git", "-C", str(repo), "merge-base", "--is-ancestor", local, remote]).returncode == 0
    if local != remote and not ahead:
        subprocess.run(["git", "-C", str(repo), "merge-base", "--is-ancestor", remote, local], check=True)
    return {"branch": branch, "commit": remote if ahead else local, "local": local, "origin": remote}


def checkouts(repo):
    """Every checkout of a repository: its worktrees, sibling clones, and T3's worktree root."""
    paths = {Path(l[9:]) for l in git(PROJECTS / repo, "worktree", "list", "--porcelain").splitlines() if l.startswith("worktree ")}
    root = git(PROJECTS / repo, "rev-list", "--max-parents=0", "HEAD").split()
    for sib in PROJECTS.iterdir():
        if (sib / ".git").exists() and subprocess.run(["git", "-C", str(sib), "rev-list", "--max-parents=0", "HEAD"], capture_output=True, text=True).stdout.split() == root:
            paths.add(sib)
    return paths | {HOME / ".t3/worktrees" / repo}


def claude_dirs():
    """Transcript directories whose name holds a checkout's encoded path (`/` and `.` become `-`)."""
    encoded = {re.sub(r"[^A-Za-z0-9]", "-", str(p)): (repo, str(p)) for repo in REPOS for p in checkouts(repo)}
    matched = {}
    for d in sorted(CLAUDE.iterdir()):
        hits = [(len(e), v) for e, v in encoded.items() if re.search(re.escape(e) + "(-|$)", d.name)]
        if d.is_dir() and hits:
            repo, checkout = max(hits)[1]
            matched[d.name] = {"repository": repo, "checkout": checkout}
    return matched


def backup_db(dest):
    """SQLite's online backup inside one read transaction, so writes by T3 cannot restart it."""
    script = f'BEGIN;\nSELECT count(*) FROM sqlite_schema;\n.backup "{dest}"\nCOMMIT;\n'
    subprocess.run(["sqlite3", "-bail", f"file:{T3_DB}?mode=ro"], input=script, text=True, check=True, stdout=subprocess.DEVNULL)
    db = sqlite3.connect(dest)  # the copy only: readers then open it read-only with no -wal/-shm
    db.execute("PRAGMA journal_mode=DELETE")
    db.close()


def copy_file(src, dst, vanished):
    dst.parent.mkdir(parents=True, exist_ok=True)
    try:
        shutil.copy2(src, dst)
    except FileNotFoundError:  # deleted by its owner after the listing
        vanished.append(str(src.relative_to(HOME)))
        return
    os.chmod(dst, 0o600)


def copy():
    if SNAP.exists():
        sys.exit(f"{SNAP} exists; the live database is backed up only once")
    m = {"snapshot": "~/" + str(SNAP.relative_to(HOME)), "cutoff": now()}
    m["repositories"] = {r: base_commit(PROJECTS / r) for r in REPOS}
    os.umask(0o077)
    SNAP.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    SNAP.mkdir(mode=0o700)
    m["cutoff_means"] = (
        "Base commits were read at the cutoff and every copy started after it. The database copy is one "
        "consistent read snapshot taken between its part's started and finished times. Transcript and session "
        "files were still being appended while they were copied, so a copy may hold records written after the "
        "cutoff, up to its part's finished time; readers needing a strict window drop records time-stamped after "
        "the cutoff. Sizes and hashes describe the copies, not the sources.")
    parts = m["parts"] = {}
    started = now()
    (SNAP / "logs").mkdir(mode=0o700)
    for r in REPOS:  # each repository's whole log; readers drop lines committed after the cutoff
        (SNAP / "logs" / f"{r}.txt").write_text(subprocess.run(["atelier", "log"], cwd=PROJECTS / r, check=True, capture_output=True, text=True).stdout)
    parts["logs"] = {"command": "atelier log", "path": "logs/<repository>.txt", "started": started, "finished": now()}
    (SNAP / DB_COPY).parent.mkdir(mode=0o700)
    started = now()
    backup_db(SNAP / DB_COPY)
    os.chmod(SNAP / DB_COPY, 0o400)
    parts["t3_database"] = {"source": "~/.t3/userdata/statev2.sqlite", "path": DB_COPY, "started": started, "finished": now(),
                            "method": "sqlite3 CLI .backup of the source opened read-only (mode=ro), inside one read "
                                      "transaction; the copy then set to journal_mode=DELETE. Open it with ?mode=ro."}
    started, dirs, vanished = now(), claude_dirs(), []
    for name in dirs:
        for src in sorted((CLAUDE / name).rglob("*")):
            if src.is_file() and not src.is_symlink() and src.relative_to(CLAUDE / name).parts[0] != "memory":
                copy_file(src, SNAP / "claude-projects" / name / src.relative_to(CLAUDE / name), vanished)
    parts["claude_transcripts"] = {"source": "~/.claude/projects", "path": "claude-projects", "started": started,
                                   "finished": now(), "excluded": "memory/ (Claude's notes, not transcripts)",
                                   "directories": dirs}
    started = now()
    for src in sorted(CODEX.rglob("*")):
        day = "-".join(src.relative_to(CODEX).parts[:3])
        if src.is_file() and len(src.relative_to(CODEX).parts) == 4 and day >= CODEX_FROM:
            copy_file(src, SNAP / "codex-sessions" / src.relative_to(CODEX), vanished)
    parts["codex_sessions"] = {"source": "~/.codex/sessions", "path": "codex-sessions", "from_date": CODEX_FROM,
                               "started": started, "finished": now()}
    m["vanished_during_copy"] = vanished
    write(m)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def snapshot_files():
    return {str(p.relative_to(SNAP)): p for p in sorted(SNAP.rglob("*")) if not p.is_dir()}


def hash_():
    m = json.loads(MANIFEST.read_text())
    with sqlite3.connect(f"file:{SNAP / DB_COPY}?mode=ro", uri=True) as db:
        sql = "\n".join(r[0] for r in db.execute("SELECT sql FROM sqlite_schema WHERE sql IS NOT NULL ORDER BY type, name"))
        m["parts"]["t3_database"]["user_version"] = db.execute("PRAGMA user_version").fetchone()[0]
        m["parts"]["t3_database"]["schema_sha256"] = hashlib.sha256(sql.encode()).hexdigest()
    m["files"] = [{"path": k, "size": p.stat().st_size, "sha256": sha256(p)} for k, p in snapshot_files().items()]
    write(m)


def write(m):
    files = m.pop("files", None)
    text = json.dumps(m, indent=2)
    if files is not None:  # one line per file keeps the manifest readable and its diffs small
        rows = ",\n".join("    " + json.dumps(f) for f in files)
        text = text[:-2] + ',\n  "files": [\n' + rows + "\n  ]\n}"
    MANIFEST.parent.mkdir(parents=True, exist_ok=True)
    MANIFEST.write_text(text + "\n")


def verify():
    files = {f["path"]: f for f in json.loads(MANIFEST.read_text())["files"]}
    found = snapshot_files()
    bad = [f"missing: {k}" for k in files.keys() - found.keys()] + [f"unlisted: {k}" for k in found.keys() - files.keys()]
    for k in files.keys() & found.keys():
        if found[k].stat().st_size != files[k]["size"] or sha256(found[k]) != files[k]["sha256"]:
            bad.append(f"changed: {k}")
    for p in [SNAP.parent, SNAP, *SNAP.rglob("*")]:
        mode = stat.S_IMODE(p.lstat().st_mode)
        if p.is_symlink() or mode & 0o077 or (p.is_dir() and mode != 0o700):
            bad.append(f"mode {oct(mode)}: {p}")
    print("\n".join(sorted(bad)) or f"ok: {len(files)} files match the manifest; modes are owner-only")
    return 1 if bad else 0


if __name__ == "__main__":
    cmd = sys.argv[1:] and sys.argv[1]
    sys.exit({"copy": copy, "hash": hash_, "verify": verify}.get(cmd, lambda: print(__doc__) or 2)())
