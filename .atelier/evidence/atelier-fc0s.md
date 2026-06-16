---
created_at: "2026-06-11T23:22:07.384180493+00:00"
id: "atelier-fc0s"
evidence_type: "validation"
captured_at: "2026-06-11T23:22:07.384127745+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nwlx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Tracked config and ignore policy validated: .atelier/config.toml is visible to git; .atelier/.gitignore was removed as generated local state; root .gitignore ignores only runtime/cache/identity/lock/worktree artifacts such as .atelier/state.db, .atelier/state.db-shm, .atelier/state.db-wal, .atelier/runtime/, .atelier/cache/, .atelier/agent.json, .atelier/.locks-cache/, .atelier-worktrees/, local hook/config/cache/rules artifacts. Validation passed: git status --short --untracked-files=all -- .atelier .gitignore .atelier-worktrees; git check-ignore -v for runtime/cache/lock/identity/worktree paths; git check-ignore -v .atelier/config.toml produced no match; git diff --check; cargo fmt -- --check; cargo test --no-run; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-nwlx."
updated_at: "2026-06-11T23:22:14.356819990+00:00"
---

Tracked config and ignore policy validated: .atelier/config.toml is visible to git; .atelier/.gitignore was removed as generated local state; root .gitignore ignores only runtime/cache/identity/lock/worktree artifacts such as .atelier/state.db, .atelier/state.db-shm, .atelier/state.db-wal, .atelier/runtime/, .atelier/cache/, .atelier/agent.json, .atelier/.locks-cache/, .atelier-worktrees/, local hook/config/cache/rules artifacts. Validation passed: git status --short --untracked-files=all -- .atelier .gitignore .atelier-worktrees; git check-ignore -v for runtime/cache/lock/identity/worktree paths; git check-ignore -v .atelier/config.toml produced no match; git diff --check; cargo fmt -- --check; cargo test --no-run; atelier export --check; atelier lint; atelier doctor; workflow validate issue atelier-nwlx.
