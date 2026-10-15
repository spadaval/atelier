---
created_at: "2026-10-15T18:45:05.788110562+00:00"
id: "atelier-g0zm"
issue_type: "bug"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make HEAD-tag shadow fixtures portable to git 2.53"
updated_at: "2026-10-15T18:46:02.834750959+00:00"
---

## Goal

The two tests that create a tag named HEAD pass on git 2.53 and older versions while still covering a `refs/tags/HEAD` shadow ref.

## Done when

`commands::git::tests::native_checkout_queries_preserve_recursive_heads_and_refresh_linked_checkouts` and `captured_checkout_refs_preserve_head_tags_aliases_and_base_worktrees` pass on git 2.53.0; the full suite passes under `cargo test` and `cargo nextest run`.

## Scope

Test fixtures only.

## Context

Git 2.53 rejects `git tag HEAD <sha>` with "'HEAD' is not a valid tag name". Repositories created with older git may still hold refs/tags/HEAD, so the shadow scenario is still wanted.
