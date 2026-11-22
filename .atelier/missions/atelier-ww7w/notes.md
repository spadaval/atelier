# Lead's notes

## 2026-11-22: order and reviewers

- `atelier-1n49`, `atelier-7637` and `atelier-js4a` run in parallel. The
  three CLI items run one at a time, `atelier-h6ij`, then `atelier-gqtw`,
  then `atelier-zc9b`, because all three edit `output.rs` and `say.rs`.
- Implementers are Claude Opus 5.5. Codex is out of allowance until
  2026-11-23 00:04, so until then the cross-lab code reviewer is Grok 4.6.
- `atelier-h6ij` must test a reverted item edit on an amendment branch, so
  the `<base>...<prev>` half of the union rule is exercised (round 5 plan
  review, residual).
- Nothing in this mission edits `t3-extension/` or
  `skills/agent-factory/roles/reviewer.md`.

## 2026-11-22: the plan's round 4

Round 4 of the plan review split: `4-a5oe` approved and `4-fr4u` asked for
changes. The planner made those changes in `250b429`, and both round 5
reviewers approved that commit. Nothing is left to rule on.

## 2026-11-22: follow-ups filed

Filed as drafts, from what `atelier-1n49` and `atelier-js4a` found and left:
`atelier-jz8q` (move `/api/history` and `/api/launch` to the log, and bring
`t3-extension/README.md` up to date, when `t3-extension/` reopens),
`atelier-tuln` and `atelier-itqc` (two `atelier log` bugs), `atelier-vfnn`
(`scripts/ci-jobs.sh` fails open) and `atelier-kjc4` (the extension's test
fails now and then in CI).

## 2026-11-22: after `atelier-7637`

Filed `atelier-10d6` (`start --backlog` advises another slug, which the garden
procedure now forbids) and marked the draft `atelier-bbia` stale: it and two
lines of `docs/product/desk.md` still describe a garden session. The live
skill checkout, `~/projects/atelier-skill`, was moved to `4d7fab468`.

## 2026-11-22: rulings and departures

- **Order.** `atelier-gqtw` and `atelier-zc9b` ran side by side, not one
  after the other as the design suggested. `atelier-zc9b` merged first and
  `atelier-gqtw` then took one base merge and a fresh review round. That
  was cheaper than leaving `atelier-zc9b` idle for the whole of
  `atelier-gqtw`.
- **An amendment's first review round** gets the delta brief from the last
  agreed round, not the whole plan, as `atelier-h6ij` built it. Kept: the
  amendment's change is what is under review.
- **`atelier-gqtw`'s reviewer.** Grok 4.6 ran out of balance 15 minutes into
  round 1 and left no verdict. Gemini 4 Neon reviewed in its place, asked
  for two changes in round 1 and approved round 2. Because the change
  deletes remote branches, the lead also read `cleanup.rs` and the claim and
  drop paths, and found nothing to block. Codex was out of allowance all
  day, so no item of this mission had a GPT reviewer.
- **`atelier-fxwl`** is outside the plan: one attribute that silences a
  warning every release build printed, found while installing `master`.
- **No rule was passed.** No `ATELIER_SKIP` was used on any item's merge.
- **A `forge.remote` given as a URL** has no remote-tracking base, so
  `start` would refuse and `merge` would keep its branches. Not filed: no
  repository here is configured that way.

## 2026-11-22: rolled out

The shared `atelier` on this host was installed from `435025919` and
`~/projects/atelier-skill` moved to it. The first real deletion was
`atelier-gqtw`'s own branch: `atelier merge` run again on it deleted the
branch on `origin` and locally and left that checkout on a detached HEAD.

## 2026-11-22: follow-ups filed at the close

`atelier-p8bu` (a skip on a plan `submit` with nothing to commit is not
recorded), `atelier-in1t` (the brief's whole-change diff reads the local
base), `atelier-5asm` (three printed commands that are refused),
`atelier-5erp` (the evals' tests leave temp directories), `atelier-a9jo`
(the cutover template still bans every remote branch deletion) and
`atelier-vv54` (the Desk hides malformed Responses lines, from deferred
finding `atelier-zc9b/1-4861.1`). The mission's other deferred findings
were settled against the items already filed for them.
