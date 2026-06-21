---
created_at: "2026-06-21T20:09:53.529954728+00:00"
id: "atelier-nt9y"
evidence_type: "validation"
captured_at: "2026-06-21T20:09:52.068070227+00:00"
command: "sh -c 'set -eu\nprintf \"== history mission scope ==\\n\"\ntarget/debug/atelier history --mission atelier-53bu --limit 3\nprintf \"== current guidance search ==\\n\"\nrg -n \"atelier mission|mission (start|status|list|view|close)|mission command\" AGENTS.md .agents/skills/agent-factory docs/product docs/architecture/quality/validation.md docs/spec -g \"*.md\" || true\nprintf \"== lint ==\\n\"\ntarget/debug/atelier lint'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-y3fj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-y3fj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Final docs guidance search after mission rework cleanup"
updated_at: "2026-06-21T20:09:58.712604303+00:00"
---

## Summary

Final docs guidance search after mission rework cleanup

## Command

```console
sh -c 'set -eu
printf "== history mission scope ==\n"
target/debug/atelier history --mission atelier-53bu --limit 3
printf "== current guidance search ==\n"
rg -n "atelier mission|mission (start|status|list|view|close)|mission command" AGENTS.md .agents/skills/agent-factory docs/product docs/architecture/quality/validation.md docs/spec -g "*.md" || true
printf "== lint ==\n"
target/debug/atelier lint'
```

Exit status: 0

## Stdout

Bytes: 3563
Truncated: no

```text
== history mission scope ==
History
=======
Scope:          mission atelier-53bu - Make workflow obligations explicit and minimal
Source:         canonical .atelier issue activity, records, evidence, status roles, review artifacts, and record links; local runtime diagnostics excluded
Ordering:       newest first, timestamp then record/path
Limit:          3
Filters:        (none)
Showing:        3 of 320 matching events

Events
------
  2026-06-21 16:08 -04:00 | evidence_attached | root | issue/atelier-y3fj | Validate docs help and Agent Factory guidance after mission rework | Attached evidence atelier-ao12
  2026-06-21 16:08 -04:00 | evidence_attached | (system) | issue/atelier-fyc9 | Normalize mission work and blocker relationships under issue commands | Attached evidence atelier-zpit to issue/atelier-fyc9 (validates)
  2026-06-21 16:08 -04:00 | evidence_attached | (system) | issue/atelier-2kfb | Delete mission root namespace and mission command shims | Attached evidence atelier-wrnt to issue/atelier-2kfb (validates)
Omitted:        317 older matching events hidden by --limit 3

Next Commands
-------------
  atelier issue show atelier-53bu
  atelier issue status atelier-53bu
  atelier history --mission atelier-53bu --limit 3
  atelier history --event-kind <kind>
== current guidance search ==
docs/spec/storage/export/rebuild/canonical-layout.md:506:Repair the CLI workflow and validation gaps so mission closeout can be audited
docs/product/command-audit/rebuild.md:32:and a successful rebuild is not evidence that issue content, mission closeout,
docs/product/command-audit/mission.md:1:# `atelier mission`
docs/product/command-audit/mission.md:29:  `mission status`: ready and blocked work, proof gaps, open blockers,
docs/product/command-audit/mission.md:45:`mission start` and active mission focus are removed rather than renamed.
docs/product/command-audit/mission.md:62:  mission commands for removal sequencing, but target guidance should teach the
docs/product/command-audit/mission.md:77:| `mission start --switch` | Manager/orchestrator | Set active mission focus. | Remove. Root status and canonical in-progress issue records own checkout orientation. |
docs/product/command-audit/mission.md:78:| `mission status` | Manager/orchestrator | See current mission health and next actions. | Remove after `issue status <objective-id>` owns objective health and terminal readiness. |
docs/product/command-audit/mission.md:79:| `mission status --verbose` | Reviewer | Inspect terminal-check detail. | Remove after type-aware issue status exposes terminal-check detail. |
docs/product/command-audit/mission.md:80:| `mission close --reason` | Manager/orchestrator | Close a mission after gates pass. | Remove. Replacement: `issue transition <objective-id> close --reason`. |
docs/product/command-audit/mission.md:81:| `mission list` | Manager/orchestrator | Select current or historical missions. | Remove. Replacement: `issue list --type objective` plus status/category filters. |
docs/product/command-audit/issue.md:36:create compatibility aliases for the removed mission commands.
docs/product/command-audit/issue.md:61:| `issue status` | Worker/reviewer | Inspect type-aware objective health and terminal readiness. | Add. This absorbs the useful `mission status` behavior for objective records. |
docs/spec/agent-factory/tracker-replacement-mvp.md:144:| Mission Control dashboards | None in Beads MVP | Future `atelier mission-control` or UI | Not required for first cutover. | Not required. | No | Deferred |
== lint ==
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

