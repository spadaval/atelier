---
schema: atelier.review
schema_version: 3
round: 1
commit: e4297c91c601863dbd526678dc85d05abd32813b
reviewers:
- thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3Aa563d0e7-3706-4845-bb8c-b4afd6f2d370
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T16:04:01.301942958Z
---

## Summary

Complexity: medium. This consolidates two specifications, carries decisions from
31 ADRs, removes several documentation trees and relocates eval dependencies.
Value: high. Agents need documentation for the installed tracker rather than
the deleted implementation.
Risk of merging: low. The changes are documentation and fixture maintenance,
with errors visible and reversible through Git.

The value warrants the size: most of the diff removes obsolete material. I
reviewed master...e4297c91c601863dbd526678dc85d05abd32813b, compared the two
specifications directly with their previous versions, checked the ADR mapping
against the deleted ADRs, inspected the retained pages and entry points, and
checked the changes outside docs for scope. Words, Lifecycle and Review are
byte-for-byte unchanged. The two new ADR 0028 rows stay within its decisions
9 and 10, though the ADR migration has the omission below.

Validation:

- All ten next-* unit suites passed, 99 tests total, with NEXT_ATELIER pointing
  at the installed binary and TMPDIR under ~/.cache. All ten setup.sh scripts
  also exited 0 and created repo/ in separate temporary directories.
- An independent Python check of relative Markdown links and heading fragments
  in docs/, skills/, the root guidance files and scripts/cutover/README.md
  reported 92 links in 26 files, 0 broken.
- atelier check passed: 133 items, 7 missions and config.toml. cargo fmt --
  --check and git diff --check master...e4297c91 passed.
- The tracked-tree search for removed documentation paths matched the Report's
  listed exceptions. I inspected the open missions' mission.md and design.md
  references and confirmed that fleet-dashboard.md was retained in atelier-bnek.
- The cutover's local counts and verification reports agree with Transition.
  I did not rerun the cutover or operate the work host. No live eval agents
  were run. PR #143's Rust test job passed when checked; Desk CI was pending.

## Finding 1: Preserve the attribution and feed boundaries before deleting their ADRs
Recommendation: fix now

At docs/design.md:1247, the combined ADR 0030/0032 row preserves the database's
contents but drops governing decisions about its writers and data sources.
ADR 0030 decision 7 explicitly survives ADR 0032: each CLI command records its
own attribution row from its session environment, rather than through an MCP
server. ADR 0032 decision 6 assigns attribution writes to atelier, measurements
and findings to the unattended daily feed, and seen state to the Desk.
Decision 7 permits only that feed to read other sources such as transcripts,
requires one adapter per source and provenance for stored data, and forbids
live pages from reading T3 Code's own database.

These rules are explicit in the deleted ADRs (0030 lines 80-81 and 0032 lines
72-84), but are absent from the design record. The retained Desk product page
mentions a daily feed without preserving these restrictions; its API page says
the feed and database are not designed there. The Report maps the ADRs as
carried and supplies no reason to retire these decisions. A later attribution
or health implementation therefore loses the agreed boundary this item was
required to retain. Carry these rules into Decisions and update the mapping.

## Finding 2: Make a newly created contributor item ready before telling the reader to start it
Recommendation: fix now

CONTRIBUTING.md:10-12 tells a reader to find or create an item with atelier new,
then run atelier start <id>. The create path cannot follow that sequence:
atelier new creates a draft, while atelier start accepts a ready item (both
commands' --help state this). The instructions omit completing the definition
and recording the go-ahead with atelier ready <id> --by <who said go>.
AGENTS.md describes that step, but the explicit sequence here skips it and
leads a contributor following it into a refusal. Include the readiness step
for new drafts, or say to select an already-ready item before starting it.
