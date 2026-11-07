---
schema: atelier.review
schema_version: 2
id: atelier-3q6i
title: finding on atelier-hzuk @59ae63e0
target: atelier-hzuk
identity: independent planning reviewer subagent (claude-opus-5-5)
commit: 59ae63e0fc0faa110587261b677874c04e938ec0
created_at: 2026-10-30T06:47:00.096476799Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: planner (claude-opus-5-5, atelier-ll3p author)
    commit: 298bd742a940e9add66c3b95a579760fb26c7e47
    recorded_at: 2026-10-30T06:48:51.279390548Z
    resolution_commit: 298bd742a940e9add66c3b95a579760fb26c7e47
    reason: Fixed in the planning revision
    migrated_from: atelier-4t5v
---

R1: Done when cannot pass as written, and the documented route touches HOME. With ATELIER_CLI_INSTALL_DIR=<dir> the v0.3.0 installer uses the cargo-home layout: in a clean HOME it installed to <dir>/bin/atelier (not <dir>/atelier) and wrote ~/.config/atelier-cli/atelier-cli-receipt.json, which in a real HOME repoints the user's updater receipt at the eval copy. ATELIER_CLI_UNMANAGED_INSTALL=<dir> installed flat to <dir>/atelier, wrote nothing under HOME and left PATH alone (checked 2026-10-30; both print atelier 0.3.0+02ffeac). Fix: document UNMANAGED_INSTALL (or INSTALL_DIR with its bin/ path and the receipt), and make the Done when check that nothing under HOME is created, not just profile files. Optionally link hzuk to atelier-vt8q, the release-distribution mission it extends.
