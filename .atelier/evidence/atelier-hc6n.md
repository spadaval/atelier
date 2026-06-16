---
created_at: "2026-06-15T18:37:30.104333239+00:00"
id: "atelier-hc6n"
evidence_type: "validation"
captured_at: "2026-06-15T18:37:29.926127621+00:00"
command: "bash -lc 'set +e; target/debug/atelier finish > /tmp/atelier-finish.out 2> /tmp/atelier-finish.err; finish_status=$?; target/debug/atelier note add issue atelier-missing legacy > /tmp/atelier-note.out 2> /tmp/atelier-note.err; note_status=$?; cat /tmp/atelier-finish.err; cat /tmp/atelier-note.err; test $finish_status -ne 0; test $note_status -ne 0; rg \"unrecognized subcommand .finish.|unrecognized subcommand .note.\" /tmp/atelier-finish.err /tmp/atelier-note.err; ! rg \"was removed|atelier issue close|atelier issue note|atelier status|atelier history\" /tmp/atelier-finish.err /tmp/atelier-note.err'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sclf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sclf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed commands fail as plain Clap unknown or invalid arguments without specialized compatibility guidance"
updated_at: "2026-06-15T18:37:34.265242216+00:00"
---

## Summary

Removed commands fail as plain Clap unknown or invalid arguments without specialized compatibility guidance

## Command

```console
bash -lc 'set +e; target/debug/atelier finish > /tmp/atelier-finish.out 2> /tmp/atelier-finish.err; finish_status=$?; target/debug/atelier note add issue atelier-missing legacy > /tmp/atelier-note.out 2> /tmp/atelier-note.err; note_status=$?; cat /tmp/atelier-finish.err; cat /tmp/atelier-note.err; test $finish_status -ne 0; test $note_status -ne 0; rg "unrecognized subcommand .finish.|unrecognized subcommand .note." /tmp/atelier-finish.err /tmp/atelier-note.err; ! rg "was removed|atelier issue close|atelier issue note|atelier status|atelier history" /tmp/atelier-finish.err /tmp/atelier-note.err'
```

Exit status: 0

## Stdout

Bytes: 440
Truncated: no

```text
error: unrecognized subcommand 'finish'

  tip: a similar subcommand exists: 'init'

Usage: atelier [OPTIONS] <COMMAND>

For more information, try '--help'.
error: unrecognized subcommand 'note'

  tip: a similar subcommand exists: 'worktree'

Usage: atelier [OPTIONS] <COMMAND>

For more information, try '--help'.
/tmp/atelier-note.err:error: unrecognized subcommand 'note'
/tmp/atelier-finish.err:error: unrecognized subcommand 'finish'
```

## Stderr

Bytes: 0
Truncated: no

```text
```
