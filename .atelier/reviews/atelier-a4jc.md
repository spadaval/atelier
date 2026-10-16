---
schema: atelier.review
schema_version: 1
id: atelier-a4jc
title: 'crates/atelier-cli/src/commands/setup.rs:104-118: default_settings_permissions hard-codes 0o644 and ignores the process umask, but its comment says ''like files editors create'', and editors create 0666 & ~umask. A user with umask 077 gets a world-readable new .claude/settings.json. That is harmless for a project file meant to be committed, but the comment is inaccurate. On non-unix, the function creates a throwaway file in the system temp dir and calls .expect on it, so setup panics if the temp dir is unavailable. Keeping the staged file''s own permissions would avoid that, since Windows only tracks readonly. The existing-mode path is correct: symlinks are refused earlier (setup.rs:11-18), and a chmod racing the write is the only thing that can be lost. The new test covers both the create and the keep branches.'
target: atelier-v44v
identity: Independent reviewer (Claude subagent)
commit: 656cef00ae542b1bad641b1c8392e5ad37dd5069
created_at: 2026-10-16T19:30:52.163760356Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/src/commands/setup.rs:104-118: default_settings_permissions hard-codes 0o644 and ignores the process umask, but its comment says 'like files editors create', and editors create 0666 & ~umask. A user with umask 077 gets a world-readable new .claude/settings.json. That is harmless for a project file meant to be committed, but the comment is inaccurate. On non-unix, the function creates a throwaway file in the system temp dir and calls .expect on it, so setup panics if the temp dir is unavailable. Keeping the staged file's own permissions would avoid that, since Windows only tracks readonly. The existing-mode path is correct: symlinks are refused earlier (setup.rs:11-18), and a chmod racing the write is the only thing that can be lost. The new test covers both the create and the keep branches.
