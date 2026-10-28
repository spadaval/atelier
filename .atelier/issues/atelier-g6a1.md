---
created_at: "2026-10-28T00:35:18.353803239+00:00"
id: "atelier-g6a1"
issue_type: "validation"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
  - kind: "issue"
    id: "atelier-vy5i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate release distribution against S1–S4"
updated_at: "2026-10-28T01:52:41.987204350+00:00"
---

## Goal

Check S1–S4 of `docs/strategies/release-distribution.md` independently.

## Done when

Each claim has an explicit result and linked evidence receipt; the handoff states failures, blocked checks, and proof limits. S3 is run against the published installer in a clean HOME with no Rust toolchain on PATH, including an upgrade over an older install.

## Context

Strategy: `docs/strategies/release-distribution.md`. Linux release binaries are built on ubuntu-22.04 and need glibc 2.35 or newer; state that as a proof limit (review of atelier-7v9q, R5).
