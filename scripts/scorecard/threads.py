"""Thread reader: each thread's repository, role, item and tool calls, from the snapshot's copy of T3's database.

The database is opened read-only and immutable, so no journal, `-wal` or `-shm` file appears
beside the copy. T3's format is internal (`user_version` 0), so the reader recognises the schema
by the tables and columns it reads and refuses any other.
"""
import json
import re
import sqlite3
from dataclasses import dataclass, field

from records import id_pattern
from snapshot import InputError, check_not_live

DB_PATH = "t3/statev2.sqlite"
SCHEMA = "T3 statev2 with orchestration_v2 projections, as copied on 2026-11-10 (manifest schema_sha256 ebd75cd5...)"
REQUIRED_COLUMNS = {
    "projection_projects": {"project_id", "workspace_root"},
    "orchestration_v2_projection_threads": {"thread_id", "project_id", "title", "created_at", "payload_json"},
    "orchestration_v2_projection_turn_items": {"thread_id", "turn_item_id", "ordinal", "type", "status", "payload_json"},
    "orchestration_v2_projection_provider_threads": {"thread_id", "payload_json"},
}
ROLE = re.compile(r"\s*Act as the ([\w-]+) sub-agent")
TOOL_CALLS = ("command_execution", "dynamic_tool", "file_change", "file_search", "web_search", "subagent")


@dataclass
class Thread:
    id: str
    repository: str
    title: str
    branch: str
    created_at: str
    first_message: str
    role: str  # from "Act as the <role> sub-agent", else "unlabelled"
    sessions: list = field(default_factory=list)  # (kind, native id) naming its transcripts


@dataclass
class Call:
    kind: str  # "shell", "edit" or "tool"
    started_at: str
    command: str = ""
    output: str = ""
    path: str = ""


def open_db(snapshot):
    path = check_not_live(snapshot) / DB_PATH
    if not path.is_file():
        raise InputError(f"no T3 database at {path}")
    db = sqlite3.connect(f"{path.as_uri()}?mode=ro&immutable=1", uri=True)
    missing = []
    for table, columns in sorted(REQUIRED_COLUMNS.items()):
        have = {row[1] for row in db.execute(f"PRAGMA table_info({table})")}
        missing += [f"{table}.{column}" for column in sorted(columns - have)]
    if missing:
        db.close()
        expected = "; ".join(f"{t}({', '.join(sorted(c))})" for t, c in sorted(REQUIRED_COLUMNS.items()))
        raise InputError(
            f"{path} is not the T3 schema these scripts were written against: {SCHEMA}. "
            f"Expected {expected}. Missing: {', '.join(missing)}"
        )
    return db


def threads(db, repository, cutoff):
    """The repository's threads created by the cutoff, sorted by id.

    A thread's sessions name its transcripts: ("claudeAgent" or "codex", native session id) for a
    thread T3 ran, and ("claudeSubagent", agent id) for a subagent Claude's Agent tool started.
    """
    projects = [pid for pid, root in db.execute("SELECT project_id, workspace_root FROM projection_projects")
                if (root or "").rstrip("/").rsplit("/", 1)[-1] == repository]
    spawned_by = {}  # child thread -> the agent id Claude gave it
    for (payload,) in db.execute("SELECT payload_json FROM orchestration_v2_projection_turn_items WHERE type = 'subagent'"):
        item = json.loads(payload)
        ref = item.get("nativeItemRef") or {}
        if item.get("childThreadId") and ref.get("nativeId"):
            spawned_by[item["childThreadId"]] = ref["nativeId"]
    found = []
    rows = db.execute(
        f"SELECT thread_id, title, created_at, payload_json FROM orchestration_v2_projection_threads "
        f"WHERE project_id IN ({','.join('?' * len(projects))}) ORDER BY thread_id", projects)
    for thread_id, title, created_at, payload in rows:
        if not cutoff(created_at):
            continue
        meta = json.loads(payload)
        first = db.execute(
            "SELECT payload_json FROM orchestration_v2_projection_turn_items WHERE thread_id = ? AND type = 'user_message' "
            "ORDER BY ordinal, turn_item_id LIMIT 1", (thread_id,)).fetchone()
        first_message = (json.loads(first[0]).get("text") or "") if first else ""
        role = ROLE.match(first_message)
        sessions = {((ref := json.loads(p).get("nativeThreadRef") or {}).get("driver"), ref.get("nativeId"))
                    for (p,) in db.execute("SELECT payload_json FROM orchestration_v2_projection_provider_threads WHERE thread_id = ?", (thread_id,))}
        if thread_id in spawned_by:
            sessions.add(("claudeSubagent", spawned_by[thread_id]))
        found.append(Thread(thread_id, repository, title or "", meta.get("branch") or "", created_at, first_message,
                            role.group(1) if role else "unlabelled", sorted(s for s in sessions if all(s))))
    return found


def calls(db, thread_id, cutoff):
    """The thread's tool calls in order, up to the cutoff: shell commands, edits and other tools."""
    rows = db.execute(
        f"SELECT type, payload_json FROM orchestration_v2_projection_turn_items WHERE thread_id = ? "
        f"AND type IN ({','.join('?' * len(TOOL_CALLS))}) ORDER BY ordinal, turn_item_id", (thread_id, *TOOL_CALLS))
    for kind, payload in rows:
        item = json.loads(payload)
        started = item.get("startedAt") or item.get("updatedAt")
        if not cutoff(started):
            continue
        if kind == "command_execution":
            yield Call("shell", started, item.get("input") or "", item.get("output") or "")
        elif kind == "file_change":
            yield Call("edit", started, path=item.get("fileName") or "")
        else:
            yield Call("tool", started)


def tie(thread, item_ids):
    """The item a thread worked on: the first known item ID in its branch, else its title, else its first message.

    Returns (item ID, source) or (None, None).
    """
    pattern = id_pattern(frozenset(item_ids))
    for source, text in (("branch", thread.branch), ("title", thread.title), ("first_message", thread.first_message)):
        match = pattern.search(text or "")
        if match:
            return match.group(1), source
    return None, None
