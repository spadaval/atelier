---
created_at: "2026-11-10T14:45:44.158755899+00:00"
id: "atelier-yd9n"
evidence_type: "test"
commit: "e1c65365d841f88c19f0544a0a58f133e62ec67e"
result: "pass"
captured_at: "2026-11-10T14:45:43.999618988+00:00"
command: "bash -c 'cd docs/architecture/quality/scorecard-baseline/samples && python3 -c '\"'\"'import json; [print(m, sum(a[\"label\"] == b[\"label\"] for a, b in zip(json.load(open(m + \".labels.json\"))[\"labels\"], json.load(open(m + \".second.json\"))[\"labels\"])), \"of 30\") for m in (\"interventions\", \"refusals\", \"findings\")]'\"'\"''"
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
title: "Agreement recomputed from the committed label files: 22, 25 and 29 of 30"
updated_at: "2026-11-10T14:45:44.298636944+00:00"
---

## Summary

Agreement recomputed from the committed label files: 22, 25 and 29 of 30

## Command

```console
bash -c 'cd docs/architecture/quality/scorecard-baseline/samples && python3 -c '"'"'import json; [print(m, sum(a["label"] == b["label"] for a, b in zip(json.load(open(m + ".labels.json"))["labels"], json.load(open(m + ".second.json"))["labels"])), "of 30") for m in ("interventions", "refusals", "findings")]'"'"''
```

Exit status: 0

## Stdout

Bytes: 59
SHA-256: 28f37fd5e9073faf752fdc6d71dd1005c3623d5550d2c0c100e24dfbe9bee21f

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
