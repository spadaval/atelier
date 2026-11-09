---
created_at: "2026-11-08T21:50:02.001056830+00:00"
id: "atelier-2yef"
issue_type: "task"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-41xn"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "skills/agent-factory/references/atelier.md:15-16 vs :66-67, and procedures/orchestrate.md:89 (eb14a31). Declining a sta…"
updated_at: "2026-11-09T00:00:54.936103655+00:00"
---

## Goal

Deferred from finding atelier-41xn on atelier-l58v @9ea0a19b, by `atelier review defer`.

medium: skills/agent-factory/references/atelier.md:15-16 vs :66-67, and procedures/orchestrate.md:89 (eb14a31). Declining a started item is now a normal ending (a stop signal's discard, a standing reject), but the skill does not say where its record goes or what happens to the branch: atelier.md says decline needs 'no branch, review or merge' while also saying 'a started item's own record changes only on its work branch'. A reader following the second sentence declines on the work branch, archives it, and the base shows the item in_progress forever. All declining eval runs guessed the same way (decline committed on main, branch tagged archive/<id> and deleted, its follow-ups and findings left only on the tag), so no failure was observed. Fix: one sentence in atelier.md saying a declined started item's record is committed on the base and its branch archive-tagged and deleted, and that follow-ups filed on that branch lapse with it.

## Done when

Done when was not specified.
