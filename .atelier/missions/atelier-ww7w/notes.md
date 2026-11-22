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
