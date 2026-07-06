#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
docs_index="$repo_root/docs/index.md"
quality_index="$repo_root/docs/architecture/quality/index.md"

rg -Fq 'docs/architecture/quality/index.md' "$docs_index"

mapfile -t quality_docs < <(
  rg -o '`[^`]+\.md`' "$quality_index" | tr -d '`' | sort -u
)

retired_command_pattern='(target/debug/)?atelier ((lint|doctor|export|rebuild|start|prime|issue close|dep (add|remove)|search|session)([[:space:]`]|$)|issue list[^`]*(--ready|--blocked))'
findings=()
checked=0

for relative in "${quality_docs[@]}"; do
  path="$repo_root/docs/architecture/quality/$relative"
  [[ -f "$path" ]] || {
    findings+=("quality index references missing file: $relative")
    continue
  }
  checked=$((checked + 1))

  # Historical/removed-command sections remain useful evidence, but their
  # command transcripts are not live workflow. The two retrospective headings
  # cover mined failure counts and the removed-command attempts they describe.
  active_content=$(awk '
    function heading_level(line, marks) {
      marks = line
      sub(/[^#].*$/, "", marks)
      return length(marks)
    }
    function excluded_heading(line, lower) {
      lower = tolower(line)
      return lower ~ /(historical|removed|retired).*(non-normative|evidence|transcript|commands|behavior|surface|inventory|classification)/ ||
             lower ~ /quantitative snapshot/ ||
             lower ~ /agents guessed command surfaces too often/
    }
    /^#{1,6} / {
      level = heading_level($0)
      if (excluded && level <= excluded_level) {
        excluded = 0
      }
      if (excluded_heading($0)) {
        excluded = 1
        excluded_level = level
      }
    }
    !excluded { print FNR ":" $0 }
  ' "$path")

  while IFS= read -r hit; do
    [[ -n "$hit" ]] || continue
    if [[ "$hit" == *"only for storage-rendering, migration, or debug claims"* ]]; then
      continue
    fi
    findings+=("$relative:$hit")
  done < <(printf '%s\n' "$active_content" | rg --no-line-number -i "$retired_command_pattern" || true)
done

if ((${#findings[@]} > 0)); then
  printf 'active quality guidance contains unclassified retired command references:\n' >&2
  printf '  - %s\n' "${findings[@]}" >&2
  exit 1
fi

printf 'active quality guidance check passed: %d indexed document(s)\n' "$checked"
