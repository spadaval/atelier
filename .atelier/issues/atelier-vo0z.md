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
closed_at: "2026-11-04T21:51:14.056594725+00:00"
status: "done"
title: "Publish static musl Linux builds so the installer works on glibc older than 2.35"
updated_at: "2026-11-04T21:51:14.056594725+00:00"
---

## Goal

The shell installer fails on Linux hosts with glibc older than 2.35 (RHEL 9 family ships 2.34): "no compatible downloads were found for your platform x86_64-unknown-linux-gnu". Linux release binaries are built on ubuntu-22.04 (glibc 2.35), and the installer's static-musl fallback has no artifact to fetch.

## Done when

- dist-workspace.toml targets include x86_64-unknown-linux-musl and aarch64-unknown-linux-musl, and the generated release workflow builds them.
- The release workflow's PR upload run produces both musl archives.

## Context

Blocks mission S3 (installer on a machine with no Rust toolchain). Atelier has no C dependencies, so static musl builds need no extra toolchain setup.

## Close Reason

Added x86_64-unknown-linux-musl and aarch64-unknown-linux-musl to dist targets, so the shell installer's static-musl fallback has an archive on Linux hosts with glibc older than 2.35 (RHEL 9 ships 2.34). release.yml is unchanged; dist plans the matrix at run time (musl jobs on ubuntu-22.04 and ubuntu-22.04-arm with musl-tools).

Proof: PR #15 release run 41918803847 built and uploaded both musl archives; the reviewer ran the CI-built installer's selection against a fake ldd (2.34 picks musl, 2.35 picks gnu) and ran the x86_64 musl binary (static). Local x86_64 musl build is static-pie and runs `atelier status`. nextest 947/947, test-hooks 953/953.

### Deviations

Also dropped "glibc 2.35 or newer" from the README install section (review finding atelier-lndo, fixed at a523e50c).

### Decisions

Publish musl archives alongside the gnu ones rather than building gnu on an older-glibc runner: the installer already falls back to musl, and Atelier has no C dependencies.

### Surprises

`dist generate` left release.yml unchanged; the target matrix comes from `dist plan` at run time.

### Open items

No real install on a RHEL 9 host and no run of the aarch64 musl binary. The fix reaches users only once a release (v0.3.1) is tagged.
