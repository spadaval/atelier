---
created_at: "2026-10-17T15:48:11.253810649+00:00"
id: "atelier-o9vx"
evidence_type: "test"
commit: "54c4f3b11cceaccbe7cafae9ea57edecbaa6ddcc"
result: "pass"
captured_at: "2026-10-17T15:47:27.682570063+00:00"
command: "/home/supadava/.cache/qymu-trial.sh"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-qymu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qymu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Every merge_checks command passes run with sh -c in a detached checkout of the branch head, as merge runs them, and leaves the checkout clean"
updated_at: "2026-10-17T15:48:11.254081544+00:00"
---

## Summary

Every merge_checks command passes run with sh -c in a detached checkout of the branch head, as merge runs them, and leaves the checkout clean

## Command

```console
/home/supadava/.cache/qymu-trial.sh
```

Exit status: 0

## Stdout

Bytes: 713
Truncated: no

```text
exit 0: git diff --check HEAD^ HEAD -- . ':!.atelier'
exit 0: cargo fmt -- --check
exit 0: cargo metadata --locked --format-version 1 > /dev/null
exit 0: c=${XDG_CACHE_HOME:-$HOME/.cache}; mkdir -p "$c" && t=$(mktemp -d "$c/atelier-merge-XXXXXX") || exit 1; TMPDIR="$t" cargo nextest run; r=$?; rm -rf "$t"; exit $r
     Summary [  10.901s] 757 tests run: 757 passed, 0 skipped
exit 0: c=${XDG_CACHE_HOME:-$HOME/.cache}; mkdir -p "$c" && t=$(mktemp -d "$c/atelier-merge-XXXXXX") || exit 1; TMPDIR="$t" cargo nextest run --features test-hooks; r=$?; rm -rf "$t"; exit $r
     Summary [  11.068s] 761 tests run: 761 passed, 0 skipped
checkout status after checks: []
commit 54c4f3b11cceaccbe7cafae9ea57edecbaa6ddcc
```

## Stderr

Bytes: 0
Truncated: no

```text
```

