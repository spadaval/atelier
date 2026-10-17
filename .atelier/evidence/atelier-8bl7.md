---
created_at: "2026-10-17T06:26:33.113490702+00:00"
id: "atelier-8bl7"
evidence_type: "validation"
commit: "e8402f1edfeaaac3e729d4b1addd95ffaafe84c8"
result: "pass"
captured_at: "2026-10-17T06:26:29.768507425+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/scripts/pipe_check.sh /home/supadava/.t3/worktrees/atelier/o3lm-5rs0/target/debug/atelier"
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
title: "Corrects atelier-v1es, whose loop skipped evidence show, evidence record and work mission. In a fresh fixture, issue show, review list, issue transition, evidence show, evidence record -- true, issue create and work mission each piped to head -1 exit 0 or 141 with empty stderr. The scenario script lives outside the repository at ~/.cache/atelier-tmp/scripts/pipe_check.sh; the closed-stdout integration test is the durable proof."
updated_at: "2026-10-17T06:26:33.114340638+00:00"
---

## Summary

Corrects atelier-v1es, whose loop skipped evidence show, evidence record and work mission. In a fresh fixture, issue show, review list, issue transition, evidence show, evidence record -- true, issue create and work mission each piped to head -1 exit 0 or 141 with empty stderr. The scenario script lives outside the repository at ~/.cache/atelier-tmp/scripts/pipe_check.sh; the closed-stdout integration test is the durable proof.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/scripts/pipe_check.sh /home/supadava/.t3/worktrees/atelier/o3lm-5rs0/target/debug/atelier
```

Exit status: 0

## Stdout

Bytes: 313
Truncated: no

```text
ok issue show tmp-retwcbnkgz-6z0p st=0
ok review list st=0
ok issue transition tmp-retwcbnkgz-6z0p st=0
ok evidence show tmp-retwcbnkgz-9q4s st=0
ok evidence record --target issue/tmp-retwcbnkgz-6z0p --kind test --summary piped -- true st=0
ok issue create Another st=141
ok work mission tmp-retwcbnkgz-a2ru st=0
```

## Stderr

Bytes: 0
Truncated: no

```text
```

