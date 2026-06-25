---
created_at: "2026-06-25T15:23:01.075759811+00:00"
id: "atelier-jcyc"
issue_type: "mission"
labels:
- "cli"
- "human-output"
- "rendering"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-g1vp"
    type: "advances"
  - kind: "issue"
    id: "atelier-idef"
    type: "advances"
  - kind: "issue"
    id: "atelier-lijo"
    type: "advances"
  - kind: "issue"
    id: "atelier-qusw"
    type: "advances"
  - kind: "issue"
    id: "atelier-sn3u"
    type: "advances"
  - kind: "issue"
    id: "atelier-ww1a"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "ready"
title: "Build CLI presentation panel rendering system"
updated_at: "2026-06-25T15:23:01.075759811+00:00"
---

## Description

Introduce a small CLI presentation layer for read-oriented command output. The layer should let commands compose specific panels such as status summaries, checkout state, issue lists, workflow transitions, objective rollups, diagnostics, and next actions while keeping domain truth in atelier-app and command input/validation in command handlers. The goal is more consistent, colorful, scan-friendly human output without turning Atelier into a generic terminal UI framework.

Constraints:
- Panels are presentation components over already-computed view data; they must not query storage, resolve IDs, run validators, or mutate state.
- Use specific panels that reflect user-visible output shapes; avoid generic key/value dumps as the default design center.
- Color and styling must be semantic, terminal-aware, honor NO_COLOR, and leave colorless output complete.
- Convert incrementally: status and work first, then selected issue show read panels, before considering mutation outputs.
- Command input, dispatch, workflow semantics, storage/cache architecture, and quiet output contracts remain out of scope except where render context consumes output flags.

Risks:
- A generic panel framework could add abstraction without improving operator decisions.
- Semantic mistakes can creep back in if panels fetch data or decide readiness instead of rendering supplied view models.
- Color can harm logs and accessibility if it carries meaning that is not present in text.
- Broad output churn can destabilize tests; migrate one command surface at a time with focused regression coverage.

Validation:
- Render context and panel/component contracts are documented and tested, including color, NO_COLOR, non-terminal, row limits, and empty-state behavior.
- atelier status and atelier work are rendered through panels and preserve quiet output boundaries.
- Selected issue show sections use panels for transition readiness, objective rollup, issue lists, relationships, diagnostics, and footer actions without changing domain semantics.
- Focused CLI tests prove colorless output remains complete, placeholders stay omitted, actions are deduped/ranked, and bounded lists report omitted counts.
- target/debug/atelier check, cargo fmt -- --check, cargo check -p atelier-cli, focused CLI tests, and git diff --check pass before closeout.

## Outcome

Atelier has a small, documented CLI presentation layer for read-oriented output. Commands compose specific panels over existing view data, semantic color/style is centralized and terminal-aware, status and work use panel rendering, selected issue-show sections use panel rendering, and focused tests prove colorless output, quiet output, bounded lists, omitted placeholders, ranked actions, and NO_COLOR/non-terminal behavior.
