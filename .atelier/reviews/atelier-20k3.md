---
schema: atelier.review
schema_version: 2
id: atelier-20k3
title: finding on atelier-u1ko @8b1b75e0
target: atelier-u1ko
identity: GPT-6 Astra independent planning reviewer
commit: 8b1b75e02cfdd1a68e3a72c829ff9170f4fde8bc
created_at: 2026-11-11T05:01:26.899894319Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:71,75,81,111-112 (plan at 8b1b75e0). The new adopt route partially addresses atelier-p7uh and atelier-x4ly but still lacks a usable bootstrap and acceptance sequence. It installs before step 6 fetches/advances the old host checkout; the default install command builds crates/atelier-cli from that still-old master (root Cargo.toml currently selects the old workspace), and moving the skill to that master also retains the old skill. A configurable install command is not a selected source or recipe, and this item releases no new archive. adopt then runs step 7, which requires counts from a conversion it never ran and atelier-serve stopped/disabled, but its sequence never stops that service. Specify how the host obtains the reviewed script and pinned new build/skill without advancing its working checkouts prematurely, the exact service transition and rollback, and adoption-specific record-count evidence from the adopted commit rather than nonexistent conversion output. Trial a genuinely pre-switch second host with its old service running and the chosen install route. No general installer framework is needed.
