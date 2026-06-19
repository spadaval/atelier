---
created_at: "2026-06-19T04:11:45.381915555+00:00"
id: "atelier-zwe9"
issue_type: "epic"
labels:
- "config"
- "doctor"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-oe7c"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T05:03:04.255795476+00:00"
status: "done"
title: "Epic: Add review backend health checks to doctor"
updated_at: "2026-06-19T05:03:04.255795476+00:00"
---

## Description

Extend `atelier doctor` so configured review backends are part of routine
repository health. When a project configures a provider-backed review or PR
backend, doctor should detect missing credentials, invalid credentials,
unreachable provider APIs, and repository/role mapping problems before workflow
closeout reaches PR validators. Room mode or projects without a configured
provider backend should report the backend check as skipped rather than
requiring provider secrets.

Scope includes non-secret diagnostics, `.env`/runtime environment parity with
normal review commands, Forgejo provider health for the current backend, and
documentation or role guidance that makes the check visible to agents. Scope
does not include creating, rotating, printing, or repairing secrets.

## Outcome

- `atelier doctor` reports review backend health whenever project configuration
  defines a provider-backed review or PR backend.
- The provider health check uses the same token environment name and `.env`
  loading behavior as review/PR commands and workflow validators.
- Missing token environment variables, invalid credentials, unreachable hosts,
  missing repositories, and unsuitable role-author/provider mappings produce
  direct corrective guidance without exposing secret values.
- Room mode and no-provider configurations report a clear skipped/not-configured
  state and do not fail because Forgejo credentials are absent.
- `doctor --fix` remains local-state repair only; it does not create, edit, or
  persist credentials.
- Mission and role guidance make `doctor` the preflight check agents can run
  before relying on review provider commands or PR-backed close validators.

## Evidence

- Focused doctor tests cover configured Forgejo success, missing token,
  invalid token or unauthorized response, unreachable backend, no provider
  configured, and room-mode skip behavior.
- Command transcript demonstrates `atelier doctor` surfaces review backend
  health without printing the token and that `atelier doctor --fix` does not
  modify secrets.
- Provider/review command tests prove the doctor check shares configuration and
  environment handling with the live review/PR command paths.
- `atelier lint atelier-zwe9`, relevant focused tests, `atelier lint`, and
  `git diff --check` pass.
