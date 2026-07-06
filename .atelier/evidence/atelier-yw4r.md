---
created_at: "2026-07-06T18:49:45.849791146+00:00"
id: "atelier-yw4r"
evidence_type: "validation"
captured_at: "2026-07-06T18:49:44.710543942+00:00"
command: "bash -c '\nset -eu\nroot_help=$(./target/debug/atelier --help)\nprintf \"%s\\n\" \"$root_help\" | rg -n \"evidence|review|history|check|prune\"\nif printf \"%s\\n\" \"$root_help\" | rg -q \"^[[:space:]]+(branch|forgejo|maintenance)[[:space:]]\"; then\n  exit 1\nfi\nreview_help=$(./target/debug/atelier review --help)\nprintf \"%s\\n\" \"$review_help\" | rg -n \"open|show|submit|resolve|merge\"\nif printf \"%s\\n\" \"$review_help\" | rg -q \"^[[:space:]]+(status|comments|link|comment|approve|request-changes)[[:space:]]\"; then\n  exit 1\nfi\n./target/debug/atelier evidence list | rg -n \"Showing: 20 of|Omitted:|evidence show <evidence-id>\"\nquiet_count=$(./target/debug/atelier evidence list --quiet | wc -l)\ntest \"$quiet_count\" -gt 20\n./target/debug/atelier history | rg -n \"Limit:[[:space:]]+20|Showing:[[:space:]]+20 of|Omitted:\"\nif ./target/debug/atelier maintenance delete atelier-vqhi >/dev/null 2>&1; then\n  exit 1\nfi\nif ./target/debug/atelier review status --issue atelier-ye11 >/dev/null 2>&1; then\n  exit 1\nfi\nif ./target/debug/atelier history --mission atelier-durs >/dev/null 2>&1; then\n  exit 1\nfi\nif ./target/debug/atelier mission status atelier-durs >/dev/null 2>&1; then\n  exit 1\nfi\n./target/debug/atelier review show --issue atelier-ye11 --comments | rg -n \"State:[[:space:]]+closed|Merged:[[:space:]]+true|comment 230|comment 241\"\n'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS integrated human CLI/help scenarios: visible and hidden surface decisions, bounded evidence/history output, no obsolete aliases, and live provider review comments."
updated_at: "2026-07-06T18:49:52.216998804+00:00"
---

## Summary

PASS integrated human CLI/help scenarios: visible and hidden surface decisions, bounded evidence/history output, no obsolete aliases, and live provider review comments.

## Command

```console
bash -c '
set -eu
root_help=$(./target/debug/atelier --help)
printf "%s\n" "$root_help" | rg -n "evidence|review|history|check|prune"
if printf "%s\n" "$root_help" | rg -q "^[[:space:]]+(branch|forgejo|maintenance)[[:space:]]"; then
  exit 1
fi
review_help=$(./target/debug/atelier review --help)
printf "%s\n" "$review_help" | rg -n "open|show|submit|resolve|merge"
if printf "%s\n" "$review_help" | rg -q "^[[:space:]]+(status|comments|link|comment|approve|request-changes)[[:space:]]"; then
  exit 1
fi
./target/debug/atelier evidence list | rg -n "Showing: 20 of|Omitted:|evidence show <evidence-id>"
quiet_count=$(./target/debug/atelier evidence list --quiet | wc -l)
test "$quiet_count" -gt 20
./target/debug/atelier history | rg -n "Limit:[[:space:]]+20|Showing:[[:space:]]+20 of|Omitted:"
if ./target/debug/atelier maintenance delete atelier-vqhi >/dev/null 2>&1; then
  exit 1
fi
if ./target/debug/atelier review status --issue atelier-ye11 >/dev/null 2>&1; then
  exit 1
fi
if ./target/debug/atelier history --mission atelier-durs >/dev/null 2>&1; then
  exit 1
fi
if ./target/debug/atelier mission status atelier-durs >/dev/null 2>&1; then
  exit 1
fi
./target/debug/atelier review show --issue atelier-ye11 --comments | rg -n "State:[[:space:]]+closed|Merged:[[:space:]]+true|comment 230|comment 241"
'
```

Exit status: 0

## Stdout

Bytes: 1479
Truncated: no

```text
12:  status        Show checkout, mission, work, and tracker signposts
19:  bundle        Preview and apply one-shot graph bundle files
22:  evidence      Capture validation evidence
23:  review        Manage configured review artifacts
24:  history       Inspect bounded canonical repository or issue activity
27:  check         Validate tracker health; use --fix for local repair
28:  prune         Prune accumulated artifacts safely
33:  atelier man reviewer
50:  atelier bundle preview <file>
52:  atelier history
56:  atelier check
57:  atelier check <issue-id>
58:  atelier check --fix
59:  atelier prune
60:  atelier prune --apply
6:  open     Open or confirm the active review artifact for an issue owner
7:  show     Show linked review state and optionally its comments
8:  merge    Merge or confirm the linked review artifact without changing Atelier workflow state
9:  submit   Submit one comment, approval, or change request
10:  resolve  Resolve a native room finding
4:Showing: 20 of 838
25:Omitted: 818 older evidence record(s) hidden by default limit 20
29:  Show proof detail: atelier evidence show <evidence-id>
6:Limit:          20
7:Showing:        20 of 8802 events
51:Omitted:        8782 older events hidden by --limit 20
4:State:     closed
8:Merged:    true
11:comment 230 - [P1] Collapsed provider approval is not executable: crates/atelier-app/src/fo...
12:comment 241 - Addressed REQUEST_CHANGES in e6ab7a7d: Forgejo comments now use list-reviews ...
```

## Stderr

Bytes: 0
Truncated: no

```text
```

