---
schema: atelier.mission
schema_version: 3
id: atelier-4pzj
title: Let agents and the human see where things stand, which work they can start, and what to do next
status: dropped
labels:
- cli-output
lead: null
ready: null
created_at: 2026-10-17T01:44:17.818841106Z
---

## Outcome

An agent or the human learns where things stand from Atelier's own views, without rebuilding it from `git log` or several commands: the checkout, each mission and each item (its branch, review state and links), which work can be started now, what holds the rest up, what changed since a given commit, and what to do next. `start` takes what those views offer as ready, and refuses anything else with one line per blocker or reason. Every view is complete or names the command that shows the rest, and reads the same when piped.

## Success criteria

- ready-means-startable: in a fixture repository with in-mission, cross-mission and unassigned blockers, parked items, and an item in flight on another branch, every ready row `status` and `work ready` show without a mark is accepted by `start`; every item `start` refuses is either not offered as ready or carries the mark that explains why; and a refusal for a blocker names the same blocker that `work blocked` and `issue list` show. Shown by one scenario test.

## Result

Dropped: Every one of its 29 items described the old tool's status, work views, issue commands or start, which the 0.5 rewrite replaced; each is dropped with its own reason. Its outcome is what status, show, list and log were rebuilt to give.
