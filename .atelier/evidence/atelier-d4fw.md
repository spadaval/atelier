---
created_at: "2026-07-09T16:44:31.060848769+00:00"
id: "atelier-d4fw"
evidence_type: "validation"
captured_at: "2026-07-09T16:44:29.594886249+00:00"
command: "bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; D=$(mktemp -d); trap \"rm -rf $D\" EXIT; env -u NO_COLOR \"$BIN\" work missions >\"$D/plain\"; NO_COLOR= \"$BIN\" work missions >\"$D/no-color\"; cmp \"$D/plain\" \"$D/no-color\"; env -u NO_COLOR TERM=xterm-256color script -qec \"$BIN work missions\" /dev/null >\"$D/tty-color\"; NO_COLOR= TERM=xterm-256color script -qec \"$BIN work missions\" /dev/null >\"$D/tty-no-color\"; sed -E \"s/\\x1B\\[[0-9;]*[mK]//g\" \"$D/tty-color\" | tr -d \"\\r\" >\"$D/tty-color-text\"; tr -d \"\\r\" <\"$D/tty-no-color\" >\"$D/tty-no-color-text\"; cmp \"$D/plain\" \"$D/tty-color-text\"; cmp \"$D/plain\" \"$D/tty-no-color-text\"; COLOR_ESC=$(LC_ALL=C tr -cd \"\\033\" <\"$D/tty-color\" | wc -c); NO_COLOR_ESC=$(LC_ALL=C tr -cd \"\\033\" <\"$D/tty-no-color\" | wc -c); PLAIN_ESC=$(LC_ALL=C tr -cd \"\\033\" <\"$D/plain\" | wc -c); test \"$COLOR_ESC\" -gt 0; test \"$NO_COLOR_ESC\" -eq 0; test \"$PLAIN_ESC\" -eq 0; printf \"PASS: ANSI-stripped interactive text exactly equals NO_COLOR TTY and captured noninteractive text. ESC counts: interactive=%s NO_COLOR_TTY=%s captured=%s. Semantic SHA256: \" \"$COLOR_ESC\" \"$NO_COLOR_ESC\" \"$PLAIN_ESC\"; sha256sum \"$D/plain\" | cut -d\" \" -f1'"
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
title: "bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; D=$(mktemp -d); trap \"rm -rf $D\" EXIT; env -u NO_COLOR \"$BIN\" work missions >\"$D/plain\"; NO_COLOR= \"$BIN\" work missions >\"$D/no-color\"; cmp \"$D/plain\" \"$D/no-color\"; env -u NO_COLOR TERM=xterm-256color script -qec \"$BIN work missions\" /dev/null >\"$D/tty-color\"; NO_COLOR= TERM=xterm-256color script -qec \"$BIN work missions\" /dev/null >\"$D/tty-no-color\"; sed -E \"s/\\x1B\\[[0-9;]*[mK]//g\" \"$D/tty-color\" | tr -d \"\\r\" >\"$D/tty-color-text\"; tr -d \"\\r\" <\"$D/tty-no-color\" >\"$D/tty-no-color-text\"; cmp \"$D/plain\" \"$D/tty-color-text\"; cmp \"$D/plain\" \"$D/tty-no-color-text\"; COLOR_ESC=$(LC_ALL=C tr -cd \"\\033\" <\"$D/tty-color\" | wc -c); NO_COLOR_ESC=$(LC_ALL=C tr -cd \"\\033\" <\"$D/tty-no-color\" | wc -c); PLAIN_ESC=$(LC_ALL=C tr -cd \"\\033\" <\"$D/plain\" | wc -c); test \"$COLOR_ESC\" -gt 0; test \"$NO_COLOR_ESC\" -eq 0; test \"$PLAIN_ESC\" -eq 0; printf \"PASS: ANSI-stripped interactive text exactly equals NO_COLOR TTY and captured noninteractive text. ESC counts: interactive=%s NO_COLOR_TTY=%s captured=%s. Semantic SHA256: \" \"$COLOR_ESC\" \"$NO_COLOR_ESC\" \"$PLAIN_ESC\"; sha256sum \"$D/plain\" | cut -d\" \" -f1'"
updated_at: "2026-07-09T16:44:31.063607902+00:00"
---

## Summary

bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; D=$(mktemp -d); trap "rm -rf $D" EXIT; env -u NO_COLOR "$BIN" work missions >"$D/plain"; NO_COLOR= "$BIN" work missions >"$D/no-color"; cmp "$D/plain" "$D/no-color"; env -u NO_COLOR TERM=xterm-256color script -qec "$BIN work missions" /dev/null >"$D/tty-color"; NO_COLOR= TERM=xterm-256color script -qec "$BIN work missions" /dev/null >"$D/tty-no-color"; sed -E "s/\x1B\[[0-9;]*[mK]//g" "$D/tty-color" | tr -d "\r" >"$D/tty-color-text"; tr -d "\r" <"$D/tty-no-color" >"$D/tty-no-color-text"; cmp "$D/plain" "$D/tty-color-text"; cmp "$D/plain" "$D/tty-no-color-text"; COLOR_ESC=$(LC_ALL=C tr -cd "\033" <"$D/tty-color" | wc -c); NO_COLOR_ESC=$(LC_ALL=C tr -cd "\033" <"$D/tty-no-color" | wc -c); PLAIN_ESC=$(LC_ALL=C tr -cd "\033" <"$D/plain" | wc -c); test "$COLOR_ESC" -gt 0; test "$NO_COLOR_ESC" -eq 0; test "$PLAIN_ESC" -eq 0; printf "PASS: ANSI-stripped interactive text exactly equals NO_COLOR TTY and captured noninteractive text. ESC counts: interactive=%s NO_COLOR_TTY=%s captured=%s. Semantic SHA256: " "$COLOR_ESC" "$NO_COLOR_ESC" "$PLAIN_ESC"; sha256sum "$D/plain" | cut -d" " -f1'

## Command

```console
bash -lc 'set -euo pipefail; BIN=$PWD/target/debug/atelier; D=$(mktemp -d); trap "rm -rf $D" EXIT; env -u NO_COLOR "$BIN" work missions >"$D/plain"; NO_COLOR= "$BIN" work missions >"$D/no-color"; cmp "$D/plain" "$D/no-color"; env -u NO_COLOR TERM=xterm-256color script -qec "$BIN work missions" /dev/null >"$D/tty-color"; NO_COLOR= TERM=xterm-256color script -qec "$BIN work missions" /dev/null >"$D/tty-no-color"; sed -E "s/\x1B\[[0-9;]*[mK]//g" "$D/tty-color" | tr -d "\r" >"$D/tty-color-text"; tr -d "\r" <"$D/tty-no-color" >"$D/tty-no-color-text"; cmp "$D/plain" "$D/tty-color-text"; cmp "$D/plain" "$D/tty-no-color-text"; COLOR_ESC=$(LC_ALL=C tr -cd "\033" <"$D/tty-color" | wc -c); NO_COLOR_ESC=$(LC_ALL=C tr -cd "\033" <"$D/tty-no-color" | wc -c); PLAIN_ESC=$(LC_ALL=C tr -cd "\033" <"$D/plain" | wc -c); test "$COLOR_ESC" -gt 0; test "$NO_COLOR_ESC" -eq 0; test "$PLAIN_ESC" -eq 0; printf "PASS: ANSI-stripped interactive text exactly equals NO_COLOR TTY and captured noninteractive text. ESC counts: interactive=%s NO_COLOR_TTY=%s captured=%s. Semantic SHA256: " "$COLOR_ESC" "$NO_COLOR_ESC" "$PLAIN_ESC"; sha256sum "$D/plain" | cut -d" " -f1'
```
Exit status: 0

## Stdout

Bytes: 235
Truncated: no

```text
PASS: ANSI-stripped interactive text exactly equals NO_COLOR TTY and captured noninteractive text. ESC counts: interactive=86 NO_COLOR_TTY=0 captured=0. Semantic SHA256: 14e8685ab7fd02fd46b89521718a3b631927bfca05e15e0c2a93721dd3dd09e4
```

## Stderr

Bytes: 0
Truncated: no

```text
```
