---
created_at: "2026-10-28T00:35:17.576928408+00:00"
id: "atelier-vt8q"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/release-distribution.md
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T00:20:42.079750052+00:00"
status: "done"
title: "Distribute Atelier as GitHub Release binaries with a one-command install and upgrade"
updated_at: "2026-11-05T00:20:42.079750052+00:00"
---

## Outcome

A tagged commit becomes a GitHub Release with prebuilt Linux and macOS binaries and a shell installer, built by CI from a tested commit. Anyone can install or upgrade Atelier with one command, and pull requests run the repository's checks on GitHub.

## Close Reason

### Outcome
Atelier ships as GitHub Release binaries with a one-command install and upgrade. CI runs fmt, both nextest suites, `atelier check` and `git diff --check` on Linux and macOS for pull requests and master, and fails when a gate fails (S1: atelier-tum4). A `v<version>` tag publishes archives for x86_64 and aarch64 Linux (gnu and static musl) and both macOS targets, with checksums and a shell installer; v0.3.0 and v0.3.1 are published; pull requests now only plan the release (S2: atelier-edle, atelier-uqro). The published installer installs `atelier` onto PATH in a clean HOME with no Rust toolchain and upgrades 0.3.0 to 0.3.1 on rerun (S3: atelier-owp3). The stale-binary message, README and development-setup docs name the installer and keep `cargo install --path crates/atelier-cli --locked` for checkouts (S4: atelier-veqw). S1, S3 and S4 were independently validated by atelier-g6a1; the two code commits after those receipts split a path_log test (atelier-cej5) and switched the release workflow's PR mode (atelier-l7gl), neither of which touches the CI gates, the installer or the docs they judge.

### Deviations
Scope added after the start: static musl Linux builds and v0.3.1 (atelier-vo0z, atelier-g8cn) so the installer works on glibc older than 2.35, and PR plan mode (atelier-l7gl) at Sumeet's request, which reworded S2 on 2026-11-04. S1's failing-run proof is a real CI failure (the macOS path_log timeout fixed by atelier-cej5) rather than a deliberately broken commit.

### Decisions
Use dist's generated workflow unedited, including dropping its build cache in plan mode (decision note on atelier-l7gl); item decisions are on their records (`atelier work decisions atelier-vt8q`).

### Surprises
The macOS CI job's path_log parity test sat at 48-51s of a 60s budget and timed out under load; macOS runner capacity (5 concurrent) was the bottleneck that led to plan mode.

### Open items
Proof limits: only x86_64 glibc Linux was installed and run; the musl fallback, aarch64 Linux and macOS binaries were checked by file type and embedded version. The plan-mode release workflow first builds on the next tag push.
