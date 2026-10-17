---
created_at: "2026-10-17T06:22:18.204925402+00:00"
id: "atelier-teld"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
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
status: "todo"
title: "Keep colour and whole tokens when an item line wraps"
updated_at: "2026-10-17T09:06:10.808633104+00:00"
---

## Goal

`ItemLine::render_wrapped` (`crates/atelier-cli/src/human_output.rs`) wraps the plain text of a line too wide for the terminal. On a narrow colour terminal the wrapped row loses all colour, and a wrap can split `⚠ todo` or `| 1 blocker` across lines. Redirected output, which agents read, never wraps, so only terminals are affected. Found in review of atelier-i2qq (finding atelier-spfr).

## Done when

- A wrapped item line keeps the colour of each field it paints on one line.
- A status with its mark, and each `| <cue>`, stays on one line unless it alone is wider than the terminal.
- Rendering tests cover a narrow coloured row.
