---
created_at: "2026-10-17T01:10:57.965183217+00:00"
id: "atelier-7ebi"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Confirm issue update, note and link with the resulting values"
updated_at: "2026-10-17T01:10:57.965183217+00:00"
---

## Goal

Mutating issue commands confirm what was run but not what the record now says, so agents run `issue show` straight afterwards. In astromech, 178 of 572 mutating issue commands are followed within two commands by `issue show`; in atelier, 48 of 347.
- `issue update` (`crates/atelier-cli/src/commands/issue.rs:972-994`) prints `Updated issue atelier-db6h (labels, priority, title)` and then Status/Priority/Type whatever changed. The new title and labels are never shown. `--remove-label nonexistent` still reports `labels` as changed (`issue.rs:886-889`). The only next command is `atelier issue show <id>`.
- `issue note` (`issue.rs:997-1018`) prints `Added note to issue atelier-db6h` without the stored line, and `-q` doesn't suppress it.
- `issue link` (`crates/atelier-cli/src/commands/relate.rs:55-62,107-113`): `issue link 1w8c f0rr` prints `Linked atelier-1w8c -> atelier-f0rr (advances)`, but the stored link is f0rr advances 1w8c. Rerunning in reverse prints `Link atelier-1w8c -> atelier-xmtr (advances) already exists`, which is not the stored direction. No titles are shown. The footer offers `atelier issue transition <source>`, which is unrelated.

## Done when

- `update` prints only the changed fields, with old and new values: `priority: medium → critical`, `title: "A" → "B"`, `labels: +size-s −foo`. Removing a label that isn't set says `label foo was not set` and is not counted as a change.
- `note` prints the stored bullet, e.g. `- 2026-10-16 [plan] Plan note`. `-q` prints nothing.
- `link`/`unlink` describe the stored link in canonical direction with titles, e.g. `atelier-f0rr "No body at all" now advances mission atelier-1w8c "Orient a session…"`. For blocked_by they add the readiness effect: `atelier-xmtr is now blocked (1 open blocker)`. "Already exists" uses the same canonical wording. The generic footer is gone.
- CLI tests cover each confirmation, including a mission-first advances link.
