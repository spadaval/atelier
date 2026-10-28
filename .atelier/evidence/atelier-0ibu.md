---
created_at: "2026-10-28T01:49:29.229807640+00:00"
id: "atelier-0ibu"
evidence_type: "ci"
commit: "575cf9d0abbb3682dda04458261b4e1ced61d016"
result: "pass"
captured_at: "2026-10-28T01:49:29.229698978+00:00"
uri: "https://github.com/spadaval/atelier/actions/runs/40616335699"
agent_identity: "Claude Opus 5.5 (orchestrator)"
target:
  kind: "issue"
  id: "atelier-7v9q"
  role: "validates"
criteria:
- slug: "S2"
  result: "pass"
criteria_mission: "atelier-vt8q"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7v9q"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "PR-mode Release run 40616335699 at 575cf9d0 (PR #13, merge commit 8c10ce75) succeeded without publishing and uploaded: atelier-cli-{x86_64,aarch64}-unknown-linux-gnu.tar.xz, atelier-cli-{aarch64,x86_64}-apple-darwin.tar.xz, per-archive .sha256, sha256.sum, source.tar.gz, atelier-cli-installer.sh. Downloaded: x86_64 Linux checksum verifies and its atelier --version prints '0.2.0+8c10ce7' (workspace version + the commit built; no -dirty). file(1): ELF x86-64, ELF aarch64, Mach-O arm64, Mach-O x86_64. dist plan lists the same set. Limits: macOS and aarch64 binaries not executed; publishing on tag push not exercised (needs a tag); Linux binaries built on ubuntu-22.04 so need glibc >= 2.35."
updated_at: "2026-10-28T01:49:29.230477760+00:00"
---

PR-mode Release run 40616335699 at 575cf9d0 (PR #13, merge commit 8c10ce75) succeeded without publishing and uploaded: atelier-cli-{x86_64,aarch64}-unknown-linux-gnu.tar.xz, atelier-cli-{aarch64,x86_64}-apple-darwin.tar.xz, per-archive .sha256, sha256.sum, source.tar.gz, atelier-cli-installer.sh. Downloaded: x86_64 Linux checksum verifies and its atelier --version prints '0.2.0+8c10ce7' (workspace version + the commit built; no -dirty). file(1): ELF x86-64, ELF aarch64, Mach-O arm64, Mach-O x86_64. dist plan lists the same set. Limits: macOS and aarch64 binaries not executed; publishing on tag push not exercised (needs a tag); Linux binaries built on ubuntu-22.04 so need glibc >= 2.35.
