---
created_at: "2026-06-23T23:35:20.451875059+00:00"
id: "atelier-6wx5"
evidence_type: "validation"
captured_at: "2026-06-23T23:35:03.214609206+00:00"
command: "bash -lc '\nset -euo pipefail\nbin=target/debug/atelier\ntmp=$(mktemp -d)\ntrap \"rm -rf \\\"$tmp\\\"\" EXIT\n\ncat <<\"MATRIX\"\nComplaint coverage matrix:\n- status/Git correctness and stale status-like output: covered by atelier-wxox and atelier-4wmp; sampled via status, issue show, issue status, mission status, transition options.\n- hidden ready work and parent-blocker ambiguity: covered by atelier-4wmp plus formatter display roles; sampled via issue list --ready/--blocked and issue status.\n- duplicate lifecycle paths and stale help/provider/admin framing: covered by atelier-ycj9 and atelier-t8ew; sampled via man worker/admin, mission status, and transition options.\n- implementation-shaped command nouns and stale public help: covered by atelier-ycj9 and command help tests; sampled via root help and role guides.\n- brittle output wording, evidence transcript budgets, and history browsing: covered by atelier-5sgx and atelier-7fof; sampled via evidence list/show and history.\nResidual risks: no blocker found in this validation pass; performance complaints remain explicitly out of scope per docs/product/command-audit/agent-complaints.md.\nMATRIX\n\nsample() {\n  name=\"$1\"\n  shift\n  outfile=\"$tmp/$name.out\"\n  \"$@\" >\"$outfile\"\n  lines=$(wc -l <\"$outfile\" | tr -d \" \")\n  bytes=$(wc -c <\"$outfile\" | tr -d \" \")\n  echo \"sample $name: $lines lines, $bytes bytes\"\n  sed -n \"1,8p\" \"$outfile\"\n}\n\nsample status \"$bin\" status\nsample mission_status \"$bin\" mission status atelier-c0qc\nsample issue_show \"$bin\" issue show atelier-3js3\nsample issue_status \"$bin\" issue status atelier-kx2y\nsample transition_options \"$bin\" issue transition atelier-3js3 --options\nsample ready_queue \"$bin\" issue list --ready\nsample blocked_queue \"$bin\" issue list --blocked\nsample evidence_list \"$bin\" evidence list --status recorded\nsample history_issue \"$bin\" history --issue atelier-3js3 --limit 5\nsample man_worker \"$bin\" man worker\nsample man_admin \"$bin\" man admin\nsample root_help \"$bin\" --help\n\nif \"$bin\" issue transition atelier-3js3 --options | grep -q $\"\\033\\\\[\"; then\n  echo \"non-interactive colorless output check failed\"\n  exit 1\nfi\necho \"non-interactive colorless output: pass\"\n\ncargo fmt -- --check\ncargo check -p atelier-cli\ncargo nextest run -p atelier-cli -E \"test(color_policy_auto_requires_terminal_and_no_color_absent) or test(colored_display_roles_wrap_text_with_ansi) or test(colorless_display_roles_keep_text_meaning) or test(test_issue_transition_options_render_guidance_and_exact_command) or test(test_issue_list_bounds_blocker_footer_actions) or test(test_evidence_list_bounds_default_output) or test(test_history_repo_wide_supports_filters_bounded_output_and_drill_downs) or test(test_man_manager_routes_to_mission_inventory_without_focus) or test(review_status_lines_lead_with_authority_and_state)\"\n\"$bin\" lint atelier-c0qc\n\"$bin\" export --check\ngit diff --check\n\necho \"focused validation commands: pass\"\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3js3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3js3"
    role: "validates"
  - kind: "issue"
    id: "atelier-c0qc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Human CLI output refresh end-to-end validation"
updated_at: "2026-06-23T23:35:36.868332393+00:00"
---

## Summary

Human CLI output refresh end-to-end validation

## Command

```console
bash -lc '
set -euo pipefail
bin=target/debug/atelier
tmp=$(mktemp -d)
trap "rm -rf \"$tmp\"" EXIT

cat <<"MATRIX"
Complaint coverage matrix:
- status/Git correctness and stale status-like output: covered by atelier-wxox and atelier-4wmp; sampled via status, issue show, issue status, mission status, transition options.
- hidden ready work and parent-blocker ambiguity: covered by atelier-4wmp plus formatter display roles; sampled via issue list --ready/--blocked and issue status.
- duplicate lifecycle paths and stale help/provider/admin framing: covered by atelier-ycj9 and atelier-t8ew; sampled via man worker/admin, mission status, and transition options.
- implementation-shaped command nouns and stale public help: covered by atelier-ycj9 and command help tests; sampled via root help and role guides.
- brittle output wording, evidence transcript budgets, and history browsing: covered by atelier-5sgx and atelier-7fof; sampled via evidence list/show and history.
Residual risks: no blocker found in this validation pass; performance complaints remain explicitly out of scope per docs/product/command-audit/agent-complaints.md.
MATRIX

sample() {
  name="$1"
  shift
  outfile="$tmp/$name.out"
  "$@" >"$outfile"
  lines=$(wc -l <"$outfile" | tr -d " ")
  bytes=$(wc -c <"$outfile" | tr -d " ")
  echo "sample $name: $lines lines, $bytes bytes"
  sed -n "1,8p" "$outfile"
}

sample status "$bin" status
sample mission_status "$bin" mission status atelier-c0qc
sample issue_show "$bin" issue show atelier-3js3
sample issue_status "$bin" issue status atelier-kx2y
sample transition_options "$bin" issue transition atelier-3js3 --options
sample ready_queue "$bin" issue list --ready
sample blocked_queue "$bin" issue list --blocked
sample evidence_list "$bin" evidence list --status recorded
sample history_issue "$bin" history --issue atelier-3js3 --limit 5
sample man_worker "$bin" man worker
sample man_admin "$bin" man admin
sample root_help "$bin" --help

if "$bin" issue transition atelier-3js3 --options | grep -q $"\033\\["; then
  echo "non-interactive colorless output check failed"
  exit 1
fi
echo "non-interactive colorless output: pass"

cargo fmt -- --check
cargo check -p atelier-cli
cargo nextest run -p atelier-cli -E "test(color_policy_auto_requires_terminal_and_no_color_absent) or test(colored_display_roles_wrap_text_with_ansi) or test(colorless_display_roles_keep_text_meaning) or test(test_issue_transition_options_render_guidance_and_exact_command) or test(test_issue_list_bounds_blocker_footer_actions) or test(test_evidence_list_bounds_default_output) or test(test_history_repo_wide_supports_filters_bounded_output_and_drill_downs) or test(test_man_manager_routes_to_mission_inventory_without_focus) or test(review_status_lines_lead_with_authority_and_state)"
"$bin" lint atelier-c0qc
"$bin" export --check
git diff --check

echo "focused validation commands: pass"
'
```

Exit status: 0

## Stdout

Bytes: 6107
Truncated: yes

```text
Complaint coverage matrix:
- status/Git correctness and stale status-like output: covered by atelier-wxox and atelier-4wmp; sampled via status, issue show, issue status, mission status, transition options.
- hidden ready work and parent-blocker ambiguity: covered by atelier-4wmp plus formatter display roles; sampled via issue list --ready/--blocked and issue status.
- duplicate lifecycle paths and stale help/provider/admin framing: covered by atelier-ycj9 and atelier-t8ew; sampled via man worker/admin, mission status, and transition options.
- implementation-shaped command nouns and stale public help: covered by atelier-ycj9 and command help tests; sampled via root help and role guides.
- brittle output wording, evidence transcript budgets, and history browsing: covered by atelier-5sgx and atelier-7fof; sampled via evidence list/show and history.
Residual risks: no blocker found in this validation pass; performance complaints remain explicitly out of scope per docs/product/command-audit/agent-complaints.md.
sample status: 38 lines, 1282 bytes
Atelier Status
==============
Tracker:       current
Ready work:    24
Current work:  2 issue(s)
  active atelier-3js3 - Validate human CLI output refresh end to end [worker]
  active atelier-kx2y - Epic: Refresh human CLI output surfaces [worker]
Current missions: 4
sample mission_status: 53 lines, 1371 bytes
Mission Status atelier-c0qc [ready] - Refresh human CLI output ergonomics
=========================================================================
Health:   needs-evidence
Tracker:  ok
Terminal: ready

Work
----
sample issue_show: 114 lines, 4545 bytes
atelier-3js3 [validation] in_progress - Validate human CLI output refresh end to end
====================================================================================
Status:   in_progress
Category: active
Type:     validation
Priority: high
Created:  2026-06-23 11:22 -04:00
Updated:  2026-06-23 19:33 -04:00
sample issue_status: 32 lines, 767 bytes
Issue Status atelier-kx2y - Epic: Refresh human CLI output surfaces
===================================================================
Health:   active
Type:     epic
Status:   in_progress

Work
----
sample transition_options: 79 lines, 2369 bytes
Issue Transitions atelier-3js3 - Validate human CLI output refresh end to end
=============================================================================
State
-----
Status:   in_progress
Type:     validation
Options:  3
Branch Context
sample ready_queue: 62 lines, 3916 bytes
Issue Queue
===========
24 total | Categories: 24 todo | Statuses: 4 ready, 20 todo | Priorities: 16 high, 8 medium | Blocked: 0

[epic] atelier-ckca high - Epic: Rewrite domain-shaped cache schema (shown for context; blocked through parent)
---------------------------------------------------------------------------------------------------------------
  blocked by 3 external blockers
    ready [feature] atelier-nxq9 - Implement domain-shaped cache tables
sample blocked_queue: 10 lines, 489 bytes
Blocked issues
==============
10 total
Drill down: atelier issue blocked <id>
  blocked atelier-ckca Epic: Rewrite domain-shaped cache schema (2 blockers)
  blocked atelier-x7lq Epic: Lazy cache access and freshness (1 blocker)
  blocked atelier-hl1n Epic: Unify record-file storage and d... (1 blocker)
  blocked atelier-idwz Rewrite cache rebuild for domain schema (1 blocker)
sample evidence_list: 80 lines, 10329 bytes
Evidence
--------
729 total
Showing: 20 of 729
  atelier-00d1   recorded      validation exit 0 target issue/atelier-62po (validates) command bash -lc 'set ... - Mission objective show/status surfaces expose rich show, compact status, verbose detail, and quiet mode
  atelier-06rb   recorded      validation exit (none) target issue/atelier-z80r (validates) command (manual) - Qualitative and quantitative validation standards documented in validation.md and Agent Factory work-item authoring; includes evaluator-context requirements for subjective review and metric/baseline/threshold requirements for numerical claims; examples cover mission list hierarchy and pe
```

## Stderr

Bytes: 1648
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.30s
────────────
 Nextest run ID 6defbe1a-7974-423b-a29f-51f5628a7969 with nextest profile: default
    Starting 9 tests across 4 binaries (465 tests skipped)
        PASS [   0.010s] (1/9) atelier-cli human_output::tests::color_policy_auto_requires_terminal_and_no_color_absent
        PASS [   0.014s] (2/9) atelier-cli human_output::tests::colorless_display_roles_keep_text_meaning
        PASS [   0.016s] (3/9) atelier-cli commands::pr::tests::review_status_lines_lead_with_authority_and_state
        PASS [   0.018s] (4/9) atelier-cli human_output::tests::colored_display_roles_wrap_text_with_ansi
        PASS [   0.264s] (5/9) atelier-cli::cli_integration setup_guidance::test_man_manager_routes_to_mission_inventory_without_focus
        PASS [   0.393s] (6/9) atelier-cli::cli_integration issues::test_history_repo_wide_supports_filters_bounded_output_and_drill_downs
        PASS [   1.304s] (7/9) atelier-cli::cli_integration setup_guidance::test_issue_transition_options_render_guidance_and_exact_command
        PASS [   1.886s] (8/9) atelier-cli::cli_integration issues::test_issue_list_bounds_blocker_footer_actions
        PASS [   2.657s] (9/9) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
────────────
     Summary [   2.658s] 9 tests run: 9 passed, 465 skipped
```

