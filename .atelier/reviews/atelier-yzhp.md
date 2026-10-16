---
schema: atelier.review
schema_version: 1
id: atelier-yzhp
title: 'docs/adr/0022-status-orients-and-output-is-dense.md:16-18 says prime''s guidance ''repeats what the harness already loads from AGENTS.md''. Claude Code, the only supported harness, loads CLAUDE.md, not AGENTS.md; this repo''s CLAUDE.md only says ''Use AGENTS.md as the shared agent entry point'' with no @AGENTS.md import (a Claude session here receives CLAUDE.md text only). Decision 2 removes the hook''s ''read AGENTS.md ... Agent Factory SKILL.md'' line on this premise, so after atelier-sdb0 a Claude session in this repo or any adopter without an import gets no automatic AGENTS.md content and only a one-line pointer. Fix: correct the claim, and either have Decision 2/sdb0 require CLAUDE.md to import AGENTS.md (e.g. setup claude or Agent Factory install checks it) or keep a one-line pointer in status.'
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:53.312149437Z
event:
  event: finding
  recommendation: fix_now
---

docs/adr/0022-status-orients-and-output-is-dense.md:16-18 says prime's guidance 'repeats what the harness already loads from AGENTS.md'. Claude Code, the only supported harness, loads CLAUDE.md, not AGENTS.md; this repo's CLAUDE.md only says 'Use AGENTS.md as the shared agent entry point' with no @AGENTS.md import (a Claude session here receives CLAUDE.md text only). Decision 2 removes the hook's 'read AGENTS.md ... Agent Factory SKILL.md' line on this premise, so after atelier-sdb0 a Claude session in this repo or any adopter without an import gets no automatic AGENTS.md content and only a one-line pointer. Fix: correct the claim, and either have Decision 2/sdb0 require CLAUDE.md to import AGENTS.md (e.g. setup claude or Agent Factory install checks it) or keep a one-line pointer in status.
