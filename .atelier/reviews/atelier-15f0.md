---
schema: atelier.review
schema_version: 3
id: atelier-15f0
title: approval on branch release-0.4.0 @46ecd1a0
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 46ecd1a00f8c8bc2d45aeb1f1a035a4c330814a3
created_at: 2026-11-09T21:25:26.285666422Z
event:
  event: approval
---

Verdict: merge. The conflict resolution preserves the 0.4.0 release entry and incorporates the sole new master increment without dropping changelog content or altering master code.
Sizes: complexity low (one changelog bullet incorporated during a merge), value medium (release notes describe the current platform policy), risk low (the resolution adds documentation only).
Contract: pass. Fitness: pass.
Reviewed commit 46ecd1a00f8c8bc2d45aeb1f1a035a4c330814a3 on release-0.4.0. Ran git diff 8a10a925 46ecd1a0 -- CHANGELOG.md: the only change is the two-line macOS note at the end of 0.4.0 Changed. Ran git diff origin/master 46ecd1a0 --stat and inspected the full non-record diff: only Cargo.toml, Cargo.lock, CHANGELOG.md and release review records differ from master; Cargo changes are solely the 0.3.1 to 0.4.0 workspace and six package version bumps. Confirmed origin/master is an ancestor. All other files, including .github/workflows/ci.yml, .config/nextest.toml and release configuration, match master unchanged.
The 0.4.0 heading and Added, Changed, Removed and Fixed lists are well formed; Unreleased is removed, the master macOS bullet is preserved verbatim, and no changelog conflict markers remain. Programmatic comparisons confirmed the preamble and every entry from 0.3.1 onward are identical to master, and the complete changelog equals the previously approved version plus that bullet.
Checked .github/workflows/release.yml and dist-workspace.toml: release jobs build the dist-generated matrix and upload its artifacts to GitHub Releases; both aarch64-apple-darwin and x86_64-apple-darwin remain release targets. The CI test job now runs on ubuntu-latest only, so the Linux-only CI and continued macOS binary claims are accurate. git log --first-parent 1858e8a6..origin/master contains only 30eb7ca5, merging t3/drop-macos-ci. Inspected its complete file list, workflow/config changes and README/development/release-policy changes; no additional user-visible change needs a changelog entry.
Proof: ran target/debug/atelier review request and all three printed checks at the reviewed head: git diff --check HEAD^ HEAD -- . with .atelier excluded, cargo fmt -- --check, and cargo metadata --locked --format-version 1 > /dev/null. All passed. Additional read-only assertions on ancestry, allowed changed paths, exact changelog preservation, heading structure, conflict markers and macOS target configuration passed.
Findings: none; no fix_now recommended. Open questions: none. Residual risk and tests not run: release builds, macOS runtime behavior and the full test suite were not rerun; this review verifies the conflict resolution and release-note claims against repository source and configuration. No build was needed to run the existing debug review command.
