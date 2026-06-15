---
created_at: "2026-06-12T22:26:35.627299496+00:00"
id: "atelier-83g9"
evidence_type: "validation"
captured_at: "2026-06-12T22:26:35.216778258+00:00"
command: "bash -lc '\nset -euo pipefail\nA=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d /tmp/atelier-6aor-create.XXXXXX)\ncleanup() { rm -rf \"$TMP\"; }\ntrap cleanup EXIT\ncd \"$TMP\"\ngit init -q\nprintf \"\\n$ %s init\\n\" \"$A\"\n\"$A\" init | sed -n \"1,6p\"\nprintf \"\\n$ %s mission create ...\\n\" \"$A\"\n\"$A\" mission create \"Readable mission validation\" --body \"Readable mission intent\" --constraint \"Keep mission records reviewable\" --risk \"Regression could reintroduce escaped data\" --validation \"Run focused validation transcript\" | sed -n \"1,24p\"\nMID=$(basename .atelier/missions/*.md .md)\nprintf \"\\n$ %s mission update %s ...\\n\" \"$A\" \"$MID\"\n\"$A\" mission update \"$MID\" --body \"Updated readable mission intent\" --constraint \"Keep mission records reviewable after update\" --risk \"Projection drift\" --validation \"Run rebuild export lint doctor\" | sed -n \"1,24p\"\nprintf \"\\n$ sed -n 1,56p .atelier/missions/%s.md\\n\" \"$MID\"\nsed -n \"1,56p\" \".atelier/missions/$MID.md\"\nprintf \"\\n$ rg ^data: .atelier/missions -g *.md\\n\"\nif rg \"^data:\" .atelier/missions -g \"*.md\"; then\n  echo \"unexpected stale mission data JSON\"\n  exit 1\nelse\n  echo \"No mission data front matter found.\"\nfi\n'"
exit_status: "0"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6aor"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Positive mission create/update writes readable records without data front matter"
updated_at: "2026-06-12T22:26:36.923380458+00:00"
---

Positive mission create/update writes readable records without data front matter

Command: bash -lc '
set -euo pipefail
A=/root/atelier/target/debug/atelier
TMP=$(mktemp -d /tmp/atelier-6aor-create.XXXXXX)
cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT
cd "$TMP"
git init -q
printf "\n$ %s init\n" "$A"
"$A" init | sed -n "1,6p"
printf "\n$ %s mission create ...\n" "$A"
"$A" mission create "Readable mission validation" --body "Readable mission intent" --constraint "Keep mission records reviewable" --risk "Regression could reintroduce escaped data" --validation "Run focused validation transcript" | sed -n "1,24p"
MID=$(basename .atelier/missions/*.md .md)
printf "\n$ %s mission update %s ...\n" "$A" "$MID"
"$A" mission update "$MID" --body "Updated readable mission intent" --constraint "Keep mission records reviewable after update" --risk "Projection drift" --validation "Run rebuild export lint doctor" | sed -n "1,24p"
printf "\n$ sed -n 1,56p .atelier/missions/%s.md\n" "$MID"
sed -n "1,56p" ".atelier/missions/$MID.md"
printf "\n$ rg ^data: .atelier/missions -g *.md\n"
if rg "^data:" .atelier/missions -g "*.md"; then
  echo "unexpected stale mission data JSON"
  exit 1
else
  echo "No mission data front matter found."
fi
'
Exit status: 0

Stdout summary:

$ /root/atelier/target/debug/atelier init
Created /tmp/atelier-6aor-create.7Utm62/.atelier
Created /tmp/atelier-6aor-create.7Utm62/.atelier/config.toml
Created /tmp/atelier-6aor-create.7Utm62/.atelier/state.db
Atelier initialized successfully!

Next steps:

$ /root/atelier/target/debug/atelier mission create ...
Mission atelier-o1z9: Readable mission validation
Status: ready

## Intent

Readable mission intent

## Constraints

- Keep mission records reviewable

## Risks

- Regression could reintroduce escaped data

## Validation

- Run focused validation transcript

$ /root/atelier/target/debug/atelier mission update atelier-o1z9 ...
Mission atelier-o1z9: Readable mission validation
Status: ready

## Intent

Updated readable mission intent

## Constraints

- Keep mission records reviewable after update

## Risks

- Projection drift

## Validation

- Run rebuild export lint doctor

$ sed -n 1,56p .atelier/missions/atelier-o1z9.md
---
created_at: "2026-06-12T22:26:35.467794481+00:00"
id: "atelier-o1z9"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Readable mission validation"
updated_at: "2026-06-12T22:26:35.544646100+00:00"
---

## Intent

Updated readable mission intent

## Constraints

- Keep mission records reviewable after update

## Risks

- Projection drift

## Validation

- Run rebuild export lint doctor

$ rg ^data: .atelier/missions -g *.md
No mission data front matter found.

Stderr summary:
Refreshed projection in /tmp/atelier-6aor-create.7Utm62/.atelier/state.db from /tmp/atelier-6aor-create.7Utm62/.atelier
Refreshed projection in /tmp/atelier-6aor-create.7Utm62/.atelier/state.db from /tmp/atelier-6aor-create.7Utm62/.atelier
