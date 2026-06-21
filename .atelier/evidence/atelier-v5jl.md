---
created_at: "2026-06-21T20:17:55.551866920+00:00"
id: "atelier-v5jl"
evidence_type: "validation"
captured_at: "2026-06-21T20:17:53.813305403+00:00"
command: "sh -c 'set -eu\nprintf \"== atelier-y3fj ==\\n\"\ntarget/debug/atelier issue show atelier-y3fj | sed -n \"1,70p\"\nprintf \"== atelier-76j0 ==\\n\"\ntarget/debug/atelier issue show atelier-76j0 | sed -n \"1,70p\"\nprintf \"== mission history ==\\n\"\ntarget/debug/atelier history --mission atelier-53bu --limit 3\nprintf \"== lint ==\\n\"\ntarget/debug/atelier lint'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-f9ci"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f9ci"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Final mission rework validation epic proof"
updated_at: "2026-06-21T20:18:00.478145523+00:00"
---

## Summary

Final mission rework validation epic proof

## Command

```console
sh -c 'set -eu
printf "== atelier-y3fj ==\n"
target/debug/atelier issue show atelier-y3fj | sed -n "1,70p"
printf "== atelier-76j0 ==\n"
target/debug/atelier issue show atelier-76j0 | sed -n "1,70p"
printf "== mission history ==\n"
target/debug/atelier history --mission atelier-53bu --limit 3
printf "== lint ==\n"
target/debug/atelier lint'
```

Exit status: 0

## Stdout

Bytes: 5999
Truncated: yes

```text
== atelier-y3fj ==
atelier-y3fj [validation] done - Validate docs help and Agent Factory guidance after mission rework
===================================================================================================
Status:   done
Category: done
Type:     validation
Priority: high
Created:  2026-06-21 12:37 -04:00
Updated:  2026-06-21 16:11 -04:00
Closed:   2026-06-21 16:11 -04:00
Labels:   mission-rework, validation
File:     /root/atelier/.atelier/issues/atelier-y3fj.md

Hierarchy
---------
Parent: atelier-f9ci [validation] high - Epic: Validate mission rework end to end

Branch Policy
----------------
Owner:    epic atelier-f9ci (epic)
Expected: epic/atelier-f9ci
Base:     master
Scope:    nested under epic; merge is deferred to epic close
Current:  epic/atelier-f9ci
State:    dirty checkout:  M .atelier/issues/atelier-f9ci.md; ?? .atelier/issues/atelier-f9ci.activity/20260621T201548184498Z.md; ?? .atelier/issues/atelier-f9ci.activity/20260621T201548184629Z.md
Options:  atelier issue transition atelier-y3fj --options
Checkout: atelier status

Transition Readiness
--------------------
  options: blocked - Issue atelier-y3fj has no configured transitions from status 'done'
  options: atelier issue transition atelier-y3fj --options

Description
-----------
Check public docs, root help, role guides, command audits, and Agent Factory bindings after mission rework lands.

Outcome
-------
- No normal guidance teaches removed mission command forms.
- Role guides point managers and orchestrators to bundle, issue, status, evidence, history, and lint surfaces.
- Command-surface lint or targeted searches catch stale examples.

Evidence
--------
- Evidence record includes root help/man excerpts or bounded summaries, targeted `rg` output, and `atelier lint` result.
- Any historical references are explicitly classified as migration history or removed-command tests.

Close Reason
------------
Validated docs help and Agent Factory guidance after mission rework

Blocked by
----------
  atelier-2kfb [done] high - Delete mission root namespace and mission command shims
  atelier-kivn [done] high - Remove mission-specific lifecycle and focus code
  atelier-kka3 [done] high - Remove mission-specific projection and storage branches

Blocking
--------
(none)

Subissues
---------
(none)

Impact
------
No downstream issues found.

Recent Activity
== atelier-76j0 ==
atelier-76j0 [validation] done - Run mission rework scenario validation
=======================================================================
Status:   done
Category: done
Type:     validation
Priority: high
Created:  2026-06-21 12:37 -04:00
Updated:  2026-06-21 16:14 -04:00
Closed:   2026-06-21 16:14 -04:00
Labels:   mission-rework, validation
File:     /root/atelier/.atelier/issues/atelier-76j0.md

Hierarchy
---------
Parent: atelier-f9ci [validation] high - Epic: Validate mission rework end to end

Branch Policy
----------------
Owner:    epic atelier-f9ci (epic)
Expected: epic/atelier-f9ci
Base:     master
Scope:    nested under epic; merge is deferred to epic close
Current:  epic/atelier-f9ci
State:    dirty checkout:  M .atelier/issues/atelier-f9ci.md; ?? .atelier/issues/atelier-f9ci.activity/20260621T201548184498Z.md; ?? .atelier/issues/atelier-f9ci.activity/20260621T201548184629Z.md
Options:  atelier issue transition atelier-76j0 --options
Checkout: atelier status

Transition Readiness
--------------------
  options: blocked - Issue atelier-76j0 has no configured transitions from status 'done'
  options: atelier issue transition atelier-76j0 --options

Description
-----------
Independently validate the new mission/objective workflow from a clean checkout-style path.

Outcome
-------
- Validation transcript creates or uses a declared mission/objective type, links work, inspects status, records evidence, exercises blockers, and reaches terminal readiness.
- Validation transcript proves old mission-only commands do not remain as supported workflow.
- Residual risks and any follow-up issue IDs are recorded.

Evidence
--------
- First-class evidence record with command trans
```

## Stderr

Bytes: 0
Truncated: no

```text
```

