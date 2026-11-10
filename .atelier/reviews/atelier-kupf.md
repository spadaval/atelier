---
schema: atelier.review
schema_version: 2
id: atelier-kupf
title: finding on atelier-yptz @3cb4e377
target: atelier-yptz
identity: gpt-6.1-sol-r1
commit: 3cb4e3779f9d8b7d862aebc8e751cffa3f714b73
created_at: 2026-11-10T20:12:35.466771099Z
event:
  event: finding
  recommendation: fix_now
---

R2, low severity: next/crates/atelier-cli/src/output.rs:469 prints repository-root-relative paths with plain git, although the command accepts invocation from a subdirectory. Reproduced at the pinned head in a scratch repository: from src/, new "Another item" succeeds and writes <repo>/.atelier/items/<id>/item.md, but running its Then line in that same directory exits 128 because Git looks under src/.atelier/items/<id>/item.md. set and ready use the same renderer; the same-checkout start readiness diagnostic and status commit recipe also omit the root. Anchor generated Git commands to the repository root, such as git -C <quoted-root>, or produce paths relative to the actual invocation directory. Add a test invoking a planning command from a nested directory and executing its Then line there, checking that it commits exactly the advertised paths. Recommendation: fix_now, because the Done when contract requires the printed Git command to commit the files it wrote.
