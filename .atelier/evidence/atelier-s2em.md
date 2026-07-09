---
created_at: "2026-07-09T16:41:01.498241468+00:00"
id: "atelier-s2em"
evidence_type: "validation"
captured_at: "2026-07-09T16:41:00.283560791+00:00"
command: "bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; $BIN issue list --help | grep -q \"List issue records as generic inventory\"; ! $BIN issue list --help | grep -Eq -- \"--ready|--blocked\"; $BIN work missions --help | grep -q \"Mission Overview\"; rg -q \"flat, deterministic inventory\" .atelier/issues/atelier-c0mp.md; rg -q \"Mission Overview\" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; $BIN work missions > /tmp/g5fl-cap; NO_COLOR= $BIN work missions > /tmp/g5fl-nc; cmp -s /tmp/g5fl-cap /tmp/g5fl-nc; test \"$(LC_ALL=C tr -cd \"\\033\" </tmp/g5fl-cap | wc -c)\" -eq 0; env -u NO_COLOR TERM=xterm-256color script -qec \"$BIN work missions\" /tmp/g5fl-tty >/dev/null; test \"$(LC_ALL=C tr -cd \"\\033\" </tmp/g5fl-tty | wc -c)\" -gt 0; env -u NO_COLOR TERM=xterm-256color script -qec \"$BIN issue list --limit 2\" /tmp/g5fl-inv-tty >/dev/null; test \"$(LC_ALL=C tr -cd \"\\033\" </tmp/g5fl-inv-tty | wc -c)\" -gt 0; echo \"PASS: help/docs/tracker contracts agree; captured and NO_COLOR outputs are identical and ANSI-free; interactive xterm outputs for both public surfaces contain ANSI semantic styling.\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-g5fl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g5fl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; $BIN issue list --help | grep -q \"List issue records as generic inventory\"; ! $BIN issue list --help | grep -Eq -- \"--ready|--blocked\"; $BIN work missions --help | grep -q \"Mission Overview\"; rg -q \"flat, deterministic inventory\" .atelier/issues/atelier-c0mp.md; rg -q \"Mission Overview\" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; $BIN work missions > /tmp/g5fl-cap; NO_COLOR= $BIN work missions > /tmp/g5fl-nc; cmp -s /tmp/g5fl-cap /tmp/g5fl-nc; test \"$(LC_ALL=C tr -cd \"\\033\" </tmp/g5fl-cap | wc -c)\" -eq 0; env -u NO_COLOR TERM=xterm-256color script -qec \"$BIN work missions\" /tmp/g5fl-tty >/dev/null; test \"$(LC_ALL=C tr -cd \"\\033\" </tmp/g5fl-tty | wc -c)\" -gt 0; env -u NO_COLOR TERM=xterm-256color script -qec \"$BIN issue list --limit 2\" /tmp/g5fl-inv-tty >/dev/null; test \"$(LC_ALL=C tr -cd \"\\033\" </tmp/g5fl-inv-tty | wc -c)\" -gt 0; echo \"PASS: help/docs/tracker contracts agree; captured and NO_COLOR outputs are identical and ANSI-free; interactive xterm outputs for both public surfaces contain ANSI semantic styling.\"'"
updated_at: "2026-07-09T16:41:01.500848256+00:00"
---

## Summary

bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; $BIN issue list --help | grep -q "List issue records as generic inventory"; ! $BIN issue list --help | grep -Eq -- "--ready|--blocked"; $BIN work missions --help | grep -q "Mission Overview"; rg -q "flat, deterministic inventory" .atelier/issues/atelier-c0mp.md; rg -q "Mission Overview" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; $BIN work missions > /tmp/g5fl-cap; NO_COLOR= $BIN work missions > /tmp/g5fl-nc; cmp -s /tmp/g5fl-cap /tmp/g5fl-nc; test "$(LC_ALL=C tr -cd "\033" </tmp/g5fl-cap | wc -c)" -eq 0; env -u NO_COLOR TERM=xterm-256color script -qec "$BIN work missions" /tmp/g5fl-tty >/dev/null; test "$(LC_ALL=C tr -cd "\033" </tmp/g5fl-tty | wc -c)" -gt 0; env -u NO_COLOR TERM=xterm-256color script -qec "$BIN issue list --limit 2" /tmp/g5fl-inv-tty >/dev/null; test "$(LC_ALL=C tr -cd "\033" </tmp/g5fl-inv-tty | wc -c)" -gt 0; echo "PASS: help/docs/tracker contracts agree; captured and NO_COLOR outputs are identical and ANSI-free; interactive xterm outputs for both public surfaces contain ANSI semantic styling."'

## Command

```console
bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; $BIN issue list --help | grep -q "List issue records as generic inventory"; ! $BIN issue list --help | grep -Eq -- "--ready|--blocked"; $BIN work missions --help | grep -q "Mission Overview"; rg -q "flat, deterministic inventory" .atelier/issues/atelier-c0mp.md; rg -q "Mission Overview" docs/product/issue-inventory-and-mission-overview.md docs/product/cli-surface.md docs/product/human-cli-output.md; $BIN work missions > /tmp/g5fl-cap; NO_COLOR= $BIN work missions > /tmp/g5fl-nc; cmp -s /tmp/g5fl-cap /tmp/g5fl-nc; test "$(LC_ALL=C tr -cd "\033" </tmp/g5fl-cap | wc -c)" -eq 0; env -u NO_COLOR TERM=xterm-256color script -qec "$BIN work missions" /tmp/g5fl-tty >/dev/null; test "$(LC_ALL=C tr -cd "\033" </tmp/g5fl-tty | wc -c)" -gt 0; env -u NO_COLOR TERM=xterm-256color script -qec "$BIN issue list --limit 2" /tmp/g5fl-inv-tty >/dev/null; test "$(LC_ALL=C tr -cd "\033" </tmp/g5fl-inv-tty | wc -c)" -gt 0; echo "PASS: help/docs/tracker contracts agree; captured and NO_COLOR outputs are identical and ANSI-free; interactive xterm outputs for both public surfaces contain ANSI semantic styling."'
```
Exit status: 0

## Stdout

Bytes: 182
Truncated: no

```text
PASS: help/docs/tracker contracts agree; captured and NO_COLOR outputs are identical and ANSI-free; interactive xterm outputs for both public surfaces contain ANSI semantic styling.
```

## Stderr

Bytes: 26
Truncated: no

```text
Broken pipe (os error 32)
```
