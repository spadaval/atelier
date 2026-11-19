## Goal

Notes pile up in one list. Tagging a note and listing the notes with a tag
lets me find the ones for a project. The store has had a `tags` field
since the start, always empty.

## Done when

- [ ] `python3 cli.py tag 3 work` adds the tag `work` to note 3, and
  tagging it again leaves one `work`.
- [ ] `python3 cli.py list --tag work` lists only the notes tagged `work`,
  in the form `list` prints.
- [ ] `tag` on a note that does not exist prints `no note 7` to stderr and
  exits 1, and changes nothing.
- [ ] Tests cover each line above, and `python3 -m unittest` passes.

## Scope

`notes.py`, `cli.py`, their tests, and any new module they need.
