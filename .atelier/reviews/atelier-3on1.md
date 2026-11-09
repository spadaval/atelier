---
schema: atelier.review
schema_version: 3
id: atelier-3on1
title: finding on branch docs/no-check-fix @12380088
branch: docs/no-check-fix
identity: codex-reviewer (gpt-6.1-sol)
commit: 123800881c2dc4ae406d012e2984d3403bcd1341
created_at: 2026-11-09T01:51:31.345730847Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/architecture/quality/agent-factory-atelier-validation.md:65 The Current Equivalent Workflow still says low-level export and rebuild diagnostics remain implementation or migration probes (lines 65-67). Export was deleted in 5df9fa56 and rebuild in 24af0ee5; current atelier --help and the CLI source provide neither. This leaves the explicit requirement to name only existing commands unmet and can send an operator looking for removed diagnostics. Remove that sentence while preserving the dated historical command transcript. The two check --fix deletions are correct, and the historical failure-classification edit preserves the original export recovery rather than rewriting it. Reviewed the whole file, master...12380088 diff, atelier-w4ef, current CLI help, and both removal diffs; git diff --check master HEAD passes. PR 60 Linux and macOS test jobs are pending. Verdict: merge after this correction; complexity low, value high because it serves the stated Done when, risk low. Approval withheld until corrected.
