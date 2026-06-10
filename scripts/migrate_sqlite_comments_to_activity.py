#!/usr/bin/env python3
"""Migrate legacy SQLite issue comments into canonical activity sidecars."""

from __future__ import annotations

import argparse
import datetime as dt
import sqlite3
import sys
from pathlib import Path


EVENT_TYPES = {
    "note": "note",
    "handoff": "handoff",
    "decision": "decision",
    "plan": "plan",
    "resolution": "close_reason",
    "close-reason": "close_reason",
}


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Migrate .atelier/state.db comments into .atelier-state issue activity sidecars."
    )
    parser.add_argument(
        "--repo",
        default=".",
        help="Repository root containing .atelier/ and .atelier-state/ (default: current directory)",
    )
    parser.add_argument("--dry-run", action="store_true", help="Report without writing files")
    args = parser.parse_args()

    repo = Path(args.repo).resolve()
    db_path = repo / ".atelier" / "state.db"
    state_dir = repo / ".atelier-state"
    if not db_path.is_file():
        print(f"missing SQLite database: {db_path}", file=sys.stderr)
        return 2
    if not state_dir.is_dir():
        print(f"missing canonical state directory: {state_dir}", file=sys.stderr)
        return 2

    with sqlite3.connect(db_path) as conn:
        conn.row_factory = sqlite3.Row
        rows = conn.execute(
            """
            SELECT c.id, c.issue_id, c.content, c.created_at, COALESCE(c.kind, 'note') AS kind
            FROM comments c
            JOIN issues i ON i.id = c.issue_id
            ORDER BY c.created_at, c.id
            """
        ).fetchall()

    planned = skipped = written = 0
    for row in rows:
        activity = activity_from_comment(row)
        issue_dir = state_dir / "issues" / f"{activity['subject_id']}.activity"
        if equivalent_activity_exists(issue_dir, activity):
            skipped += 1
            continue
        activity_id = allocate_activity_id(issue_dir, timestamp_id(activity["created_at"]))
        path = issue_dir / f"{activity_id}.md"
        planned += 1
        if args.dry_run:
            print(f"would write {path.relative_to(repo)}")
            continue
        issue_dir.mkdir(parents=True, exist_ok=True)
        path.write_text(render_activity(activity_id, activity), encoding="utf-8")
        written += 1

    print(
        f"comments scanned: {len(rows)}; planned: {planned}; written: {written}; skipped: {skipped}; dry_run: {args.dry_run}"
    )
    return 0


def activity_from_comment(row: sqlite3.Row) -> dict[str, str | dt.datetime]:
    kind = row["kind"] or "note"
    event_type = EVENT_TYPES.get(kind, "comment")
    body = row["content"] or ""
    if event_type == "close_reason":
        body = body.removeprefix("Close reason:").strip()
    created_at = parse_timestamp(row["created_at"])
    return {
        "subject_id": row["issue_id"],
        "event_type": event_type,
        "actor": "sqlite-migration",
        "created_at": created_at,
        "summary": summary_for(event_type),
        "body": body.replace("\r\n", "\n").replace("\r", "\n"),
    }


def parse_timestamp(value: str) -> dt.datetime:
    normalized = value.replace("Z", "+00:00")
    if "." in normalized:
        head, tail = normalized.split(".", 1)
        offset = ""
        for marker in ("+", "-"):
            if marker in tail:
                fraction, offset_part = tail.split(marker, 1)
                offset = marker + offset_part
                break
        else:
            fraction = tail
        normalized = f"{head}.{fraction[:6]:0<6}{offset}"
    parsed = dt.datetime.fromisoformat(normalized)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def timestamp_id(value: dt.datetime) -> str:
    return value.strftime("%Y%m%dT%H%M%S") + f"{value.microsecond:06d}Z"


def allocate_activity_id(issue_dir: Path, base: str) -> str:
    for suffix in range(100):
        candidate = base if suffix == 0 else f"{base}-{suffix:02d}"
        if not (issue_dir / f"{candidate}.md").exists():
            return candidate
    raise RuntimeError(f"no available activity id for {issue_dir.name} at {base}")


def equivalent_activity_exists(issue_dir: Path, activity: dict[str, str | dt.datetime]) -> bool:
    if not issue_dir.is_dir():
        return False
    created = activity["created_at"].isoformat().replace("+00:00", "Z")
    for path in issue_dir.glob("*.md"):
        text = path.read_text(encoding="utf-8")
        if (
            f"event_type: {yaml_string(str(activity['event_type']))}\n" in text
            and f"created_at: {yaml_string(created)}\n" in text
            and f"\n{str(activity['body']).rstrip()}\n" in text
        ):
            return True
    return False


def summary_for(event_type: str) -> str:
    return {
        "close_reason": "Migrated close reason",
        "handoff": "Migrated handoff",
        "decision": "Migrated decision",
        "plan": "Migrated plan",
        "note": "Migrated note",
    }.get(event_type, "Migrated comment")


def render_activity(activity_id: str, activity: dict[str, str | dt.datetime]) -> str:
    created = activity["created_at"].isoformat().replace("+00:00", "Z")
    body = str(activity["body"]).rstrip()
    return (
        "---\n"
        'schema: "atelier.activity"\n'
        "schema_version: 1\n"
        f"id: {yaml_string(activity_id)}\n"
        'subject_kind: "issue"\n'
        f"subject_id: {yaml_string(str(activity['subject_id']))}\n"
        f"event_type: {yaml_string(str(activity['event_type']))}\n"
        f"actor: {yaml_string(str(activity['actor']))}\n"
        f"created_at: {yaml_string(created)}\n"
        f"summary: {yaml_string(str(activity['summary']))}\n"
        "---\n\n"
        f"{body}\n"
    )


def yaml_string(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


if __name__ == "__main__":
    raise SystemExit(main())
