#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
docs_index="$repo_root/docs/index.md"
quality_index="$repo_root/docs/architecture/quality/index.md"
command_surface_source="$repo_root/crates/atelier-app/src/command_surface.rs"
command_audit_index="$repo_root/docs/product/command-audit/index.md"
category_review="$repo_root/docs/product/command-audit/category-review.md"

rg -Fq 'docs/architecture/quality/index.md' "$docs_index"

mapfile -t quality_docs < <(
  rg -o '[A-Za-z0-9][A-Za-z0-9._-]*\.md' "$quality_index" | sort -u
)

((${#quality_docs[@]} > 0)) || {
  printf 'quality index yielded no Markdown documents: %s\n' "$quality_index" >&2
  exit 1
}

index_section_links() {
  local start=$1
  local end=$2
  sed -n "/^## $start/,/^## $end/p" "$command_audit_index" |
    rg -o '^[-] \[[^]]+\]' | sed -E 's/^- \[([^]]+)\]/\1/'
}

mapfile -t visible_roots < <(
  index_section_links 'Current Help-Visible Root Command Files' 'Hidden Advanced Or Migration Command Notes'
)

mapfile -t hidden_index_roots < <(
  index_section_links 'Hidden Advanced Or Migration Command Notes' 'Cross-Cutting Audit Artifacts'
)

mapfile -t retired_index_roots < <(
  index_section_links 'Retired Or Deferred Notes' 'Summary'
)

mapfile -t verdict_removed_roots < <(
  {
    rg --no-filename '^# (Removed|Retired|Deferred) `atelier [a-z0-9-]+`' \
      "$repo_root"/docs/product/command-audit/*.md |
      sed -E 's/^# (Removed|Retired|Deferred) `atelier ([a-z0-9-]+)`.*/\2/'
    awk -F'|' '
      tolower($3) ~ /(remove|retire)/ {
        surface = $2
        if (match(surface, /`[^`]+`/)) {
          surface = substr(surface, RSTART + 1, RLENGTH - 2)
          split(surface, words, /[[:space:]\/]+/)
          print words[1]
        }
      }
    ' "$category_review"
  } | sort -u
)

mapfile -t removed_roots < <(
  {
    sed -n '/^const REMOVED_ROOTS:/,/^];/p' "$command_surface_source" |
      rg -o '"[^"]+"' | tr -d '"'
    printf '%s\n' "${retired_index_roots[@]}" "${verdict_removed_roots[@]}"
    # Additional root aliases are recorded in cli-surface.md's Removed Behavior
    # section rather than represented as Rust root enum variants.
    printf '%s\n' \
      prime dep claim import create show list ready close update block unblock \
      relate related tree cascade falsify finish current-work stop worker orchestrator
  } | sort -u
)

array_contains() {
  local needle=$1
  shift
  local value
  for value in "$@"; do
    [[ "$value" == "$needle" ]] && return 0
  done
  return 1
}

mapfile -t restricted_roots < <(
  for root in "${hidden_index_roots[@]}"; do
    array_contains "$root" "${verdict_removed_roots[@]}" || printf '%s\n' "$root"
  done | sort -u
)

mapfile -t removed_paths < <(
  sed -n '/^const REMOVED_COMMAND_PATHS:/,/^];/p' "$command_surface_source" |
    awk -F'"' '/^[[:space:]]*&\["/ {
      path = $2
      for (field = 4; field <= NF; field += 2) {
        if ($field != "") path = path " " $field
      }
      print path
    }'
)

join_alternatives() {
  local joined
  joined=$(printf '%s\n' "$@" | sort -u | paste -sd'|' -)
  printf '%s' "$joined"
}

root_alternatives=$(join_alternatives "${removed_roots[@]}")
path_alternatives=$(join_alternatives "${removed_paths[@]}")
restricted_root_alternatives=$(join_alternatives "${restricted_roots[@]}")
atelier_prefix='(target/debug/)?atelier '
command_boundary='([[:space:]`]|$)'

# Removed roots and paths come from the same source used by the product's
# command-surface drift check plus the durable retired/deferred audit index.
# The remaining forms are subcommand/option cuts recorded by the issue, review,
# history, work, maintenance, and migration audits.
prohibited_command_pattern="$atelier_prefix((${root_alternatives})${command_boundary}|(${path_alternatives})${command_boundary}|issue (close|claim|new|quick|subissue|search|relate|tree|tested)${command_boundary}|issue update[^\x60]*--claim|issue list[^\x60]*(--ready|--blocked)|work (start|status|queue)${command_boundary}|maintenance delete${command_boundary}|export[^\x60]*--format|review (link|status|comments|comment|approve|request-changes)${command_boundary}|review open[^\x60]*--(title|body|source-branch|target-branch)|history[^\x60]*--(mission|epic|include-descendants|event-kind|actor|since)${command_boundary})"

# These commands can remain callable as hidden/admin implementation surfaces,
# but active quality guidance may name them only with an explicit setup,
# recovery, migration, historical, or diagnostic boundary.
restricted_command_pattern="$atelier_prefix((${restricted_root_alternatives})${command_boundary})"
restricted_context_pattern='(^|[^[:alnum:]_-])(hidden|admin|setup|recovery|repair|migration|diagnostic|debug|historical|non-normative)([^[:alnum:]_-]|$)|implementation probe|not (a |the )?(normal|routine|workflow)'

active_content() {
  awk '
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
  '
}

scan_content() {
  local content
  local hit
  content=$(cat)

  while IFS= read -r hit; do
    [[ -n "$hit" ]] || continue
    if printf '%s\n' "$hit" | rg -q -i "$prohibited_command_pattern"; then
      printf '%s\n' "$hit"
      continue
    fi
    if printf '%s\n' "$hit" | rg -q -i "$restricted_command_pattern" &&
      ! printf '%s\n' "$hit" | rg -q -i "$restricted_context_pattern"; then
      printf '%s\n' "$hit"
    fi
  done < <(
    printf '%s\n' "$content" |
      rg --no-line-number -i "$prohibited_command_pattern|$restricted_command_pattern" || true
  )
}

missing_index_entries() {
  local relative
  for relative in "$@"; do
    [[ -f "$repo_root/docs/architecture/quality/$relative" ]] ||
      printf 'quality index references missing file: %s\n' "$relative"
  done
}

run_inventory() {
  local failures=0
  local root
  local -a help_roots=()
  local -a audit_token_roots=()

  mapfile -t help_roots < <(
    target/debug/atelier --help |
      awk '
        /^Common commands:/ { exit }
        /^[A-Za-z][^:]*:$/ { in_section = 1; next }
        in_section && /^  [a-z][a-z0-9-]+[[:space:]]/ { print $1 }
      ' | sort -u
  )

  for root in "${visible_roots[@]}"; do
    if ! array_contains "$root" "${help_roots[@]}"; then
      printf 'inventory: audit-visible root missing from current help: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done
  for root in "${help_roots[@]}"; do
    if ! array_contains "$root" "${visible_roots[@]}"; then
      printf 'inventory: current help root missing from visible audit category: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done

  for root in "${hidden_index_roots[@]}"; do
    if array_contains "$root" "${verdict_removed_roots[@]}"; then
      array_contains "$root" "${removed_roots[@]}" || {
        printf 'inventory: Removed hidden-category root is not prohibited: %s\n' "$root" >&2
        failures=$((failures + 1))
      }
    elif ! array_contains "$root" "${restricted_roots[@]}"; then
      printf 'inventory: hidden-category root lacks context restriction: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done

  for root in "${retired_index_roots[@]}" "${verdict_removed_roots[@]}"; do
    if ! array_contains "$root" "${removed_roots[@]}"; then
      printf 'inventory: authoritative Removed/Retired root is not prohibited: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done

  mapfile -t audit_token_roots < <(
    rg -o --no-filename '`(target/debug/)?atelier [a-z0-9-]+' \
      "$repo_root"/docs/product/command-audit/*.md |
      sed -E 's/^`(target\/debug\/)?atelier //' | rg -v '^-' | sort -u
  )
  for root in "${audit_token_roots[@]}"; do
    if ! array_contains "$root" "${visible_roots[@]}" &&
      ! array_contains "$root" "${hidden_index_roots[@]}" &&
      ! array_contains "$root" "${removed_roots[@]}" &&
      [[ "$root" != help ]]; then
      printf 'inventory: command-audit token root is outside categorized inventory: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done

  ((failures == 0)) || exit 1
  printf 'command inventory passed: %d visible, %d hidden, %d removed, %d audit-token root(s)\n' \
    "${#visible_roots[@]}" "${#hidden_index_roots[@]}" "${#removed_roots[@]}" "${#audit_token_roots[@]}"
}

run_self_test() {
  local failures=0
  local checked=0
  local root
  local example
  local output
  local -a prohibited_examples=()
  local -a restricted_examples=()

  for root in "${restricted_roots[@]}"; do
    restricted_examples+=("atelier $root")
  done
  restricted_examples+=(
    'atelier doctor --fix'
    'atelier export --check'
    'atelier import-beads backup.jsonl'
    'atelier workflow check'
    'atelier diagnostics slow'
    'atelier branch merge atelier-demo'
    'atelier forgejo status'
  )

  for example in "${removed_roots[@]}"; do
    prohibited_examples+=("atelier $example")
  done
  for example in "${removed_paths[@]}"; do
    prohibited_examples+=("atelier $example")
  done
  prohibited_examples+=(
    'atelier maintenance'
    'atelier issue close atelier-demo --reason done'
    'atelier issue claim atelier-demo'
    'atelier issue new demo'
    'atelier issue quick demo'
    'atelier issue subissue demo'
    'atelier issue search demo'
    'atelier issue relate atelier-demo atelier-other'
    'atelier issue tree atelier-demo'
    'atelier issue tested atelier-demo'
    'atelier issue update atelier-demo --claim'
    'atelier issue list --ready'
    'atelier issue list --blocked'
    'atelier work start atelier-demo'
    'atelier work status'
    'atelier work queue'
    'atelier maintenance delete atelier-demo'
    'atelier export --format json'
    'atelier review link atelier-demo'
    'atelier review status atelier-demo'
    'atelier review comments atelier-demo'
    'atelier review comment atelier-demo text'
    'atelier review approve atelier-demo'
    'atelier review request-changes atelier-demo'
    'atelier review open --title manual'
    'atelier review open --body manual'
    'atelier review open --source-branch manual'
    'atelier review open --target-branch manual'
    'atelier history --mission atelier-demo'
    'atelier history --epic atelier-demo'
    'atelier history --include-descendants'
    'atelier history --event-kind note'
    'atelier history --actor worker'
    'atelier history --since 2026-01-01'
  )

  for root in "${retired_index_roots[@]}" "${verdict_removed_roots[@]}"; do
    if ! array_contains "$root" "${removed_roots[@]}"; then
      printf 'self-test missing authoritative removed root classification: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
    if ! array_contains "atelier $root" "${prohibited_examples[@]}"; then
      printf 'self-test missing generated prohibited fixture: atelier %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done

  if ! array_contains maintenance "${verdict_removed_roots[@]}"; then
    printf 'self-test did not derive maintenance from a Removed verdict\n' >&2
    failures=$((failures + 1))
  fi

  for example in "${prohibited_examples[@]}" "${restricted_examples[@]}"; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test failed to reject: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  output=$(
    {
      printf '# Historical Transcript (Non-Normative)\n'
      printf '%s\n' "${prohibited_examples[@]}" "${restricted_examples[@]}"
    } | active_content | scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected explicitly historical commands:\n%s\n' "$output" >&2
    failures=$((failures + 1))
  fi

  for example in "${restricted_examples[@]}"; do
    output=$(
      printf '# Setup And Recovery\nHidden admin diagnostic or migration recovery only: %s\n' "$example" |
        active_content | scan_content
    )
    if [[ -n "$output" ]]; then
      printf 'self-test rejected context-bounded hidden/admin command: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  output=$(missing_index_entries '__guard-self-test-missing__.md')
  if [[ -z "$output" ]]; then
    printf 'self-test did not detect a missing indexed document\n' >&2
    failures=$((failures + 1))
  fi

  ((failures == 0)) || exit 1
  printf 'active quality guidance self-test passed: %d prohibited/context-restricted example(s)\n' "$checked"
}

if [[ ${1:-} == '--self-test' ]]; then
  run_self_test
  exit 0
fi

if [[ ${1:-} == '--inventory' ]]; then
  run_inventory
  exit 0
fi

if (($# > 0)); then
  printf 'usage: %s [--self-test|--inventory]\n' "$0" >&2
  exit 2
fi

findings=()
checked=0

while IFS= read -r missing; do
  [[ -n "$missing" ]] && findings+=("$missing")
done < <(missing_index_entries "${quality_docs[@]}")

for relative in "${quality_docs[@]}"; do
  path="$repo_root/docs/architecture/quality/$relative"
  [[ -f "$path" ]] || continue
  checked=$((checked + 1))

  while IFS= read -r hit; do
    [[ -n "$hit" ]] || continue
    findings+=("$relative:$hit")
  done < <(active_content < "$path" | scan_content)
done

if ((${#findings[@]} > 0)); then
  printf 'active quality guidance contains unclassified retired command references:\n' >&2
  printf '  - %s\n' "${findings[@]}" >&2
  exit 1
fi

printf 'active quality guidance check passed: %d indexed document(s)\n' "$checked"
