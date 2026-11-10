---
schema: atelier.review
schema_version: 2
id: atelier-yb6u
title: finding on atelier-dxrj @ff2b4a8a
target: atelier-dxrj
identity: sol-dxrj-r1
commit: ff2b4a8a5540c069cd8e1c18887fede7432a5e81
created_at: 2026-11-10T22:25:05.463603973Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/plan.rs:715 A failed review write truncates the original review, violating the both-or-neither requirement for new --from. Reproduced in a scratch repository with a valid 10,028-byte review (long Summary, small deferred finding), ignoring SIGXFSZ and setting RLIMIT_FSIZE to 4096 for the child: new Follow up --from demo-d0ne/1-aaaa.1 exits 2 and removes its new item, but leaves the review truncated to 4096 bytes, with the finding lost, while claiming nothing changed. ReviewFile::write uses truncating fs::write, and save_with rolls back only the new item. Write the replacement review to a temporary file and publish it atomically only after it is complete; remove the new item on failure and check rollback errors. Add a failure test that fails after the review file opens, rather than only the current read-only-file test. Contract and Fitness fail at this head; scope is worth merging once the recorded fixes pass fresh review. Sizes: complexity medium (three command extensions and one act over existing records), value high (required settlement workflow), risk medium (failed writes damage tracked review records). Fresh checks: all 176 next tests pass, next clippy with warnings denied and root/next fmt pass, root locked cargo metadata and both whitespace checks pass. The passing suite does not exercise this partial-write case.
