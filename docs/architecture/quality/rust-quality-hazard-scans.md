# Rust Quality Hazard Scans

Use these commands when a slice needs explicit debt-marker, dead/unused-code,
large-function, or unused-dependency review. They complement normal handoff
checks such as `cargo fmt -- --check`, `target/debug/atelier lint`, and
`target/debug/atelier export --check`; they are the repo's focused hazard scans
rather than a replacement for broader validation.

Run commands from the repository root.

## Classification

| Class | Purpose | Command | Notes |
| --- | --- | --- | --- |
| Required | TODO/FIXME debt markers in Rust-owned source and tests | `rg -n "TODO|FIXME" src tests build.rs` | Treat any hit as debt that must be removed or converted into explicit tracker work before handoff. |
| Required | Dead or unused code in the default workspace targets | `cargo clippy --all-targets -- -A warnings -D dead_code -D unused_variables -D unused_imports -D unused_mut` | This is intentionally narrower than `-D warnings`; it is meant to catch unused items without turning unrelated style lints into a handoff blocker. |
| Extended | Large-function review | `cargo clippy --all-targets -- -W clippy::too_many_lines` | Use this when a slice touches dense parser, workflow, rebuild, or command-dispatch code, or when a review suspects readability or extraction risk. Findings should be triaged into refactors, justified local exceptions, or follow-up issues. |
| Required when dependency manifests change | Unused dependency cross-check on stable toolchains | `cargo machete` | First-class repo-supported unused-dependency scan. Install with `cargo install cargo-machete --locked` when unavailable, then rerun before handoff. |
| Advisory | Unused dependency cross-check with nightly coverage | `cargo udeps` | Deferred as a first-class requirement because it commonly needs nightly plus an installed subcommand. Record `deferred` evidence with a named follow-up owner when a slice explicitly needs this deeper cross-check and the tool is unavailable. |

## Routing

- Required scans belong in issue-level proof for slices that change Rust code,
  tests, or build glue.
- Extended scans belong in issue proof when the touched surface is large,
  already dense, or under cleanup review.
- `cargo machete` is the supported unused-dependency scan for dependency
  manifest changes. If it is unavailable, install `cargo-machete` with the
  command above and rerun; if local installation cannot be completed in the
  slice, record `deferred` evidence that names the installation blocker and the
  follow-up owner.
- `cargo udeps` is not a first-class repo requirement. Use it only as an
  advisory nightly cross-check when a reviewer asks for deeper dependency
  analysis; unavailable-tool results should be recorded as `deferred`, not as a
  failed required readiness check.
- When a scan reports a real finding, either fix it in the current slice or
  convert it into an explicit Atelier issue before closeout.

## Transcript Guidance

- For a debt-marker baseline, attach the raw `rg` transcript or a no-match
  confirmation.
- For dead/unused-code scans, attach the failing `clippy` transcript when it
  reports unused variables, dead code, or unread fields.
- For large-function review, attach the `clippy` transcript directly or a
  filtered view such as `cargo clippy --all-targets -- -W
  clippy::too_many_lines 2>&1 | rg "too many lines|-->"` when a shorter review
  artifact is easier to inspect.
- For unused-dependency review, attach the `cargo machete` transcript. A clean
  run reports that it found no unused dependencies; findings should be fixed in
  the dependency manifest or converted into explicit tracker work.
