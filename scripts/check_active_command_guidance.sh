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
bare_prohibited_command_core="((${root_alternatives})${command_boundary}|(${path_alternatives})${command_boundary}|issue (close|claim|new|quick|subissue|search|relate|tree|tested)${command_boundary}|issue update.*--claim|issue list.*(--ready|--blocked)|work (start|status|queue)${command_boundary}|maintenance delete${command_boundary}|export.*--format|review (link|status|comments|comment|approve|request-changes)${command_boundary}|review open.*--(title|body|source-branch|target-branch)|history.*--(mission|epic|include-descendants|event-kind|actor|since)${command_boundary})"
# `work queue` is still callable and explicitly bounded by the c0mp contract;
# it is not a Removed/Retired root. Its named section-level boundary remains
# distinct from the strict no-mention rule for removed commands.
legacy_callable_command_core="atelier work queue${command_boundary}"
bare_legacy_callable_command_core="work queue${command_boundary}"
legacy_callable_document='docs/product/issue-inventory-and-mission-overview.md'
legacy_callable_heading='## legacy queue boundary'

# These commands can remain callable as hidden/admin implementation surfaces,
# but indexed active guidance may name them only with an explicit setup,
# recovery, migration, historical, or diagnostic boundary.
restricted_command_core="atelier ((${restricted_root_alternatives})${command_boundary})"
bare_restricted_command_core="((${restricted_root_alternatives})${command_boundary})"

active_content() {
  local source=${1:-}
  source=${source#"$repo_root/"}
  awk -v source="$source" '
    function heading_level(line, marks) {
      marks = line
      sub(/[^#].*$/, "", marks)
      return length(marks)
    }
    function excluded_heading(line, lower) {
      lower = tolower(line)
      return lower ~ /(historical|removed|retired).*(non-normative|evidence|transcript|commands?|behavior|surface|inventory|classification)/ ||
             lower ~ /(rejected alternatives|alternatives considered)/ ||
             lower ~ /quantitative snapshot/ ||
             lower ~ /agents guessed command surfaces too often/
    }
    {
      fence_marker = ($0 ~ /^[[:space:]]*(```|~~~)/)
      if (fence_marker) {
        if (in_fence) {
          in_fence = 0
          fence_kind = 0
        } else {
          marker = tolower($0)
          if (marker ~ /^[[:space:]]*(```|~~~)[[:space:]]*(sh|bash|shell|console|terminal|zsh)[[:space:]]*$/) {
            fence_kind = 1
          } else if (marker ~ /^[[:space:]]*(```|~~~)[[:space:]]*$/) {
            fence_kind = 2
          } else if (marker ~ /^[[:space:]]*(```|~~~)[[:space:]]*[^[:space:]]+[[:space:]]*$/) {
            fence_kind = 3
          } else {
            fence_kind = 0
          }
          in_fence = 1
        }
        fenced_content = 0
      } else {
        fenced_content = in_fence ? fence_kind : 0
      }
    }
    !in_fence && /^#{1,6} / {
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
    !excluded {
      print source "|" FNR "|" fenced_content "|" heading " :: " $0
    }
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

guidance_content() {
  local path=$1
  document_is_non_normative "$path" && return 0
  active_content "$path" < "$path"
}

restricted_occurrence_has_explicit_context() {
  local source=$1
  local heading=$2

  case "$source" in
    docs/product/command-audit/branch.md | docs/product/command-audit/diagnostics.md | docs/product/command-audit/doctor.md | docs/product/command-audit/export.md | docs/product/command-audit/forgejo.md | docs/product/command-audit/import-beads.md | docs/product/command-audit/lint.md | docs/product/command-audit/rebuild.md | docs/product/command-audit/workflow.md)
      return 0
      ;;
  esac

  [[ "$heading" =~ ^#{1,6}[[:space:]]+(setup[[:space:]]+and[[:space:]]+recovery|recovery|admin[[:space:]]+setup|diagnostics|merge[[:space:]]+conflict[[:space:]]+and[[:space:]]+recovery[[:space:]]+guidance|hidden[[:space:]]+diagnostic:[[:space:]]+slow[[:space:]]+command[[:space:]]+query[[:space:]]+defaults|hidden[[:space:]]+rebuild[[:space:]]+diagnostic[[:space:]]+and[[:space:]]+freshness)$ ]]
}

prefixed_candidate_is_finding() {
  local candidate=$1
  local source=$2
  local heading=$3

  if printf '%s\n' "$candidate" | rg -q "^$legacy_callable_command_core" &&
    [[ "$source" == "$legacy_callable_document" ]] &&
    [[ "$heading" == "$legacy_callable_heading" ]]; then
    return 1
  fi
  if printf '%s\n' "$candidate" | rg -q "^$prohibited_command_core"; then
    return 0
  fi
  if printf '%s\n' "$candidate" | rg -q "^$restricted_command_core" &&
    ! restricted_occurrence_has_explicit_context "$source" "$heading"; then
    return 0
  fi
  return 1
}

bare_candidate_is_finding() {
  local candidate=$1
  local source=$2
  local heading=$3
  local first_token
  local command_token_pattern='^[a-z0-9-]+[,.;:!?)]?$'
  local legacy_pattern="^$bare_legacy_callable_command_core"
  local prohibited_pattern="^$bare_prohibited_command_core"
  local restricted_pattern="^$bare_restricted_command_core"

  candidate=${candidate#"${candidate%%[![:space:]]*}"}
  candidate=${candidate%"${candidate##*[![:space:]]}"}
  [[ -n "$candidate" ]] || return 1
  [[ "$candidate" != atelier\ * ]] || return 1
  [[ "$candidate" != target/debug/atelier\ * ]] || return 1
  first_token=${candidate%%[[:space:]]*}
  [[ "$first_token" =~ $command_token_pattern ]] || return 1

  if [[ "$candidate" =~ $legacy_pattern ]] &&
    [[ "$source" == "$legacy_callable_document" ]] &&
    [[ "$heading" == "$legacy_callable_heading" ]]; then
    return 1
  fi
  if [[ "$candidate" =~ $prohibited_pattern ]]; then
    return 0
  fi
  if [[ "$candidate" =~ $restricted_pattern ]] &&
    ! restricted_occurrence_has_explicit_context "$source" "$heading"; then
    return 0
  fi
  return 1
}

fence_line_is_data() {
  local candidate=$1
  local yaml_key_pattern='^[-]?[[:space:]]*[A-Za-z_][A-Za-z0-9_.-]*:[[:space:]]*([^[:space:]].*)?$'
  local quoted_yaml_key_pattern='^[-]?[[:space:]]*["'"'][^"'"']+["'"']:[[:space:]]*.*$'
  local yaml_scalar_list_pattern='^-[[:space:]]+([A-Za-z0-9_.-]+|"[^"]*"|'"'"'[^'"'"']*'"'"')$'
  local structured_literal_pattern='^[[{].*[]}][,]?$'
  local structured_close_pattern='^[]}][,]?$'

  candidate=${candidate#"${candidate%%[![:space:]]*}"}
  candidate=${candidate%"${candidate##*[![:space:]]}"}
  [[ -z "$candidate" ]] && return 0
  [[ "$candidate" == '---' || "$candidate" == '...' ]] && return 0
  [[ "$candidate" =~ $yaml_key_pattern ]] && return 0
  [[ "$candidate" =~ $quoted_yaml_key_pattern ]] && return 0
  [[ "$candidate" =~ $yaml_scalar_list_pattern ]] && return 0
  [[ "$candidate" =~ $structured_literal_pattern ]] && return 0
  [[ "$candidate" =~ $structured_close_pattern ]] && return 0
  return 1
}

record_graph_node_has_indented_relation() {
  local node=$1
  local next_line=$2
  local node_pattern='^(mission|epic|task|issue|validation[[:space:]]+issue)[[:space:]]+atelier-[a-z0-9-]+$'
  local relation_pattern='^[[:space:]]+(advances|child|blocks|relates|contains|depends)([[:space:]]+[[:alnum:]_-]+){0,3}[[:space:]]+atelier-[a-z0-9-]+$'

  [[ "$node" =~ $node_pattern ]] && [[ "$next_line" =~ $relation_pattern ]]
}

inline_suffix_is_data_context() {
  local suffix=${1,,}
  local descriptor='((record|schema|data)[[:space:]]+)?(type|transition|role|value|label|data)([[:space:]]+(name|type|value|label))?'
  local as_or_for_pattern="^[[:space:]]+(as|for)[[:space:]]+(a|an|the)?[[:space:]]*$descriptor([^[:alnum:]_-]|$)"
  local descriptor_usage_pattern="^[[:space:]]+(to|for|when)[[:space:]]+([[:alpha:]][[:alnum:]_-]*[[:space:]]+){1,3}(as[[:space:]]+)?(a|an|the)?[[:space:]]*$descriptor([^[:alnum:]_-]|$)"
  local immediate_descriptor_pattern='^[[:space:]]*(transition|field|type|status|value|label|role|key)([^[:alnum:]_-]|$)'

  [[ "$suffix" =~ $as_or_for_pattern ]] ||
    [[ "$suffix" =~ $descriptor_usage_pattern ]] ||
    [[ "$suffix" =~ $immediate_descriptor_pattern ]]
}

inline_context_is_action() {
  local before=${1,,}
  local after=${2,,}
  local label_pattern='(^|[|])[[:space:]]*([[:alnum:]_-]+[[:space:]]+)*(command|workflow|route)[[:space:]]*((is|remains)[[:space:]]+|:[[:space:]]*|[|][[:space:]]*|$)'
  local directive_pattern='(^|.*[^[:alnum:]_-])(run|use|invoke|execute|rerun|retry|try|enter|prefer|prefers|preferred|recommend|recommends|recommended|choose|chooses|select|selects|call|calls|called|adopt|adopts|adopted|pick|picks|picked)([[:space:]]+(the|this|command))?[[:space:]]*$'
  local migration_pattern='(^|.*[^[:alnum:]_-])((switch|migrate|move|transition|shift)(ed|s)?|fall(s|ing)?[[:space:]]+back|fell[[:space:]]+back|revert(ed|s)?)([[:space:]]+(from[[:space:]]+)?[[:alnum:]_.:/-]+){0,5}[[:space:]]+to[[:space:]]*$'
  local ownership_pattern='(^|[^[:alnum:]_-])(owns?|handles?|serves?|validates|reports?|mutates?|((is|are|remains?)[[:space:]]+)?(responsible|accountable)[[:space:]]+for)([^[:alnum:]_-]|$)'

  [[ "$before" =~ $label_pattern ]] ||
    [[ "$before" =~ $directive_pattern ]] ||
    [[ "$before" =~ $migration_pattern ]] ||
    [[ "$after" =~ $ownership_pattern ]]
}

scan_content() {
  local content
  local hit
  local text
  local metadata
  local source
  local heading
  local location
  local fenced
  local remaining
  local before
  local full_before
  local candidate
  local previous
  local consumed
  local finding
  local span
  local structural_candidate
  local structural_context
  local inline_code_pattern='`([^`]*)`'
  local list_command_shape_pattern="^(((${path_alternatives})${command_boundary})|(dep (add|remove)${command_boundary})|(issue (close|claim|new|quick|subissue|search|relate|tree|tested|update|list)${command_boundary})|(work (start|status|queue)${command_boundary})|(maintenance delete${command_boundary})|(review (link|status|comments|comment|approve|request-changes|open)${command_boundary})|(history[[:space:]]+--)|(worktree (create|for|list|remove)${command_boundary})|(mission (atelier-|--|<|create|show|start|status|close|list|update|note|add-work|unlink|add-blocker))|((${root_alternatives}|${restricted_root_alternatives})[[:space:]]+(--|atelier-|<)))"
  local single_command_token_pattern='^[a-z0-9-]+[,.;:!?)]?$'
  local inline_remaining
  local inline_before
  local inline_before_window
  local inline_after
  local inline_after_segment
  local inline_after_window
  local list_shape_regex
  local structural_kind
  local line_is_fence_data
  local shell_prompt_pattern='^(\([^)]*\)[[:space:]]+)?[^[:space:]$#%]*[[:space:]]*[$#%][[:space:]]+(.*)$'
  local -a content_lines=()
  local line_index
  local next_hit
  local next_text
  local next_metadata
  local next_location
  local next_fenced
  content=$(cat)
  mapfile -t content_lines <<< "$content"

  for ((line_index = 0; line_index < ${#content_lines[@]}; line_index++)); do
    hit=${content_lines[$line_index]}
    [[ -n "$hit" ]] || continue

    if [[ "$hit" == *' :: '* ]]; then
      metadata=${hit%% :: *}
      text=${hit#* :: }
      source=${metadata%%|*}
      location=${metadata#*|}
      location=${location#*|}
      fenced=${location%%|*}
      heading=${location#*|}
    else
      metadata=''
      text=$hit
      source=''
      heading=''
      fenced=0
    fi

    line_is_fence_data=0
    if ((fenced == 2 || fenced == 3)); then
      if fence_line_is_data "$text"; then
        line_is_fence_data=1
      elif ((line_index + 1 < ${#content_lines[@]})); then
        next_hit=${content_lines[$((line_index + 1))]}
        next_text=$next_hit
        next_fenced=0
        if [[ "$next_hit" == *' :: '* ]]; then
          next_metadata=${next_hit%% :: *}
          next_text=${next_hit#* :: }
          next_location=${next_metadata#*|}
          next_location=${next_location#*|}
          next_fenced=${next_location%%|*}
        fi
        if [[ "$next_fenced" == "$fenced" ]] &&
          record_graph_node_has_indented_relation "$text" "$next_text"; then
          line_is_fence_data=1
        fi
      fi
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
        if prefixed_candidate_is_finding "$candidate" "$source" "$heading"; then
          finding=1
          break
        fi
      fi

      consumed+="$before"'atelier '
      remaining=${remaining#*atelier }
    done

    # Bare retired/hidden commands are scanned only where Markdown or shell
    # structure makes the text command-shaped: inline code, fenced code,
    # prompts, block quotes, and list items. Ordinary prose words remain out of
    # scope even when they happen to equal a retired root such as "mission".
    if ((!finding)) && ((!line_is_fence_data)) && [[ "$text" == *\`* ]]; then
      inline_remaining=$text
      while [[ "$inline_remaining" =~ $inline_code_pattern ]]; do
        span=${BASH_REMATCH[0]}
        candidate=${BASH_REMATCH[1]}
        inline_before=${inline_remaining%%"$span"*}
        inline_after=${inline_remaining#*"$span"}
        inline_after_segment=${inline_after%%\`*}
        inline_before_window=$inline_before
        inline_after_window=$inline_after_segment
        ((${#inline_before_window} <= 80)) ||
          inline_before_window=${inline_before_window: -80}
        ((${#inline_after_window} <= 160)) ||
          inline_after_window=${inline_after_window:0:160}
        if ! inline_suffix_is_data_context "$inline_after" &&
          inline_context_is_action "$inline_before_window" "$inline_after_window" &&
          bare_candidate_is_finding "$candidate" "$source" "$heading"; then
          finding=1
          break
        fi
        inline_remaining=$inline_after
      done
    fi

    if ((!finding)); then
      structural_candidate=${text#"${text%%[![:space:]]*}"}
      structural_context=$fenced
      structural_kind=''
      ((fenced == 1)) && structural_kind='shell'
      ((fenced == 2)) && structural_kind='untyped'
      ((fenced == 3)) && structural_kind='non_shell'
      if [[ "$structural_candidate" =~ $shell_prompt_pattern ]]; then
        structural_context=1
        structural_kind='shell'
        structural_candidate=${BASH_REMATCH[2]}
      elif ((line_is_fence_data)); then
        structural_context=0
      else
        while [[ "$structural_candidate" =~ ^(\>|-|\*|\+|[0-9]+\.)[[:space:]]+(.*)$ ]]; do
          structural_context=1
          [[ "$structural_kind" == shell || "$structural_kind" == untyped || "$structural_kind" == non_shell ]] ||
            structural_kind='list'
          structural_candidate=${BASH_REMATCH[2]}
        done
      fi
      if ((structural_context)); then
        if [[ "$structural_kind" == shell ]] &&
          bare_candidate_is_finding "$structural_candidate" "$source" "$heading"; then
          finding=1
        elif [[ "$structural_kind" == list || "$structural_kind" == untyped || "$structural_kind" == non_shell ]]; then
          list_shape_regex=$list_command_shape_pattern
          if [[ "$structural_candidate" =~ $single_command_token_pattern ||
            "$structural_candidate" =~ $list_shape_regex ]] &&
            bare_candidate_is_finding "$structural_candidate" "$source" "$heading"; then
            finding=1
          fi
        fi
      fi
    fi

    if ((finding)); then
      printf '%s\n' "$hit"
    fi
  done
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
  local -a bare_prohibited_examples=()
  local -a bare_restricted_examples=()
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
  local -a required_bare_examples=(
    'lint'
    'doctor --fix'
    'dep add atelier-demo atelier-blocker'
    'dep remove atelier-demo atelier-blocker'
    'start atelier-demo'
    'mission atelier-demo'
    'worktree create atelier-demo'
    'issue close atelier-demo --reason done'
    'maintenance delete atelier-demo'
    'review approve atelier-demo'
    'history --mission atelier-demo'
  )
  local -a restricted_adversarial_examples=(
    'The hidden API is gone; use atelier doctor --fix now.'
    'This is not a recovery command; use atelier export --check now.'
    'The admin path differed; invoke atelier rebuild now.'
    'Diagnostics changed; run atelier workflow check now.'
    'Recovery is unrelated; use atelier branch merge atelier-demo now.'
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
    'Do not forget to run atelier start now.'
    'You must not avoid atelier start.'
    'Never skip atelier start.'
    'atelier start must not be skipped; use it now.'
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
  restricted_examples+=("${restricted_adversarial_examples[@]}")

  for root in "${restricted_roots[@]}"; do
    bare_restricted_examples+=("$root")
  done
  bare_restricted_examples+=(
    'doctor --fix'
    'export --check'
    'import-beads backup.jsonl'
    'workflow check'
    'diagnostics slow'
    'branch merge atelier-demo'
    'forgejo status'
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
  prohibited_examples+=("${local_negative_examples[@]}")

  for example in "${removed_roots[@]}"; do
    bare_prohibited_examples+=("$example")
  done
  for example in "${removed_paths[@]}"; do
    bare_prohibited_examples+=("$example")
  done
  bare_prohibited_examples+=(
    'dep add atelier-demo atelier-blocker'
    'dep remove atelier-demo atelier-blocker'
    'start atelier-demo'
    'mission atelier-demo'
    'worktree create atelier-demo'
    'issue close atelier-demo --reason done'
    'issue update atelier-demo --claim'
    'issue list --ready'
    'work start atelier-demo'
    'work status'
    'work queue'
    'maintenance delete atelier-demo'
    'export --format json'
    'review link atelier-demo'
    'review status atelier-demo'
    'review comments atelier-demo'
    'review comment atelier-demo text'
    'review approve atelier-demo'
    'review request-changes atelier-demo'
    'review open --title manual'
    'history --mission atelier-demo'
    'history --epic atelier-demo'
    'history --include-descendants'
    'history --event-kind note'
    'history --actor worker'
    'history --since 2026-01-01'
  )

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
  for example in "${required_bare_examples[@]}"; do
    if ! array_contains "$example" "${bare_prohibited_examples[@]}" &&
      ! array_contains "$example" "${bare_restricted_examples[@]}"; then
      printf 'self-test missing required bare structural fixture: %s\n' "$example" >&2
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

  # Every authoritative removed root/path is exercised without the binary
  # prefix inside an actual fenced-command context. This keeps bare coverage
  # derived from the same inventory as prefixed coverage.
  for example in "${bare_prohibited_examples[@]}" "${bare_restricted_examples[@]}"; do
    checked=$((checked + 1))
    output=$(
      printf '# Live Guidance\n```sh\n%s\n```\n' "$example" |
        active_content | scan_content
    )
    if [[ -z "$output" ]]; then
      printf 'self-test failed to reject bare fenced command: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exercise each structural entry path through active_content | scan_content,
  # including arguments, punctuation, bullets, prompts, and inline spans.
  for example in \
    'Use `lint --all` for normal validation.' \
    'Run `doctor --fix`, then continue.' \
    'Use `dep add atelier-demo atelier-blocker`.' \
    'Use `dep remove atelier-demo atelier-blocker`.' \
    '- start atelier-demo, then continue' \
    '1. mission atelier-demo.' \
    '> worktree create atelier-demo' \
    '* issue close atelier-demo --reason done' \
    '+ maintenance delete atelier-demo' \
    'Use `review request-changes atelier-demo`.' \
    'Use `history --mission atelier-demo`.' \
    '$ review approve atelier-demo'; do
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed bare structural variant: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact independent-review fixtures from atelier-b8xw. These use the same
  # active_content | scan_content pipeline as the broad indexed-doc check.
  for example in \
    'Hidden/admin export and deterministic-check surfaces remain compatibility tools for migration or targeted maintenance. `doctor --fix` owns explicit ignored-state repair for normal operators.' \
    '- `lint` validates `.atelier/` Markdown directly, `doctor` reports local projection/runtime health.' \
    '| `dep add` and `dep remove` | RecordStore-owned Markdown-first | Top-level Agent Factory dependency aliases mutate canonical issue relationship front matter before projection refresh. |'; do
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-b8xw bare-command fixture: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact atelier-pgwg/atelier-ztnc actionable misses plus systematic
  # imperative and ownership variants. Context is evaluated adjacent to each
  # inline span, not as a line-wide sentiment exemption.
  for example in \
    'Prefer `lint --all` for validation.' \
    'The current command is `doctor --fix`.' \
    'Call `dep add atelier-demo atelier-blocker` to link records.' \
    'Normal workflow: `mission show atelier-demo`.' \
    'Recommend `lint --all` for validation.' \
    'Choose `doctor --fix` for repair.' \
    'Retry `dep add atelier-demo atelier-blocker`.' \
    'The supported command is `mission show atelier-demo`.' \
    '`mission show atelier-demo` handles objective detail.' \
    'Enter `history --mission atelier-demo` for the old view.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed actionable bare inline variant: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact data/schema false positives plus systematic explicit `as ...`
  # variants. Only the syntax adjacent to the candidate grants this exemption.
  for example in \
    'Use `mission` as the record type.' \
    'Use `close` as the transition name.' \
    'Use `worker` as the role value.' \
    'Use `list` as a data label.' \
    'Use `mission` as a type.' \
    'Use `close` as the transition value.' \
    'Use `worker` as a role label.' \
    'Use `list` as the data value.' \
    'Use `mission` as the schema type.' \
    'Use `close` as data.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for explicit data/schema syntax: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```\nmission: atelier-demo\nstatus: todo\n```' \
    $'```\n- mission: atelier-demo\n  status: todo\n```' \
    $'```\n{"mission": "atelier-demo", "status": "todo"}\n```' \
    $'```yaml\nmission: atelier-demo\nstatus: todo\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for untyped/typed structured data fence:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```\nmission show atelier-demo\n```' \
    $'```\ndoctor --fix\n```' \
    $'```\nreview open --title manual\n```' \
    $'```\n$ mission show atelier-demo\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed command-shaped untyped fence line:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact independent atelier-igno action/table cases.
  for example in \
    'Adopt `lint --all` for validation.' \
    'Pick `doctor --fix` for repair.' \
    'Switch to `dep add atelier-demo atelier-blocker`.' \
    'Default command: `mission show atelier-demo`.' \
    'Standard workflow: `mission show atelier-demo`.' \
    '| Default command | `lint --all` |'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-igno action/table case: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact independent atelier-igno `for the ...` data cases.
  for example in \
    'Use `mission` for the record type.' \
    'Use `close` for the transition name.' \
    'Select `worker` for the role value.' \
    'Prefer `list` for the data label.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for exact atelier-igno data case: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```console\nuser@host$ doctor --fix\n```' \
    $'```console\n(venv) $ lint --all\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-igno console prompt:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```yaml\n- mission\n- task\n```' \
    $'```\n- mission\n- task\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for exact atelier-igno YAML scalar list:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Systematic inflection/prompt/scalar variants retain the same bounded
  # classification without weakening shell subcommand or option recognition.
  for example in \
    'We adopted `lint --all` for validation.' \
    'They picked `doctor --fix` for repair.' \
    'The team switched to `dep add atelier-demo atelier-blocker`.' \
    'Default route: `mission show atelier-demo`.' \
    'Standard command: `mission show atelier-demo`.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed systematic atelier-igno action variant: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    'Use `mission` for a schema type.' \
    'Prefer `list` for the data value.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for systematic `for ...` data variant: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```console\ndev.user@host:~/repo$ doctor --fix\n```' \
    $'```console\n(atelier-dev) $ lint --all\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed systematic console prompt variant:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```yaml\n- "mission"\n- task\n```' \
    $'```\n- mission\n- 42\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for systematic YAML scalar variant:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact independent atelier-xs4r command-label and migration cases.
  for example in \
    'Canonical command: `lint --all`.' \
    'Primary command: `doctor --fix`.' \
    'Migrate to `dep add atelier-demo atelier-blocker`.' \
    'Fall back to `mission show atelier-demo`.' \
    '| Primary command | `lint --all` |'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-xs4r label/migration case: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    'Use `mission` to represent the record type.' \
    'Select `close` when setting the transition name.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for exact atelier-xs4r data case: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```console\nhost$ doctor --fix\n```' \
    $'```console\n% lint --all\n```' \
    $'```console\nroot@host# doctor --fix\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-xs4r console prompt:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```text\n- mission\n- task\n```' \
    $'```markdown\n- mission\n- task\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for exact atelier-xs4r scalar-list fence:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Category-level variants prove labels are adjective-independent, migration
  # is grammatical, prompts use recognizable markers, and all non-shell fence
  # labels share the same data-vs-command classifier.
  for example in \
    'Authoritative command: `lint --all`.' \
    'The selected operator command is `doctor --fix`.' \
    '| Recovery command | `lint --all` |' \
    'Operator workflow: `mission show atelier-demo`.' \
    'Fallback route: `mission show atelier-demo`.' \
    'Move to `dep add atelier-demo atelier-blocker`.' \
    'Transition to `mission show atelier-demo`.' \
    'Shift to `mission show atelier-demo`.' \
    'Revert to `doctor --fix`.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed structural label/migration category variant: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    'Use `mission` to encode the schema type.' \
    'Prefer `worker` to denote the role label.' \
    'Select `close` when assigning the transition value.' \
    'Use `list` when storing the data label.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for structural data-precedence variant: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```console\ndevbox# doctor --fix\n```' \
    $'```console\noperator% lint --all\n```' \
    $'```console\n(test-env) # doctor --fix\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed structural prompt category variant:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```plaintext\n- mission\n- task\n```' \
    $'```md\n- "mission"\n- task\n```' \
    $'```rst\n- mission\n- 42\n```' \
    $'```text\nmission atelier-demo\n  advances epic atelier-child\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for non-shell fence data variant:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```text\ndoctor --fix\n```' \
    $'```markdown\nmission show atelier-demo\n```' \
    $'```plaintext\n$ lint --all\n```' \
    $'```console\nmission atelier-demo\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed command-shaped non-shell fence variant:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  # Exact independent atelier-ypbt category-invariant cases.
  for example in \
    'Migrate from check to `dep add atelier-demo atelier-blocker`.' \
    'Switch the workflow to `mission show atelier-demo`.' \
    'Fall back from check to `doctor --fix`.' \
    '`lint --all` is responsible for normal validation.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-ypbt action/ownership case: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    'Use `mission` to be used as the record type.' \
    'Prefer `worker` for use as the role label.' \
    'Select `close` when used as the transition value.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for exact atelier-ypbt data case: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```console\n(venv) user@host$ doctor --fix\n```' \
    $'```console\nroot@host # lint --all\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed exact atelier-ypbt prompt case:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  example=$'```\nmission atelier-demo\n  advances epic atelier-child\n```'
  checked=$((checked + 1))
  output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
  if [[ -n "$output" ]]; then
    printf 'self-test false-positive for exact atelier-ypbt record graph:\n%s\n' \
      "$example" >&2
    failures=$((failures + 1))
  fi

  # Neighboring variants exercise bounded complements, responsibility forms,
  # passive descriptor use, composite prompts, and graph adjacency.
  for example in \
    'Move from the old workflow to `dep add atelier-demo atelier-blocker`.' \
    'Transition this operator workflow to `mission show atelier-demo`.' \
    'Revert from doctor to `lint --all`.' \
    '`doctor --fix` is accountable for local repair.' \
    '`lint --all` remains responsible for validation.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed bounded complement/responsibility variant: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    'Use `mission` to be treated as the schema type.' \
    'Prefer `worker` for storage as the role value.' \
    'Select `close` when encoded as the transition label.'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for passive descriptor variant: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```console\n(dev) user@host:~/repo # doctor --fix\n```' \
    $'```console\n(test) root@host% lint --all\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test missed composite prompt variant:\n%s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  for example in \
    $'```\nmission atelier-demo\n```' \
    $'```text\nmission atelier-demo\nadvances epic atelier-child\n```' \
    $'```\nmission atelier-demo\n  narrative without a relation\n```' \
    $'```console\nmission atelier-demo\n  advances epic atelier-child\n```'; do
    checked=$((checked + 1))
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -z "$output" ]]; then
      printf 'self-test broadly allowed root+ID without graph structure:\n%s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi
  done

  output=$(
    printf '%s\n' \
      '# Live Guidance' \
      '```console' \
      'history --event-kind note' \
      '```' |
      active_content | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test missed bare command in fenced console transcript\n' >&2
    failures=$((failures + 1))
  fi

  for example in \
    'Mission records remain canonical.' \
    'Start time is recorded separately.' \
    'A doctor reports health in ordinary prose.' \
    'The `mission_id` field is not a command.' \
    'The `mission::Record` type is not a command.' \
    'The `starter` helper is not a command.' \
    'The `worktree-like` adjective is not a command.' \
    'The service may report that the `close` transition is blocked.'; do
    output=$(printf '# Live Guidance\n%s\n' "$example" | active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test false-positive for non-command prose/code: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  output=$(
    printf '%s\n' \
      '## Historical Commands (Non-Normative)' \
      'Use `mission atelier-demo`.' |
      active_content | scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected historical bare command section\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Historical Commands (Non-Normative)' \
      'Use `mission atelier-demo`.' \
      '## Live Guidance' \
      'Use `mission atelier-demo`.' |
      active_content | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test missed bare command re-entry after historical section\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '# Setup And Recovery' \
      '```sh' \
      'doctor --fix' \
      '```' |
      active_content | scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected context-bounded bare restricted command\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '# Setup And Recovery' \
      'Use `doctor --fix`.' \
      '# Live Guidance' \
      'Use `doctor --fix`.' |
      active_content | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test missed bare restricted-command live re-entry\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '# Audit\nUse `lint` only as classified here.\n' |
      active_content "$repo_root/docs/product/command-audit/lint.md" |
      scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected bare restricted command in exact audit document\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary' \
      'Use `work queue` now.' |
      active_content "$repo_root/$legacy_callable_document" | scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected bare c0mp legacy-callable boundary\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary Extended' \
      'Use `work queue` now.' |
      active_content "$repo_root/$legacy_callable_document" | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test accepted bare c0mp extended heading spoof\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary' \
      'Use `work queue` now.' |
      active_content "$repo_root/docs/product/work-view-ordering.md" | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test accepted bare c0mp command in wrong document\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary' \
      'Use `work queue` now.' \
      '## Live Guidance' \
      'Use `work queue` now.' |
      active_content "$repo_root/$legacy_callable_document" | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test missed bare c0mp live re-entry\n' >&2
    failures=$((failures + 1))
  fi

  for example in "${local_negative_examples[@]}"; do
    output=$(printf '## Historical Commands (Non-Normative)\n%s\n' "$example" |
      active_content | scan_content)
    if [[ -n "$output" ]]; then
      printf 'self-test rejected explicit section-level historical classification: %s\n' \
        "$example" >&2
      failures=$((failures + 1))
    fi

    output=$(
      printf '%s\n' \
        '## Historical Commands (Non-Normative)' \
        "$example" \
        '## Live Guidance' \
        "$example" |
        active_content | scan_content
    )
    if [[ -z "$output" ]]; then
      printf 'self-test missed live re-entry for negative-language command: %s\n' \
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

    output=$(
      printf '%s\n' \
        '# Setup And Recovery' \
        "$example" \
        '# Live Guidance' \
        "$example" |
        active_content | scan_content
    )
    if [[ -z "$output" ]]; then
      printf 'self-test missed restricted-command live re-entry: %s\n' "$example" >&2
      failures=$((failures + 1))
    fi
  done

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary' \
      'Use `atelier work queue` now.' |
      active_content "$repo_root/$legacy_callable_document" | scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected exact c0mp legacy-callable boundary\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary Extended' \
      'Use `atelier work queue` now.' |
      active_content "$repo_root/$legacy_callable_document" | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test accepted extended c0mp legacy heading spoof\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary' \
      'Use `atelier work queue` now.' |
      active_content "$repo_root/docs/product/work-view-ordering.md" | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test accepted c0mp legacy heading in the wrong document\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '## Legacy Queue Boundary' \
      'The legacy `atelier work queue` is bounded here.' \
      '## Live Guidance' \
      'Use `atelier work queue` now.' |
      active_content "$repo_root/$legacy_callable_document" | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test missed live re-entry after exact c0mp legacy boundary\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '# Diagnostics Extended' \
      'Use atelier doctor --fix now.' |
      active_content | scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test accepted extended restricted-context heading spoof\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '# Unrelated' \
      'Use atelier export --check now.' |
      active_content "$repo_root/docs/product/command-audit/export.md" |
      scan_content
  )
  if [[ -n "$output" ]]; then
    printf 'self-test rejected exact restricted-command audit document\n' >&2
    failures=$((failures + 1))
  fi

  output=$(
    printf '%s\n' \
      '# Unrelated' \
      'Use atelier export --check now.' |
      active_content "$repo_root/docs/product/command-audit/export.md.extended" |
      scan_content
  )
  if [[ -z "$output" ]]; then
    printf 'self-test accepted restricted-audit document path spoof\n' >&2
    failures=$((failures + 1))
  fi

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
    "$checked" "$(( ${#adversarial_prohibited_examples[@]} + ${#local_negative_examples[@]} + ${#restricted_adversarial_examples[@]} ))"
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
