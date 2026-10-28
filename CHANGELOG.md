# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

The first published release. Atelier is a verified, repo-native work tracker:
missions, issues, evidence and reviews are Markdown records under `.atelier/`,
committed with the code they describe, and checked at the point of action
([ADR 0019](docs/adr/0019-verified-repo-native-tracker.md)). It replaces the
earlier SQLite-backed tracker; there is no database or cache.

### Added
- Records: `init`, `issue` (create, show, update, transition, note, link),
  `bundle` for multi-record graphs, and `check` to validate records and the
  repository's workflow policy (`.atelier/workflow.yaml`).
- Orientation: `status` and the `work` views (ready, human, blocked, active,
  missions, mission, parent, decisions).
- Proof and review: commit-pinned `evidence` receipts, and `review`
  approvals, findings, dispositions and trivial exemptions.
- Integration: `start` prepares a work branch; `merge` integrates reviewed
  work only after the base branch's policy gates pass; `land` checks the
  handoff; `doctor` rechecks every done claim.
- Mission reviews: `brief` and `debrief`, as Markdown or a static HTML page.
- Claude Code hooks via `setup claude`, running `status` at session start and
  `land` at stop.
- Prebuilt Linux (x86_64, aarch64) and macOS (arm64, x86_64) binaries with a
  shell installer, published as GitHub Releases.
