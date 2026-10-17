---
schema: atelier.review
schema_version: 1
id: atelier-spfr
title: crates/atelier-cli/src/human_output.rs:271-288 — ItemLine::render_wrapped measures and wraps self.text(), so a row too wide for a colour terminal loses its status/priority/lead colours entirely; IssueListPanel rows did not wrap on master, so narrow TTYs (COLUMNS=40 work all) newly print those rows uncoloured. Wrapping can also split '⚠ todo' or '| 1 blocker' across lines. Low impact (redirected output is unaffected); a follow-up could wrap the painted segments or keep the mark with its word.
target: atelier-i2qq
identity: Independent reviewer (Claude subagent)
commit: c4b06e91043b5f0b07fce4a2cb9a0ba9e6cd2945
created_at: 2026-10-17T06:18:11.121638255Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/human_output.rs:271-288 — ItemLine::render_wrapped measures and wraps self.text(), so a row too wide for a colour terminal loses its status/priority/lead colours entirely; IssueListPanel rows did not wrap on master, so narrow TTYs (COLUMNS=40 work all) newly print those rows uncoloured. Wrapping can also split '⚠ todo' or '| 1 blocker' across lines. Low impact (redirected output is unaffected); a follow-up could wrap the painted segments or keep the mark with its word.
