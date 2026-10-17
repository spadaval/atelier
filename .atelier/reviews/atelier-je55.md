---
schema: atelier.review
schema_version: 1
id: atelier-je55
title: 'crates/atelier-cli/src/human_output.rs:62 — stdout_width reads COLUMNS without checking that stdout is a terminal, so redirected output wraps whenever COLUMNS is exported (COLUMNS=30 atelier work ready | cat wraps every row at master and HEAD alike). That contradicts docs/product/human-cli-output.md:58-60 (''Redirected output has no width limit'') and this item''s premise that redirected output never wraps. Pre-existing and not worsened by this change; follow-up: take width only when stdout is a terminal.'
target: atelier-teld
identity: Independent reviewer (Claude subagent)
commit: ceb96d6ebfcc5fb62cfdd106508e0df37977ab40
created_at: 2026-10-17T09:26:33.928038503Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/human_output.rs:62 — stdout_width reads COLUMNS without checking that stdout is a terminal, so redirected output wraps whenever COLUMNS is exported (COLUMNS=30 atelier work ready | cat wraps every row at master and HEAD alike). That contradicts docs/product/human-cli-output.md:58-60 ('Redirected output has no width limit') and this item's premise that redirected output never wraps. Pre-existing and not worsened by this change; follow-up: take width only when stdout is a terminal.
