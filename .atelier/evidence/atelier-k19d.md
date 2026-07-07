---
created_at: "2026-07-06T20:11:15.888121508+00:00"
id: "atelier-k19d"
evidence_type: "validation"
captured_at: "2026-07-06T20:11:15.700363541+00:00"
command: "bash -lc '\nset -euo pipefail\nfailures=0\ncheck() {\n  local pattern=$1\n  local file=$2\n  if rg -n -- \"$pattern\" \"$file\"; then failures=$((failures + 1)); fi\n}\ncheck \"atelier work queue\" docs/product/work-view-ordering.md\ncheck \"Normal workflow.*work queue|work queue.*Normal workflow\" docs/product/command-audit/category-review.md\ncheck \"atelier work queue\" docs/architecture/markdown-first-record-store.md\ncheck \"atelier lint|atelier doctor --fix\" docs/spec/storage/export/rebuild/canonical-layout.md\ncheck \"atelier start|atelier issue close|atelier worktree for-mission\" docs/adr/0004-work-lock-sync-policy.md\ncheck \"atelier start|atelier issue close\" docs/adr/0007-mission-workspaces-and-epic-review-branches.md\nprintf \"broad normative stale-guidance findings: %d file-pattern group(s)\\n\" \"$failures\" >&2\n((failures == 0))\n'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "SUPPLEMENTAL PR51 REVALIDATION: FAIL after maintenance-root repair. atelier-3ezp is resolved and atelier-fc6x's maintenance/coverage guard gap is superseded: verdict derivation, exact bare maintenance rejection, inventory 11/10/44/30, all 103 self-tests, allowances, indexed scan, help, focused tests, c0mp preservation, fmt, master-range whitespace, and atelier check pass. The required broad stale-guidance scan still finds current normative retired guidance outside the quality-only guard: work-view-ordering applies its live contract to removed work queue; category-review classifies work queue as Normal workflow; markdown-first-record-store still assigns behavior to work queue; canonical-layout directs normal edits through removed lint and doctor --fix; accepted ADR 0004/0007 still teach removed start, issue close, and worktree paths. Therefore atelier-p0am's product/spec/architecture/current-command outcome and the mission publication gate remain failed. No implementation changes made."
updated_at: "2026-07-06T20:11:42.883054357+00:00"
---

## Summary

SUPPLEMENTAL PR51 REVALIDATION: FAIL after maintenance-root repair. atelier-3ezp is resolved and atelier-fc6x's maintenance/coverage guard gap is superseded: verdict derivation, exact bare maintenance rejection, inventory 11/10/44/30, all 103 self-tests, allowances, indexed scan, help, focused tests, c0mp preservation, fmt, master-range whitespace, and atelier check pass. The required broad stale-guidance scan still finds current normative retired guidance outside the quality-only guard: work-view-ordering applies its live contract to removed work queue; category-review classifies work queue as Normal workflow; markdown-first-record-store still assigns behavior to work queue; canonical-layout directs normal edits through removed lint and doctor --fix; accepted ADR 0004/0007 still teach removed start, issue close, and worktree paths. Therefore atelier-p0am's product/spec/architecture/current-command outcome and the mission publication gate remain failed. No implementation changes made.

## Command

```console
bash -lc '
set -euo pipefail
failures=0
check() {
  local pattern=$1
  local file=$2
  if rg -n -- "$pattern" "$file"; then failures=$((failures + 1)); fi
}
check "atelier work queue" docs/product/work-view-ordering.md
check "Normal workflow.*work queue|work queue.*Normal workflow" docs/product/command-audit/category-review.md
check "atelier work queue" docs/architecture/markdown-first-record-store.md
check "atelier lint|atelier doctor --fix" docs/spec/storage/export/rebuild/canonical-layout.md
check "atelier start|atelier issue close|atelier worktree for-mission" docs/adr/0004-work-lock-sync-policy.md
check "atelier start|atelier issue close" docs/adr/0007-mission-workspaces-and-epic-review-branches.md
printf "broad normative stale-guidance findings: %d file-pattern group(s)\n" "$failures" >&2
((failures == 0))
'
```

Exit status: 1

## Stdout

Bytes: 1556
Truncated: no

```text
9:- `atelier work queue`
9:| Normal workflow | `status`, `issue show`, `issue transition`, `work queue`, `work mission`, `evidence record`, `review show`, `check` | `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete`, provider setup commands | Normal commands answer operator questions in domain terms and may be cited for ordinary handoff, validation, committed-state health, and terminal readiness. |
306:rendering. `atelier work queue` also matches issue titles and bodies from
182:2. Run `atelier lint` to validate schema, path, front matter, relationships,
185:   `atelier doctor --fix` when local projection/runtime repair is explicitly
191:but `atelier lint` and repair commands may report non-canonical ordering as
250:- `atelier lint atelier-z1p8` reports no findings.
282:2. Run `atelier lint`.
286:4. Run `atelier doctor --fix` if ignored local projection/runtime state is
288:5. Re-run `atelier lint` and the workflow validator for the issue, epic, or
335:When `atelier lint` reports invalid canonical Markdown, fix the Markdown rather
30:Root `atelier start`, `atelier issue close`, root `atelier status`, and
31:`atelier worktree for-mission` own the default ergonomic path:
43:not own routine branch preparation, which belongs to `atelier start`.
48:   Routine workers use `atelier start <id>` to prepare the correct owner
51:   `atelier issue close <id>` commits the close-state tracker change on that
107:  lifecycle-owned branch preparation through `atelier start <id>` rather than
```

## Stderr

Bytes: 65
Truncated: no

```text
broad normative stale-guidance findings: 6 file-pattern group(s)
```
