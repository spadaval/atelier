"""Notes kept in one JSON file: a list of {"id", "text", "tags"}."""
import json
from pathlib import Path


def load(path):
    path = Path(path)
    return json.loads(path.read_text()) if path.exists() else []


def save(path, notes):
    Path(path).write_text(json.dumps(notes, indent=2) + "\n")


def add(path, text):
    """Add a note and return its id."""
    notes = load(path)
    note_id = max((n["id"] for n in notes), default=0) + 1
    notes.append({"id": note_id, "text": text, "tags": []})
    save(path, notes)
    return note_id


def find(notes, note_id):
    """The note with `note_id`, or None."""
    return next((n for n in notes if n["id"] == note_id), None)
