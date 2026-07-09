---
created_at: "2026-07-09T16:40:51.267329034+00:00"
id: "atelier-vzhx"
evidence_type: "test"
captured_at: "2026-07-09T16:40:39.712566781+00:00"
command: "bash -lc 'set -e; tmp=$(mktemp -d); trap \"rm -rf $tmp\" EXIT; git clone --quiet --no-local . \"$tmp/repo\"; test \"$(git -C \"$tmp/repo\" rev-parse HEAD)\" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; perl -0pi -e '\"'\"'s/status: \"blocked\"/status: \"done\"/'\"'\"' \"$tmp/repo/.atelier/issues/atelier-2uim.md\"; cd \"$tmp/repo\"; atelier work ready | rg \"atelier-(amfw|wyxn|l5mw)\"; for id in atelier-amfw atelier-wyxn atelier-l5mw; do test \"$(atelier issue show \"$id\" | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2; done'"
exit_status: "0"
agent_identity: "agent-factory.validate"
target:
  kind: "issue"
  id: "atelier-2uim"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2uim"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -e; tmp=$(mktemp -d); trap \"rm -rf $tmp\" EXIT; git clone --quiet --no-local . \"$tmp/repo\"; test \"$(git -C \"$tmp/repo\" rev-parse HEAD)\" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; perl -0pi -e '\"'\"'s/status: \"blocked\"/status: \"done\"/'\"'\"' \"$tmp/repo/.atelier/issues/atelier-2uim.md\"; cd \"$tmp/repo\"; atelier work ready | rg \"atelier-(amfw|wyxn|l5mw)\"; for id in atelier-amfw atelier-wyxn atelier-l5mw; do test \"$(atelier issue show \"$id\" | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2; done'"
updated_at: "2026-07-09T16:40:55.418123008+00:00"
---

## Summary

bash -lc 'set -e; tmp=$(mktemp -d); trap "rm -rf $tmp" EXIT; git clone --quiet --no-local . "$tmp/repo"; test "$(git -C "$tmp/repo" rev-parse HEAD)" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; perl -0pi -e '"'"'s/status: "blocked"/status: "done"/'"'"' "$tmp/repo/.atelier/issues/atelier-2uim.md"; cd "$tmp/repo"; atelier work ready | rg "atelier-(amfw|wyxn|l5mw)"; for id in atelier-amfw atelier-wyxn atelier-l5mw; do test "$(atelier issue show "$id" | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2; done'

## Command

```console
bash -lc 'set -e; tmp=$(mktemp -d); trap "rm -rf $tmp" EXIT; git clone --quiet --no-local . "$tmp/repo"; test "$(git -C "$tmp/repo" rev-parse HEAD)" = efddfa3f97074ec49ac80dd5ef567178b8d8a261; perl -0pi -e '"'"'s/status: "blocked"/status: "done"/'"'"' "$tmp/repo/.atelier/issues/atelier-2uim.md"; cd "$tmp/repo"; atelier work ready | rg "atelier-(amfw|wyxn|l5mw)"; for id in atelier-amfw atelier-wyxn atelier-l5mw; do test "$(atelier issue show "$id" | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2; done'
```

Exit status: 0

## Stdout

Bytes: 266
Truncated: no

```text
  ready atelier-amfw [todo] high - Enforce direct and transitive dependency readiness
  ready atelier-l5mw [todo] high - Add the Agent Factory mission-review subskill
  ready atelier-wyxn [todo] high - Represent mission-plan review provenance and revision freshness
```

## Stderr

Bytes: 146
Truncated: no

```text
2026-07-09T16:40:49.471106Z  WARN Runtime projection database was missing; rebuilt local SQLite projection from /tmp/tmp.k4HV46fEaf/repo/.atelier
```
