---
created_at: "2026-07-16T23:59:25.809109211+00:00"
id: "atelier-havm"
evidence_type: "validation"
captured_at: "2026-07-16T23:59:25.500582462+00:00"
command: "bash -lc 'set -euo pipefail\nskill=.agents/skills/agent-factory/SKILL.md\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\nmission_review_line=$(grep -nF \"3. If the work starts from an exact mission draft\" \"$skill\" | cut -d: -f1)\norchestrate_line=$(grep -nF \"4. If work spans a mission, epic, or multiple tracker items\" \"$skill\" | cut -d: -f1)\ntest \"$mission_review_line\" -lt \"$orchestrate_line\"\ngrep -Fq \"This rule takes precedence over generic mission, epic, or\" \"$skill\"\ngrep -Fq \"Expected route: mission-review.\" \"$fixture\"\ngrep -Fq \"Forbidden route: orchestrate until Atelier reports current independent approval\" \"$fixture\"\ngrep -Fq \"Assertion: exact-draft mission-review routing takes precedence over generic\" \"$fixture\"\nprintf \"routing precedence: mission-review rule line %s precedes orchestrate rule line %s; ambiguous fixture selects mission-review\\n\" \"$mission_review_line\" \"$orchestrate_line\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3wg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3wg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\nskill=.agents/skills/agent-factory/SKILL.md\nfixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md\nmission_review_line=$(grep -nF \"3. If the work starts from an exact mission draft\" \"$skill\" | cut -d: -f1)\norchestrate_line=$(grep -nF \"4. If work spans a mission, epic, or multiple tracker items\" \"$skill\" | cut -d: -f1)\ntest \"$mission_review_line\" -lt \"$orchestrate_line\"\ngrep -Fq \"This rule takes precedence over generic mission, epic, or\" \"$skill\"\ngrep -Fq \"Expected route: mission-review.\" \"$fixture\"\ngrep -Fq \"Forbidden route: orchestrate until Atelier reports current independent approval\" \"$fixture\"\ngrep -Fq \"Assertion: exact-draft mission-review routing takes precedence over generic\" \"$fixture\"\nprintf \"routing precedence: mission-review rule line %s precedes orchestrate rule line %s; ambiguous fixture selects mission-review\\n\" \"$mission_review_line\" \"$orchestrate_line\"'"
updated_at: "2026-07-16T23:59:25.811528860+00:00"
---

## Summary

bash -lc 'set -euo pipefail
skill=.agents/skills/agent-factory/SKILL.md
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
mission_review_line=$(grep -nF "3. If the work starts from an exact mission draft" "$skill" | cut -d: -f1)
orchestrate_line=$(grep -nF "4. If work spans a mission, epic, or multiple tracker items" "$skill" | cut -d: -f1)
test "$mission_review_line" -lt "$orchestrate_line"
grep -Fq "This rule takes precedence over generic mission, epic, or" "$skill"
grep -Fq "Expected route: mission-review." "$fixture"
grep -Fq "Forbidden route: orchestrate until Atelier reports current independent approval" "$fixture"
grep -Fq "Assertion: exact-draft mission-review routing takes precedence over generic" "$fixture"
printf "routing precedence: mission-review rule line %s precedes orchestrate rule line %s; ambiguous fixture selects mission-review\n" "$mission_review_line" "$orchestrate_line"'

## Command

```console
bash -lc 'set -euo pipefail
skill=.agents/skills/agent-factory/SKILL.md
fixture=.agents/skills/agent-factory/fixtures/mission-review-handoffs.md
mission_review_line=$(grep -nF "3. If the work starts from an exact mission draft" "$skill" | cut -d: -f1)
orchestrate_line=$(grep -nF "4. If work spans a mission, epic, or multiple tracker items" "$skill" | cut -d: -f1)
test "$mission_review_line" -lt "$orchestrate_line"
grep -Fq "This rule takes precedence over generic mission, epic, or" "$skill"
grep -Fq "Expected route: mission-review." "$fixture"
grep -Fq "Forbidden route: orchestrate until Atelier reports current independent approval" "$fixture"
grep -Fq "Assertion: exact-draft mission-review routing takes precedence over generic" "$fixture"
printf "routing precedence: mission-review rule line %s precedes orchestrate rule line %s; ambiguous fixture selects mission-review\n" "$mission_review_line" "$orchestrate_line"'
```

Exit status: 0

## Stdout

Bytes: 124
Truncated: no

```text
routing precedence: mission-review rule line 72 precedes orchestrate rule line 77; ambiguous fixture selects mission-review
```

## Stderr

Bytes: 0
Truncated: no

```text
```
