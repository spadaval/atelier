---
created_at: "2026-11-04T21:42:37.746303104+00:00"
id: "atelier-vo0z"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-vo0z
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Publish static musl Linux builds so the installer works on glibc older than 2.35"
updated_at: "2026-11-04T21:42:41.232168707+00:00"
---

## Goal

The shell installer fails on Linux hosts with glibc older than 2.35 (RHEL 9 family ships 2.34): "no compatible downloads were found for your platform x86_64-unknown-linux-gnu". Linux release binaries are built on ubuntu-22.04 (glibc 2.35), and the installer's static-musl fallback has no artifact to fetch.

## Done when

- dist-workspace.toml targets include x86_64-unknown-linux-musl and aarch64-unknown-linux-musl, and the generated release workflow builds them.
- The release workflow's PR upload run produces both musl archives.

## Context

Blocks mission S3 (installer on a machine with no Rust toolchain). Atelier has no C dependencies, so static musl builds need no extra toolchain setup.
