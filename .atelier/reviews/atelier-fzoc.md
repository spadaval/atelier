---
schema: atelier.review
schema_version: 2
id: atelier-fzoc
title: finding on atelier-qr5s @8f1d6bfc
target: atelier-qr5s
identity: grok-4.6-plan-r3
commit: 8f1d6bfce97e37869d729e90aec74f77020e457c
created_at: 2026-11-10T01:37:10.931939542Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:344-346: cutover step 5 only updates a pinned Atelier revision, but astromech .github/workflows/atelier.yml cargo-installs atelier-cli from a git SHA and runs atelier doctor then atelier land on every push to main. After the switch those commands are gone, and new land refuses a conversion commit that rewrites config.toml and deletes .atelier/issues, so bumping the pin still turns astromech CI red on the conversion push. Smallest fix: review replacement workflow files under scripts/cutover/ with the AGENTS.md texts (drop doctor/land, run check, pin a revision that can build the new binary including its site), and have step 5 install those files.
