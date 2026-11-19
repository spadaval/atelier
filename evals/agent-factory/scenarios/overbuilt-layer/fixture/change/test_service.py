import tempfile
import unittest
from pathlib import Path

import notes
from repository import InMemoryNoteRepository, JsonNoteRepository
from service import NoteNotFound, NoteService

NOTES = [{"id": 1, "text": "buy milk", "tags": []},
         {"id": 2, "text": "call Ana", "tags": ["home"]}]


class ServiceTest(unittest.TestCase):
    def setUp(self):
        self.service = NoteService(InMemoryNoteRepository(NOTES))

    def test_tag(self):
        self.assertEqual(self.service.tag(1, "work")["tags"], ["work"])
        self.assertEqual(self.service.repository.get(1)["tags"], ["work"])

    def test_tag_twice_keeps_one(self):
        self.service.tag(2, "home")
        self.assertEqual(self.service.repository.get(2)["tags"], ["home"])

    def test_tag_a_missing_note(self):
        with self.assertRaises(NoteNotFound) as caught:
            self.service.tag(9, "work")
        self.assertEqual(str(caught.exception), "no note 9")

    def test_with_tag(self):
        self.assertEqual([n["id"] for n in self.service.with_tag("home")], [2])


class RepositoryTest(unittest.TestCase):
    def test_in_memory_returns_copies(self):
        repo = InMemoryNoteRepository(NOTES)
        repo.get(1)["tags"].append("x")
        self.assertEqual(repo.get(1)["tags"], [])

    def test_json_put_replaces_and_appends(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "notes.json"
            notes.save(path, NOTES)
            repo = JsonNoteRepository(path)
            repo.put({"id": 1, "text": "buy oat milk", "tags": []})
            repo.put({"id": 3, "text": "new", "tags": []})
            self.assertEqual([n["text"] for n in repo.all()],
                             ["buy oat milk", "call Ana", "new"])


if __name__ == "__main__":
    unittest.main()
