#!/usr/bin/env bash
# The cutover runbook (docs/strategies/redesign-architecture.md, Transition,
# part 4). Read scripts/cutover/README.md first.
#
#   cutover.sh preflight   check that the five repositories are quiet; changes nothing
#   cutover.sh             run the cutover, from the worktree that has atelier's master
#   cutover.sh adopt       take on remotes another host converted, from a fresh clone
#                          of atelier's converted master
#
# It stops at the first failure, says which step failed and what rollback.sh
# will undo, and keeps a journal in its state directory ($STATE/journal).
# Run again after a stop, it picks up from the journal. Exit status: 0 when
# it finished, 1 when it stopped, 3 when CI on the conversion commit had not
# finished within its bound (run it again).
#
# CUTOVER_FAIL_AT=<point> stops it at that point, for the trials only.
set -eEuo pipefail
source "$(dirname "$0")/common.sh"

STEP="" STEP_NAME="" STEP_T0=0
step() { # <n> <name>
  [ -z "$STEP" ] || finish_step
  STEP=$1 STEP_NAME=$2 STEP_T0=$(date +%s)
  say "== Step $1: $2"
}
finish_step() {
  local secs=$(( $(date +%s) - STEP_T0 ))
  printf '%s\t%s\t%s\n' "$STEP" "$STEP_NAME" "$secs" >> "$STATE/times"
  say "   step $STEP took ${secs} s"
}
point() { [ "${CUTOVER_FAIL_AT:-}" != "$1" ] || die "stopped at $1, as CUTOVER_FAIL_AT asked (trials only)"; }

on_exit() {
  local rc=$?
  [ $rc -ne 0 ] && [ $rc -ne 3 ] && [ -n "$STEP" ] || return 0
  say ""
  say "STOPPED in step $STEP ($STEP_NAME), exit $rc."
  say "rollback.sh will undo what the journal ($JOURNAL) says was started:"
  undo_summary | sed 's/^/  /'
  say "Run scripts/cutover/rollback.sh, or, if the cause is fixed and nothing was rolled back, run this script again."
}
trap on_exit EXIT

# What a rollback would undo, from the journal, in plain words.
undo_summary() {
  local k f
  for k in $(jkeys ""); do
    f=$(jfields "$k")
    case $k in
      commit:*) say "a commit by the runbook in ${k#commit:} ($(jstate "$k"))" ;;
      push:*) say "a push of ${k#push:} ($(jstate "$k"); the remote's tip decides)" ;;
      convert:*) say "the conversion of ${k#convert:} ($(jstate "$k"))" ;;
      ref:*) say "the base of ${k#ref:} moved: $f" ;;
      checkout:*) say "a worktree switched: ${k#checkout:} ($f)" ;;
      install) say "the new binary installed into $INSTALL_ROOT" ;;
      setup:*) say "Atelier's Git hooks set up in ${k#setup:}; every hooks directory goes back from its copy" ;;
      skill) say "the skill checkout moved: $f" ;;
      skill-dir) say "the skill installed into $SKILL_DIR" ;;
      service) say "$SERVICE stopped and disabled (was: $f)" ;;
    esac
  done
}

# ---------------------------------------------------------------- preflight

# The items the installed old binary reports in progress in one repository:
# every worktree's own records, and the branches `status` lists.
old_in_progress() { # <name>
  local wt
  while read -r wt _; do
    [ -d "$wt" ] || continue
    (cd "$wt" && atelier work active -q 2>/dev/null) || true
  done < <(worktrees "$(clone_of "$1")")
  (cd "$(clone_of "$1")" && atelier status 2>/dev/null) | awk '
    /^Branches$/ { s = 1; next }
    s && /^$/ { s = 0 }
    s { for (i = 3; i <= NF; i++) if ($i == "in_progress," || $i == "in_progress") { print $2; break } }'
}

# Directories where a T3 thread other than this one (and the threads waiting
# on it) has a run going, one per line; status 2 when T3's database cannot
# be read. One read-only, bounded query.
t3_thread_dirs() {
  [ -f "$T3_DB" ] && command -v sqlite3 >/dev/null || return 2
  sqlite3 -readonly -bail "file:$T3_DB?mode=ro" "
    WITH RECURSIVE mine(id) AS (
      SELECT '${T3CODE_THREAD_ID:-none}'
      UNION SELECT json_extract(t.payload_json, '\$.lineage.parentThreadId')
      FROM orchestration_v2_projection_threads t JOIN mine ON t.thread_id = mine.id)
    SELECT DISTINCT coalesce(json_extract(t.payload_json, '\$.worktreePath'), p.workspace_root)
    FROM orchestration_v2_projection_runs r
    JOIN orchestration_v2_projection_threads t ON t.thread_id = r.thread_id
    LEFT JOIN projection_projects p ON p.project_id = t.project_id
    WHERE r.status IN ('running', 'queued')
      AND r.thread_id NOT IN (SELECT id FROM mine WHERE id IS NOT NULL)
    LIMIT 200;" 2>/dev/null || return 2
}

switch_merged() { git -C "$(clone_of atelier)" cat-file -e "refs/heads/$(base_of atelier):scripts/cutover/cutover.sh" 2>/dev/null; }

# Checks that every repository is quiet; prints each problem and returns 1
# if there is one. <mode> is cutover or adopt.
preflight_checks() {
  local mode=$1 bad=0 r c b wt head local_tip tip ids allowed dirs d n
  local version
  version=$(atelier --version 2>/dev/null || true)
  case $version in "atelier 0.4"*) ;; *) say "  the installed atelier is \"$version\", not the old tool 0.4"; bad=1 ;; esac
  allowed=""
  if [ "$mode" = cutover ] && ! switch_merged; then allowed=$SWITCH_ITEM; fi
  dirs=$(t3_thread_dirs) || dirs="unreadable"
  for r in $(repo_names); do
    c=$(clone_of "$r") b=$(base_of "$r")
    [ -d "$c/.git" ] || [ -f "$c/.git" ] || { say "  $r: no clone at $c"; bad=1; continue; }
    while read -r wt head; do
      if [ ! -d "$wt" ]; then say "  $r: worktree $wt is missing; run git -C $c worktree prune"; bad=1; continue; fi
      n=$(git -C "$wt" status --porcelain | wc -l)
      [ "$n" -eq 0 ] || { say "  $r: $wt has $n uncommitted or untracked paths"; bad=1; }
      [ "$head" = "$b" ] || say "  $r: (information) $wt is on $head, not $b"
    done < <(worktrees "$c")
    local_tip=$(git -C "$c" rev-parse -q --verify "refs/heads/$b" || true)
    tip=$(remote_tip "$r") || { say "  $r: cannot read $REMOTE's $b"; bad=1; continue; }
    if [ "$mode" = adopt ]; then
      git -C "$c" fetch -q "$REMOTE" "+refs/heads/$b:refs/cutover/remote-base"
      if ! git -C "$c" merge-base --is-ancestor "$local_tip" refs/cutover/remote-base; then
        say "  $r: local $b is ahead of or apart from $REMOTE's; push or drop that work first"; bad=1
      fi
      git -C "$c" update-ref -d refs/cutover/remote-base
    elif [ "$local_tip" != "$tip" ]; then
      say "  $r: local $b is at ${local_tip:0:8}, $REMOTE's at ${tip:0:8}; bring them together first"; bad=1
    fi
    ids=$(old_in_progress "$r" | sort -u | grep -vx -- "${allowed:-^$}" || true)
    [ -z "$ids" ] || { say "  $r: in progress: $(echo $ids)"; bad=1; }
    n=$(git -C "$c" branch --format='%(refname:short)' --no-merged "refs/heads/$b" | wc -l)
    [ "$n" -eq 0 ] || say "  $r: (information) $n local branches not merged into $b; their reviews are not carried over: $(git -C "$c" branch --format='%(refname:short)' --no-merged "refs/heads/$b" | tr '\n' ' ')"
    if [ "$dirs" != unreadable ]; then
      while read -r d; do
        [ -n "$d" ] || continue
        while read -r wt _; do
          case "$d/" in "$wt"/*) say "  $r: a T3 thread other than this one is running in $d"; bad=1; break ;; esac
        done < <(worktrees "$c")
      done <<< "$dirs"
    fi
  done
  if [ "$dirs" = unreadable ]; then
    if [ "${CUTOVER_THREADS_CHECKED:-}" = yes ]; then
      say "  T3's database could not be read at $T3_DB; the hand check was made (CUTOVER_THREADS_CHECKED=yes)."
    else
      say "  T3's database could not be read at $T3_DB. Check by hand: in T3 Code, no thread other than this"
      say "  one is running in any of the five repositories or their worktrees. Then run again with"
      say "  CUTOVER_THREADS_CHECKED=yes."
      bad=1
    fi
  fi
  return $bad
}

# Records what the rollback needs: each base commit, what each worktree has
# checked out, a copy of each hooks directory, the installed binary, the
# skill and the service.
record() { # <mode>
  local r c b wt head h copy
  jstart mode "$1"; jdone mode
  for r in $(repo_names); do
    c=$(clone_of "$r") b=$(base_of "$r")
    jstart "pre:base:$r" "$(git -C "$c" rev-parse "refs/heads/$b")"; jdone "pre:base:$r"
    jstart "pre:tracking:$r" "$(git -C "$c" rev-parse -q --verify "refs/remotes/$REMOTE/$b" || echo none)"; jdone "pre:tracking:$r"
    while read -r wt head; do
      jstart "pre:wt:$r:$wt" "$head"; jdone "pre:wt:$r:$wt"
    done < <(worktrees "$c")
    h=$(hooks_dir "$c") copy=$STATE/hooks/$r
    rm -rf "$copy"; mkdir -p "$STATE/hooks"
    if [ -d "$h" ]; then cp -a "$h" "$copy"; jstart "pre:hooks:$r" "$h	$copy"; else jstart "pre:hooks:$r" "$h	absent"; fi
    jdone "pre:hooks:$r"
  done
  local bin; bin=$(command -v atelier)
  jstart pre:binary "$(atelier --version)	$(sha256sum "$bin" | cut -d' ' -f1)	$bin"; jdone pre:binary
  if git -C "$SKILL" rev-parse -q --verify HEAD >/dev/null 2>&1; then
    jstart pre:skill "$(git -C "$SKILL" rev-parse HEAD)"; jdone pre:skill
  fi
  if [ -n "$SKILL_DIR" ]; then
    rm -rf "$STATE/skill-dir"
    if [ -e "$SKILL_DIR" ]; then cp -a "$SKILL_DIR" "$STATE/skill-dir"; jstart pre:skill-dir "$SKILL_DIR"; else jstart pre:skill-dir "$SKILL_DIR	absent"; fi
    jdone pre:skill-dir
  fi
  jstart pre:service "$(service_state)"; jdone pre:service
}
service_state() { echo "$($SYSTEMCTL is-enabled "$SERVICE" 2>/dev/null || true) $($SYSTEMCTL is-active "$SERVICE" 2>/dev/null || true)"; }

# ---------------------------------------------------------------- conversion

CONVERT_MESSAGE="Convert Atelier records to the new layout, by the cutover runbook"

# Converts the checkout in the current directory: records, AGENTS.md,
# workflows and harness hooks. Writes nothing it is not told to.
convert_here() { # <name> <base commit> <atelier binary>
  local r=$1 bin=$3 out
  out=$STATE/convert-$r.txt
  python3 "$here/convert_records.py" > "$out"
  counts_from "$out" > "$STATE/report/$r.counts"
  if [ "$r" = atelier ]; then
    cp "$here/agents/atelier.md" AGENTS.md
    git rm -r -q skills/agent-factory
    git mv skills/agent-factory-next skills/agent-factory
  else
    replace_process_section "$r"
    replace_workflows "$r"
  fi
  "$bin" setup claude
  "$bin" setup codex
}
# The converter's counts of items and missions, as "<items> <missions>".
counts_from() {
  awk '/^Records converted/ { s = 1; next } /^[^ ]/ { s = 0 }
       s && $1 ~ /^item/ { i += $NF } s && $1 ~ /^mission/ { m += $NF }
       END { print i + 0, m + 0 }' "$1"
}

# The rule: AGENTS.md's sections (from a heading of level 1 or 2 to the
# next) that name a removed Atelier command. The first is replaced by the
# reviewed process section and the others are removed. Stops when none
# matches or more than the recorded number do.
REMOVED='\batelier\s+(init|work|issue|evidence|land|bundle|doctor|brief|debrief|serve|prune|migrate|review\s+request)\b'
replace_process_section() { # <name>
  local most; most=$(setting_for "$SECTIONS" "$1")
  [ -n "$most" ] || die "$1: no section count recorded in CUTOVER_SECTIONS"
  python3 - "$1" "$most" "$REMOVED" "$here/agents/product.md" "$(base_of "$1")" "$STATE/report/$1.sections" <<'EOF'
import re, sys
name, most, removed, text, base, report = sys.argv[1], int(sys.argv[2]), re.compile(sys.argv[3]), sys.argv[4], sys.argv[5], sys.argv[6]
lines = open("AGENTS.md", encoding="utf-8").read().split("\n")
starts = [i for i, l in enumerate(lines) if re.match(r"#{1,2} ", l)] + [len(lines)]
spans = [(s, e) for s, e in zip(starts, starts[1:]) if removed.search("\n".join(lines[s:e]))]
if not spans:
    sys.exit(f"{name}: no section of AGENTS.md names a removed Atelier command; the rule matches nothing")
if len(spans) > most:
    sys.exit(f"{name}: {len(spans)} sections of AGENTS.md name a removed Atelier command, more than the {most} recorded")
new = open(text, encoding="utf-8").read().replace("@BASE@", base).rstrip("\n").split("\n") + [""]
out, i = [], 0
for n, (s, e) in enumerate(spans):
    out += lines[i:s] + (new if n == 0 else [])
    i = e
out += lines[i:]
open("AGENTS.md", "w", encoding="utf-8").write("\n".join(out))
left = [l for l in out if removed.search(l)]
if left:
    sys.exit(f"{name}: AGENTS.md still names a removed command after the replacement")
open(report, "w").write(f"{len(spans)} sections, {sum(e - s for s, e in spans)} lines replaced\n")
EOF
}
replace_workflows() { # <name>
  local w f conv
  for w in $WORKFLOWS; do
    [ "${w%%:*}" = "$1" ] || continue
    f=${w#*:}
    [ -f "$f" ] || die "$1: $f, the workflow to replace, is missing"
    conv=$(jresult commit:atelier)
    [ -n "$conv" ] || die "$1: atelier's conversion commit is not recorded"
    sed "s/@ATELIER_REV@/$conv/" "$here/astromech-atelier.yml" > "$f"
  done
  # No other workflow may run a removed command or install Atelier.
  for f in .github/workflows/*; do
    [ -f "$f" ] || continue
    case " $WORKFLOWS " in *" $1:$f "*) continue ;; esac
    ! grep -qE "atelier[ \"-]+(init|work|issue|evidence|land|doctor|serve|prune|migrate|brief|debrief|bundle)\b|spadaval/atelier" "$f" \
      || die "$1: $f runs Atelier and has no reviewed replacement"
  done
}

# Commits the conversion in <dir>, whose HEAD must be <base>; settles a
# commit that was started and not marked done.
commit_conversion() { # <name> <dir> <base>
  local r=$1 d=$2 b=$3 k=commit:$1 head
  case $(jstate "$k") in
    done) return 0 ;;
    started)
      head=$(git -C "$d" rev-parse HEAD)
      if [ "$head" != "$b" ] && [ "$(git -C "$d" rev-parse HEAD^)" = "$b" ] \
        && [ "$(git -C "$d" log -1 --format=%s)" = "$CONVERT_MESSAGE" ]; then jdone "$k" "$head"; return 0; fi
      [ "$head" = "$b" ] || die "$r: $d is at ${head:0:8}, neither the base nor its conversion" ;;
  esac
  jstart "$k" "$d	$b"
  # Everything: the converter also moves strategy files out of docs/. The
  # checkout was clean at the preflight, so nothing else is there.
  git -C "$d" add -A
  git -C "$d" commit -q -m "$CONVERT_MESSAGE"
  jdone "$k" "$(git -C "$d" rev-parse HEAD)"
}

# Pushes <dir>'s base to the remote. A push started and not marked done is
# settled from the remote's tip.
push_base() { # <key> <name> <dir> <old> <new>
  local k=$1 r=$2 d=$3 old=$4 new=$5 tip
  case $(jstate "$k") in
    done) return 0 ;;
    started)
      tip=$(remote_tip "$r")
      if [ "$tip" = "$new" ]; then jdone "$k" "found on the remote"; return 0; fi
      [ "$tip" = "$old" ] || die "$r: the remote's $(base_of "$r") is at ${tip:0:8}, neither ${old:0:8} nor ${new:0:8}" ;;
  esac
  jstart "$k" "$old	$new"
  ATELIER_SKIP="$SKIP_REASON" git -C "$d" push -q "$REMOTE" "refs/heads/$(base_of "$r")" \
    || die "$r: the push of $(base_of "$r") failed; the remote's tip decides what happened when this runs again"
  jdone "$k"
}

# The count of relative Markdown links in tracked files that resolve, as
# "<resolving> of <all>".
links_resolving() { # <dir>
  (cd "$1" && git ls-files -z '*.md' | python3 -c '
import os, re, sys
link = re.compile(r"\]\(\s*<?([^)\s>]+)>?")
ok = all_ = 0
for f in sys.stdin.read().split("\0"):
    if not f or not os.path.exists(f): continue
    for m in link.finditer(open(f, encoding="utf-8", errors="replace").read()):
        t = m.group(1).split("#")[0]
        if not t or re.match(r"[a-z][a-z0-9+.-]*:", t, re.I) or t.startswith("/"): continue
        all_ += 1
        ok += os.path.exists(os.path.normpath(os.path.join(os.path.dirname(f), t)))
print(f"{ok} of {all_}")')
}

# ---------------------------------------------------------------- the steps

convert_atelier() {
  local W=$1 B=$2 bin=$1/target/release/atelier p aside
  if [ "$(jstate build)" != done ]; then
    jstart build "$BUILD"
    (cd "$W" && eval "$BUILD")
    jdone build
  fi
  [ -x "$bin" ] || die "the build left no binary at $bin"
  if [ "$(jstate convert:atelier)" != done ]; then
    if [ "$(jstate convert:atelier)" = started ]; then
      git -C "$W" reset -q --hard "$B"
      git -C "$W" clean -fdq -- $CONVERTED_PATHS
    fi
    [ "$(git -C "$W" rev-parse HEAD)" = "$B" ] || die "atelier: $W is not at the recorded base"
    aside=$STATE/aside/atelier
    jstart convert:atelier "$W	$B	$aside"
    # Ignored files under the converted paths, such as the old tool's
    # runtime directory, go aside: --verify accepts nothing old under
    # .atelier/, and the rollback puts them back.
    while IFS= read -r p; do
      mkdir -p "$aside/$(dirname "$p")"
      mv "$W/$p" "$aside/$p"
    done < <(git -C "$W" status --porcelain --ignored -- $CONVERTED_PATHS | sed -n 's/^!! //p' | sed 's:/$::')
    (cd "$W" && convert_here atelier "$B" "$bin")
    jdone convert:atelier
  fi
  commit_conversion atelier "$W" "$B"
}

wait_for_ci() { # <commit>
  local sha=$1 end=$(( $(date +%s) + CI_WAIT )) runs id
  while :; do
    runs=$($GH run list -R "$GH_REPO" --workflow "$CI_WORKFLOW" --commit "$sha" --event push \
      --json databaseId,status,conclusion --limit 5 2>/dev/null || echo '[]')
    id=$(jq -r '[.[] | select(.status == "completed")][0].databaseId // empty' <<< "$runs")
    if [ -n "$id" ]; then
      [ "$(jq -r "[.[] | select(.databaseId == $id)][0].conclusion" <<< "$runs")" = success ] \
        || die "CI run $id on ${sha:0:8} did not pass"
      # The run must have checked the records with the tree's binary, not
      # skipped the job that does.
      $GH run view "$id" -R "$GH_REPO" --json jobs \
        | jq -e '[.jobs[] | select(.name == "test") | .conclusion == "success" and ([.steps[] | select(.name == "atelier check") | .conclusion] == ["success"])] == [true]' >/dev/null \
        || die "CI run $id on ${sha:0:8} passed without its test job's atelier check step passing"
      say "   CI run $id passed on ${sha:0:8}, its atelier check step included"
      return 0
    fi
    if [ "$(date +%s)" -ge "$end" ]; then
      say "CI on ${sha:0:8} has not finished after ${CI_WAIT} s. Nothing failed: run cutover.sh again to keep waiting."
      exit 3
    fi
    sleep 20
  done
}

# Step 6 for one repository: its real clone onto the converted base.
bring_onto_base() { # <name>
  local r=$1 c b new old bw M head k
  c=$(clone_of "$r") b=$(base_of "$r")
  git -C "$c" fetch -q "$REMOTE"
  new=$(git -C "$c" rev-parse "refs/remotes/$REMOTE/$b")
  [ "$new" = "$(remote_tip "$r")" ] || die "$r: the fetched $b is not the remote's tip"
  bw=$(base_worktree "$r")
  k=ref:$r
  old=$(jfields "pre:base:$r")
  if [ "$(jstate "$k")" != done ]; then
    jstart "$k" "$old	$new	${bw:-no worktree}"
    if [ -n "$bw" ]; then
      [ "$(git -C "$bw" rev-parse HEAD)" = "$new" ] || git -C "$bw" merge -q --ff-only "$new"
    else
      git -C "$c" update-ref "refs/heads/$b" "$new"
    fi
    jdone "$k"
  fi
  M=$c
  head=$(worktrees "$c" | awk -v m="$M" '$1 == m { print $2 }')
  k=checkout:$r:$M
  if [ "$head" != "$b" ] && [ "$(jstate "$k")" != done ]; then
    jstart "$k" "$head"
    if [ -n "$bw" ] && [ "$bw" != "$M" ]; then
      git -C "$M" checkout -q --detach "refs/heads/$b"
    else
      git -C "$M" switch -q "$b"
    fi
    jdone "$k"
  fi
  if [ "$(jstate "setup:$r")" != done ]; then
    jstart "setup:$r"
    (cd "$M" && atelier setup)
    jdone "setup:$r"
  fi
}
old_layout_worktrees() { # <name>
  local wt head
  while read -r wt head; do
    git -C "$wt" cat-file -e HEAD:.atelier/issues 2>/dev/null || continue
    say "   $1: $wt ($head) is on an old-layout branch: atelier and the hooks refuse there until it takes the converted base (README.md)"
  done < <(worktrees "$(clone_of "$1")")
}

# Step 7 for one repository's main and base worktrees.
accept_repo() { # <name> <compare counts: yes|no>
  local r=$1 c b wt out items missions want
  c=$(clone_of "$r") b=$(base_of "$r")
  for wt in $( (echo "$c"; base_worktree "$r") | sort -u); do
    out=$(cd "$wt" && atelier check) || die "$r: atelier check fails in $wt"
    (cd "$wt" && atelier status > /dev/null) || die "$r: atelier status fails in $wt"
    items=$(sed -n 's/.*valid: \([0-9]*\) items\{0,1\}, \([0-9]*\) missions\{0,1\}.*/\1 \2/p' <<< "$out" | head -1)
    [ -n "$items" ] || die "$r: atelier check in $wt printed no count of items and missions"
    if [ "$2" = yes ]; then
      want=$(cat "$STATE/report/$r.counts")
      [ "$items" = "$want" ] || die "$r: $wt has $items items and missions; the conversion printed $want"
    else
      echo "$items" > "$STATE/report/$r.counts"
    fi
  done
  [ "$(git -C "$c" rev-parse "refs/heads/$b")" = "$(remote_tip "$r")" ] || die "$r: $b is not at the remote's tip"
  say "   $r: check and status pass in $(echo $( (echo "$c"; base_worktree "$r") | sort -u)); items and missions: $items; $b at the remote's tip"
}
accept_host() { # <skill commit or empty> <tree holding the new skill>
  local want
  want=$("${TREE_BIN:-atelier}" --version)
  [ "$(atelier --version)" = "$want" ] || die "the installed atelier is \"$(atelier --version)\", not \"$want\""
  if [ -n "$1" ]; then
    git -C "$(clone_of atelier)" show "$1:skills/agent-factory/SKILL.md" | cmp -s - "$SKILL/skills/agent-factory/SKILL.md" \
      || die "the skill checkout's skills/agent-factory/SKILL.md is not the new text"
  fi
  if [ -n "$SKILL_DIR" ]; then
    cmp -s "$2/skills/agent-factory/SKILL.md" "$SKILL_DIR/SKILL.md" || die "$SKILL_DIR/SKILL.md is not the new text"
  fi
  case $(service_state) in *" active"|enabled*) die "$SERVICE is still enabled or running: $(service_state)" ;; esac
  say "   atelier --version: $want; the skill is the new text; $SERVICE: $(service_state)"
}
stop_service() {
  [ "$(jstate service)" != done ] || return 0
  local was; was=$(service_state)
  jstart service "$was"
  case $was in not-found*|" "*|"") say "   $SERVICE is not on this host" ;; *) $SYSTEMCTL disable --now "$SERVICE" >/dev/null 2>&1 || $SYSTEMCTL disable --now "$SERVICE" ;; esac
  jdone service
}

final_report() {
  local r c wt head
  say ""
  say "Cutover report"
  for r in $(repo_names); do
    c=$(clone_of "$r")
    say "$r: conversion $(jresult "commit:$r" | cut -c1-12); items and missions: $(cat "$STATE/report/$r.counts" 2>/dev/null); --verify: $(cat "$STATE/report/$r.verify" 2>/dev/null); relative links resolving: $(cat "$STATE/report/$r.links" 2>/dev/null)$( [ -f "$STATE/report/$r.sections" ] && echo "; AGENTS.md: $(cat "$STATE/report/$r.sections")")"
    while read -r wt head; do say "    $wt: $head"; done < <(worktrees "$c")
  done
  say "Step times:"
  awk -F'\t' '{ printf "  step %s %-40s %5d s\n", $1, $2, $3 }' "$STATE/times"
}

cutover() {
  mkdir -p "$STATE/report"
  local W B conv r c d b new verify tip
  W=$(base_worktree atelier)
  [ -n "$W" ] || die "no worktree of atelier has $(base_of atelier) checked out"
  [ "$(git -C "$here" rev-parse --show-toplevel)" = "$W" ] || die "run this from $W, the worktree that has $(base_of atelier)"
  [ "$(jstate mode)" != done ] || [ "$(jfields mode)" = cutover ] || die "the journal in $STATE is an adopt's; give another CUTOVER_STATE"
  export TREE_BIN=$W/target/release/atelier

  step 1 "Preflight"
  if [ "$(jstate mode)" = done ]; then
    say "   recorded already; resuming from the journal"
  else
    switch_merged || die "the switch ($SWITCH_ITEM) has not merged into atelier's $(base_of atelier)"
    preflight_checks cutover || die "the preflight found the problems above"
    record cutover
  fi
  point step1-done

  step 2 "Convert atelier"
  B=$(jfields pre:base:atelier)
  convert_atelier "$W" "$B"
  conv=$(jresult commit:atelier)
  point step2-committed
  (cd "$W" && python3 "$here/convert_records.py" --verify "$B") > "$STATE/report/atelier.verify" \
    || { cat "$STATE/report/atelier.verify"; die "atelier: --verify fails on the conversion"; }
  (cd "$W" && "$TREE_BIN" check) || die "atelier: the tree's atelier check fails"
  links_resolving "$W" > "$STATE/report/atelier.links"

  step 3 "Install"
  if [ "$(jstate install)" != done ]; then
    jstart install "$INSTALL_ROOT"
    (cd "$W" && eval "$INSTALL")
    jdone install
  fi
  [ "$(atelier --version)" = "$("$TREE_BIN" --version)" ] || die "atelier on PATH is not the binary just installed"
  point step3-installed
  if [ "$(jstate setup:atelier)" != done ]; then
    jstart setup:atelier
    (cd "$W" && atelier setup && atelier setup claude && atelier setup codex)
    [ -z "$(git -C "$W" status --porcelain)" ] || die "atelier: setup changed files the conversion commit should hold"
    jdone setup:atelier
  fi
  if git -C "$SKILL" rev-parse -q --verify HEAD >/dev/null 2>&1 && [ "$(jstate skill)" != done ]; then
    jstart skill "$(jfields pre:skill)	$conv"
    git -C "$SKILL" checkout -q --detach "$conv"
    jdone skill
  fi
  stop_service
  point step3-done

  step 4 "Push atelier's $(base_of atelier)"
  push_base push:atelier atelier "$W" "$B" "$(git -C "$W" rev-parse HEAD)"
  point step4-done

  step 5 "Convert the product repositories"
  for r in $(repo_names); do
    [ "$r" != atelier ] || continue
    c=$(clone_of "$r") b=$(base_of "$r") B=$(jfields "pre:base:$r") d=$STATE/convert/$r
    if [ "$(jstate "convert:$r")" != done ]; then
      jstart "convert:$r" "$d	$B"
      rm -rf "$d"
      git clone -q -b "$b" "$c" "$d"
      git -C "$d" remote set-url "$REMOTE" "$(git -C "$c" config "remote.$REMOTE.url")"
      if git -C "$c" config "remote.$REMOTE.pushurl" >/dev/null; then
        git -C "$d" config "remote.$REMOTE.pushurl" "$(git -C "$c" config "remote.$REMOTE.pushurl")"
      fi
      [ "$(git -C "$d" rev-parse HEAD)" = "$B" ] || die "$r: the fresh clone of $b is not at the recorded base"
      (cd "$d" && convert_here "$r" "$B" atelier)
      jdone "convert:$r"
    fi
    commit_conversion "$r" "$d" "$B"
    (cd "$d" && python3 "$here/convert_records.py" --verify "$B") > "$STATE/report/$r.verify" \
      || { cat "$STATE/report/$r.verify"; die "$r: --verify fails on the conversion"; }
    (cd "$d" && atelier check) || die "$r: atelier check fails on the conversion"
    links_resolving "$d" > "$STATE/report/$r.links"
    say "   $r: converted ($(cat "$STATE/report/$r.sections")); $(cat "$STATE/report/$r.verify" | tail -1)"
    point "step5-$r-committed"
    push_base "push:$r" "$r" "$d" "$B" "$(git -C "$d" rev-parse HEAD)"
    point "step5-$r-pushed"
  done

  step 6 "Bring each clone onto the converted base"
  for r in $(repo_names); do
    bring_onto_base "$r"
    point "step6-$r-moved"
  done
  for r in $(repo_names); do old_layout_worktrees "$r"; done

  step 7 "Accept"
  for r in $(repo_names); do accept_repo "$r" yes; done
  accept_host "$conv" "$W"
  wait_for_ci "$conv"
  jstart accept; jdone accept

  step 8 "Close $CUTOVER_ITEM"
  local parent k=commit:close
  if [ "$(jstate "$k")" = started ]; then
    parent=$(jfields "$k")
    if [ "$(git -C "$W" rev-parse HEAD)" != "$parent" ] && [ "$(git -C "$W" rev-parse HEAD^)" = "$parent" ]; then
      jdone "$k" "$(git -C "$W" rev-parse HEAD)"
    fi
  fi
  if [ "$(jstate "$k")" != done ]; then
    parent=$(git -C "$W" rev-parse HEAD)
    jstart "$k" "$parent"
    (cd "$W" && atelier close "$CUTOVER_ITEM" "The cutover ran on this host with scripts/cutover/cutover.sh; its report is in the mission's Result.")
    [ "$(git -C "$W" rev-parse HEAD^)" = "$parent" ] || die "atelier close made no commit on $(base_of atelier)"
    jdone "$k" "$(git -C "$W" rev-parse HEAD)"
  fi
  point step8-committed
  push_base push:close atelier "$W" "$(jfields "$k")" "$(jresult "$k")"
  point step8-pushed
  finish_step; STEP=""
  rm -rf "$STATE/convert"
  final_report
}

adopt() {
  mkdir -p "$STATE/report"
  local src r
  src=$(git -C "$here" rev-parse --show-toplevel)
  [ ! -d "$src/.atelier/issues" ] && [ -f "$src/.atelier/config.toml" ] || die "run adopt from a clone of atelier's converted $(base_of atelier)"
  [ "$(git -C "$src" rev-parse HEAD)" = "$(git -C "$src" ls-remote "$REMOTE" "refs/heads/$(base_of atelier)" | cut -f1)" ] \
    || die "$src is not at the remote's $(base_of atelier)"
  [ "$(jstate mode)" != done ] || [ "$(jfields mode)" = adopt ] || die "the journal in $STATE is a cutover's; give another CUTOVER_STATE"
  export TREE_BIN=atelier

  step 1 "Preflight"
  if [ "$(jstate mode)" = done ]; then
    say "   recorded already; resuming from the journal"
  else
    preflight_checks adopt || die "the preflight found the problems above"
    record adopt
  fi
  point adopt-preflight-done

  step 2 "Install"
  if [ "$(jstate install)" != done ]; then
    jstart install "$INSTALL_ROOT"
    (cd "$src" && eval "$INSTALL")
    jdone install
  fi
  case $(atelier --version) in "atelier 0.4"*) die "atelier on PATH is still the old tool" ;; esac
  point adopt-installed
  if [ -n "$SKILL_INSTALL" ]; then
    if [ "$(jstate skill-dir)" != done ]; then jstart skill-dir "$SKILL_DIR"; (cd "$src" && eval "$SKILL_INSTALL"); jdone skill-dir; fi
  elif git -C "$SKILL" rev-parse -q --verify HEAD >/dev/null 2>&1 && [ "$(jstate skill)" != done ]; then
    jstart skill "$(jfields pre:skill)	$(git -C "$src" rev-parse HEAD)"
    git -C "$SKILL" fetch -q "$REMOTE"
    git -C "$SKILL" checkout -q --detach "$(git -C "$src" rev-parse HEAD)"
    jdone skill
  fi
  stop_service

  step 3 "Bring each clone onto the converted base"
  for r in $(repo_names); do
    bring_onto_base "$r"
    point "adopt-$r-moved"
  done
  for r in $(repo_names); do old_layout_worktrees "$r"; done

  step 4 "Accept"
  for r in $(repo_names); do accept_repo "$r" no; done
  accept_host "$( [ "$(jstate skill)" != done ] || git -C "$src" rev-parse HEAD)" "$src"
  jstart accept; jdone accept
  finish_step; STEP=""
  say ""
  say "Adopt report"
  for r in $(repo_names); do
    say "$r: $(base_of "$r") at $(git -C "$(clone_of "$r")" rev-parse --short "refs/heads/$(base_of "$r")"); items and missions by atelier check: $(cat "$STATE/report/$r.counts")"
    worktrees "$(clone_of "$r")" | sed 's/^/    /'
  done
  say "Step times:"
  awk -F'\t' '{ printf "  step %s %-40s %5d s\n", $1, $2, $3 }' "$STATE/times"
}

main() {
  mkdir -p "$STATE"
  case ${1:-} in
    preflight)
      if preflight_checks "${2:-cutover}"; then say "Preflight passed: every repository is quiet."; else die "the preflight found the problems above"; fi ;;
    adopt) adopt ;;
    "") cutover ;;
    *) die "usage: cutover.sh [preflight | adopt]" ;;
  esac
}
main "$@"
exit
