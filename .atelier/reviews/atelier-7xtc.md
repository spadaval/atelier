---
schema: atelier.review
schema_version: 1
id: atelier-7xtc
title: approval on atelier-2bjz @6ea3eb5f
target: atelier-2bjz
identity: claude-review-2bjz
commit: 6ea3eb5f5c3cd2e45f14349fa20da5d4d957a7d1
created_at: 2026-11-05T00:08:24.973335940Z
event:
  event: approval
---

Reviewed the whole branch 18ce46d0..6ea3eb5f against s5x4, w5hx and juyb Done when and decision atelier-6306. s5x4: status_is_done falls back to the status name without a policy and is used by decided() and the mission timeline; CLI test removes workflow.yaml. w5hx: --one-way gone from help and is a clap usage error (exit 2, checked); work decisions lists current records then notes; decision_brief_lines shows up to three current records newest-first, then the standard omitted-rows line naming issue show --full (CLI test); debrief has no one-way section, sidebar, dec-link or .oneway; the four notes read [decision] while fip8's Design text and quoted tag stay; [decision one-way] is now a malformed tag; the grep over crates and docs/product prints nothing. juyb: check refuses near-miss labels, CLI and unit tests. Strategy S1/S4/S6 edits stay within atelier-6306. Ran: git diff --check, cargo fmt --check, cargo metadata --locked, atelier check (pass), nextest atelier-core 12/12, atelier-app 106/106, atelier-cli decision|debrief|brief|status|note 99/99. Findings atelier-nu4k (label heuristic false positives) and atelier-kt8y (unrelated macOS path_log timeout) are defer. CI: Linux passed; macOS red on the unrelated timeout, so merge must wait for a green rerun, which merge-ci-passed.sh enforces.
