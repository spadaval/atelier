---
created_at: "2026-10-28T16:19:34.563471497+00:00"
id: "atelier-a755"
evidence_type: "release"
commit: "26867fc9671dca44fc811cc7f2315119a4452648"
result: "pass"
captured_at: "2026-10-28T16:19:34.563362164+00:00"
uri: "https://github.com/spadaval/atelier/releases/tag/v0.3.0"
agent_identity: "Claude Opus 5.5 (orchestrator)"
target:
  kind: "issue"
  id: "atelier-gddg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-gddg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "v0.3.0 (annotated, on master 02ffeac8) pushed after master CI run 40633250518 passed on ubuntu and macos. Release workflow run 40721515038 passed and published GitHub Release '0.3.0 - 2026-10-28', not draft, not prerelease, with the four atelier-cli-<target>.tar.xz archives, their .sha256, sha256.sum, source.tar.gz(+.sha256), dist-manifest.json and atelier-cli-installer.sh. Smoke check: the README one-liner in a clean HOME (env -i, no Rust on PATH) installed ~/.cargo/bin/atelier reporting 0.3.0+02ffeac and added ~/.cargo/env to .profile. Limits: upgrade-over-older-install and macOS not exercised here; independent check is atelier-g6a1."
updated_at: "2026-10-28T16:19:34.564295994+00:00"
---

v0.3.0 (annotated, on master 02ffeac8) pushed after master CI run 40633250518 passed on ubuntu and macos. Release workflow run 40721515038 passed and published GitHub Release '0.3.0 - 2026-10-28', not draft, not prerelease, with the four atelier-cli-<target>.tar.xz archives, their .sha256, sha256.sum, source.tar.gz(+.sha256), dist-manifest.json and atelier-cli-installer.sh. Smoke check: the README one-liner in a clean HOME (env -i, no Rust on PATH) installed ~/.cargo/bin/atelier reporting 0.3.0+02ffeac and added ~/.cargo/env to .profile. Limits: upgrade-over-older-install and macOS not exercised here; independent check is atelier-g6a1.
