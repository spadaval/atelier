---
schema: atelier.review
schema_version: 2
id: atelier-2mz6
title: approval on atelier-va99 @634498ff
target: atelier-va99
identity: codex-sol-va99-r2
commit: 634498ff2bdadce71dbd3d4c69210ab37f013697
created_at: 2026-11-11T08:43:04.091242691Z
event:
  event: approval
---

Verdict: merge. Sizes: complexity medium (a user service, install and clone scripts, two acceptance commands); value high (makes the Desk usable on this host and proves both mission success criteria); risk medium (host installation and scratch filesystem writes, bounded by service limits and isolation). Contract: pass. Fitness: pass. No new findings or unresolved fix_now recommendations. The canonical-path guard now checks both the clones directory and its work directory against the physical projects directory and each source before any write; all seven direct and symlink refusal cases passed, resolving atelier-vi6k. Each added component serves the stated install, update, attribution, shutdown or acceptance requirements.

Reviewed the branch-owned change ea76c0f3..634498ff excluding commits merged from master, the final files, the Done when lines, the Report, the mission strategy and service documentation. Fresh proof at the reviewed head: git diff --check HEAD^ HEAD and the full range excluding .atelier; cargo fmt and locked metadata in both workspaces; cargo nextest run -j 4 in next/ (285 passed, including SIGTERM/SIGINT token revocation and built_from); pnpm typecheck, pnpm test (43 passed), pnpm build and pnpm e2e (52 passed, desktop and phone footer coverage). make-clones.test.sh refused all seven unsafe paths without writing into the scratch sources. make-clones.sh into a new directory under TMPDIR cloned, converted, verified and checked all five repositories successfully (137/8, 240/4, 19/1, 47/3 and 37/7 items/missions). No served clone or live repository was modified.

desk-matches-cli against http://buildbot.tail47515a.ts.net:3780 matched all 23 open missions and 5 inboxes. act-is-a-record on a copy of the converted atelier clone passed: one new commit each, matching messages, changed paths and records except ready.at, and Ready on the page. Its --differ run produced the expected ready.by difference and exit 1; both scratch directories were removed. The acceptance commands remain outside the desk CI job.

Read-only host checks: atelier-desk active and enabled, lingering enabled, CPUQuotaPerSecUSec=1s, MemoryMax=1073741824, Nice=10 and Restart=on-failure. Installed unit and t3 wrapper match the repository. Loopback, tailnet IP and hostname serve all five error-free repositories and report last code commit 6f8eeabe2b21bb65213fd256b43eb0490c17b901; an unrelated Host receives 403. T3 Code is reachable. The journal independently confirms the prior SIGKILL at 03:38:36 and automatic recovery at 03:38:42; NRestarts=1. t3code and atelier-serve are active, and tailscale serve retains the reported publications.

Open questions: none. Residual risk and tests not run: no service was stopped, restarted, killed or reinstalled during this review, and no tailscale setting changed. Boot and a future T3 Code upgrade were not simulated; the unit, lingering and wrapper were inspected. The orchestrator must update the service from master after merge; atelier-oc9k remains the independent post-merge validation.
