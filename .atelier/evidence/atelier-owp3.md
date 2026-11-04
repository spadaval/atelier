---
created_at: "2026-11-04T23:15:46.914756729+00:00"
id: "atelier-owp3"
evidence_type: "validation"
commit: "46e219e680b005481e43451862e6e910b8d483a3"
result: "pass"
captured_at: "2026-11-04T23:15:46.914600958+00:00"
path: "README.md"
uri: "https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh"
agent_identity: "Claude Opus 5.5 (independent validator, atelier-g6a1)"
target:
  kind: "issue"
  id: "atelier-g6a1"
  role: "validates"
criteria:
- slug: "S3"
  result: "pass"
criteria_mission: "atelier-vt8q"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g6a1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S3. Clean HOME ~/.cache/atelier-g6a1/s3/home (new, empty), commands run under env -i with PATH=/usr/bin:/bin and SHELL=/bin/bash; command -v showed cargo, rustc, rustup and atelier all absent before each install. (1) Older install: curl --proto '=https' --tlsv1.2 -LsSf .../releases/download/v0.3.0/atelier-cli-installer.sh | sh exited 0, 'downloading atelier-cli 0.3.0 x86_64-unknown-linux-gnu', installed ~/.cargo/bin/atelier, wrote ~/.cargo/env and appended '. \"$HOME/.cargo/env\"' to ~/.profile (and .zshrc, fish conf.d); a new login shell (bash -l and sh -l, starting from PATH=/usr/bin:/bin) resolved atelier to ~/.cargo/bin/atelier, 'atelier 0.3.0+02ffeac', still no cargo/rustc/rustup. (2) Upgrade: the README one-liner (releases/latest/download/atelier-cli-installer.sh, byte-identical to the v0.3.1 release asset, sha256 cb8ad6af...) exited 0, 'downloading atelier-cli 0.3.1'; a new login shell then gave 'atelier 0.3.1+d14d2e5'; ~/.profile unchanged on rerun (no duplicate line); install receipt version 0.3.1; the installed binary is byte-identical to the release x86_64-unknown-linux-gnu archive's. Second clean HOME (s3-bashrc) with a pre-existing empty ~/.bashrc: the installer appended the env line there and non-login interactive bash -i resolved atelier 0.3.1. Transcripts in ~/.cache/atelier-g6a1/s3/0[1-6]-*.log. Limits: x86_64 Linux only, glibc 2.43 host, so only the x86_64-unknown-linux-gnu path ran. Not run: the musl fallback (gnu builds need GLIBC_2.34 by objdump, and the installer switches to the static musl build v0.3.1 added when check_glibc 2.35 fails, but no older-glibc host or container was used), aarch64 Linux, and macOS. 'No Rust toolchain' was shown by PATH and an empty HOME, not a separate machine; the host has a toolchain under the real HOME. In a HOME with no ~/.bashrc, the installer writes only ~/.profile, so a non-login interactive bash finds atelier only after the next login (README says to open a new shell)."
updated_at: "2026-11-04T23:15:46.915658352+00:00"
---

S3. Clean HOME ~/.cache/atelier-g6a1/s3/home (new, empty), commands run under env -i with PATH=/usr/bin:/bin and SHELL=/bin/bash; command -v showed cargo, rustc, rustup and atelier all absent before each install. (1) Older install: curl --proto '=https' --tlsv1.2 -LsSf .../releases/download/v0.3.0/atelier-cli-installer.sh | sh exited 0, 'downloading atelier-cli 0.3.0 x86_64-unknown-linux-gnu', installed ~/.cargo/bin/atelier, wrote ~/.cargo/env and appended '. "$HOME/.cargo/env"' to ~/.profile (and .zshrc, fish conf.d); a new login shell (bash -l and sh -l, starting from PATH=/usr/bin:/bin) resolved atelier to ~/.cargo/bin/atelier, 'atelier 0.3.0+02ffeac', still no cargo/rustc/rustup. (2) Upgrade: the README one-liner (releases/latest/download/atelier-cli-installer.sh, byte-identical to the v0.3.1 release asset, sha256 cb8ad6af...) exited 0, 'downloading atelier-cli 0.3.1'; a new login shell then gave 'atelier 0.3.1+d14d2e5'; ~/.profile unchanged on rerun (no duplicate line); install receipt version 0.3.1; the installed binary is byte-identical to the release x86_64-unknown-linux-gnu archive's. Second clean HOME (s3-bashrc) with a pre-existing empty ~/.bashrc: the installer appended the env line there and non-login interactive bash -i resolved atelier 0.3.1. Transcripts in ~/.cache/atelier-g6a1/s3/0[1-6]-*.log. Limits: x86_64 Linux only, glibc 2.43 host, so only the x86_64-unknown-linux-gnu path ran. Not run: the musl fallback (gnu builds need GLIBC_2.34 by objdump, and the installer switches to the static musl build v0.3.1 added when check_glibc 2.35 fails, but no older-glibc host or container was used), aarch64 Linux, and macOS. 'No Rust toolchain' was shown by PATH and an empty HOME, not a separate machine; the host has a toolchain under the real HOME. In a HOME with no ~/.bashrc, the installer writes only ~/.profile, so a non-login interactive bash finds atelier only after the next login (README says to open a new shell).
