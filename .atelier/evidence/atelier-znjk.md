---
created_at: "2026-07-06T18:49:13.873339912+00:00"
id: "atelier-znjk"
evidence_type: "validation"
captured_at: "2026-07-06T18:49:13.616364524+00:00"
command: "bash -c '\nset -eu\ngit show 60634f2b:docs/product/issue-inventory-and-mission-overview.md | rg -n \"atelier issue list|atelier work missions|atelier work mission <mission-id>|Mission Overview\"\nrg -n \"mission list.*work missions.*selection.*issue list.*inventory|work mission.*work epic.*issue list\" docs/product/command-audit/command-surface-cut-plan.md docs/product/command-audit/work.md\nif git diff master...HEAD -- docs PRODUCT_INTENT.md CONTEXT.md | rg -q \"Mission Overview|Showing <shown> of <matching> issues|at most 20 missions|at most 10 directly linked epics\"; then\n  exit 1\nfi\nif ./target/debug/atelier work epic atelier-eqq6 | rg -q \"atelier branch for-epic\"; then\n  exit 1\nfi\n./target/debug/atelier work epic atelier-eqq6 | rg -n \"Work Epic|Show epic record|Inspect transitions\"\n'"
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
title: "PASS c0mp contract sequencing comparison: durs guidance preserves the c0mp distinction between flat issue inventory, cross-mission selection via work missions, and scoped work mission/work epic drill-down; durs does not copy c0mp's replacement layout wording; integrated work epic output does not promote branch for-epic. Compared against mission/atelier-c0mp commit 60634f2b and issue atelier-vgqe."
updated_at: "2026-07-06T18:49:20.401359250+00:00"
---

## Summary

PASS c0mp contract sequencing comparison: durs guidance preserves the c0mp distinction between flat issue inventory, cross-mission selection via work missions, and scoped work mission/work epic drill-down; durs does not copy c0mp's replacement layout wording; integrated work epic output does not promote branch for-epic. Compared against mission/atelier-c0mp commit 60634f2b and issue atelier-vgqe.

## Command

```console
bash -c '
set -eu
git show 60634f2b:docs/product/issue-inventory-and-mission-overview.md | rg -n "atelier issue list|atelier work missions|atelier work mission <mission-id>|Mission Overview"
rg -n "mission list.*work missions.*selection.*issue list.*inventory|work mission.*work epic.*issue list" docs/product/command-audit/command-surface-cut-plan.md docs/product/command-audit/work.md
if git diff master...HEAD -- docs PRODUCT_INTENT.md CONTEXT.md | rg -q "Mission Overview|Showing <shown> of <matching> issues|at most 20 missions|at most 10 directly linked epics"; then
  exit 1
fi
if ./target/debug/atelier work epic atelier-eqq6 | rg -q "atelier branch for-epic"; then
  exit 1
fi
./target/debug/atelier work epic atelier-eqq6 | rg -n "Work Epic|Show epic record|Inspect transitions"
'
```

Exit status: 0

## Stdout

Bytes: 2960
Truncated: no

```text
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
docs/product/command-audit/work.md:44:the behavior belongs in `work mission`, `work epic`, or `issue list`.
docs/product/command-audit/work.md:55:| `work queue` | Legacy only | Browse repo-wide actionable work. | Retired from normal guidance. Use `work ready`, `work blocked`, `work active`, `work mission`, `work epic`, or `issue list` for the actual decision. |
docs/product/command-audit/command-surface-cut-plan.md:50:| `mission list` | `work missions` for mission selection, or `issue list --issue-type mission` for inventory |
docs/product/command-audit/command-surface-cut-plan.md:233:- `mission list` with `work missions` for selection or `issue list --issue-type mission` for inventory
1:Work Epic atelier-eqq6 - Epic: Hide or remove provider and recovery escape hatches
20:  Show epic record: atelier issue show atelier-eqq6
21:  Inspect transitions: atelier issue transition atelier-eqq6
```

## Stderr

Bytes: 0
Truncated: no

```text
```

