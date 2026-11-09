---
schema: atelier.review
schema_version: 2
id: atelier-ychj
title: finding on atelier-ah82 @661625da
target: atelier-ah82
identity: codex-reviewer (gpt-6.1-sol)
commit: 661625dad956b33c3967870c050ec88418166ebc
created_at: 2026-11-09T02:56:23.781511869Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/product/workflow-configuration.md:203 R4: The unconditional no-forge/no-network statement says merge reads no CI without a forge block, but this very repository has no forge block and runs scripts/merge-ci-passed.sh through shell.check, which calls gh run list. Existing land --push also works without a forge, and the update check is already an acknowledged exception. Literal no-network promises here, reviews-and-merge.md:580, PRODUCT_INTENT.md:106, CONTEXT.md:28 and ADR 0031:61 contradict retaining existing behavior. Scope the guarantee to the new built-in forge operations and state that explicitly configured shell checks and existing explicit push/update commands retain their behavior; do not imply this repository is offline until its policy migrates.
