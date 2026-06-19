---
created_at: "2026-06-19T01:24:05.576534698+00:00"
id: "atelier-q2bt"
issue_type: "task"
labels:
- "docs"
- "process"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Document provider-neutral PR review artifact guidance"
updated_at: "2026-06-19T01:24:05.576534698+00:00"
---

## Description

Align the PR/review-artifact product guidance with the current policy choice:
code-changing epic work should use a PR-equivalent review workspace, but
Atelier should not require Forgejo specifically and should not build a native
Markdown/JSON PR system now.

## Outcome

Product, domain, workflow, and Agent Factory guidance describe review artifacts
in provider-neutral terms. The docs explain when workers, reviewers,
validators, and orchestrators should use a PR-equivalent workspace, what belongs
there, what remains in Atelier evidence/activity, and that Forgejo is the
current provider rather than the product concept. Native Markdown state remains
proof/history, not an alternate PR platform.

## Evidence

Documentation diff maps the provider-neutral review-artifact vocabulary across
CONTEXT, product workflow/validation/CLI guidance, ADR 0010, and Agent Factory
role guidance. Focused search and lint/export checks show stale Forgejo-specific
target-state wording was removed or deliberately scoped to the current provider.
