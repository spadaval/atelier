---
created_at: "2026-07-06T18:55:06.192071765+00:00"
id: "atelier-7rfk"
evidence_type: "validation"
captured_at: "2026-07-06T18:55:01.558084755+00:00"
command: "bash -c '\nset -eu\ndoc=docs/spec/agent-factory/tracker-replacement-mvp.md\nif rg -n \"atelier (search|issue close|dep |lint|doctor|export|rebuild|import-beads|issue claim|session)\" \"$doc\"; then\n  exit 1\nfi\nrg -n \"not .*owner of the current CLI|non-normative|Current Atelier path|historical Beads-to-Atelier cutover crosswalk|not compatibility aliases\" \"$doc\"\nfor required in \"atelier issue list\" \"atelier issue show <id>\" \"atelier issue transition <id>\" \"atelier issue link <blocked> <blocker> --role blocked_by\" \"atelier issue unlink <blocked> <blocker> --role blocked_by\" \"atelier work ready\" \"atelier work blocked\" \"atelier check --fix\" \"atelier init --import-beads\" \"atelier evidence record\" \"atelier work missions\" \"atelier work mission <mission-id>\"; do\n  rg -Fq \"$required\" \"$doc\"\ndone\ngit show 60634f2b:docs/product/issue-inventory-and-mission-overview.md | rg -n \"atelier issue list|atelier work missions|atelier work mission <mission-id>|Mission Overview\"\nrg -n \"atelier-c0mp.*issue-inventory and Mission Overview|atelier issue list|atelier work missions|atelier work mission <mission-id>\" \"$doc\"\nif git diff master...HEAD -- \"$doc\" | rg -q \"Showing <shown> of <matching> issues|at most 20 missions|at most 10 directly linked epics\"; then\n  exit 1\nfi\nif ./target/debug/atelier work epic atelier-eqq6 | rg -q \"atelier branch for-epic\"; then\n  exit 1\nfi\n./target/debug/atelier --help | rg -n \"issue|work|check\"\n./target/debug/atelier work --help | rg -n \"missions|blocked\"\nenv HOME=/tmp/atelier-vqhi-full-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -E \"test(command_surface) or test(test_obsolete_command_surfaces_are_removed_without_aliases)\"\n./target/debug/atelier check\n'"
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
title: "PASS independent remediation revalidation; supersedes fail evidence atelier-cpcm after inspecting fix and implementer evidence atelier-vu1i/atelier-aw1h. The MVP spec now labels historical material non-normative, uses current paths in every live/current Atelier column and crosswalk, contains no removed Atelier command spelling as guidance, preserves the c0mp inventory/overview split without copying replacement layout wording, and current help/tests/work-epic behavior agree."
updated_at: "2026-07-06T18:55:12.341482621+00:00"
---

## Summary

PASS independent remediation revalidation; supersedes fail evidence atelier-cpcm after inspecting fix and implementer evidence atelier-vu1i/atelier-aw1h. The MVP spec now labels historical material non-normative, uses current paths in every live/current Atelier column and crosswalk, contains no removed Atelier command spelling as guidance, preserves the c0mp inventory/overview split without copying replacement layout wording, and current help/tests/work-epic behavior agree.

## Command

```console
bash -c '
set -eu
doc=docs/spec/agent-factory/tracker-replacement-mvp.md
if rg -n "atelier (search|issue close|dep |lint|doctor|export|rebuild|import-beads|issue claim|session)" "$doc"; then
  exit 1
fi
rg -n "not .*owner of the current CLI|non-normative|Current Atelier path|historical Beads-to-Atelier cutover crosswalk|not compatibility aliases" "$doc"
for required in "atelier issue list" "atelier issue show <id>" "atelier issue transition <id>" "atelier issue link <blocked> <blocker> --role blocked_by" "atelier issue unlink <blocked> <blocker> --role blocked_by" "atelier work ready" "atelier work blocked" "atelier check --fix" "atelier init --import-beads" "atelier evidence record" "atelier work missions" "atelier work mission <mission-id>"; do
  rg -Fq "$required" "$doc"
done
git show 60634f2b:docs/product/issue-inventory-and-mission-overview.md | rg -n "atelier issue list|atelier work missions|atelier work mission <mission-id>|Mission Overview"
rg -n "atelier-c0mp.*issue-inventory and Mission Overview|atelier issue list|atelier work missions|atelier work mission <mission-id>" "$doc"
if git diff master...HEAD -- "$doc" | rg -q "Showing <shown> of <matching> issues|at most 20 missions|at most 10 directly linked epics"; then
  exit 1
fi
if ./target/debug/atelier work epic atelier-eqq6 | rg -q "atelier branch for-epic"; then
  exit 1
fi
./target/debug/atelier --help | rg -n "issue|work|check"
./target/debug/atelier work --help | rg -n "missions|blocked"
env HOME=/tmp/atelier-vqhi-full-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -E "test(command_surface) or test(test_obsolete_command_surfaces_are_removed_without_aliases)"
./target/debug/atelier check
'
```

Exit status: 0

## Stdout

Bytes: 5279
Truncated: yes

```text
16:a cutover record. It is non-normative: current work treats Markdown records as
129:| Agent Factory operation | Beads command at cutover | Current Atelier path | Required text behavior | Machine-readable replacement | Required at cutover | Historical owner |
183:This table is a historical Beads-to-Atelier cutover crosswalk. The Beads column
187:session paths are not compatibility aliases.
1:# Issue Inventory And Mission Overview
3:`atelier issue list` and `atelier work missions` are separate read surfaces.
11:| `atelier issue list` | Which issue records match these simple metadata filters, and which ID should I inspect? | Mission membership, hierarchy, blocker-aware selection, progress rollups, or operational queue grouping. |
12:| `atelier work missions` | Which current missions and directly linked epics define the mission backlog, and what work sits outside the visible mission structure? | Leaf-task selection, record mutation, exhaustive issue inventory, or one mission's full execution detail. |
13:| `atelier work mission <mission-id>` | What is happening inside this one mission, including its actionable leaf work? | Cross-mission comparison or generic inventory. |
15:`work missions` is the **Mission Overview** in headings, help, and operator
21:The default `atelier issue list` includes every canonical issue record,
57:atelier-4fip  epic        todo         high      Build the formatted Mission Overview
59:atelier-vgqe  task        in_progress  high      Define the issue inventory and Mission Overview contract
71:$ atelier issue list --issue-type mission --category active --quiet
75:## Mission Overview Membership
81:The Mission Overview classifies direct roots as follows:
100:By default, `atelier work missions` excludes mission records whose configured
101:status category is `done`. `atelier work missions --all` includes them after
116:ready. Operators use `atelier issue list` to browse the underlying records,
120:## Mission Overview Ordering And Budgets
131:focused drill-down is `atelier work mission <mission-id>` for a mission and
136:Quiet Mission Overview output prints only visible mission IDs, one per line, in
145:Mission Overview
150:  atelier-4fip  epic  todo  high  Build the formatted Mission Overview
161:  Browse records: atelier issue list
164:`atelier work missions --all` uses the same shape and adds done missions. The
188:The legacy `atelier work queue` is not the Mission Overview and is not the
7:`atelier-c0mp` issue-inventory and Mission Overview contract further defines
60:  `atelier issue transition <objective-id>`, `atelier issue list`,
139:| List/filter work | `bd list --status=open` | `atelier issue list --status todo` | Print compact rows with ID, status, priority, type, title, and assignee. | Focused inventory command backed by ProjectionIndex; durable fields remain in canonical records. | Yes | `atelier-z1p.3` |
140:| Find work without a known ID | `bd search "<topic>"` | There is no root search command. Use metadata filters on `atelier issue list`, then `atelier issue show <id>` for the selected record. | Print a bounded flat inventory with stable IDs and exact metadata; detail remains a separate drill-down. | Canonical issue records plus the bounded inventory projection; full-text search is not part of the current public surface. | Yes | `atelier-z1p.3` |
155:| Mission dashboards | None in Beads MVP | `atelier work missions` for the cross-mission overview, `atelier work mission <mission-id>` for one mission, and `atelier work epic <epic-id>` for one epic. | Not required for first cutover; these bounded views now own current coordination. | Canonical records and ProjectionIndex-backed views. | No | Deferred at cutover; now specified |
198:| `bd list --status=open` | `atelier issue list --status todo` |
199:| `bd search "<topic>"` | Use metadata filters on `atelier issue list`, then `atelier issue show <id>`; root full-text search is not supported. |
239:- Rich UI beyond the bounded `atelier work missions`, `atelier work mission`,
1:Mission and proof oriented work coordination for agents
12:  st
```

## Stderr

Bytes: 1529
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.18s
────────────
 Nextest run ID 786070b6-ed51-4361-a463-5a066bbe3462 with nextest profile: default
    Starting 9 tests across 9 binaries (708 tests skipped)
        PASS [   0.010s] (1/9) atelier-app command_surface::tests::visible_grouped_review_references_target_subcommand_help
        PASS [   0.010s] (2/9) atelier-app command_surface::tests::root_help_parser_includes_work_section
        PASS [   0.010s] (3/9) atelier-app command_surface::tests::expands_slash_command_references
        PASS [   0.010s] (4/9) atelier-app command_surface::tests::nested_visible_group_references_target_nested_subcommand_help
        PASS [   0.012s] (5/9) atelier-app command_surface::tests::no_argument_issue_transition_reference_targets_subcommand_help
        PASS [   0.012s] (6/9) atelier-app command_surface::tests::subcommand_help_parser_extracts_commands_section
        PASS [   0.012s] (7/9) atelier-app command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.012s] (8/9) atelier-app command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.190s] (9/9) atelier-cli::cli_integration setup_guidance::test_obsolete_command_surfaces_are_removed_without_aliases
────────────
     Summary [   0.192s] 9 tests run: 9 passed, 708 skipped
```

