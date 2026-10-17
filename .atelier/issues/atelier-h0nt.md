---
created_at: "2026-10-17T01:10:57.970167371+00:00"
id: "atelier-h0nt"
issue_type: "feature"
labels:
- "bundle"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Draw bundle preview as a tree with type, status and new/updated marks"
updated_at: "2026-10-17T01:10:57.970167371+00:00"
---

## Goal

Bundle preview and apply print counts, a flat `client_ref -> (new)  title` list and a flat edge list. Excerpt:

```
Bundle preview is valid.
Applied:       false
Preview:       true
Records
-------
  issues: 5
  updated: 1
...
Relationships
-------------
  issue/m -> issue/p (advances)
  issue/p -> issue/p.a (parent)
  issue/atelier-wtnp -> issue/p.b (blocks)
```

Problems: each record's type, initial status and priority are never shown, so you cannot confirm that a defaulted entry became a `todo` task or that `standalone` became a low-priority `backlog` bug. Existing records appear as bare IDs with no title. The `issue/` prefix repeats on every edge. `Applied:` and `Preview:` repeat the header; they are booleans from the internal JSON summary (crates/atelier-cli/src/commands/bundle.rs:1470-1482). `issues: 5` next to `updated: 1` reads as if 5 includes the update. After apply the edges become bare ID pairs. Answering "what graph will this create" means rebuilding the tree by hand.

## Done when

Preview and apply render one line per record, nested under its mission and parent, with blockers inline and existing records titled. A snapshot test covers a bundle with a mission, a parent with two children, a standalone item, a blocker on an existing ID and an update entry. Target sketch:

```
Bundle "Make prune explain itself": 5 new, 1 updated (preview; nothing written)
◆ m  new mission · draft · high   Prune output explains every kept record
  ├ p  new feature · todo        Summarise prune protection by root
  │ ├ p.a  new task · todo       Compute retained roots
  │ └ p.b  new task · todo · blocked by p.a, old, atelier-wtnp (Define bundle JSON schema, todo)
  └ standalone  new bug · backlog · low   Drop the hard-coded deferred section
~ old = atelier-uyuu  updated
```

Apply prints the same tree with allocated IDs (`m = atelier-fz5l`). The `Applied:`/`Preview:` lines and the `issue/` prefixes are gone.

## Context

Follows the one-line-per-item rule in ADR 0022 and atelier-deyy. Field changes for update entries are a separate item in this mission.
