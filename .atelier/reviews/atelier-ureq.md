---
schema: atelier.review
schema_version: 2
id: atelier-ureq
title: finding on atelier-dxrj @ff2b4a8a
target: atelier-dxrj
identity: sol-dxrj-r1
commit: ff2b4a8a5540c069cd8e1c18887fede7432a5e81
created_at: 2026-11-10T22:25:23.217046887Z
event:
  event: finding
  recommendation: fix_now
---

low: next/crates/atelier-records/src/body.rs:127 A name printed by list --deferred can panic in both settlement commands because the reader and writer match finding headings differently. A review containing ## Finding 01: Defer this followed by Recommendation: defer is accepted and listed as demo-d0ne/1-aaaa.1; drop demo-d0ne/1-aaaa.1 --reason Obsolete then exits 101 at plan.rs:595 (assertion failed: review.settle). The reader parses the heading number as u32, but settle and finding_text look for the literal prefix Finding 1: , which does not match Finding 01: . new --from reaches the same assertion and also cannot quote the finding text. Use the same numeric heading recognition for reading, quoting and settling, or consistently refuse noncanonical headings before listing them. Add a round-trip test feeding a listed name into both commands for every heading form the reader accepts.
