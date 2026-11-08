---
schema: atelier.review
schema_version: 2
id: atelier-d37l
title: approval on atelier-45gr @3d8be7a4
target: atelier-45gr
identity: reviewer-45gr
commit: 3d8be7a4ca8ff9e250ebd16032468977b2a430df
created_at: 2026-11-08T07:07:24.014551136Z
event:
  event: approval
---

Read the whole branch diff (freshness.rs, status/doctor/main wiring, update_check.rs, .cargo/config.toml, ci.yml, docs/product) against all seven Done when lines; each holds. Ran: git diff --check (clean), cargo fmt --check (clean), cargo metadata --locked (ok), scripts/merge-ci-passed.sh (exit 2, not a merge commit; expected before CI), cargo nextest run -j 4 -p atelier-cli 816/816 and --features test-hooks 822/822. Manual runs in a scratch HOME and XDG_CACHE_HOME: status with no cache starts one refresh-update-check in its own process group (pgid = pid, reparented, null stdio), the hook pipeline returns without waiting for it, it finished in ~0.5 s and wrote latest: 0.3.1 against real GitHub, and no refresh process remained; status timing with the check on vs ATELIER_NO_UPDATE_CHECK=1, 6 runs each on this repository: 3316-3388 ms vs 3302-3594 ms, within noise; doctor against real git ls-remote and npx reports 0.3.1 current and skill not installed with the add line. The installed binary reads the variable at runtime, so .cargo/config.toml's [env] affects only cargo-run processes (tests, cargo run), not builds or cargo install. Every network/npx call goes through run() with a timeout and process-group kill; status only reads the cache and spawns at most one refresh per 10 minutes. The plan finding atelier-l7mt (fetch method and telemetry) is met: git ls-remote, DISABLE_TELEMETRY=1. One low finding deferred (atelier-wgan: reader joins unbounded after child exit). Not covered: a hanging Git server, macOS cache path, a live tracked skills install. Duplicate link detection with atelier-8bj8 is left to their planned reconciliation.
