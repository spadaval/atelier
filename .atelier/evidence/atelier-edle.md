---
created_at: "2026-11-04T23:15:15.378891847+00:00"
id: "atelier-edle"
evidence_type: "validation"
commit: "526fb45908eb8f307c6ddccf81c2d954a00beffb"
result: "pass"
captured_at: "2026-11-04T23:15:15.378834640+00:00"
uri: "https://github.com/spadaval/atelier/releases/tag/v0.3.1"
agent_identity: "Claude Opus 5.5 (independent validator, atelier-g6a1)"
target:
  kind: "issue"
  id: "atelier-g6a1"
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
    id: "atelier-g6a1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S2. Tag publish: pushing v0.3.1 (d14d2e5e, an ancestor of this HEAD) ran Release 41931151380: plan, 6 build-local-artifacts, build-global, host and announce all success; GitHub Release v0.3.1 published 2026-11-04T23:14:32Z (not draft/prerelease, now Latest) with tar.xz archives for x86_64/aarch64 linux-gnu, x86_64/aarch64 linux-musl, aarch64/x86_64 apple-darwin, a .sha256 per archive, sha256.sum, dist-manifest.json, source.tar.gz and atelier-cli-installer.sh. Downloaded all assets to ~/.cache/atelier-g6a1/release-v0.3.1: sha256sum -c sha256.sum and every sidecar OK; x86_64 gnu and musl binaries run and print 'atelier 0.3.1+d14d2e5' (tag version and commit); the aarch64 and macOS binaries are the right ELF/Mach-O arch and embed '0.3.1+d14d2e5'; musl build is statically linked; gnu builds need at most GLIBC_2.34; installer falls back to musl when check_glibc 2.35 fails. PR dry run: Release run 41919897633 (pull_request, bug/atelier-vo0z e70496f7, which changed dist-workspace.toml) built and uploaded all 6 target artifacts plus global (installer, sha256.sum) with host and announce skipped (no publish); its x86_64 gnu and musl archives pass their sha256 and print 'atelier 0.3.0+8d9d79f', 8d9d79f being GitHub's PR merge commit of e70496f7 into b7030fb6 (the version bump to 0.3.1 came later). Limits: aarch64 Linux and both macOS binaries were not executed (no such host here); only inspected by file type and embedded version string. Release workflow is dist-generated; its contents were not re-audited."
updated_at: "2026-11-04T23:15:15.379430613+00:00"
---

S2. Tag publish: pushing v0.3.1 (d14d2e5e, an ancestor of this HEAD) ran Release 41931151380: plan, 6 build-local-artifacts, build-global, host and announce all success; GitHub Release v0.3.1 published 2026-11-04T23:14:32Z (not draft/prerelease, now Latest) with tar.xz archives for x86_64/aarch64 linux-gnu, x86_64/aarch64 linux-musl, aarch64/x86_64 apple-darwin, a .sha256 per archive, sha256.sum, dist-manifest.json, source.tar.gz and atelier-cli-installer.sh. Downloaded all assets to ~/.cache/atelier-g6a1/release-v0.3.1: sha256sum -c sha256.sum and every sidecar OK; x86_64 gnu and musl binaries run and print 'atelier 0.3.1+d14d2e5' (tag version and commit); the aarch64 and macOS binaries are the right ELF/Mach-O arch and embed '0.3.1+d14d2e5'; musl build is statically linked; gnu builds need at most GLIBC_2.34; installer falls back to musl when check_glibc 2.35 fails. PR dry run: Release run 41919897633 (pull_request, bug/atelier-vo0z e70496f7, which changed dist-workspace.toml) built and uploaded all 6 target artifacts plus global (installer, sha256.sum) with host and announce skipped (no publish); its x86_64 gnu and musl archives pass their sha256 and print 'atelier 0.3.0+8d9d79f', 8d9d79f being GitHub's PR merge commit of e70496f7 into b7030fb6 (the version bump to 0.3.1 came later). Limits: aarch64 Linux and both macOS binaries were not executed (no such host here); only inspected by file type and embedded version string. Release workflow is dist-generated; its contents were not re-audited.
