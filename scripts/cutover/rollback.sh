#!/usr/bin/env bash
# Undoes what cutover.sh's journal says was started, cutover or adopt, in
# this order (docs/strategies/redesign-architecture.md, Transition):
#
#   1. Every hooks directory goes back from its copy.
#   2. For each repository the runbook converted, the remote's base tip
#      decides: still at the recorded base, nothing was published, so the
#      conversion is reset away; holding the runbook's commits, each is
#      reverted, newest first, and the reverts are pushed; anything else
#      stops that repository, untouched.
#   3. Each real clone's base goes where its remote now is (an adopt's goes
#      back to the recorded commit), and each worktree back to what it had
#      checked out.
#   4. If the install had started: the fallback release is reinstalled from
#      its pinned archive, its setup run, the skill moved back, and the old
#      service enabled, started and checked to answer.
#
# It is safe to run twice. Settings come from common.sh, as for cutover.sh.
#
# Each repository's steps 2 and 3 run as `( set -e; ... )` in their own
# subshell, never in a condition, so that any command failing in them stops
# that repository with its error said and its recovery clone kept. If any
# repository is not restored, step 4 installs nothing and the exit status is
# 1: run it again once the cause is fixed.
set -euo pipefail
source "$(dirname "$0")/common.sh"
[ -f "$JOURNAL" ] || die "no journal at $JOURNAL: nothing to roll back"
MODE=$(jfields mode)
REVERT_TRAILER="Cutover-Revert"
failed=""

# The runbook's commits in a repository, by what the journal recorded and,
# for a commit started and not marked done, what the directory it was made
# in holds.
runbook_commits() { # <name>
  local k f d parent head
  for k in "commit:$1" $( [ "$1" = atelier ] && echo commit:close); do
    case $(jstate "$k") in
      done) jresult "$k" ;;
      started)
        f=$(jfields "$k")
        if [ "$k" = commit:close ]; then d=$(base_worktree atelier) parent=$f; else d=${f%%	*} parent=${f#*	}; fi
        [ -d "$d" ] || continue
        head=$(git -C "$d" rev-parse HEAD)
        [ "$head" = "$parent" ] || [ "$(git -C "$d" rev-parse HEAD^)" != "$parent" ] || echo "$head" ;;
    esac
  done
}

# Step 2 for one repository. Prints the tip its clones go to.
undo_conversion() { # <name>
  local r=$1 c b B T rb known sha subj list reverted=0 todo=""
  c=$(clone_of "$r") b=$(base_of "$r") B=$(jfields "pre:base:$r")
  T=$(remote_tip "$r") || { say "$r: cannot read the remote's $b; left as it is" >&2; return 1; }
  known=$(runbook_commits "$r")
  if [ "$T" = "$B" ]; then
    if [ "$r" = atelier ]; then
      local W; W=$(jfields convert:atelier | cut -f1)
      git -C "$W" reset -q --hard "$B"
      # shellcheck disable=SC2086 # CONVERTED_PATHS is a list of paths
      git -C "$W" clean -fdq -- $CONVERTED_PATHS
    else
      rm -rf "$STATE/convert/$r"
    fi
    say "$r: nothing was published; the conversion is reset away" >&2
    echo "$B"; return 0
  fi
  rb=$STATE/rollback/$r
  rm -rf "$rb"
  git clone -q --shared -n "$c" "$rb"
  git -C "$rb" remote set-url origin "$(git -C "$c" config "remote.$REMOTE.url")"
  if git -C "$c" config "remote.$REMOTE.pushurl" >/dev/null; then
    git -C "$rb" config remote.origin.pushurl "$(git -C "$c" config "remote.$REMOTE.pushurl")"
  fi
  git -C "$rb" fetch -q origin "+refs/heads/$b:refs/remotes/origin/$b"
  git -C "$rb" checkout -q -B "$b" "refs/remotes/origin/$b"
  if ! git -C "$rb" merge-base --is-ancestor "$B" "$T"; then
    say "$r: the remote's $b (${T:0:8}) does not hold the recorded base ${B:0:8}; left as it is" >&2; return 1
  fi
  list=$(git -C "$rb" rev-list --first-parent "$B..$T")
  for sha in $list; do
    if grep -qx "$sha" <<< "$known"; then
      # A failed log reads as "not reverted": the commit is reverted again and
      # the tree check below then refuses to push.
      if git -C "$rb" log --format=%B "$B..$T" | grep -qx "$REVERT_TRAILER: $sha"; then reverted=$((reverted + 1)); else todo="$todo $sha"; fi
    # A failed log reads as "not the runbook's", which stops this repository.
    elif ! git -C "$rb" log -1 --format=%B "$sha" | grep -q "^$REVERT_TRAILER: "; then
      say "$r: the remote's $b holds ${sha:0:8} \"$(git -C "$rb" log -1 --format=%s "$sha")\", which the runbook did not make; left as it is" >&2
      return 1
    fi
  done
  for sha in $todo; do # rev-list lists newest first
    subj=$(git -C "$rb" log -1 --format=%s "$sha")
    git -C "$rb" revert --no-commit "$sha"
    git -C "$rb" commit -q -m "Undo the cutover's commit ${sha:0:8}: $subj" -m "$REVERT_TRAILER: $sha"
  done
  [ "$(git -C "$rb" rev-parse "HEAD^{tree}")" = "$(git -C "$rb" rev-parse "$B^{tree}")" ] \
    || { say "$r: the reverts do not give back the recorded tree; nothing pushed" >&2; return 1; }
  local mine
  mine=$(git -C "$rb" rev-parse HEAD)
  if [ -n "$todo" ]; then
    if ! git -C "$rb" push -q origin "refs/heads/$b"; then
      # A push can be accepted and still report failure: the remote decides.
      [ "$(remote_tip "$r")" = "$mine" ] \
        || { say "$r: the push of the reverts failed; nothing is reset, and the reverts are kept in $rb" >&2; return 1; }
    fi
    say "$r: reverted $(wc -w <<< "$todo") published commits and pushed the reverts" >&2
  else
    say "$r: every published commit is reverted already ($reverted)" >&2
  fi
  [ "$(remote_tip "$r")" = "$mine" ] || { say "$r: the remote's $b is not the reverts' tip; the reverts are kept in $rb" >&2; return 1; }
  rm -rf "$STATE/convert/$r"
  echo "$mine"
  rm -rf "$rb"
}

# Step 3 for one repository: its base to <target>, each worktree back.
restore_clone() { # <name> <target>
  local r=$1 target=$2 c b bw head known moved k wt was now
  c=$(clone_of "$r") b=$(base_of "$r")
  if [ "$MODE" = adopt ]; then
    local tracking; tracking=$(jfields "pre:tracking:$r")
    [ "$tracking" = none ] || git -C "$c" update-ref "refs/remotes/$REMOTE/$b" "$tracking"
  else
    git -C "$c" fetch -q "$REMOTE"
  fi
  known=$(runbook_commits "$r")
  moved=$(jfields "ref:$r" | cut -f2)
  bw=$(base_worktree "$r")
  head=$(git -C "$c" rev-parse "refs/heads/$b")
  if [ "$head" != "$target" ]; then
    if git -C "$c" merge-base --is-ancestor "$head" "$target"; then
      if [ -n "$bw" ]; then git -C "$bw" merge -q --ff-only "$target"; else git -C "$c" update-ref "refs/heads/$b" "$target"; fi
    else
      # Only commits the runbook made, or the base it moved to, are left behind.
      local extra; extra=$(git -C "$c" rev-list "$target..$head")
      while read -r sha; do
        [ -z "$sha" ] || grep -qx "$sha" <<< "$known" || [ "$head" = "$moved" ] \
          || { say "$r: $b holds ${sha:0:8}, which the runbook did not make; left as it is"; return 1; }
      done <<< "$extra"
      if [ -n "$bw" ]; then
        local dirty; dirty=$(git -C "$bw" status --porcelain)
        [ -z "$dirty" ] || { say "$r: $bw has uncommitted changes; left as it is"; return 1; }
        git -C "$bw" reset -q --hard "$target"
      else
        git -C "$c" update-ref "refs/heads/$b" "$target"
      fi
    fi
  fi
  local keys
  keys=$(jkeys "pre:wt:$r:")
  for k in $keys; do
    wt=${k#"pre:wt:$r:"} was=$(jfields "$k")
    # A worktree removed since the preflight has nothing to put back.
    [ -d "$wt" ] || continue
    now=$(worktrees "$c" | awk -v w="$wt" '$1 == w { print $2 }')
    [ "$now" != "$was" ] || continue
    case $was in
      detached:*) git -C "$wt" checkout -q --detach "${was#detached:}" ;;
      *) git -C "$wt" switch -q "$was" ;;
    esac
  done
  # The ignored paths the conversion set aside go back whole, each as it
  # was saved (files, links, empty directories, modes). One already back is
  # skipped; a destination in the way stops this repository, with the
  # saved copy kept.
  local list=$STATE/aside/atelier.list
  if [ "$r" = atelier ] && [ -f "$list" ]; then
    local W src p; W=$(jfields convert:atelier | cut -f1)
    while IFS= read -r p; do
      [ -n "$p" ] || continue
      src=$STATE/aside/atelier/$p
      [ -e "$src" ] || [ -L "$src" ] || continue
      if [ -e "$W/$p" ] || [ -L "$W/$p" ]; then
        say "$r: $W/$p is in the way of the ignored path the cutover set aside, kept at $src; move it aside, then run rollback.sh again"
        return 1
      fi
      mkdir -p "$(dirname "$W/$p")"
      mv "$src" "$W/$p"
    done < "$list"
  fi
  # What was asked for, read back.
  [ "$(git -C "$c" rev-parse "refs/heads/$b")" = "$target" ] || { say "$r: $b is not at ${target:0:8} after restoring"; return 1; }
  for k in $keys; do
    wt=${k#"pre:wt:$r:"} was=$(jfields "$k")
    [ -d "$wt" ] || continue
    now=$(worktrees "$c" | awk -v w="$wt" '$1 == w { print $2 }')
    [ "$now" = "$was" ] || { say "$r: $wt has $now checked out, not $was"; return 1; }
  done
  say "$r: $b at $(git -C "$c" rev-parse --short "refs/heads/$b"); worktrees as recorded"
}

say "Rolling back the $MODE recorded in $JOURNAL"

say "1. Hooks directories"
keys=$(jkeys pre:hooks:)
for k in $keys; do
  # A copy the preflight did not finish is no copy; that hooks directory was
  # not changed after it, since nothing runs past an unfinished preflight.
  [ "$(jstate "$k")" = done ] || continue
  f=$(jfields "$k") h=${f%%	*} copy=${f#*	}
  rm -rf "$h"
  [ "$copy" = absent ] || cp -a "$copy" "$h"
  say "${k#pre:hooks:}: $h back from its copy"
done

say "2. Published commits"
declare -A target
for r in $(repo_names); do
  B=$(jfields "pre:base:$r")
  [ -n "$B" ] || continue
  if [ "$MODE" = adopt ]; then target[$r]=$B; continue; fi
  if [ "$(jstate "convert:$r")" = none ] && [ "$(jstate "commit:$r")" = none ]; then
    target[$r]=$(remote_tip "$r")
    [ -n "${target[$r]}" ] || die "$r: cannot read the remote's $(base_of "$r")"
    continue
  fi
  set +e
  ( set -e; undo_conversion "$r" > "$STATE/rollback-$r.target" )
  rc=$?
  set -e
  if [ $rc -eq 0 ]; then
    target[$r]=$(cat "$STATE/rollback-$r.target")
  else
    say "$r: not rolled back: its step 2 stopped (exit $rc), as said above"
    failed="$failed $r"
  fi
done

say "3. Clones and worktrees"
for r in $(repo_names); do
  [ -n "${target[$r]:-}" ] || continue
  set +e
  ( set -e; restore_clone "$r" "${target[$r]}" )
  rc=$?
  set -e
  [ $rc -eq 0 ] || { say "$r: not restored: its step 3 stopped (exit $rc), as said above"; failed="$failed $r"; }
done

if [ -n "$failed" ]; then
  say "Not rolled back:$failed. Each is said above; nothing more was changed there, and the old tool was not installed,"
  say "since it cannot read a repository left converted. Fix the cause and run rollback.sh again."
  exit 1
fi

if [ "$(jstate install)" != none ] || [ "$(jstate skill)" != none ] || [ "$(jstate skill-dir)" != none ] || [ "$(jstate service)" != none ]; then
  say "4. The old tool"
  if [ "$(jstate install)" != none ]; then
    fb=$STATE/fallback
    rm -rf "$fb"; mkdir -p "$fb"
    curl --proto '=https,file' -fsSL -o "$fb/atelier.tar.xz" "$FALLBACK_URL"
    echo "$FALLBACK_SHA256  $fb/atelier.tar.xz" | sha256sum -c --quiet -
    tar -xJf "$fb/atelier.tar.xz" -C "$fb"
    mkdir -p "$INSTALL_ROOT/bin"
    install -m 755 "$fb"/atelier-cli-*/atelier "$INSTALL_ROOT/bin/atelier"
    say "installed $("$INSTALL_ROOT/bin/atelier" --version) into $INSTALL_ROOT/bin, its archive's SHA-256 checked"
    # Its setup: the harness hooks of each repository that uses them, which
    # the reverts gave back, so nothing may change.
    for r in $(repo_names); do
      c=$(clone_of "$r")
      # No settings file, or none of the old setup's hooks: nothing to set up.
      grep -qs atelier-managed "$c/.claude/settings.json" || continue
      (cd "$c"; "$INSTALL_ROOT/bin/atelier" setup claude > /dev/null)
      changed=$(git -C "$c" status --porcelain -- .claude)
      [ -z "$changed" ] || { say "$r: the old setup claude changed .claude/; left for a look"; failed="$failed $r"; }
    done
  fi
  if [ "$(jstate skill)" != none ]; then
    git -C "$SKILL" checkout -q --detach "$(jfields pre:skill)"
    say "the skill checkout is back at $(git -C "$SKILL" rev-parse --short HEAD)"
  fi
  if [ "$(jstate skill-dir)" != none ] && [ "$(jstate pre:skill-dir)" = done ]; then
    f=$(jfields pre:skill-dir)
    rm -rf "$SKILL_DIR"
    case $f in *absent) ;; *) cp -a "$STATE/skill-dir" "$SKILL_DIR" ;; esac
    say "$SKILL_DIR is back from its copy"
  fi
  if [ "$(jstate service)" != none ]; then
    was=$(jfields pre:service)
    case $was in enabled*) $SYSTEMCTL enable "$SERVICE" > /dev/null ;; esac
    case $was in *" active") $SYSTEMCTL start "$SERVICE" ;; esac
    case $was in
      *" active")
        ok=no
        # Asked until it answers, for 20 s; no answer is a failure, said below.
        for _ in $(seq 20); do if eval "$SERVICE_CHECK" >/dev/null 2>&1; then ok=yes; break; fi; sleep 1; done
        [ "$ok" = yes ] || { say "$SERVICE does not answer"; failed="$failed service"; }
        say "$SERVICE: $($SYSTEMCTL is-enabled "$SERVICE" 2>/dev/null) $($SYSTEMCTL is-active "$SERVICE" 2>/dev/null), answering: $ok" ;;
      *) say "$SERVICE was not running before; left stopped" ;;
    esac
  fi
fi

if [ -n "$failed" ]; then
  say "Rolled back, except:$failed. Each is said above; nothing was reset there."
  exit 1
fi
say "Rolled back."
