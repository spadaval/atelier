"""Operations on notes, independent of where they are stored."""
from repository import NoteRepository


class NoteNotFound(LookupError):
    def __init__(self, note_id):
        super().__init__(f"no note {note_id}")
        self.note_id = note_id


class NoteService:
    def __init__(self, repository: NoteRepository):
        self.repository = repository

    def tag(self, note_id, tag):
        """Add `tag` to the note, once; raise NoteNotFound if there is none."""
        note = self.repository.get(note_id)
        if note is None:
            raise NoteNotFound(note_id)
        if tag not in note["tags"]:
            note["tags"].append(tag)
            self.repository.put(note)
        return note

    def with_tag(self, tag):
        return [n for n in self.repository.all() if tag in n["tags"]]
