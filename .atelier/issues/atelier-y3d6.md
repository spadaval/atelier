---
created_at: "2026-11-08T21:50:02.001056830+00:00"
id: "atelier-y3d6"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-zwkw"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T22:23:18.757499464+00:00"
status: "declined"
title: "skills/agent-factory/procedures/orchestrate.md:77 (eb14a31). 'record fix_now with why it cannot wait' means review disp…"
updated_at: "2026-11-08T22:23:18.757499464+00:00"
---

## Goal

Deferred from finding atelier-zwkw on atelier-l58v @9ea0a19b, by `atelier review defer`.

low: skills/agent-factory/procedures/orchestrate.md:77 (eb14a31). 'record fix_now with why it cannot wait' means review disposition --decision fix_now. It works on 0.3.1+44cbe69c (tested in a scratch repo: disposition fix_now on a defer finding, then a later approval, merge --check passes), but the binary's help describes --decision as defer, no_action or wont_fix and calls a fix_now disposition an older Atelier's form (--resolution-commit). The skill relies on a value its help treats as legacy. Fix: either the help documents fix_now as overriding a defer recommendation, or the skill says to record the reason as a decision note and fix it, with no disposition.

## Done when

Done when was not specified.

## Report

Moot: agents-skills 12c8a15 removed the 'record fix_now' sentence on Sumeet's direction; the skill no longer records a scope choice as a fix_now disposition.
