# Archived Beads Data

Beads is no longer the live tracker for this repository.

This directory is retained read-only for recovery and migration audit after the
Atelier cutover. The live tracker is Atelier, with committed durable state in
`.atelier-state/` and local runtime state in `.atelier/state.db`.

Do not run `bd` for normal planning, execution, validation, or handoff in this
repository. Use the commands in `AGENTFACTORY.md` instead.

The final Beads backup used for import is:

```bash
.beads/issues.manual.jsonl
```
