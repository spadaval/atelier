---
created_at: "2026-11-09T21:12:49.967045048+00:00"
id: "atelier-245a"
evidence_type: "validation"
commit: "56472e68d53c1f38d4ca05ad8baaed1aa3257ad3"
result: "pass"
captured_at: "2026-11-09T21:12:49.966928291+00:00"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "atelier-uma8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uma8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Host run on this branch's release build (cargo build --release -p atelier-cli at 8427aa68, the last code commit), served with 'atelier serve --bind 127.0.0.1:7351 --root ~/projects', then 'ATELIER_SERVE=http://127.0.0.1:7351 pnpm exec playwright test -c playwright.host.config.ts': inbox spec '5 repositories, 50 items, 5 draft missions, 61 pages, 0 failed'; missions spec '5 repositories, 42 missions, 84 pages, 0 failed'. /api/inbox count 55. atelier status -q human= on base checkouts matched the inbox: astromech 35, cosplay-studio 2, worldcrafter 2. The atelier-serve service was not touched."
updated_at: "2026-11-09T21:12:50.170600052+00:00"
---

Host run on this branch's release build (cargo build --release -p atelier-cli at 8427aa68, the last code commit), served with 'atelier serve --bind 127.0.0.1:7351 --root ~/projects', then 'ATELIER_SERVE=http://127.0.0.1:7351 pnpm exec playwright test -c playwright.host.config.ts': inbox spec '5 repositories, 50 items, 5 draft missions, 61 pages, 0 failed'; missions spec '5 repositories, 42 missions, 84 pages, 0 failed'. /api/inbox count 55. atelier status -q human= on base checkouts matched the inbox: astromech 35, cosplay-studio 2, worldcrafter 2. The atelier-serve service was not touched.
