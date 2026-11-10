---
schema: atelier.review
schema_version: 2
id: atelier-v04q
title: finding on atelier-nafc @f317ac78
target: atelier-nafc
identity: gpt-6.1-sol
commit: f317ac78fdffcef88ab75a2773d9ee5f59688361
created_at: 2026-11-10T21:55:39.954941792Z
event:
  event: finding
  recommendation: fix_now
---

R4 .atelier/issues/atelier-nafc.md:52: medium. The prescribed local-bare remote cannot exercise the existing GitHub forge path. next/crates/atelier-cli/src/forge.rs Github::open parses the configured remote URL as a GitHub host/owner/repository and explicitly rejects local filesystem and file:// URLs before gh is invoked. Replacing every remote with a local bare path, as Scope also requires, therefore makes submit warn and merge refuse even with a correct gh double. Specify the existing isolated test pattern from next/crates/atelier-cli/tests/merge.rs: a synthetic GitHub-shaped URL with pushes rewritten to a scratch bare repository, plus the gh double. Make Git transport fail closed against any nonlocal destination and assert the double and local bare repository were actually used. Do not add filesystem-forge support to the product or weaken the converted forge configuration to make the rehearsal pass.
