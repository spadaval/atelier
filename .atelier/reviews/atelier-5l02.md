---
schema: atelier.review
schema_version: 1
id: atelier-5l02
title: approval on atelier-ot1c @06ab91f8
target: atelier-ot1c
identity: claude-reviewer-ot1c
commit: 06ab91f810a29db8a0aaa1fd7a790ba2f19c9e5b
created_at: 2026-11-04T22:43:38.169833886Z
event:
  event: approval
---

Re-reviewed c4b7c6f5..06ab91f8. Fixes for atelier-rbyl, m0x6, zrt0, yelg verified. Ran scripts/merge-ci-passed.sh in synthetic merge checkouts built on the g8cn branch (real CI success on 95a48e1a): passes for historical merge, record commit after CI, base record-only drift, record-only branch (also with base code drift), and the real record-only merges 728aa41c and 11e654e7; fails for base code drift, base workflow.yaml and config.toml drift, policy-only and config-only branches without CI, workflow.yaml change after CI; exits 2 with a clear message outside a merge commit; earlier run showed gh auth failure fails closed. Checked merge.rs: merge_commit always builds a two-parent commit and shell checks run with cwd at the merged checkout. git diff --check, cargo fmt --check, cargo metadata --locked pass. AGENTS.md, repository-contribution.md, standards.md and development-setup.md agree on the rule and the targeted-local/full-in-CI guidance. Full nextest suites left to PR CI by instruction.
