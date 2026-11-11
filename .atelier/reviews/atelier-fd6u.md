---
schema: atelier.review
schema_version: 2
id: atelier-fd6u
title: finding on atelier-oci2 @30c445e1
target: atelier-oci2
identity: GPT-6.1 Sol independent planning reviewer
commit: 30c445e14e674de9cfacb4ba9ac4b6559dd8b599
created_at: 2026-11-11T03:01:27.299429398Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-oci2.md:42: low: The specification-correction checklist omits the prose immediately above the crate table. docs/strategies/redesign-architecture.md:77-79 still says the new workspace has the crates the old one has, less atelier-workflow. The old Cargo.toml lists six crates including atelier-site, whereas next/Cargo.toml lists only app, cli, core and records. That sentence therefore still promises atelier-site even after every listed table and dependency-direction check passes. Include this prose in the Done when line and require it to name the four tracker crates or explicitly omit both old crates, keeping any Desk addition separate. This is within the two allowed documents and outside Words, Lifecycle and Review. Most other named site corrections, including the cutover and rollback service actions, already hold at 30c445e1; preserve them rather than recreating the estimated 60-line rewrite. Planning review only; no Cargo command run.
