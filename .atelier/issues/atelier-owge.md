---
created_at: "2026-06-25T15:24:02.913748258+00:00"
id: "atelier-owge"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Specify semantic style tokens and formatting preferences"
updated_at: "2026-06-25T15:24:02.913748258+00:00"
---

## Description

Define semantic styling for headings, secondary/context text, ready/success, warnings, danger/blocked/fail, and active states. Text must remain complete when color is disabled.

## Outcome

Semantic style tokens and formatting preferences are documented for the panel system. The design keeps tokens Atelier-owned, maps them through StylePolicy to the chosen ANSI/style library, keeps raw color-library calls out of panel and command code, and proves colorless output preserves all status, blocker, diagnostic, and action meaning in text.
