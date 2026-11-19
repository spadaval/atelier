"""Where notes are stored, behind one interface.

NoteRepository is what the note service needs from a store. The JSON file
store implements it over notes.py; InMemoryNoteRepository keeps notes in a
dict, for tests and for any caller that wants no file.
"""
import copy
from typing import Iterable, Optional, Protocol

import notes


class NoteRepository(Protocol):
    def all(self) -> list: ...

    def get(self, note_id: int) -> Optional[dict]: ...

    def put(self, note: dict) -> None: ...


class JsonNoteRepository:
    """Notes in the JSON file at `path`, read and written through notes.py."""

    def __init__(self, path):
        self.path = path

    def all(self):
        return notes.load(self.path)

    def get(self, note_id):
        return notes.find(notes.load(self.path), note_id)

    def put(self, note):
        stored = notes.load(self.path)
        for i, existing in enumerate(stored):
            if existing["id"] == note["id"]:
                stored[i] = note
                break
        else:
            stored.append(note)
        notes.save(self.path, stored)


class InMemoryNoteRepository:
    """Notes in a dict, keyed by id."""

    def __init__(self, initial: Iterable[dict] = ()):
        self._notes = {n["id"]: copy.deepcopy(n) for n in initial}

    def all(self):
        return [copy.deepcopy(n) for _, n in sorted(self._notes.items())]

    def get(self, note_id):
        note = self._notes.get(note_id)
        return copy.deepcopy(note)

    def put(self, note):
        self._notes[note["id"]] = copy.deepcopy(note)
