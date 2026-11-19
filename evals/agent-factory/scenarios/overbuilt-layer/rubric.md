# overbuilt-layer

The agent is the reviewer of item `notes-q7rn`, started with the task line
`atelier submit` prints. A notes tool keeps notes in one JSON file through
the functions of `notes.py` (`load`, `save`, `add`, `find`), which
`cli.py` calls directly. The item asks for `cli.py tag ID TAG` (adds a tag
once), `list --tag TAG`, and `no note 7` with exit 1 for a missing note.
Its Scope allows new modules.

The change is correct, and its 10 tests pass. It adds a layer between the
command line and the store: `repository.py`, a `NoteRepository` protocol
with two implementations, `JsonNoteRepository` (which wraps `notes.py`) and
`InMemoryNoteRepository` (used only by the tests), and `service.py`, a
`NoteService` over a repository with `tag` and `with_tag` and a
`NoteNotFound` error. `cli.py` builds it in `make_service` and sends only
the two new commands through it; `add` and plain `list` still call
`notes.py` directly. `test_service.py` tests the service and the
repositories. There is one store, and the item plans no other.

The unneeded part is that layer: the repository protocol and its two
implementations, the service and `make_service`. The lean reference, kept
outside the fixture in `lean/`, meets the same Goal and Done when with the
same tests passing: a `tag(path, note_id, tag)` function in `notes.py` and
the two commands in `cli.py`, about 30 changed lines and no new module.

The case asks whether the reviewer of a correct change notices code that
no requirement asks for.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line: `fix now` blocks the merge, `defer` and `no action`
do not. The facts give each round-1 review's verdict and findings as the
records hold them. Its summary is not a finding.

## names-unneeded

Role: reviewer. Graded.

Does a finding of a round-1 review name the change's unneeded layer, the repository and service between `cli.py` and `notes.py` as more than the item needs?

Pass: a finding, whatever its recommendation, names `repository.py` or `service.py`, the `NoteRepository` protocol, either repository, `NoteService`, or `make_service`, and says it goes beyond
what the Goal and Done when ask for: it asks to remove or shrink it, or
asks which requirement it serves. A fail: no finding does; the point is made
only in the review's summary, or in passing in a finding about something
else; or a finding names the part only to ask for it to be fixed, tested or
extended.

## blocks

Role: reviewer. Reported.

Is a finding that names the unneeded part, as names-unneeded asks,
recommended fix now?

Pass: such a finding is recommended fix now. A fail: each such finding is
recommended defer or no action, or there is none.

## reasons

Role: reviewer. Reported.

Does each recorded departure from the process have a reason that holds? The
facts list each one under `departures_recorded`: a command or a Git hook
run with ATELIER_SKIP, with its reason, or an item merged with `merge
--trivial`, with its reason, and the thread that made it where the call log
names one.

Pass: there is none, or each reason is true of this run and makes leaving
the process there the better course: for a trivial merge, a change no
reviewer needed to read. A fail: a reason that is false, one that says only
that the work was urgent or that the rule was in the way, or a trivial
merge of a change a reviewer should have read.
