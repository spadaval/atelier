---
schema: atelier.mission
schema_version: 3
id: atelier-bnek
title: Record which agent did what, and compare models by role
status: draft
labels:
- agent-attribution
- fleet-dashboard
lead: null
ready: null
created_at: 2026-11-09T01:04:22.410061463Z
---

## Why

Codex models now share the work with Claude, and nothing records which model
implemented, reviewed or planned what. Error rates, review counts and token
cost cannot be compared by model or role, so choosing which model does which
job is guesswork. T3 Code, Atelier and Agent Factory are Sumeet's whole stack,
so Atelier may lean on T3 Code wherever that helps.

## Outcome

Every `atelier` command run in a T3 thread records which thread, harness,
provider, model, effort and role did it; review records name the reviewer's
provider and model in structured fields; and the site from the fleet-dashboard
mission shows who is working on what, links each thread into T3, and reports
work, review outcomes and token cost by model and role.

## Success criteria

- who-did-what: on one real item implemented by a Claude model and reviewed by
  a Codex model, the item's page names each thread's provider, model, effort
  and role, links it into T3, and gives each role's token cost, matching the
  transcripts within the reported unattributed share. Shown by an independent
  validation item.

## Scope

In: the session database and recording, the reminder hook, structured reviewer
identity, the T3 read spike, live thread status on the site, usage ingestion,
the attribution check, the metrics page, the backfill, and the validation.

Out: the site itself (fleet-dashboard mission, which this mission's pages
build on); forking T3 Code; setting `T3CODE_THREAD_ID` for Grok in T3 (worth an
upstream request, not needed here).

Environment: this host, all Atelier repositories under `~/projects`.

## Authority

The orchestrator may change items, order and design within ADR 0030 and this
Shape, including the attribution rule. Changing the Outcome or criterion, or
recording outside the CLI, needs Sumeet.
