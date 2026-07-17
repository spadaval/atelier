---
created_at: "2026-07-09T16:31:43.686551693+00:00"
id: "atelier-vr5e"
evidence_type: "test"
captured_at: "2026-07-09T16:31:32.557061341+00:00"
command: "bash -lc 'set -e; tmp=$(mktemp -d); trap \"rm -rf $tmp\" EXIT; git clone --quiet --no-local . \"$tmp/repo\"; perl -0pi -e '\"'\"'s/status: \"todo\"/status: \"done\"/'\"'\"' \"$tmp/repo/.atelier/issues/atelier-2uim.md\"; cd \"$tmp/repo\"; atelier work ready | rg \"atelier-(amfw|wyxn|l5mw)\"; test \"$(atelier issue show atelier-amfw | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2; test \"$(atelier issue show atelier-wyxn | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2; test \"$(atelier issue show atelier-l5mw | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2'"
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
title: "bash -lc 'set -e; tmp=$(mktemp -d); trap \"rm -rf $tmp\" EXIT; git clone --quiet --no-local . \"$tmp/repo\"; perl -0pi -e '\"'\"'s/status: \"todo\"/status: \"done\"/'\"'\"' \"$tmp/repo/.atelier/issues/atelier-2uim.md\"; cd \"$tmp/repo\"; atelier work ready | rg \"atelier-(amfw|wyxn|l5mw)\"; test \"$(atelier issue show atelier-amfw | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2; test \"$(atelier issue show atelier-wyxn | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2; test \"$(atelier issue show atelier-l5mw | sed -n '\"'\"'/Blocked by/,/Blocking/p'\"'\"' | rg -c '\"'\"'atelier-(2uim|wlk4)'\"'\"')\" -eq 2'"
updated_at: "2026-07-09T16:31:48.396183034+00:00"
---

## Summary

bash -lc 'set -e; tmp=$(mktemp -d); trap "rm -rf $tmp" EXIT; git clone --quiet --no-local . "$tmp/repo"; perl -0pi -e '"'"'s/status: "todo"/status: "done"/'"'"' "$tmp/repo/.atelier/issues/atelier-2uim.md"; cd "$tmp/repo"; atelier work ready | rg "atelier-(amfw|wyxn|l5mw)"; test "$(atelier issue show atelier-amfw | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2; test "$(atelier issue show atelier-wyxn | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2; test "$(atelier issue show atelier-l5mw | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2'

## Command

```console
bash -lc 'set -e; tmp=$(mktemp -d); trap "rm -rf $tmp" EXIT; git clone --quiet --no-local . "$tmp/repo"; perl -0pi -e '"'"'s/status: "todo"/status: "done"/'"'"' "$tmp/repo/.atelier/issues/atelier-2uim.md"; cd "$tmp/repo"; atelier work ready | rg "atelier-(amfw|wyxn|l5mw)"; test "$(atelier issue show atelier-amfw | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2; test "$(atelier issue show atelier-wyxn | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2; test "$(atelier issue show atelier-l5mw | sed -n '"'"'/Blocked by/,/Blocking/p'"'"' | rg -c '"'"'atelier-(2uim|wlk4)'"'"')" -eq 2'
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
2026-07-09T16:31:41.927348Z  WARN Runtime projection database was missing; rebuilt local SQLite projection from /tmp/tmp.WChamR4XX1/repo/.atelier
```
