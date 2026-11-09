---
schema: atelier.review
schema_version: 2
id: atelier-11g2
title: finding on atelier-2js4 @e4f24e73
target: atelier-2js4
identity: gpt-6.1-sol
commit: e4f24e732b2774334504e46604b52dde672bb400
created_at: 2026-11-09T05:05:15.257416264Z
event:
  event: finding
  recommendation: fix_now
---

medium: .github/workflows/ci.yml:46 An unavailable nonzero BASE prevents the promised Rust fallback. ci-jobs.sh correctly emits rust=true for a missing commit, but the following git diff --check exits 128 and fails changes. Since test needs changes and has no failure-status override, GitHub skips both Rust jobs. Reproduced the workflow commands locally with BASE=1234567890abcdef1234567890abcdef12345678: rust=true, then fatal: bad object, exit 128. This violates the missing-commit Done when line; the script-only test does not cover the dependent job. Handle an unverifiable base in the whitespace step without failing changes (for example, compare HEAD with the empty tree), and add proof of the whole fallback path so the Rust jobs actually run.
