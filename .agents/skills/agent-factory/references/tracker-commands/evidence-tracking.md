# Evidence Tracking

Use this reference to record or inspect proof in the tracker. The procedures
decide what needs recording. As a rule of thumb, record evidence in the tracker
when later work or a later agent will rely on it, for example epic or mission
acceptance, independent validation, migrations, and non-pass results. For
ordinary slices, proof in the PR or handoff is enough.

## Evidence Receipts

An evidence receipt is a durable record in the tracker, not prose buried in a
status update. It states the claim, what was done to check it, the result, and
links to transcripts or artifacts. Cite the receipt's ID or URL.

## Shared Vocabulary

Use these terms verbatim so evidence stays searchable across trackers:

- Claim results: `pass`, `fail`, `blocked`, `not-applicable`.
- Cause of a failure: `defect in this change`, `expected breakage`,
  `environment/tooling`, `pre-existing`. `expected breakage` applies only on
  an isolated rewrite branch declared in the plan, never on the default
  branch.
- Finding dispositions: `FIX NOW`, `DEFER`, `NO ACTION`.

## Atelier

- Use `atelier evidence record` for manual proof or captured command output.
  Manual receipts require an explicit `--result`; command capture derives pass
  or fail from exit status and pins current HEAD.
- Every receipt identifies a full Git commit and one result: `pass`, `fail`,
  `blocked`, or `not-applicable`. Presence alone is not passing proof.
- Use `atelier evidence show` and `list` to inspect receipts. Use `attach` only
  when reusing valid proof for another item.
- Attach proof to the work it supports; name the producer, kind, summary,
  artifacts, and limits. Add a new receipt to correct earlier proof.
- Freshness matters: an older pass cannot cover a later relevant change, and
  a newer failure blocks completion. Check `atelier help evidence` for fields.

## GitHub Issues

- Record a receipt as a structured comment on the relevant issue, using
  `gh issue comment <number> --body-file <file>` for non-interactive capture.
- Use the comment URL as the evidence reference. Link Actions runs, checks,
  commits, PRs, and screenshots rather than pasting long transcripts.
- For independent validation, name the validator and give each claim a result.
- Correct evidence by appending a new receipt. Do not silently edit evidence
  that later work may rely on.
