"""What every reader shares: the manifest, the window, the snapshot directory and Git at a manifest commit.

Readers take their inputs from a snapshot directory given on the command line and refuse
T3's, Claude's and Codex's live stores, so a rerun reads exactly what was frozen.
"""
import datetime
import json
import subprocess
from pathlib import Path

WINDOW_START = "2026-10-14T00:00:00Z"
REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST = REPO_ROOT / "docs/architecture/quality/scorecard-baseline/manifest.json"
LIVE_STORES = (
    Path.home() / ".t3/userdata",
    Path.home() / ".claude/projects",
    Path.home() / ".codex/sessions",
)


class InputError(Exception):
    """An input is missing, live, or not what the scripts were written against."""


def check_not_live(path):
    """Refuse a path inside one of the live stores; return it resolved."""
    resolved = Path(path).expanduser().resolve()
    for live in LIVE_STORES:
        if resolved == live.resolve() or resolved.is_relative_to(live.resolve()):
            raise InputError(f"{path} is inside the live store {live}; give the frozen snapshot directory instead")
    return resolved


def parse_time(text):
    """An aware UTC datetime from an ISO 8601 string (with `Z` or an offset) or epoch seconds."""
    if isinstance(text, (int, float)):
        return datetime.datetime.fromtimestamp(text, datetime.timezone.utc)
    value = datetime.datetime.fromisoformat(text.replace("Z", "+00:00"))
    return value if value.tzinfo else value.replace(tzinfo=datetime.timezone.utc)


class Window:
    """From WINDOW_START to the manifest's cutoff, both inclusive."""

    def __init__(self, start, cutoff):
        self.start = parse_time(start)
        self.cutoff = parse_time(cutoff)

    def __contains__(self, when):
        return self.start <= parse_time(when) <= self.cutoff

    def before_cutoff(self, when):
        return when is not None and parse_time(when) <= self.cutoff


def load_manifest(path=MANIFEST):
    with open(path) as handle:
        return json.load(handle)


def git(repo, *args):
    """Run a read-only Git command in `repo` and return its output."""
    return subprocess.run(["git", "-C", str(repo), *args], check=True, capture_output=True, text=True).stdout
