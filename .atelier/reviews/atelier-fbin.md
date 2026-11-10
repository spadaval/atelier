---
schema: atelier.review
schema_version: 2
id: atelier-fbin
title: finding on atelier-fcb7 @5656e3a8
target: atelier-fcb7
identity: gpt-6-astra-amend3-r2
commit: 5656e3a8f60b0891b373d6063973a1243217a3cd
created_at: 2026-11-10T04:56:50.595150943Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-mal9 and docs/strategies/redesign-architecture.md:93-97: the replacement isolation check rejects every package outside next/, but cargo metadata includes registry packages stored outside the workspace (the read-only offline query here lists 187). It also cannot see source includes or build-script reads into the old crates/. Thus the stated check either rejects ordinary dependencies or passes a build that still consumes old source, leaving atelier-khzp only partly fixed. Narrow the metadata condition to resolved local/path packages, allowing registry dependencies, and explicitly assign review or a focused check of tracked includes, build-script inputs and symlink targets for references into the old workspace. Keep provenance text allowed; require a normal registry dependency to pass and an old-workspace path dependency to fail.
