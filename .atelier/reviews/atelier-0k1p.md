---
schema: atelier.review
schema_version: 2
id: atelier-0k1p
title: finding on atelier-u1ko @8b1b75e0
target: atelier-u1ko
identity: GPT-6 Astra independent planning reviewer
commit: 8b1b75e02cfdd1a68e3a72c829ff9170f4fde8bc
created_at: 2026-11-11T05:01:16.388321560Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-u1ko.md:68,70,78-79 (plan at 8b1b75e0). Rollback now enumerates commits, refs, checkouts and installations but drops the old protocol rule for an uncommitted conversion: reset to the recorded base and clean the generated tree. convert_records.py:698-705 deletes .atelier, writes new untracked items/config, and removes moved strategies without making a commit. A failure during those writes, the AGENTS/skill/settings edits, or the subsequent commit leaves no conversion commit for the specified undo loop, and checking out the same branch does not remove that dirt. Trial 1 still demands a precommit verify failure although steps 2 and 5 put verify after the commit. Settle the journal entry before mutation and the precise reset/cleanup of runbook-owned uncommitted paths, preserving pre-existing ignored files; explicitly exercise a failure during conversion or before commit on the actual script path as well as postcommit verification. Keep this bounded to the runbook, not a converter rewrite.
