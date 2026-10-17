---
schema: atelier.review
schema_version: 1
id: atelier-xsiv
title: 'crates/atelier-core/src/lib.rs:640 — EvidenceResult::ALL is a hand-kept list with nothing tying it to the enum or to FromStr. Failure scenario: someone adds a variant and its FromStr arm but not ALL; the CLI''s PossibleValuesParser then rejects the new value with exit 2, though records and bundles (which use FromStr) accept it. The .expect in evidence_result_parser cannot fire today, because every ALL entry round-trips. A small unit test that round-trips ALL through as_str/from_str and checks the enum is exhausted (for example with a match) would close this gap. Parsing is unchanged today: the accepted set, case-sensitivity and exit 2 for bad values are all the same; only the error wording changed to clap''s.'
target: atelier-7w1g
identity: Independent reviewer (Claude subagent)
commit: b88141491d3879d251fd8f2effa493ff119c1c0b
created_at: 2026-10-17T07:31:06.063326724Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-core/src/lib.rs:640 — EvidenceResult::ALL is a hand-kept list with nothing tying it to the enum or to FromStr. Failure scenario: someone adds a variant and its FromStr arm but not ALL; the CLI's PossibleValuesParser then rejects the new value with exit 2, though records and bundles (which use FromStr) accept it. The .expect in evidence_result_parser cannot fire today, because every ALL entry round-trips. A small unit test that round-trips ALL through as_str/from_str and checks the enum is exhausted (for example with a match) would close this gap. Parsing is unchanged today: the accepted set, case-sensitivity and exit 2 for bad values are all the same; only the error wording changed to clap's.
