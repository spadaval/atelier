---
created_at: "2026-11-10T14:45:42.504364294+00:00"
id: "atelier-oy57"
evidence_type: "test"
commit: "e1c65365d841f88c19f0544a0a58f133e62ec67e"
result: "pass"
captured_at: "2026-11-10T14:45:25.608979968+00:00"
command: "bash -c 'D=$HOME/.cache/atelier-3kr7/redraw; rm -rf \"$D\"; nice -n 19 ionice -c3 python3 scripts/scorecard/draw_samples.py --out \"$D\" && for m in interventions refusals findings; do cmp \"$D/$m.json\" docs/architecture/quality/scorecard-baseline/samples/$m.json && echo \"$m identical\"; done'"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-3kr7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3kr7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The three samples redraw byte-identical from the frozen snapshot and records with seed 20261110"
updated_at: "2026-11-10T14:45:42.651131252+00:00"
---

## Summary

The three samples redraw byte-identical from the frozen snapshot and records with seed 20261110

## Command

```console
bash -c 'D=$HOME/.cache/atelier-3kr7/redraw; rm -rf "$D"; nice -n 19 ionice -c3 python3 scripts/scorecard/draw_samples.py --out "$D" && for m in interventions refusals findings; do cmp "$D/$m.json" docs/architecture/quality/scorecard-baseline/samples/$m.json && echo "$m identical"; done'
```

Exit status: 0

## Stdout

Bytes: 2537
SHA-256: 96a0f6e2344c647487cd89846140c19e8bbc470414eab694550eebb4d534aa31

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
