---
created_at: "2026-11-09T06:41:35.705830108+00:00"
id: "atelier-59fz"
issue_type: "task"
labels:
- "fleet-dashboard"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6rts"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Remove --html and .atelier/runtime/reviews, and send humans to the site's brief and debrief"
updated_at: "2026-11-09T06:41:47.247033464+00:00"
---

## Goal

Make the site the one place a human reads a brief or debrief: remove the CLI's `--html` output and `.atelier/runtime/reviews/`, and point Agent Factory and the docs at the site's pages, so agents stop writing HTML files or publishing ad-hoc artifacts. The Markdown `atelier brief` and `atelier debrief` stay for agents.

## Done when

- [ ] Provisional: `atelier brief` and `atelier debrief` no longer accept `--html`, nothing in the CLI writes `.atelier/runtime/reviews/`, and their footers no longer suggest `--html`; when the site's address is configured they print the mission's page URL. CLI tests show `--html` refused on both commands with no `.atelier/runtime/reviews/` created, and the URL printed when configured.
- [ ] Provisional: `skills/agent-factory/procedures/plan.md` and `orchestrate.md` tell agents to send the human the mission's site page, and `PRODUCT_INTENT.md`, `docs/product/mission-reviews.md` and `docs/product/human-cli-output.md` describe the site instead of `--html`; `git grep -n -E -- "--html|runtime/reviews"` over `skills/`, `docs/` (historical strategies aside), `PRODUCT_INTENT.md`, `CONTEXT.md`, `README.md` and `crates/*/src/` finds only code that refuses the flag, and tests under `crates/*/tests/` may keep naming it to prove the refusal.
- [ ] Provisional: every open item whose Done when names the HTML debrief or brief (atelier-wg6p, atelier-les9, atelier-rdsa, atelier-okfy, atelier-ztuq, atelier-k7vp and any other the search finds) names the site's page instead, with its wording otherwise unchanged; the Report lists them.
- [ ] `scripts/smoke-repos.sh` passes, and its Report names any repository whose own docs still call `--html`.

## Context

Can start only once loose end atelier-6rts has the site installed and published, so Sumeet never loses a way to read a brief or debrief. Sumeet's 2026-11-08 direction on atelier-yfqu: "the CLI's --html output and .atelier/runtime/reviews/ go away, while Markdown brief and debrief stay for agents."

## Unknowns

- How the CLI learns the site's address (an environment variable or host config); settle it at the start and record a decision note.
