---
created_at: "2026-10-16T18:59:58.517648334+00:00"
id: "atelier-42o0"
evidence_type: "validation"
commit: "a1a46c8284873af4fd7708fa333f0cf3f3a6903d"
result: "pass"
captured_at: "2026-10-16T18:58:57.041308398+00:00"
command: "python3 scripts/bench_read_commands.py --runs 3"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-83xi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-83xi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "python3 scripts/bench_read_commands.py --runs 3"
updated_at: "2026-10-16T18:59:58.518088936+00:00"
---

## Summary

python3 scripts/bench_read_commands.py --runs 3

## Command

```console
python3 scripts/bench_read_commands.py --runs 3
```

Exit status: 0

## Stdout

Bytes: 3023
Truncated: no

```text
Fixture: 1000 issues, 200 evidence records in /tmp/atelier-bench-8wi06h_v
Guard fixture: 1000 issues, 201 evidence, one approval in /tmp/atelier-bench-guards-03wg0ftt
History fixture: 60 integrated owners, 1745 commits in /tmp/atelier-bench-history-579o_vln
Binary: /home/supadava/projects/atelier-wt/83xi/target/release/atelier; 3 runs after one warm-up
CPU cores available: 12 of 12

| Command | median ms | max ms |
| --- | ---: | ---: |
| query: `atelier work` | 41.8 | 46.4 |
| query: `atelier work ready` | 42.2 | 44.5 |
| query: `atelier work blocked` | 43.9 | 47.6 |
| query: `atelier work active` | 41.4 | 43.5 |
| query: `atelier work all` | 41.7 | 43.0 |
| query: `atelier work missions` | 41.4 | 42.1 |
| query: `atelier work missions --all` | 43.7 | 46.4 |
| query: `atelier work parent <epic>` | 72.2 | 73.9 |
| query: `atelier status` | 50.2 | 52.5 |
| query: `atelier issue show <epic>` | 41.9 | 43.2 |
| query: `atelier issue show <mission>` | 41.7 | 43.3 |
| query: `atelier issue show atelier-t000` | 47.8 | 49.7 |
| query: `atelier work mission <mission>` | 39.0 | 42.2 |
| query: `atelier work mission <mission> --ready` | 40.4 | 40.5 |
| query: `atelier work mission <mission> --blocked` | 39.3 | 39.6 |
| query: `atelier work mission <mission> --active` | 39.1 | 39.4 |
| query: `atelier work mission <mission> --done` | 38.3 | 39.2 |
| query: `atelier work mission <mission> --all` | 38.8 | 39.8 |
| query: `atelier issue list` | 38.8 | 39.0 |
| query: `atelier issue list --limit 1000` | 38.8 | 39.3 |
| query: `atelier issue list --status done` | 38.2 | 39.3 |
| query: `atelier issue list --label cli --issue-type task` | 39.8 | 41.2 |
| query: `atelier issue transition atelier-e002` | 44.2 | 45.5 |
| query: `atelier issue transition atelier-e002 --verbose` | 41.7 | 45.6 |
| query: `atelier evidence list` | 38.3 | 39.0 |
| query: `atelier evidence list --result pass` | 38.3 | 38.8 |
| query: `atelier evidence show atelier-v000` | 41.1 | 42.9 |
| query: `atelier review list` | 38.1 | 38.6 |
| query: `atelier review list --json` | 39.4 | 40.5 |
| query: `atelier check` | 41.4 | 43.2 |
| query: `atelier check <epic>` | 40.8 | 41.5 |
| query: `atelier prime` | 58.9 | 59.5 |
| query: `atelier prune` | 61.6 | 63.4 |
| query: `atelier prune --quiet` | 63.8 | 65.0 |
| query: `atelier bundle preview /home/supadava/projects/atelier-wt/83xi/docs/spec/bundle/fixtures/valid-bundle.json` | 39.2 | 40.5 |
| guard: `atelier prime` | 76.7 | 77.4 |
| guard: `atelier land` | 129.0 | 130.5 |
| guard: `atelier land atelier-s000` | 121.9 | 124.6 |
| guard: `atelier merge atelier-s000 --check` | 129.9 | 140.0 |
| guard: `atelier merge atelier-s000 --check --json` | 129.9 | 136.3 |
| guard: `atelier review list atelier-s000` | 36.1 | 36.5 |
| guard: `atelier review list atelier-s000 --json` | 36.7 | 38.0 |
| guard: `atelier review show atelier-bench-8wi06h-v-ia9h` | 36.6 | 40.3 |
| refusal: `atelier land` | 69.1 | 69.2 |
| history: `atelier land atelier-bench-8wi06h-v-xbfk` | 3294.4 | 3638.6 |
```

## Stderr

Bytes: 0
Truncated: no

```text
```

