---
schema: atelier.review
schema_version: 2
id: atelier-stns
title: finding on atelier-1sbw @bf9a2e65
target: atelier-1sbw
identity: gpt-6.1-sol-r3
commit: bf9a2e656ca3994af04fa9b0aa6059c043e0c084
created_at: 2026-11-10T23:26:16.560635370Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:501 R2 Multiline Markdown reference definitions are truncated when copied between split strategy documents and their destinations are not rebased. In the invented fixture replace the single-line [m]: mocks/a.md#a-mock definition under Shape with [m]: followed by a newline and two spaces then mocks/a.md#a-mock. CommonMark recognizes this definition in the original strategy. Conversion leaves mission.md using [the mock][m] but appends only [m]: with no destination, and design.md retains the old relative mocks/a.md destination. A local markdown_it CommonMark parse confirms mission.md has no M definition and the design.md reference resolves to a missing file. Conversion and --verify both exit 0. REFERENCE captures only one line and LINK accepts no newline before a reference destination. Preserve the complete definition, including continuation lines, and rewrite its destination before copying it into each output that uses it. Add a regression checking reference resolution in each output document, rather than just the presence of a [m]: prefix.
