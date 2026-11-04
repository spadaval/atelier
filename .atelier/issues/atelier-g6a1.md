---
created_at: "2026-10-28T00:35:18.353803239+00:00"
id: "atelier-g6a1"
issue_type: "validation"
labels: []
workflow_branch:
  branch_base: master
  work_branch: validation/atelier-g6a1
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-gddg"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
  - kind: "issue"
    id: "atelier-vy5i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Validate release distribution against S1–S4"
updated_at: "2026-11-04T23:12:48.453631084+00:00"
---

## Goal

Check S1–S4 of `docs/strategies/release-distribution.md` independently.

## Done when

Each claim has an explicit result and linked evidence receipt; the handoff states failures, blocked checks, and proof limits. S3 is run against the published installer in a clean HOME with no Rust toolchain on PATH, including an upgrade over an older install.

## Context

Strategy: `docs/strategies/release-distribution.md`. Linux release binaries are built on ubuntu-22.04 and need glibc 2.35 or newer; state that as a proof limit (review of atelier-7v9q, R5).

## Notes

- 2026-11-04 [note] S4 check script (~/.cache/atelier-g6a1/s4-check.sh) used by atelier-veqw:
  ```sh
  #!/bin/sh
  # S4: message tests plus doc/metadata inspection, run from the repo root.
  set -eu
  export TMPDIR="$HOME/.cache/atelier-tmp" RUST_BACKTRACE=0
  I="curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh"
  S="cargo install --path crates/atelier-cli --locked"
  cargo nextest run -j 4 -p atelier-cli -p atelier-workflow -E 'test(a_record_from_a_newer_atelier_says_to_reinstall) | test(a_policy_from_a_newer_atelier_says_to_reinstall) | test(test_queries_fail_naming_the_invalid_record_file) | test(test_lint_has_stable_diagnostics_for_hard_invalid_markdown_records) | test(a_newer_policy_needs_a_newer_atelier)'
  grep -qF "$I" crates/atelier-core/src/lib.rs && echo "message source: names installer"
  grep -qF "$I" README.md && grep -qF "$S" README.md && echo "README: installer + source-checkout command"
  grep -qF 'README.md#install' docs/product/development-setup.md && grep -qF "$S" docs/product/development-setup.md && echo "development-setup: points to README installer + source-checkout command"
  grep -q '^repository = "https://github.com/spadaval/atelier"' Cargo.toml && echo "Cargo.toml: repository = spadaval/atelier"
  for c in crates/*/Cargo.toml; do grep -q 'repository.workspace = true' "$c" || { echo "missing repository in $c"; exit 1; }; done; echo "all crates inherit repository"
  grep -q 'Markdown records under `.atelier/`' CHANGELOG.md && grep -q 'replaces the' CHANGELOG.md && echo "CHANGELOG: describes Markdown-record model, SQLite only as replaced"
  ```
