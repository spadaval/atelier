---
schema: atelier.review
schema_version: 2
id: atelier-fqry
title: 'crates/atelier-cli/tests/cli_integration/criteria.rs: the mark rules that carry the honesty argument are untested: a newer receipt pinned outside HEAD''s history being skipped for an older one, blocked mapping to failing, not_applicable mapping to unproven, and work mission in a repository with no commits (manually verified working here). Fix: add a CLI test that records proof on a side branch and checks the mark on the base, plus the result mappings.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:23.961130640Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e
    recorded_at: 2026-10-17T18:41:33.235898753Z
    resolution_commit: fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e
    reason: Fixed in fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e with tests; see the commit message
    migrated_from: atelier-x7j2
---

crates/atelier-cli/tests/cli_integration/criteria.rs: the mark rules that carry the honesty argument are untested: a newer receipt pinned outside HEAD's history being skipped for an older one, blocked mapping to failing, not_applicable mapping to unproven, and work mission in a repository with no commits (manually verified working here). Fix: add a CLI test that records proof on a side branch and checks the mark on the base, plus the result mappings.
