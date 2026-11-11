# Notes of atelier-295u

What the old mission record held beyond its mission sections.

## Outcome

`atelier log` prints the history of acts. A gardener run starts from that, from `atelier list` and from a readiness checklist for the repository, and reads transcripts only to explain a number that moved. It ends with one page for Sumeet: the numbers, the worst spots and at most three proposals, each a root cause with the trips that show it. One run has been made in each of the five repositories.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](../../../docs/product.md) and the [architecture specification](../../../docs/design.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-09 [note] Amended on Sumeet's direction (2026-11-09, in conversation), before any mission was made ready: the gardener is named as a role; the mission builds one read command, `atelier log`, the history of acts, and the scorecard's arithmetic lives in the gardener's procedure, not in a `scorecard` command; a bug item names what caused it; a deferred finding is settled with `new --from` or `drop`. The command count is 18.
- 2026-11-11 [note] Where the mission stands at the end of the lead's run. Every item is done: atelier-vl5n (pull request 144), atelier-8b0a (145), atelier-9dku (146), atelier-x3n5 (147), atelier-f6dv (149) and atelier-613a, whose Report lists the five run items. The mission is left open on purpose, with no Result written: the five runs are approved and wait for their sessions with Sumeet, and their merges are his. Both criteria hold today. `one-page`: five pages of 36 to 46 lines, nine proposals in all. `from-log`: the procedure's steps take time in each status, rounds, take-overs, escalations and skips from `atelier log`, and `scripts/scorecard/` no longer walks record history or matches a thread's title. Whoever closes the mission runs `atelier lead atelier-295u`, writes the Result and runs `atelier close atelier-295u`. Follow-ups outside the mission: atelier-h201, atelier-e7iy, atelier-52g7; atelier-c1z3 now waits only on the rerun's frozen inputs.
