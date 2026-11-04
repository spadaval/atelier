---
created_at: "2026-11-04T23:12:44.988074307+00:00"
id: "atelier-veqw"
evidence_type: "validation"
commit: "97a94898a5904088823950811b40ab9712d613a3"
result: "pass"
captured_at: "2026-11-04T23:12:42.719413595+00:00"
command: "/home/supadava/.cache/atelier-g6a1/s4-check.sh"
exit_status: "0"
path: "README.md"
agent_identity: "Claude Opus 5.5 (independent validator, atelier-g6a1)"
target:
  kind: "issue"
  id: "atelier-g6a1"
  role: "validates"
criteria:
- slug: "S4"
  result: "pass"
criteria_mission: "atelier-vt8q"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g6a1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S4 at HEAD: the 5 tests that assert the stale-binary (newer record/policy schema) message pass and the message names the release installer one-liner plus 'cargo install --path crates/atelier-cli --locked' for source checkouts; README Install names the installer for install and upgrade and keeps the source-checkout command; docs/product/development-setup.md points to the README installer and keeps the checkout command; Cargo.toml repository/homepage = github.com/spadaval/atelier, inherited by every crate; CHANGELOG 0.3.0 describes Markdown records under .atelier/ and mentions SQLite only as what it replaced. Script: ~/.cache/atelier-g6a1/s4-check.sh (contents in an atelier-g6a1 note). Limits: Linux only locally (CI runs the same tests on macOS); doc checks are string presence plus my reading of the sections, not a full docs audit; only five message tests were run, not the suite."
updated_at: "2026-11-04T23:12:44.988369318+00:00"
---

## Summary

S4 at HEAD: the 5 tests that assert the stale-binary (newer record/policy schema) message pass and the message names the release installer one-liner plus 'cargo install --path crates/atelier-cli --locked' for source checkouts; README Install names the installer for install and upgrade and keeps the source-checkout command; docs/product/development-setup.md points to the README installer and keeps the checkout command; Cargo.toml repository/homepage = github.com/spadaval/atelier, inherited by every crate; CHANGELOG 0.3.0 describes Markdown records under .atelier/ and mentions SQLite only as what it replaced. Script: ~/.cache/atelier-g6a1/s4-check.sh (contents in an atelier-g6a1 note). Limits: Linux only locally (CI runs the same tests on macOS); doc checks are string presence plus my reading of the sections, not a full docs audit; only five message tests were run, not the suite.

## Command

```console
/home/supadava/.cache/atelier-g6a1/s4-check.sh
```

Exit status: 0

## Stdout

Bytes: 288
SHA-256: 683cdcf68ba2e753984bbba233f511d17334be9a3985303e58b48414fb1f4969

## Stderr

Bytes: 1008
SHA-256: 321e8cc17a1be3e15d6a8605b9d53270e00b08871bd42ac7f7aca77820f50290
Test summary: Summary [   0.285s] 5 tests run: 5 passed, 764 skipped
