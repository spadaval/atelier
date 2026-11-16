---
schema: atelier.review
schema_version: 3
round: 1
commit: d8452c5bc803314ca4f915d2e14c841512f6c2e7
reviewers:
- thread:delegated-task:command%3Amcp%3Afeedcf94-7495-48a7-8311-629d7d01f305%3Adelegate-task%3A442b03ff-717f-4152-be16-fc6e6fa78bdf
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T16:36:48.484759133Z
---

## Summary

Complexity: low. The change updates the workspace version, two lockfile
entries and the changelog, with no runtime change.

Value: high. It gives the requested release a matching version and explains
what agents and hosts need to know before updating.

Risk of merging it: low. The version change is mechanically checked, and
an error in the release notes would be visible and cheap to correct.

The value warrants this scope; there is no smaller change that delivers the
release and its update guidance.

Reviewed `master...d8452c5bc803314ca4f915d2e14c841512f6c2e7` against every
Done when line. Compared the changelog with all 24 first-parent commits in
`v0.6.0..origin/master`, then checked its claims against the merged sources:
the CLI and pre-commit guard, planner role, Desk router, action request,
embedded shell, host messages, new views, repository identity, read caches
and command limits, disputed-page fix, and service installer. The entry
correctly describes the final `/t` behavior rather than the earlier page
selection that later merges replaced.

Compared the 0.6 record/config types and record parser with their moved
0.7 counterparts. Their formats are unchanged; `setup.rs` is unchanged,
and the installed hooks still call the binary. Checked the old Desk's
unknown-field refusal and the new installer's default and preservation of
existing configuration, supporting the documented `embed_origins` update
order. Checked release configuration: only the CLI is distributed, while
the Desk remains separately installed and versioned 0.0.0.

`cargo fmt -- --check`, `cargo metadata --locked --format-version 1`,
`atelier check`, and whitespace checks of both the checkout and reviewed
diff pass. `cargo run --locked -q -p atelier-cli -- --version` prints
`atelier 0.7.0`. A tree search found no stale current-version reference to
0.6.0 outside history. GitHub's release-plan check passed; the Rust CI job
was pending when checked, and Desk CI was skipped for this diff.

I did not install or restart the live Desk, exercise T3 Code's browser
integration, or tag/build release archives. Those are outside this release
metadata change; the release workflow remains gated on the later tag.
No findings.
