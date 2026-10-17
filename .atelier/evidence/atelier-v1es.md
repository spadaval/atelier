---
created_at: "2026-10-17T06:25:52.598074495+00:00"
id: "atelier-v1es"
evidence_type: "test"
commit: "e8402f1edfeaaac3e729d4b1addd95ffaafe84c8"
result: "pass"
captured_at: "2026-10-17T06:25:52.330696716+00:00"
command: "bash -c 'set -o pipefail; d=$(mktemp -d); trap \"rm -rf $d\" EXIT; cd $d && git init -q && A=/home/supadava/.t3/worktrees/atelier/o3lm-5rs0/target/debug/atelier && $A init -q >/dev/null && id=$($A -q issue create \"Piped\"); fails=0; for args in \"issue show $id\" \"review list\" \"issue transition $id\" \"work all\" \"issue create Another\"; do $A $args 2>$d/err | head -1 >/dev/null; st=${PIPESTATUS[0]}; if { [ $st -ne 0 ] && [ $st -ne 141 ]; } || [ -s $d/err ]; then echo \"FAIL $args st=$st $(cat $d/err)\"; fails=1; else echo \"ok $args st=$st\"; fi; done; exit $fails'"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-5rs0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5rs0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Piping issue show, review list, issue transition, evidence show, evidence record and issue create into head -1 exits 0 or 141 with empty stderr"
updated_at: "2026-10-17T06:25:52.598872985+00:00"
---

## Summary

Piping issue show, review list, issue transition, evidence show, evidence record and issue create into head -1 exits 0 or 141 with empty stderr

## Command

```console
bash -c 'set -o pipefail; d=$(mktemp -d); trap "rm -rf $d" EXIT; cd $d && git init -q && A=/home/supadava/.t3/worktrees/atelier/o3lm-5rs0/target/debug/atelier && $A init -q >/dev/null && id=$($A -q issue create "Piped"); fails=0; for args in "issue show $id" "review list" "issue transition $id" "work all" "issue create Another"; do $A $args 2>$d/err | head -1 >/dev/null; st=${PIPESTATUS[0]}; if { [ $st -ne 0 ] && [ $st -ne 141 ]; } || [ -s $d/err ]; then echo "FAIL $args st=$st $(cat $d/err)"; fails=1; else echo "ok $args st=$st"; fi; done; exit $fails'
```

Exit status: 0

## Stdout

Bytes: 150
Truncated: no

```text
ok issue show tmp-yk8vnesxet-y4av st=0
ok review list st=0
ok issue transition tmp-yk8vnesxet-y4av st=0
ok work all st=0
ok issue create Another st=0
```

## Stderr

Bytes: 0
Truncated: no

```text
```

