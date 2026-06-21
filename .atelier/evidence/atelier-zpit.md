---
created_at: "2026-06-21T18:58:31.861719812+00:00"
id: "atelier-zpit"
evidence_type: "validation"
captured_at: "2026-06-21T18:58:27.107131323+00:00"
command: "bash -lc 'set -euo pipefail\nbin=\"$PWD/target/debug/atelier\"\ntmp=$(mktemp -d)\ncd \"$tmp\"\ngit init -q\n\"$bin\" init >/tmp/fyc9-init.out\nmission=$(\"$bin\" issue create \"fyc9 mission\" --issue-type mission | grep -o \"atelier-[a-z0-9]*\" | head -1)\nwork=$(\"$bin\" issue create \"fyc9 work\" | grep -o \"atelier-[a-z0-9]*\" | head -1)\nblocker=$(\"$bin\" issue create \"fyc9 blocker\" | grep -o \"atelier-[a-z0-9]*\" | head -1)\n\"$bin\" issue link \"$mission\" \"$work\" --role advances | rg \"Linked $mission -> $work \\(advances\\)|atelier issue status $mission\"\n\"$bin\" issue status \"$mission\" | rg \"$work|fyc9 work\"\n\"$bin\" issue block \"$mission\" \"$blocker\" | rg \"Linked $mission -> $blocker \\(blocked_by\\)\"\n\"$bin\" issue status \"$mission\" | rg \"Mission blockers: 1 open|$blocker\"\n\"$bin\" issue unblock \"$mission\" \"$blocker\" | rg \"Unlinked $mission -> $blocker \\(blocked_by\\)\"\n\"$bin\" issue unlink \"$mission\" \"$work\" --role advances | rg \"Unlinked $mission -> $work \\(advances\\)\"\nif \"$bin\" issue link \"$mission\" \"$work\" --role not_a_role >/tmp/fyc9-invalid-role.out 2>&1; then\n  echo \"invalid role unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg \"Invalid link type.*not_a_role\" /tmp/fyc9-invalid-role.out\nevidence=$(\"$bin\" evidence record --kind validation \"wrong kind fixture\" | grep -o \"atelier-[a-z0-9]*\" | head -1)\nif \"$bin\" issue link \"$evidence\" \"$work\" --role advances >/tmp/fyc9-wrong-kind.out 2>&1; then\n  echo \"wrong-kind link unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg \"evidence record|not an issue|Use\" /tmp/fyc9-wrong-kind.out\ncd /root/atelier-vays\ncargo nextest run -p atelier-cli test_issue_status_renders_mission_objective_links_and_blockers test_issue_status_includes_linked_issue_hierarchy\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fyc9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fyc9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Issue link/unlink/block validate mission work and blocker relationship surfaces"
updated_at: "2026-06-21T18:58:36.909326165+00:00"
---

## Summary

Issue link/unlink/block validate mission work and blocker relationship surfaces

## Command

```console
bash -lc 'set -euo pipefail
bin="$PWD/target/debug/atelier"
tmp=$(mktemp -d)
cd "$tmp"
git init -q
"$bin" init >/tmp/fyc9-init.out
mission=$("$bin" issue create "fyc9 mission" --issue-type mission | grep -o "atelier-[a-z0-9]*" | head -1)
work=$("$bin" issue create "fyc9 work" | grep -o "atelier-[a-z0-9]*" | head -1)
blocker=$("$bin" issue create "fyc9 blocker" | grep -o "atelier-[a-z0-9]*" | head -1)
"$bin" issue link "$mission" "$work" --role advances | rg "Linked $mission -> $work \(advances\)|atelier issue status $mission"
"$bin" issue status "$mission" | rg "$work|fyc9 work"
"$bin" issue block "$mission" "$blocker" | rg "Linked $mission -> $blocker \(blocked_by\)"
"$bin" issue status "$mission" | rg "Mission blockers: 1 open|$blocker"
"$bin" issue unblock "$mission" "$blocker" | rg "Unlinked $mission -> $blocker \(blocked_by\)"
"$bin" issue unlink "$mission" "$work" --role advances | rg "Unlinked $mission -> $work \(advances\)"
if "$bin" issue link "$mission" "$work" --role not_a_role >/tmp/fyc9-invalid-role.out 2>&1; then
  echo "invalid role unexpectedly succeeded" >&2
  exit 1
fi
rg "Invalid link type.*not_a_role" /tmp/fyc9-invalid-role.out
evidence=$("$bin" evidence record --kind validation "wrong kind fixture" | grep -o "atelier-[a-z0-9]*" | head -1)
if "$bin" issue link "$evidence" "$work" --role advances >/tmp/fyc9-wrong-kind.out 2>&1; then
  echo "wrong-kind link unexpectedly succeeded" >&2
  exit 1
fi
rg "evidence record|not an issue|Use" /tmp/fyc9-wrong-kind.out
cd /root/atelier-vays
cargo nextest run -p atelier-cli test_issue_status_renders_mission_objective_links_and_blockers test_issue_status_includes_linked_issue_hierarchy
'
```

Exit status: 0

## Stdout

Bytes: 1786
Truncated: no

```text
Linked atelier-1dpr -> atelier-ezsn (advances)
  atelier issue status atelier-1dpr
  atelier-ezsn: Issue section Evidence entry 1 must name an observable proof target (command, transcript, evidence record, test, review artifact, file change, or manual check) for issue atelier-ezsn, section Evidence, path .atelier/issues/atelier-ezsn.md
  ready atelier-ezsn - fyc9 work | no open blockers; mission-linked root; proof missing
Attached Proof: missing - issue proof gaps: atelier-ezsn
Work: open - atelier-ezsn
Checkout: dirty - git checkout has 5 dirty entries: ?? .atelier/issues/atelier-a5dy.md; ?? .atelier/issues/atelier-ezsn.md; ?? .atelier/missions/atelier-1dpr.md; ?? .atelier/workflow.yaml; ?? .gitignore
  Inspect selectable mission work transitions (1 selectable issue(s)): atelier issue transition atelier-ezsn --options
Linked atelier-1dpr -> atelier-a5dy (blocked_by)
  atelier-a5dy: Issue section Evidence entry 1 must name an observable proof target (command, transcript, evidence record, test, review artifact, file change, or manual check) for issue atelier-a5dy, section Evidence, path .atelier/issues/atelier-a5dy.md
Mission blockers: 1 open
Open Blockers: 1 open - atelier-a5dy
Blockers: open - atelier-a5dy
Checkout: dirty - git checkout has 5 dirty entries: ?? .atelier/issues/atelier-a5dy.md; ?? .atelier/issues/atelier-ezsn.md; ?? .atelier/missions/atelier-1dpr.md; ?? .atelier/workflow.yaml; ?? .gitignore
Unlinked atelier-1dpr -> atelier-a5dy (blocked_by)
Unlinked atelier-1dpr -> atelier-ezsn (advances)
Error: Invalid link type 'not_a_role'. Valid values: advances, blocked_by, contributes_to, validates, evidenced_by, implements, part_of, supersedes, derived_from, duplicates, related
Error: atelier-iwss is a evidence record, not an issue or mission record
```

## Stderr

Bytes: 619
Truncated: no

```text
Error: Lint failed with 2 finding(s)
Error: Lint failed with 2 finding(s)
   Compiling atelier-cli v0.2.0 (/root/atelier-vays/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.48s
────────────
 Nextest run ID bd659155-2fc1-4860-9525-438e0326448f with nextest profile: default
    Starting 1 test across 4 binaries (441 tests skipped)
        PASS [   0.444s] (1/1) atelier-cli::cli_integration setup_guidance::test_issue_status_includes_linked_issue_hierarchy
────────────
     Summary [   0.445s] 1 test run: 1 passed, 441 skipped
```

