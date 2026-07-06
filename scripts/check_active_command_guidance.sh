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

guidance_path_is_in_scope() {
  local path=$1
  local relative=${path#"$repo_root/"}

  case "$relative" in
    docs/index.md | docs/product/*.md | docs/architecture/*.md | docs/spec/*.md | PRODUCT_INTENT.md | CONTEXT.md)
      return 0
      ;;
    docs/adr/*.md)
      [[ -f "$path" ]] || return 0
      head -n 14 "$path" | rg -q -i '^(Accepted([.:]|$)|Status:[[:space:]]*Accepted)'
      return
      ;;
  esac
  return 1
}

declare -a active_guidance_docs=()
declare -a missing_guidance_docs=()

collect_indexed_guidance() {
  local cursor=0
  local path
  local relative
  local link
  local candidate
  local -a queue=("$docs_index" "$quality_index")
  declare -A seen=()

  for relative in "${quality_docs[@]}"; do
    queue+=("$repo_root/docs/architecture/quality/$relative")
  done

  while ((cursor < ${#queue[@]})); do
    path=$(realpath -m "${queue[$cursor]}")
    cursor=$((cursor + 1))
    [[ -z ${seen[$path]+x} ]] || continue
    seen[$path]=1
    guidance_path_is_in_scope "$path" || continue

    relative=${path#"$repo_root/"}
    if [[ ! -f "$path" ]]; then
      missing_guidance_docs+=("$relative")
      continue
    fi
    active_guidance_docs+=("$path")

    while IFS= read -r link; do
      [[ -n "$link" ]] || continue
      link=${link#<}
      link=${link%>}
      link=${link%%#*}
      [[ "$link" != *://* ]] || continue
      candidate=$(realpath -m "$(dirname "$path")/$link")
      guidance_path_is_in_scope "$candidate" || continue
      queue+=("$candidate")
    done < <(
      rg -o '\]\([^)]*\.md(#[^)]*)?\)' "$path" |
        sed -E 's/^\]\(//; s/\)$//' || true
    )
  done
}

collect_indexed_guidance

((${#active_guidance_docs[@]} > 0)) || {
  printf 'documentation map yielded no active guidance: %s\n' "$docs_index" >&2
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
      relate related tree cascade falsify finish current-work stop worker orchestrator \
      pr migrate
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
command_boundary='([^[:alnum:]_-]|$)'

# Removed roots and paths come from the same source used by the product's
# command-surface drift check plus the durable retired/deferred audit index.
# The remaining forms are subcommand/option cuts recorded by the issue, review,
# history, work, maintenance, and migration audits.
prohibited_command_core="atelier ((${root_alternatives})${command_boundary}|(${path_alternatives})${command_boundary}|issue (close|claim|new|quick|subissue|search|relate|tree|tested)${command_boundary}|issue update[^\x60]*--claim|issue list[^\x60]*(--ready|--blocked)|work (start|status|queue)${command_boundary}|maintenance delete${command_boundary}|export[^\x60]*--format|review (link|status|comments|comment|approve|request-changes)${command_boundary}|review open[^\x60]*--(title|body|source-branch|target-branch)|history[^\x60]*--(mission|epic|include-descendants|event-kind|actor|since)${command_boundary})"

# These commands can remain callable as hidden/admin implementation surfaces,
# but indexed active guidance may name them only with an explicit setup,
# recovery, migration, historical, or diagnostic boundary.
restricted_command_core="atelier ((${restricted_root_alternatives})${command_boundary})"
restricted_context_pattern='(^|[^[:alnum:]_-])(hidden|advanced|admin|maintenance|setup|recovery|repair|migration|diagnostic|debug|historical|non-normative)([^[:alnum:]_-]|$)|implementation probe|not (part of )?(a |the )?(normal|routine|workflow)'
recommendation_pattern='(^|[^[:alnum:]_-])(run|use|execute|invoke|try|continue|now|current|currently|recommend|recommended)([^[:alnum:]_-]|$)'

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
             lower ~ /(rejected alternatives|alternatives considered)/ ||
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
      if (!excluded) {
        heading = tolower($0)
      }
    }
    !excluded { print FNR ":" heading " :: " $0 }
  '
}

document_is_non_normative() {
  case "$1" in
    */docs/product/command-audit/export-check-reference-classification.md)
      return 0
      ;;
  esac
  head -n 1 "$1" |
    rg -q '^# (Removed|Retired|Deferred) `atelier [a-z0-9-]+`'
}

document_is_restricted_command_audit() {
  local path=$1
  local root
  [[ "$path" == "$repo_root"/docs/product/command-audit/*.md ]] || return 1
  root=$(head -n 1 "$path" |
    sed -nE 's/^# `atelier ([a-z0-9-]+)`.*/\1/p')
  [[ -n "$root" ]] || return 1
  array_contains "$root" "${restricted_roots[@]}"
}

guidance_content() {
  local path=$1
  document_is_non_normative "$path" && return 0
  if document_is_restricted_command_audit "$path"; then
    active_content < "$path" |
      sed 's/^/hidden diagnostic, recovery, or migration command audit: /'
  else
    active_content < "$path"
  fi
}

prohibited_occurrence_is_negated() {
  local before=$1
  local after=$2
  local before_clause=${before##*[.;|]}
  local after_clause=${after%%[.;|]*}
  local lower_before
  local lower_after
  local trailing
  local surrounding

  lower_before=$(printf '%s' "$before_clause" | tr '[:upper:]' '[:lower:]')
  lower_after=$(printf '%s' "$after_clause" | tr '[:upper:]' '[:lower:]')

  # Direct negation stays local to this occurrence. A later contrasting
  # recommendation in the same clause cancels the allowance.
  if printf '%s\n' "$lower_before" |
    rg -q '(^|[^[:alnum:]_-])((do|does|did|must|should|can|could|would)[[:space:]]+not|never)([^[:alnum:]_-]|$)' &&
    ! printf '%s\n' "$lower_before" |
      rg -q '(but|then|instead)[^.;|]*(run|use|execute|invoke|try)([^[:alnum:]_-]|$)'; then
    return 0
  fi

  # Classification words before the command apply only when no imperative or
  # current-routing marker appears after the closest classification word.
  if [[ "$lower_before" =~ ^(.*[^[:alnum:]_-]|)(legacy|retired|removed|obsolete|superseded|gone|no|not)([^[:alnum:]_-])(.*)$ ]]; then
    trailing=${BASH_REMATCH[4]}
    if ! printf '%s\n' "$trailing" | rg -q "$recommendation_pattern"; then
      return 0
    fi
  fi

  # Negative grammar after the occurrence is also local. Recommendation words
  # elsewhere in that clause make the line live guidance instead.
  if [[ "$lower_after" =~ ^(.*)((does|do)[[:space:]]+not[[:space:]]+exist|(is|are|was|were)[[:space:]]+not[[:space:]]+(a[[:space:]]+)?(current|live|normal|supported)[[:space:]]+command|no[[:space:]]+longer[[:space:]]+exists)(.*)$ ]]; then
    surrounding="${BASH_REMATCH[1]}${BASH_REMATCH[7]}"
    if ! printf '%s\n' "$surrounding" | rg -q "$recommendation_pattern"; then
      return 0
    fi
  fi

  if [[ "$lower_after" =~ ^(.*)(removed|retired|obsolete|superseded|gone)(.*)$ ]]; then
    surrounding="${BASH_REMATCH[1]}${BASH_REMATCH[3]}"
    if ! printf '%s\n' "$surrounding" | rg -q "$recommendation_pattern"; then
      return 0
    fi
  fi

  if printf '%s\n' "$lower_after" |
    rg -q '^[^.;|]*([^[:alnum:]_-]|^)(must|should|do|does)[[:space:]]+not([^[:alnum:]_-]|$)' &&
    ! printf '%s\n' "$lower_after" |
      rg -q '(but|then|instead)[^.;|]*(run|use|execute|invoke|try)([^[:alnum:]_-]|$)'; then
    return 0
  fi

  return 1
}

scan_content() {
  local content
  local hit
  local text
  local remaining
  local before
  local full_before
  local candidate
  local matched
  local after
  local previous
  local consumed
  local finding
  content=$(cat)

  while IFS= read -r hit; do
    [[ -n "$hit" ]] || continue
    [[ "$hit" == *'atelier '* ]] || continue

    if [[ "$hit" == *' :: '* ]]; then
      text=${hit#* :: }
    else
      text=$hit
    fi

    remaining=$text
    consumed=''
    finding=0
    while [[ "$remaining" == *'atelier '* ]]; do
      before=${remaining%%atelier *}
      candidate="atelier ${remaining#*atelier }"
      full_before="$consumed$before"
      previous=''
      [[ -z "$full_before" ]] || previous=${full_before: -1}

      if [[ -z "$previous" || ! "$previous" =~ [[:alnum:]_-] ]]; then
        if matched=$(printf '%s\n' "$candidate" |
          rg -o -m 1 "^$prohibited_command_core"); then
          after=${candidate:${#matched}}
          if ! prohibited_occurrence_is_negated "$full_before" "$after"; then
            finding=1
            break
          fi
        elif printf '%s\n' "$candidate" | rg -q "^$restricted_command_core" &&
          ! printf '%s\n' "$hit" | rg -q -i "$restricted_context_pattern"; then
          finding=1
          break
        fi
      fi

      consumed+="$before"'atelier '
      remaining=${remaining#*atelier }
    done

    if ((finding)); then
      printf '%s\n' "$hit"
    fi
  done <<< "$content"
}

missing_quality_index_entries() {
  local relative
  for relative in "$@"; do
    [[ -f "$repo_root/docs/architecture/quality/$relative" ]] ||
      printf 'quality index references missing file: %s\n' "$relative"
  done
}

missing_guidance_entries() {
  local path
  for path in "$@"; do
    if guidance_path_is_in_scope "$path" && [[ ! -f "$path" ]]; then
      printf 'documentation map references missing active guidance: %s\n' \
        "${path#"$repo_root/"}"
    fi
  done
}

indexed_guidance_findings() {
  local missing
  local path
  local relative
  local hit

  for missing in "${missing_guidance_docs[@]}"; do
    printf 'missing-indexed-document:%s\n' "$missing"
  done

  for path in "${active_guidance_docs[@]}"; do
    relative=${path#"$repo_root/"}
    while IFS= read -r hit; do
      [[ -n "$hit" ]] || continue
      printf '%s:%s\n' "$relative" "$hit"
    done < <(guidance_content "$path" | scan_content)
  done
}

run_inventory() {
  local failures=0
  local root
  local path
  local finding
  local -a help_roots=()
  local -a audit_token_roots=()
  local -a guidance_token_roots=()
  local -a guidance_findings=()

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

  mapfile -t guidance_token_roots < <(
    for path in "${active_guidance_docs[@]}"; do
      guidance_content "$path" |
        rg -o --no-filename '`(target/debug/)?atelier [a-z0-9-]+' || true
    done |
      sed -E 's/^`(target\/debug\/)?atelier //' | rg -v '^-' | sort -u
  )
  for root in "${guidance_token_roots[@]}"; do
    if ! array_contains "$root" "${visible_roots[@]}" &&
      ! array_contains "$root" "${hidden_index_roots[@]}" &&
      ! array_contains "$root" "${removed_roots[@]}" &&
      [[ "$root" != help ]]; then
      printf 'inventory: indexed active-guidance root is uncategorized: %s\n' "$root" >&2
      failures=$((failures + 1))
    fi
  done

  mapfile -t guidance_findings < <(indexed_guidance_findings)
  for finding in "${guidance_findings[@]}"; do
    printf 'inventory: indexed active guidance contains prohibited routing: %s\n' \
      "$finding" >&2
    failures=$((failures + 1))
  done

  ((failures == 0)) || exit 1
  printf 'command inventory passed: %d visible, %d hidden, %d removed, %d audit-token root(s), %d indexed-guidance token root(s)\n' \
    "${#visible_roots[@]}" "${#hidden_index_roots[@]}" "${#removed_roots[@]}" \
    "${#audit_token_roots[@]}" "${#guidance_token_roots[@]}"
}

run_self_test() {
  local failures=0
  local checked=0
  local root
  local example
  local output
  local relative
  local -a prohibited_examples=()
  local -a restricted_examples=()
  local -a required_prohibited_examples=(
    'atelier work queue'
    'atelier start'
    'atelier issue close atelier-demo --reason done'
    'atelier worktree'
    'atelier lint'
  )
  local -a required_restricted_examples=(
    'atelier doctor --fix'
  )
  local -a adversarial_prohibited_examples=(
    '- atelier start'
    '* atelier start'
    '1. atelier start'
    '> atelier start'
    'atelier start, then continue'
    'atelier start.'
    'atelier start: then continue'
    '`atelier start`; then continue'
    '"atelier start" is the current route.'
    'target/debug/atelier start, then continue'
    'Run atelier start now.'
    'The legacy API is gone; run `atelier start` now.'
    'Use (`atelier start`) now.'
    'Current route: atelier start'
    'The legacy API is gone; run atelier start now.'
    'The legacy API is gone but run atelier start now.'
    'The retired command differed; use atelier issue close demo for current work.'
    'The command was retired; now use atelier issue close demo.'
    'Use atelier issue close demo, then continue.'
    'Do not run atelier start; use atelier issue close demo now.'
    'atelier start is retired; use atelier issue close demo now.'
    'The legacy atelier start differed, but use atelier issue close demo now.'
    '> Use atelier work queue now.'
  )
  local -a local_negative_examples=(
    'The command `atelier start` is removed.'
    'Do not run `atelier start`.'
    'Use `atelier issue transition`, not `atelier start`.'
    '`atelier issue close demo` is not a current command.'
    'There is no `atelier worktree` command.'
    'The legacy `atelier work queue` command is gone.'
  )
  local -a required_guidance_docs=(
    'docs/product/work-view-ordering.md'
    'docs/product/command-audit/category-review.md'
    'docs/architecture/markdown-first-record-store.md'
    'docs/spec/storage/export/rebuild/canonical-layout.md'
    'docs/adr/0004-work-lock-sync-policy.md'
    'docs/adr/0007-mission-workspaces-and-epic-review-branches.md'
  )

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
    'atelier lint'
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
  prohibited_examples+=("${adversarial_prohibited_examples[@]}")

  for example in "${required_prohibited_examples[@]}"; do
    if ! array_contains "$example" "${prohibited_examples[@]}"; then
      printf 'self-test missing required active-context fixture: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done
  for example in "${required_restricted_examples[@]}"; do
    if ! array_contains "$example" "${restricted_examples[@]}"; then
      printf 'self-test missing required restricted-context fixture: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

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

  for example in "${local_negative_examples[@]}"; do
    output=$(printf '# Live Negative Classification\n%s\n' "$example" |
      active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test rejected command-local negative classification: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Meta-test the production section filter and occurrence scanner together so
  # a simplified fixture regex cannot diverge from the live pipeline.
  output=$(
    printf '%s\n' \
      '## Historical Commands (Non-Normative)' \
      'atelier start' \
      '## Live Guidance' \
      '- atelier start' |
      active_content | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test production pipeline missed live re-entry after historical section\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '# Current Contract' \
      '## Rejected Alternatives' \
      'atelier issue close demo' \
      '## Current Routing' \
      '> atelier issue close demo' |
      active_content | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test production pipeline missed live re-entry after rejected section\n' >&2
    failures=$((failures + 1))
  fi

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

  output=$(missing_quality_index_entries '__guard-self-test-missing__.md')
  if [[ -z "$output" ]]; then
    printf 'self-test did not detect a missing quality-index document\n' >&2
    failures=$((failures + 1))
  fi

  output=$(missing_guidance_entries \
    "$repo_root/docs/product/__guard-self-test-missing__.md")
  if [[ -z "$output" ]]; then
    printf 'self-test did not detect a missing repository-indexed document\n' >&2
    failures=$((failures + 1))
  fi

  for relative in "${required_guidance_docs[@]}"; do
    if ! array_contains "$repo_root/$relative" "${active_guidance_docs[@]}"; then
      printf 'self-test required guidance is not reachable from docs/index.md: %s\n' \
        "$relative" >&2
      failures=$((failures + 1))
    fi
  done
  for relative in "${quality_docs[@]}"; do
    if ! array_contains "$repo_root/docs/architecture/quality/$relative" \
      "${active_guidance_docs[@]}"; then
      printf 'self-test quality guidance lost full-index coverage: %s\n' \
        "$relative" >&2
      failures=$((failures + 1))
    fi
  done

  ((failures == 0)) || exit 1
  printf 'active command guidance self-test passed: %d prohibited/context-restricted example(s), including %d adversarial occurrence fixture(s) and all prior quality cases\n' \
    "$checked" "${#adversarial_prohibited_examples[@]}"
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
checked=${#active_guidance_docs[@]}
quality_checked=0

mapfile -t findings < <(indexed_guidance_findings)

for relative in "${quality_docs[@]}"; do
  path="$repo_root/docs/architecture/quality/$relative"
  if [[ -f "$path" ]] && array_contains "$path" "${active_guidance_docs[@]}"; then
    quality_checked=$((quality_checked + 1))
  else
    findings+=("quality guidance is not reachable from docs/index.md: $relative")
  fi
done

if ((${#findings[@]} > 0)); then
  printf 'indexed active guidance contains unclassified retired command references:\n' >&2
  printf '  - %s\n' "${findings[@]}" >&2
  exit 1
fi

printf 'active command guidance check passed: %d repository-indexed document(s), including %d quality document(s)\n' \
  "$checked" "$quality_checked"
