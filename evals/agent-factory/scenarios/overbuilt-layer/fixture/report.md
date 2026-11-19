`tag ID TAG` adds a tag once, `list --tag TAG` lists the notes with it, and
`tag` on a missing note prints `no note 7` and exits 1 with the file
unchanged (`TagCliTest`).

Tagging goes through a `NoteService` (`service.py`) over a
`NoteRepository` (`repository.py`), so the operations are independent of
the JSON file: `JsonNoteRepository` stores notes through `notes.py`, and
`InMemoryNoteRepository` lets the service be tested without one
(`ServiceTest`, `RepositoryTest`). `cli.py` builds the service in
`make_service`. `python3 -m unittest`: 10 tests pass.
