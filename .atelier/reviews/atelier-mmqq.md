---
schema: atelier.review
schema_version: 2
id: atelier-mmqq
title: finding on atelier-psjw @ef737fde
target: atelier-psjw
identity: GPT-6.1 Sol independent planning reviewer
commit: ef737fdeeee66301b00a990de3a7caf81570b035
created_at: 2026-11-11T03:34:03.939459508Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-psjw.md:39,45: medium severity. The enumerated specification scope omits passages the planned boundary and commit changes make false. redesign-architecture.md:816-818 (The log) says the acts where Atelier makes a commit are start, submit, verdict and merge; ready, drop, close and lead now join them. Lines 917-929 (Hooks and the bypass) place every rule refusal in atelier-cli::skip::refuse and again list only the old committing acts. Those statements cannot remain true while ready and decide run in atelier-app with the one shared bypass policy and the four planning commands commit. Status at lines 443-448 also attributes gathering acts::Checkout to atelier-cli, although the methods that gather it (Repo::checkout, since_approval and uncommitted) move to atelier-app. The Done when list names only the no-status-only-commit sentence in Status and does not authorize edits to The log or Hooks and the bypass; Scope limits changes to the named passages. A worker can therefore satisfy the listed documentation edits and still leave the governing architecture contradicting the result, or must exceed Scope to correct it. Extend that line and Scope to cover these passages, explicitly preserve skip printing and trailers for the new committing acts, and require observable coverage of a skipped planning commit. This changes no Words, Lifecycle or Review rule and adds no command or flag.
