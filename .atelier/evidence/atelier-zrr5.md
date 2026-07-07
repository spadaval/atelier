---
created_at: "2026-07-06T18:13:24.059567703+00:00"
id: "atelier-zrr5"
evidence_type: "validation"
captured_at: "2026-07-06T18:13:16.195624065+00:00"
command: "sh -c 'set -eu\nbin=target/debug/atelier\nroot=\"$($bin --help)\"\nif printf \"%s\\n\" \"$root\" | grep -Eq \"^  (forgejo|branch|workflow|diagnostics|export|rebuild|import-beads|maintenance)([[:space:]]|$)\"; then\n  echo \"hidden or removed command leaked into root help\" >&2\n  exit 1\nfi\nerr=$(mktemp)\nif $bin maintenance delete mission atelier-none --force 2>\"$err\"; then\n  echo \"maintenance delete unexpectedly succeeded\" >&2\n  exit 1\nfi\ngrep -q \"unrecognized subcommand .maintenance.\" \"$err\"\n$bin forgejo roles provision --help | grep -q \"Create missing role author users\"\n$bin branch for-epic --help | grep -q \"failed start transition\"\n$bin export --help | grep -q \"normal health uses check\"\n$bin rebuild --help | grep -q \"explicit local repair uses check --fix\"\n$bin workflow check --help | grep -q \"normal operator checks use check\"\n$bin diagnostics slow --help | grep -q \"stable local-only JSON\"\n$bin init --help | grep -q -- \"--import-beads\"\nepic=\"$($bin work epic atelier-eqq6)\"\nprintf \"%s\\n\" \"$epic\" | grep -q \"atelier issue transition atelier-eqq6\"\nif printf \"%s\\n\" \"$epic\" | grep -q \"atelier branch\"; then\n  echo \"work epic promoted branch recovery\" >&2\n  exit 1\nfi\nfor role in worker reviewer validator manager; do\n  guide=\"$($bin man \"$role\")\"\n  if printf \"%s\\n\" \"$guide\" | grep -Eq \"atelier (branch|export|rebuild|import-beads|workflow check)|forgejo roles\"; then\n    echo \"$role guide promoted hidden escape hatch\" >&2\n    exit 1\n  fi\ndone\necho \"PASS root-help, removed-command, focused recovery/help, role-guide, and work-epic scenarios\"'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-eqq6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-eqq6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'set -eu\nbin=target/debug/atelier\nroot=\"$($bin --help)\"\nif printf \"%s\\n\" \"$root\" | grep -Eq \"^  (forgejo|branch|workflow|diagnostics|export|rebuild|import-beads|maintenance)([[:space:]]|$)\"; then\n  echo \"hidden or removed command leaked into root help\" >&2\n  exit 1\nfi\nerr=$(mktemp)\nif $bin maintenance delete mission atelier-none --force 2>\"$err\"; then\n  echo \"maintenance delete unexpectedly succeeded\" >&2\n  exit 1\nfi\ngrep -q \"unrecognized subcommand .maintenance.\" \"$err\"\n$bin forgejo roles provision --help | grep -q \"Create missing role author users\"\n$bin branch for-epic --help | grep -q \"failed start transition\"\n$bin export --help | grep -q \"normal health uses check\"\n$bin rebuild --help | grep -q \"explicit local repair uses check --fix\"\n$bin workflow check --help | grep -q \"normal operator checks use check\"\n$bin diagnostics slow --help | grep -q \"stable local-only JSON\"\n$bin init --help | grep -q -- \"--import-beads\"\nepic=\"$($bin work epic atelier-eqq6)\"\nprintf \"%s\\n\" \"$epic\" | grep -q \"atelier issue transition atelier-eqq6\"\nif printf \"%s\\n\" \"$epic\" | grep -q \"atelier branch\"; then\n  echo \"work epic promoted branch recovery\" >&2\n  exit 1\nfi\nfor role in worker reviewer validator manager; do\n  guide=\"$($bin man \"$role\")\"\n  if printf \"%s\\n\" \"$guide\" | grep -Eq \"atelier (branch|export|rebuild|import-beads|workflow check)|forgejo roles\"; then\n    echo \"$role guide promoted hidden escape hatch\" >&2\n    exit 1\n  fi\ndone\necho \"PASS root-help, removed-command, focused recovery/help, role-guide, and work-epic scenarios\"'"
updated_at: "2026-07-06T18:13:27.959750475+00:00"
---

## Summary

sh -c 'set -eu
bin=target/debug/atelier
root="$($bin --help)"
if printf "%s\n" "$root" | grep -Eq "^  (forgejo|branch|workflow|diagnostics|export|rebuild|import-beads|maintenance)([[:space:]]|$)"; then
  echo "hidden or removed command leaked into root help" >&2
  exit 1
fi
err=$(mktemp)
if $bin maintenance delete mission atelier-none --force 2>"$err"; then
  echo "maintenance delete unexpectedly succeeded" >&2
  exit 1
fi
grep -q "unrecognized subcommand .maintenance." "$err"
$bin forgejo roles provision --help | grep -q "Create missing role author users"
$bin branch for-epic --help | grep -q "failed start transition"
$bin export --help | grep -q "normal health uses check"
$bin rebuild --help | grep -q "explicit local repair uses check --fix"
$bin workflow check --help | grep -q "normal operator checks use check"
$bin diagnostics slow --help | grep -q "stable local-only JSON"
$bin init --help | grep -q -- "--import-beads"
epic="$($bin work epic atelier-eqq6)"
printf "%s\n" "$epic" | grep -q "atelier issue transition atelier-eqq6"
if printf "%s\n" "$epic" | grep -q "atelier branch"; then
  echo "work epic promoted branch recovery" >&2
  exit 1
fi
for role in worker reviewer validator manager; do
  guide="$($bin man "$role")"
  if printf "%s\n" "$guide" | grep -Eq "atelier (branch|export|rebuild|import-beads|workflow check)|forgejo roles"; then
    echo "$role guide promoted hidden escape hatch" >&2
    exit 1
  fi
done
echo "PASS root-help, removed-command, focused recovery/help, role-guide, and work-epic scenarios"'

## Command

```console
sh -c 'set -eu
bin=target/debug/atelier
root="$($bin --help)"
if printf "%s\n" "$root" | grep -Eq "^  (forgejo|branch|workflow|diagnostics|export|rebuild|import-beads|maintenance)([[:space:]]|$)"; then
  echo "hidden or removed command leaked into root help" >&2
  exit 1
fi
err=$(mktemp)
if $bin maintenance delete mission atelier-none --force 2>"$err"; then
  echo "maintenance delete unexpectedly succeeded" >&2
  exit 1
fi
grep -q "unrecognized subcommand .maintenance." "$err"
$bin forgejo roles provision --help | grep -q "Create missing role author users"
$bin branch for-epic --help | grep -q "failed start transition"
$bin export --help | grep -q "normal health uses check"
$bin rebuild --help | grep -q "explicit local repair uses check --fix"
$bin workflow check --help | grep -q "normal operator checks use check"
$bin diagnostics slow --help | grep -q "stable local-only JSON"
$bin init --help | grep -q -- "--import-beads"
epic="$($bin work epic atelier-eqq6)"
printf "%s\n" "$epic" | grep -q "atelier issue transition atelier-eqq6"
if printf "%s\n" "$epic" | grep -q "atelier branch"; then
  echo "work epic promoted branch recovery" >&2
  exit 1
fi
for role in worker reviewer validator manager; do
  guide="$($bin man "$role")"
  if printf "%s\n" "$guide" | grep -Eq "atelier (branch|export|rebuild|import-beads|workflow check)|forgejo roles"; then
    echo "$role guide promoted hidden escape hatch" >&2
    exit 1
  fi
done
echo "PASS root-help, removed-command, focused recovery/help, role-guide, and work-epic scenarios"'
```

Exit status: 0

## Stdout

Bytes: 92
Truncated: no

```text
PASS root-help, removed-command, focused recovery/help, role-guide, and work-epic scenarios
```

## Stderr

Bytes: 0
Truncated: no

```text
```
