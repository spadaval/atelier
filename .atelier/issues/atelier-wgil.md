---
created_at: "2026-10-16T20:22:22.446783913+00:00"
id: "atelier-wgil"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Reject entries no side wrote in list resolutions of base merges"
updated_at: "2026-10-16T20:22:22.446783913+00:00"
---

## Goal

The three-way list merge in review_gate.rs checks only that a resolved list has the expected length and contains every expected entry, so with a duplicate already on one side a resolution can replace the duplicate with an entry nobody wrote (reproduced with hand-edited labels [dup, dup]).

## Done when

The list check also requires every resolved entry to be expected (or compares as sets), with a test for the duplicate case.

## Context

Deferred finding atelier-6dwg from the atelier-1isk review (approval atelier-c1ws). The CLI de-duplicates labels and links, so only hand-edited records can trigger it, and in-scope records stay status-only.
