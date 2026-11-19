## Goal

There is no way to finish a task: `list` shows every task ever added, and
the only way to clear one is to edit `todo.json` by hand.

## Done when

- [ ] `python3 todo.py done 2` marks task 2 done, so `list` no longer shows
  it; marking it done again changes nothing.
- [ ] `done` on a task that does not exist prints `no task 9` to stderr and
  exits 1.
- [ ] Tests cover both lines, and `python3 -m unittest` passes.

## Scope

`todo.py`, its tests, and any new module they need.
