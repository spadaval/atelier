---
created_at: "2026-06-21T20:08:26.375162003+00:00"
id: "atelier-ao12"
evidence_type: "validation"
captured_at: "2026-06-21T20:08:24.936114531+00:00"
command: "sh -c 'set -eu\nprintf \"== history mission scope ==\\n\"\ntarget/debug/atelier history --mission atelier-53bu --limit 3\nprintf \"== live guidance stale mission root search ==\\n\"\nrg -n \"atelier mission|mission (start|status|list|view|close)|mission command\" AGENTS.md .agents/skills/agent-factory docs/product docs/architecture/quality/validation.md docs/spec -g \"*.md\" || true\nprintf \"== lint ==\\n\"\ntarget/debug/atelier lint'"
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
title: "Post-fix mission history and guidance validation"
updated_at: "2026-06-21T20:08:31.299707989+00:00"
---

## Summary

Post-fix mission history and guidance validation

## Command

```console
sh -c 'set -eu
printf "== history mission scope ==\n"
target/debug/atelier history --mission atelier-53bu --limit 3
printf "== live guidance stale mission root search ==\n"
rg -n "atelier mission|mission (start|status|list|view|close)|mission command" AGENTS.md .agents/skills/agent-factory docs/product docs/architecture/quality/validation.md docs/spec -g "*.md" || true
printf "== lint ==\n"
target/debug/atelier lint'
```

Exit status: 0

## Stdout

Bytes: 5325
Truncated: yes

```text
== history mission scope ==
History
=======
Scope:          mission atelier-53bu - Make workflow obligations explicit and minimal
Source:         canonical .atelier issue activity, records, evidence, status roles, review artifacts, and record links; local runtime diagnostics excluded
Ordering:       newest first, timestamp then record/path
Limit:          3
Filters:        (none)
Showing:        3 of 317 matching events

Events
------
  2026-06-21 16:05 -04:00 | evidence_attached | root | issue/atelier-y3fj | Validate docs help and Agent Factory guidance after mission rework | Attached evidence atelier-jhf6
  2026-06-21 16:05 -04:00 | evidence_attached | (system) | issue/atelier-fyc9 | Normalize mission work and blocker relationships under issue commands | Attached evidence atelier-zpit to issue/atelier-fyc9 (validates)
  2026-06-21 16:05 -04:00 | evidence_attached | (system) | issue/atelier-2kfb | Delete mission root namespace and mission command shims | Attached evidence atelier-wrnt to issue/atelier-2kfb (validates)
Omitted:        314 older matching events hidden by --limit 3

Next Commands
-------------
  atelier issue show atelier-53bu
  atelier issue status atelier-53bu
  atelier history --mission atelier-53bu --limit 3
  atelier history --event-kind <kind>
== live guidance stale mission root search ==
docs/architecture/quality/validation.md:14:lint run, or mission status page can support proof, but it is weak by itself
docs/architecture/quality/validation.md:91:| Agent-facing command freshness for `AGENTS.md`, product docs, and command-surface tests | Atelier-owned product behavior | `atelier lint`, `atelier issue status <mission-id> --verbose`, mission completion | Routine handoff uses visible lint/status surfaces; mission status and completion surface docs/help drift validators when a mission is being closed. |
docs/architecture/quality/validation.md:181:  evidence, mission status, Agent Factory process, or Mission Control;
docs/architecture/quality/validation.md:209:information-hierarchy issue for `mission list` should name the user task,
docs/architecture/quality/validation.md:212:should be concrete: "reduce `mission status` wall time on fixture X from about
docs/architecture/quality/validation.md:214:than "make mission status faster."
docs/architecture/quality/validation.md:254:For a subjective `mission list` information-hierarchy task:
docs/architecture/quality/validation.md:260:- Executable issue outcome says `mission list` output groups active missions by
docs/architecture/quality/validation.md:364:mission status, validation issue output, and completion failure output rather
docs/architecture/quality/validation.md:383:  focused `show` commands, `issue transition --options`, and mission status or
docs/spec/storage/export/rebuild/canonical-layout.md:441:| Direct mission blockers | Mission `relationships.relates[]` entries with `kind: issue` and `type: blocked_by`. Linked work item blockers remain ordinary issue dependency edges and are projected into mission status; do not duplicate them in mission prose. |
docs/spec/storage/export/rebuild/canonical-layout.md:506:Repair the CLI workflow and validation gaps so mission closeout can be audited
docs/product/command-audit/issue.md:36:create compatibility aliases for the removed mission commands.
docs/product/command-audit/issue.md:61:| `issue status` | Worker/reviewer | Inspect type-aware objective health and terminal readiness. | Add. This absorbs the useful `mission status` behavior for objective records. |
docs/product/command-audit/rebuild.md:32:and a successful rebuild is not evidence that issue content, mission closeout,
docs/product/command-audit/mission.md:1:# `atelier mission`
docs/product/command-audit/mission.md:29:  `mission status`: ready and blocked work, proof gaps, open blockers,
docs/product/command-audit/mission.md:45:`mission start` and active mission focus are removed rather than renamed.
docs/product/command-audit/mission.md:62:  mission commands for removal sequencing, but target guidance should teach the
docs/product/command-audit/mission.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

