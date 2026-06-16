---
created_at: "2026-06-13T23:12:18.000593620+00:00"
id: "atelier-mrjv"
evidence_type: "validation"
captured_at: "2026-06-13T23:12:18.000522336+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pa33"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Decision: cargo machete is the repo-supported unused dependency scan for dependency manifest changes, installed with cargo install cargo-machete --locked and documented in docs/architecture/quality/rust-quality-hazard-scans.md plus docs/product/development-setup.md. cargo udeps is explicitly advisory/deferred because it commonly requires nightly and is not required setup. Unavailable-tool routing is documented in docs/architecture/quality/validation.md. Baseline cargo machete finding is owned by atelier-gzel; transcript evidence is atelier-uybh."
updated_at: "2026-06-13T23:12:20.490814047+00:00"
---

Decision: cargo machete is the repo-supported unused dependency scan for dependency manifest changes, installed with cargo install cargo-machete --locked and documented in docs/architecture/quality/rust-quality-hazard-scans.md plus docs/product/development-setup.md. cargo udeps is explicitly advisory/deferred because it commonly requires nightly and is not required setup. Unavailable-tool routing is documented in docs/architecture/quality/validation.md. Baseline cargo machete finding is owned by atelier-gzel; transcript evidence is atelier-uybh.
