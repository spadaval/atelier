---
schema: atelier.review
schema_version: 1
id: atelier-ztla
title: 'low: crates/atelier-cli/src/commands/setup.rs:72: the ''set -o pipefail; '' prefix changes nothing for the hook''s own command (it has no pipe) and depends on the shell Claude Code runs hooks in. Claude Code 2.1.306 defaults command hooks to bash on Unix, where it works; under a /bin/sh that is dash older than 0.5.12-7 (Debian 12, Ubuntu 22.04/24.04), ''set -o pipefail'' is an illegal option of a special builtin, the shell exits 2, and a Stop hook exiting 2 blocks every stop. Replacement of already installed hooks is backward compatible (old handlers end with the same atelier-managed marker and are replaced; unreplaced old hooks still work). Keep as is unless hooks may run under sh; then use ''(set -o pipefail) 2>/dev/null && set -o pipefail; ''.'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 6b8b1ccb31042d1a3a7b167903f6a43b4a9acdf7
created_at: 2026-10-22T21:06:42.513565619Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-cli/src/commands/setup.rs:72: the 'set -o pipefail; ' prefix changes nothing for the hook's own command (it has no pipe) and depends on the shell Claude Code runs hooks in. Claude Code 2.1.306 defaults command hooks to bash on Unix, where it works; under a /bin/sh that is dash older than 0.5.12-7 (Debian 12, Ubuntu 22.04/24.04), 'set -o pipefail' is an illegal option of a special builtin, the shell exits 2, and a Stop hook exiting 2 blocks every stop. Replacement of already installed hooks is backward compatible (old handlers end with the same atelier-managed marker and are replaced; unreplaced old hooks still work). Keep as is unless hooks may run under sh; then use '(set -o pipefail) 2>/dev/null && set -o pipefail; '.
