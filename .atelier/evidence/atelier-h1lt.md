---
created_at: "2026-06-12T22:27:11.173440101+00:00"
id: "atelier-h1lt"
evidence_type: "validation"
captured_at: "2026-06-12T22:27:10.747639656+00:00"
command: "bash -lc '\nset -euo pipefail\nA=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d /tmp/atelier-6aor-negative.XXXXXX)\ncleanup() { rm -rf \"$TMP\"; }\ntrap cleanup EXIT\ncd \"$TMP\"\ngit init -q\n\"$A\" init >/dev/null\n\"$A\" mission create \"Readable mission validation\" --body \"Readable mission intent\" --constraint \"Keep mission records reviewable\" --risk \"Projection drift\" --validation \"Run rebuild export lint doctor\" >/dev/null\nMID=$(basename .atelier/missions/*.md .md)\nprintf \"\\n$ rg ^data: .atelier/missions -g *.md\\n\"\nif rg \"^data:\" .atelier/missions -g \"*.md\"; then\n  echo \"unexpected stale mission data JSON\"\n  exit 1\nelse\n  echo \"No mission data front matter found after create.\"\nfi\nprintf \"\\n$ remove required Validation section from .atelier/missions/%s.md\\n\" \"$MID\"\nperl -0pi -e \"s/\\n## Validation\\n\\n- Run rebuild export lint doctor\\n?//s\" \".atelier/missions/$MID.md\"\nprintf \"\\n$ %s rebuild (expect failure)\\n\" \"$A\"\nif \"$A\" rebuild >invalid-rebuild.out 2>invalid-rebuild.err; then\n  cat invalid-rebuild.out\n  cat invalid-rebuild.err >&2\n  echo \"unexpected rebuild success for invalid mission record\"\n  exit 1\nelse\n  cat invalid-rebuild.err >&2\nfi\nprintf \"\\n$ %s export --check (expect failure)\\n\" \"$A\"\nif \"$A\" export --check >invalid-export.out 2>invalid-export.err; then\n  cat invalid-export.out\n  cat invalid-export.err >&2\n  echo \"unexpected export --check success for invalid mission record\"\n  exit 1\nelse\n  cat invalid-export.err >&2\nfi\nprintf \"\\n$ %s lint (observed residual: projection lint passes before rebuild)\\n\" \"$A\"\n\"$A\" lint\n'"
exit_status: "0"
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
status: "recorded"
title: "Negative invalid mission record and stale data emission transcript"
updated_at: "2026-06-12T22:27:12.516307088+00:00"
---

Negative invalid mission record and stale data emission transcript

Command: bash -lc '
set -euo pipefail
A=/root/atelier/target/debug/atelier
TMP=$(mktemp -d /tmp/atelier-6aor-negative.XXXXXX)
cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT
cd "$TMP"
git init -q
"$A" init >/dev/null
"$A" mission create "Readable mission validation" --body "Readable mission intent" --constraint "Keep mission records reviewable" --risk "Projection drift" --validation "Run rebuild export lint doctor" >/dev/null
MID=$(basename .atelier/missions/*.md .md)
printf "\n$ rg ^data: .atelier/missions -g *.md\n"
if rg "^data:" .atelier/missions -g "*.md"; then
  echo "unexpected stale mission data JSON"
  exit 1
else
  echo "No mission data front matter found after create."
fi
printf "\n$ remove required Validation section from .atelier/missions/%s.md\n" "$MID"
perl -0pi -e "s/\n## Validation\n\n- Run rebuild export lint doctor\n?//s" ".atelier/missions/$MID.md"
printf "\n$ %s rebuild (expect failure)\n" "$A"
if "$A" rebuild >invalid-rebuild.out 2>invalid-rebuild.err; then
  cat invalid-rebuild.out
  cat invalid-rebuild.err >&2
  echo "unexpected rebuild success for invalid mission record"
  exit 1
else
  cat invalid-rebuild.err >&2
fi
printf "\n$ %s export --check (expect failure)\n" "$A"
if "$A" export --check >invalid-export.out 2>invalid-export.err; then
  cat invalid-export.out
  cat invalid-export.err >&2
  echo "unexpected export --check success for invalid mission record"
  exit 1
else
  cat invalid-export.err >&2
fi
printf "\n$ %s lint (observed residual: projection lint passes before rebuild)\n" "$A"
"$A" lint
'
Exit status: 0

Stdout summary:

$ rg ^data: .atelier/missions -g *.md
No mission data front matter found after create.

$ remove required Validation section from .atelier/missions/atelier-81k4.md

$ /root/atelier/target/debug/atelier rebuild (expect failure)

$ /root/atelier/target/debug/atelier export --check (expect failure)

$ /root/atelier/target/debug/atelier lint (observed residual: projection lint passes before rebuild)
Lint passed.

Stderr summary:
Refreshed projection in /tmp/atelier-6aor-negative.ssTDFx/.atelier/state.db from /tmp/atelier-6aor-negative.ssTDFx/.atelier
Error: Missing required mission body section 'Validation' in .atelier/missions/atelier-81k4.md
Error: Canonical export is stale:
invalid: canonical tracker Markdown is invalid; run `atelier lint` for details: Missing required mission body section 'Validation' in .atelier/missions/atelier-81k4.md
