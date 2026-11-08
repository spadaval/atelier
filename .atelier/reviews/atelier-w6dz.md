---
schema: atelier.review
schema_version: 2
id: atelier-w6dz
title: finding on atelier-8bj8 @5cf579c4
target: atelier-8bj8
identity: reviewer-8bj8
commit: 5cf579c407d1090c215bb64d63ba9bf9b57608f3
created_at: 2026-11-08T06:14:01.107601078Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: orchestrator-g6rp
    commit: 1ecb34ccd181c40e9bcb9b8ca3b56df4bdf6f9f6
    recorded_at: 2026-11-08T06:14:41.497840558Z
    reason: The ignored --binary flag with --link and --unlink changes no state; not worth a change now.
---

low: crates/atelier-cli/src/main.rs:233 --binary carries requires = "harness", but clap does not enforce it with the link flags: setup --unlink --binary x and setup --link <co> --binary x both exit 0 and ignore --binary. No state is affected; the attribute just reads as a guard it is not. One-line fix if main.rs is touched again: conflicts_with_all = ["link", "unlink"] on binary, plus a line in setup_still_needs_a_harness_without_link_flags.
