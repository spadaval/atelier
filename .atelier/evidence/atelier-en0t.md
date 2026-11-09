---
created_at: "2026-11-09T20:14:45.242432587+00:00"
id: "atelier-en0t"
evidence_type: "validation"
commit: "6f661b97ebad43dab6945361a3c31c3c5bafe0af"
result: "pass"
captured_at: "2026-11-09T20:14:45.242352859+00:00"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-59fz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-59fz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "scripts/smoke-repos.sh --new <this tree's debug build>: no regression across 5 repositories. git grep -E -- '--html|runtime/reviews' over skills/, docs/ (strategies aside), PRODUCT_INTENT.md, CONTEXT.md, README.md and crates/*/src/ finds nothing. On this host, with ~/.config/atelier/site-url set, the debug build's brief atelier-bnek and debrief atelier-kgcr footers name http://buildbot.tail47515a.ts.net:7341/atelier/mission/<id>/brief|debrief, which the live site answers 200; brief --html exits 2 with clap's unexpected-argument error."
updated_at: "2026-11-09T20:14:45.376199191+00:00"
---

scripts/smoke-repos.sh --new <this tree's debug build>: no regression across 5 repositories. git grep -E -- '--html|runtime/reviews' over skills/, docs/ (strategies aside), PRODUCT_INTENT.md, CONTEXT.md, README.md and crates/*/src/ finds nothing. On this host, with ~/.config/atelier/site-url set, the debug build's brief atelier-bnek and debrief atelier-kgcr footers name http://buildbot.tail47515a.ts.net:7341/atelier/mission/<id>/brief|debrief, which the live site answers 200; brief --html exits 2 with clap's unexpected-argument error.
