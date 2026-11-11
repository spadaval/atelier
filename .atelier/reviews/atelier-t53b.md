---
schema: atelier.review
schema_version: 2
id: atelier-t53b
title: finding on atelier-psjw @55d88b72
target: atelier-psjw
identity: GPT-6.1 Sol independent reviewer
commit: 55d88b727d94831f51f000775954e84bf194d75f
created_at: 2026-11-11T05:10:26.081557802Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-app/src/git.rs:184 R2. Give the committing Git subprocess the invocation Skip reason instead of inheriting ATELIER_SKIP from the process. commit_paths uses Skip for its trailers but relayed/output spawn git without passing or clearing that variable, so the installed pre-commit hook judges a library invocation using ambient process state. Independently reproduced in ~/.cache/atelier-qr5s/psjw/review-1/library-hook with Atelier hooks installed and a draft whose blocked_by names a missing item: ready with Skip::new(Some("review reason")) passes validation, then the hook refuses and the function returns an error when ATELIER_SKIP is absent; the identical call succeeds when the environment variable is also set. The first run restored the record and HEAD. This defeats the explicit per-invocation bypass interface for library callers. Set the child environment from Skip::reason(), clearing any inherited value when it is None, without mutating the parent environment. Add library coverage with installed hooks, both an explicit reason with no ambient variable and conflicting ambient and invocation reasons.
