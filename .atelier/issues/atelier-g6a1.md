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
closed_at: "2026-11-04T23:16:24.348290534+00:00"
status: "done"
title: "Validate release distribution against S1–S4"
updated_at: "2026-11-04T23:16:24.348290534+00:00"
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

## Close Reason

S1–S4 each pass with a fresh receipt from the independent validator: S1 atelier-tum4, S2 atelier-edle, S3 atelier-owp3 (published v0.3.0 installer then the v0.3.1 one-liner in a clean HOME with no Rust on PATH, upgrade confirmed), S4 atelier-veqw.

### Deviations
S1's failing run is a real CI failure (the atelier-cej5 macOS timeout in runs 41144966277 and 40725124384) rather than a deliberately broken commit; ci.yml is unchanged since, and each gate is its own step without continue-on-error.

### Decisions
None.

### Surprises
v0.3.1 published during the validation, so S3 tested the upgrade 0.3.0 to 0.3.1.

### Open items
Proof limits, recorded on the receipts: only x86_64 glibc Linux was installed and run; the installer's musl fallback for glibc older than 2.35, aarch64 Linux and macOS were checked by file type and embedded version only. Switching PR runs to `pr-run-mode = "plan"` is atelier-l7gl.
