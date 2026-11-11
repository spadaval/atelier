---
schema: atelier.review
schema_version: 2
id: atelier-vi6k
title: finding on atelier-va99 @6108a416
target: atelier-va99
identity: codex-sol-va99-r1
commit: 6108a416999eb4be8cfeee948cc850f89be27006
created_at: 2026-11-11T08:35:21.227281625Z
event:
  event: finding
  recommendation: fix_now
---

Medium: desk/service/make-clones.sh:32-35 checks the output path only after mkdir and uses logical pwd. Passing a new directory inside a source repository creates it before refusing; passing an existing symlink outside DESK_PROJECTS that points to a directory inside a source repository bypasses the check entirely and installs all five converted clones inside that source. This violates the Done when promise that the script never writes to a live repository. Resolve the physical source, output and work paths, including symlinked parents of nonexistent paths, and reject overlap before the first filesystem mutation. Prove both direct nested paths and symlink aliases refuse without changing the source.

R1 reproduction used only fresh scratch copies under ~/.cache/atelier-desk/tmp/va99-review-clones.7mgDv2. The ordinary make-clones run passed for all five. With DESK_PROJECTS set to these scratch sources (base refs restored to their unconverted parents), alias -> clones/atelier/review-served bypassed the guard: make-clones.sh alias printed All five clones are converted and pass check and exited 0, writing five repositories under the atelier source. A direct clones/atelier/refused-output exited 2 but left that directory created. No live repository or installed service was changed.

Verdict: merge after R1 is fixed and independently reviewed; the service and acceptance commands serve the approved outcome without an unneeded layer. Sizes: complexity medium (unit, install/clone scripts, two acceptance commands), value high (the mission success criteria), risk medium (host service and filesystem writes). Contract: fail on the no-write safety boundary. Fitness: fail until the path guard runs before writes and resolves aliases.

Fresh proof otherwise passed: git diff --check HEAD^ HEAD and master...6108a416 excluding .atelier; cargo fmt and locked metadata in both workspaces; cargo nextest run -j 4 -p atelier-desk --test signals --test desk (9 passed); pnpm typecheck; pnpm test (43 passed); footer Playwright tests at desktop and phone widths (2 passed). desk-matches-cli over the installed tailnet hostname matched all 23 missions and 5 inboxes; changing one scratch mission title produced DIFF and exit 1. act-is-a-record passed, while --differ produced the expected record DIFF and exit 1. The installed service is active, enabled, limited to one core/1 GiB/Nice 10, and reports the last code commit c6db1865. Loopback, tailnet IP and hostname returned 200 for the five clones and 403 for another Host. Installed unit/wrapper match the repository; the wrapper resolves the running T3 version; lingering is enabled. SIGKILL recovery and clean stop were read in the journal, not repeated. T3 Code and atelier-serve remain active.

Open questions: none. Residual risk and checks not run: no installed-service stop/restart/kill or tailscale changes; no full Rust suite or full browser suite rerun. Post-merge update from master and validation atelier-oc9k remain the planned next steps. Approval withheld for R1.
