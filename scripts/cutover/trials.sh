#!/usr/bin/env bash
# Tries cutover.sh and rollback.sh in sandboxes, at every point the cutover
# can stop, and runs the full cutover, the adopting host and the preflight's
# cases:
#
#   scripts/cutover/trials.sh <work directory> <new binary> [trial...]
#
# <new binary> is the release build of this tree (cargo install --root).
# Each trial builds its own sandbox under <work directory>/<trial>/ from the
# five real repositories, which it only reads: clones whose remotes are local
# bare repositories, Git limited to local files, a scratch install root
# holding the fallback release, a skill checkout, and doubles for
# `systemctl` (which runs the old `atelier serve`), `gh` and T3's database.
# Atelier's master in the sandbox is the real master with this branch
# merged into it, as the switch's merge will leave it.
#
# It prints one line per trial and checks "restored" the same way after
# every rollback. Logs and sandboxes hold the private repositories' records:
# read them there, and delete the work directory afterwards. A fingerprint
# of what a cutover would change on the real host (each repository's hooks
# and remote config, the installed binary, the skill checkout and its links,
# and the service) is taken before and after, and must match.
set -euo pipefail
[ $# -ge 2 ] || { echo "usage: $0 <work directory> <new binary> [trial...]" >&2; exit 2; }
mkdir -p "$1"
W=$(cd "$1" && pwd) NEW=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
shift 2
KIT=$(cd "$(dirname "$0")" && pwd)
TREE=$(git -C "$KIT" rev-parse --show-toplevel)
REAL=${TRIAL_REAL_PROJECTS:-/home/supadava/projects}
REAL_SKILL=$REAL/atelier-skill
SPECS="atelier:master astromech:main demiurge:master life-on-rails:main cosplay-studio:main"
FALLBACK_URL=https://github.com/spadaval/atelier/releases/download/v0.4.0/atelier-cli-x86_64-unknown-linux-gnu.tar.xz
FALLBACK_SHA256=d13f5e4df5c3382715a1247f1ede175d5e4361309e63a5a36e96ab55b1ce20d7
FALLBACK_BIN_SHA256=f34ae285b45f38dfcb459d61ff4da05c3c734275e6c99280caf4d868fd986893
SQLITE_DIR=$(dirname "$(command -v sqlite3)")
case $W in "$HOME"/.cache/*) ;; *) echo "the work directory must be under $HOME/.cache" >&2; exit 2 ;; esac
[ -x "$NEW" ] && "$NEW" --version | grep -q '^atelier 0\.5' || { echo "$NEW is not the new binary" >&2; exit 2; }

say() { printf '%s\n' "$*"; }

# ---------------------------------------------------------------- the real host, read only

fingerprint() {
  local s r c
  for s in $SPECS; do
    r=${s%%:*} c=$REAL/$r
    # Not branches or worktree heads, which other threads move while the
    # trials run: only what a cutover would change.
    echo "$r remote $(git -C "$c" config --get-regexp '^(remote|url)\.' | tr '\n' ' ')"
    echo "hooks $(cd "$(git -C "$c" rev-parse --path-format=absolute --git-common-dir)/hooks" && ls -A | sort | tr '\n' ' ')"
    echo "hooksPath $(git -C "$c" config core.hooksPath || true)"
  done
  echo "binary $(sha256sum "$HOME/.cargo/bin/atelier" | cut -d' ' -f1)"
  echo "skill $(git -C "$REAL_SKILL" rev-parse HEAD)"
  echo "links $(readlink "$HOME/.claude/skills/agent-factory") $(readlink "$HOME/.agents/skills/agent-factory")"
  echo "service $(systemctl --user is-enabled atelier-serve) $(systemctl --user is-active atelier-serve) $(systemctl --user show -p MainPID --value atelier-serve)"
}

# ---------------------------------------------------------------- shared inputs

prepare() {
  local s r b url tmp base
  mkdir -p "$W/cache" "$W/tmp"
  export TMPDIR=$W/tmp
  if [ ! -f "$W/cache/fallback.tar.xz" ]; then
    curl --proto '=https' --tlsv1.2 -fsSL -o "$W/cache/fallback.tar.xz" "$FALLBACK_URL"
  fi
  echo "$FALLBACK_SHA256  $W/cache/fallback.tar.xz" | sha256sum -c --quiet -
  rm -rf "$W/cache/fallback"; mkdir -p "$W/cache/fallback"
  tar -xJf "$W/cache/fallback.tar.xz" -C "$W/cache/fallback"
  OLD=$(ls "$W"/cache/fallback/*/atelier)
  [ "$(sha256sum "$OLD" | cut -d' ' -f1)" = "$FALLBACK_BIN_SHA256" ]
  # Bare remotes: each real base as it stands, and atelier's in two forms,
  # before the switch (the real master) and after it (this branch merged).
  rm -rf "$W/cache/remotes" "$W/cache/urls"; mkdir -p "$W/cache/remotes"
  for s in $SPECS; do
    r=${s%%:*} b=${s#*:}
    url=$(git -C "$REAL/$r" config remote.origin.url)
    echo "$r $url" >> "$W/cache/urls"
    git clone -q --bare --single-branch -b "$b" "$REAL/$r" "$W/cache/remotes/$r.git"
  done
  # Atelier's master is the one this branch last took from master (its
  # merge base), so that the switch merges as it will once the branch is
  # brought up to date; master may hold newer work the branch has not
  # taken yet.
  base=$(git -C "$TREE" merge-base HEAD "$(git -C "$REAL/atelier" rev-parse master)")
  git --git-dir="$W/cache/remotes/atelier.git" update-ref refs/heads/master "$base"
  # The day's condition: no item in progress on any base. An item a real
  # base holds in progress today goes back to todo, in the sandbox only.
  for s in $SPECS; do
    r=${s%%:*} b=${s#*:} tmp=$W/tmp/quiet-$r
    rm -rf "$tmp"; git clone -q -b "$b" "$W/cache/remotes/$r.git" "$tmp"
    if grep -lq '^status: "in_progress"$' "$tmp"/.atelier/issues/*.md 2>/dev/null; then
      say "prepared: $r's base holds $(grep -l '^status: "in_progress"$' "$tmp"/.atelier/issues/*.md | wc -l) item(s) in progress today; the sandbox sets them back to todo"
      sed -i 's/^status: "in_progress"$/status: "todo"/' "$tmp"/.atelier/issues/*.md
      git -C "$tmp" -c user.name=trials -c user.email=trials@example.invalid commit -q -am "Nothing in progress, as on the day of the cutover"
      git -C "$tmp" push -q origin "$b"
    fi
    rm -rf "$tmp"
  done
  git clone -q --bare --single-branch -b master "$W/cache/remotes/atelier.git" "$W/cache/remotes/atelier-pre.git"
  tmp=$W/tmp/switch
  rm -rf "$tmp"
  git clone -q -b master "$W/cache/remotes/atelier.git" "$tmp"
  git -C "$tmp" -c user.name=trials -c user.email=trials@example.invalid fetch -q "$TREE" "HEAD:refs/heads/atelier-u1ko"
  git -C "$tmp" -c user.name=trials -c user.email=trials@example.invalid merge -q --no-ff atelier-u1ko -m "Merge atelier-u1ko, as the switch's merge will"
  sed -i 's/^status: "in_progress"$/status: "done"/' "$tmp/.atelier/issues/atelier-u1ko.md"
  git -C "$tmp" -c user.name=trials -c user.email=trials@example.invalid commit -q -a --amend --no-edit
  git -C "$tmp" push -q "$W/cache/remotes/atelier.git" master
  rm -rf "$tmp"
  say "prepared: fallback v0.4.0 checked; remotes for the five repositories; atelier's master with the switch merged at $(git -C "$W/cache/remotes/atelier.git" rev-parse --short master)"
}

# ---------------------------------------------------------------- one sandbox

# sandbox <dir> <port> [pre]: the five repositories as they stand on this
# host, in what matters. With `pre`, atelier is before the switch.
sandbox() {
  local S=$1 port=$2 form=${3:-post} s r b c url
  rm -rf "$S"; mkdir -p "$S"/{remotes,projects,worktrees,bin,root/bin,home,state,gh,service,tmp}
  cp -r "$W/cache/remotes/." "$S/remotes/"
  [ "$form" = post ] || { rm -rf "$S/remotes/atelier.git"; mv "$S/remotes/atelier-pre.git" "$S/remotes/atelier.git"; }
  rm -rf "$S/remotes/atelier-pre.git"
  : > "$S/gitconfig"
  git config -f "$S/gitconfig" user.name sandbox
  git config -f "$S/gitconfig" user.email sandbox@example.invalid
  git config -f "$S/gitconfig" advice.detachedHead false
  git config -f "$S/gitconfig" init.defaultBranch main
  while read -r r url; do git config -f "$S/gitconfig" --add "url.$S/remotes/$r.git.insteadOf" "$url"; done < "$W/cache/urls"
  install -m 755 "$OLD" "$S/root/bin/atelier"
  doubles "$S" "$port"
  (
    sandbox_env "$S"
    for s in $SPECS; do
      r=${s%%:*} b=${s#*:} c=$S/projects/$r url=$(awk -v r="$r" '$1 == r { print $2 }' "$W/cache/urls")
      git clone -q -b "$b" "$url" "$c"
      [ "$(git -C "$c" config remote.origin.url)" = "$url" ]
      mkdir -p "$c/.atelier/runtime" && echo sandbox > "$c/.atelier/runtime/state"
    done
    # atelier: the main worktree detached behind master, master in a linked
    # worktree, and the skill a detached worktree, as on this host.
    c=$S/projects/atelier
    git -C "$c" checkout -q --detach master^
    git -C "$c" worktree add -q "$S/worktrees/atelier-master" master
    mkdir -p "$S/worktrees/atelier-master/.atelier/runtime" && echo sandbox > "$S/worktrees/atelier-master/.atelier/runtime/state"
    git -C "$c" worktree add -q --detach "$S/skill" master
    # astromech: the main worktree on a feature branch, a second worktree on
    # another, both one commit past the base, each touching an old record.
    c=$S/projects/astromech
    local f
    f=$(cd "$c" && ls .atelier/issues/*.md | head -1)
    git -C "$c" switch -q -c feature/sandbox
    echo "feature" >> "$c/README.md"; git -C "$c" commit -q -am "A feature, in the sandbox"
    git -C "$c" branch -q other/sandbox main
    git -C "$c" worktree add -q "$S/worktrees/astromech-other" other/sandbox
    echo "other" >> "$S/worktrees/astromech-other/README.md"
    sed -i 's/^priority: .*/priority: "P3"/' "$S/worktrees/astromech-other/$f"
    git -C "$S/worktrees/astromech-other" commit -q -am "Other work, in the sandbox"
    # life-on-rails: a Git hook of its own.
    c=$S/projects/life-on-rails
    printf '#!/bin/sh\necho "own pre-commit hook ran" >> %s/own-hook.log\n' "$S" > "$c/.git/hooks/pre-commit"
    chmod +x "$c/.git/hooks/pre-commit"
  )
  SANDBOX_PORT=$port
}

doubles() {
  local S=$1 port=$2
  cat > "$S/bin/systemctl" <<EOF
#!/bin/bash
# systemctl --user <verb> [--now] atelier-serve: the service is the installed
# atelier's serve on port $port.
d=$S/service
[ "\$1" != --user ] || shift
verb=\$1; shift; now=no; [ "\${1:-}" != --now ] || { now=yes; shift; }
[ "\$1" = atelier-serve ] || { echo "Unit \$1 not found." >&2; exit 4; }
running() { [ -f \$d/pid ] && kill -0 "\$(cat \$d/pid)" 2>/dev/null; }
start() { running || { setsid $S/root/bin/atelier serve --bind 127.0.0.1:$port --root $S/projects > \$d/log 2>&1 < /dev/null & echo \$! > \$d/pid; sleep 0.5; running; }; }
stop() { if running; then kill "\$(cat \$d/pid)"; fi; rm -f \$d/pid; }
case \$verb in
  is-enabled) e=\$(cat \$d/enabled 2>/dev/null || echo disabled); echo \$e; [ \$e = enabled ] ;;
  is-active) if running; then echo active; else echo inactive; exit 3; fi ;;
  enable) echo enabled > \$d/enabled; [ \$now = no ] || start ;;
  disable) echo disabled > \$d/enabled; [ \$now = no ] || stop ;;
  start) start ;;
  stop) stop ;;
  *) echo "systemctl double: \$verb" >&2; exit 1 ;;
esac
EOF
  # CI on any commit: the answer in gh/ci, success unless a trial says.
  cat > "$S/bin/gh" <<EOF
#!/bin/bash
echo "\$*" >> $S/gh/calls
c=\$(cat $S/gh/ci 2>/dev/null || echo success)
case "\$1 \$2" in
  "run list") echo "[{\"databaseId\":1,\"status\":\"completed\",\"conclusion\":\"\$c\"}]" ;;
  "run view") echo "{\"jobs\":[{\"name\":\"changes\",\"conclusion\":\"success\",\"steps\":[]},{\"name\":\"test\",\"conclusion\":\"\$c\",\"steps\":[{\"name\":\"atelier check\",\"conclusion\":\"\$c\"}]}]}" ;;
  *) echo "gh double: no answer for: \$*" >&2; exit 1 ;;
esac
EOF
  # A push the remote accepts that then reports failure, once: the push
  # whose ordinal git-fails-push holds (the first when it is empty).
  cat > "$S/bin/git" <<EOF
#!/bin/bash
if [ -e $S/git-fails-push ] && [[ " \$* " == *" push "* ]]; then
  n=\$(( \$(cat $S/pushes 2>/dev/null || echo 0) + 1 )); echo \$n > $S/pushes
  if [ "\$n" = "\$(cat $S/git-fails-push)" ] || [ ! -s $S/git-fails-push ]; then
    rm -f $S/git-fails-push
    /usr/bin/git "\$@" || exit
    echo "fatal: the remote end hung up unexpectedly (a double: the push was accepted)" >&2
    exit 128
  fi
fi
# A subcommand that fails, every time, while git-fails-cmd names it.
if [ -s $S/git-fails-cmd ] && [[ " \$* " == *" \$(cat $S/git-fails-cmd) "* ]]; then
  echo "fatal: \$(cat $S/git-fails-cmd) failed (a double)" >&2
  exit 128
fi
exec /usr/bin/git "\$@"
EOF
  chmod +x "$S/bin/systemctl" "$S/bin/gh" "$S/bin/git"
  echo enabled > "$S/service/enabled"
  "$S/bin/systemctl" --user start atelier-serve
  # T3's database: this thread running in atelier, its parent waiting on it,
  # and a finished thread in astromech.
  sqlite3 "$S/t3.sqlite" "
    CREATE TABLE projection_projects (project_id TEXT PRIMARY KEY, workspace_root TEXT);
    CREATE TABLE orchestration_v2_projection_threads (thread_id TEXT PRIMARY KEY, project_id TEXT, payload_json TEXT);
    CREATE TABLE orchestration_v2_projection_runs (run_id TEXT PRIMARY KEY, thread_id TEXT, status TEXT);
    INSERT INTO projection_projects VALUES ('pa', '$S/projects/atelier'), ('pb', '$S/projects/astromech');
    INSERT INTO orchestration_v2_projection_threads VALUES
      ('sandbox-parent', 'pa', '{\"worktreePath\":null,\"lineage\":{}}'),
      ('sandbox-cutover', 'pa', '{\"worktreePath\":null,\"lineage\":{\"parentThreadId\":\"sandbox-parent\"}}'),
      ('sandbox-done', 'pb', '{\"worktreePath\":null,\"lineage\":{}}');
    INSERT INTO orchestration_v2_projection_runs VALUES ('r1', 'sandbox-parent', 'running'), ('r2', 'sandbox-cutover', 'running'), ('r3', 'sandbox-done', 'completed');"
}

# The environment every script runs in: nothing outside the sandbox.
sandbox_env() {
  local S=$1
  export HOME=$S/home TMPDIR=$S/tmp PATH="$S/bin:$S/root/bin:/usr/local/bin:/usr/bin:/bin:$SQLITE_DIR"
  export GIT_CONFIG_GLOBAL=$S/gitconfig GIT_CONFIG_NOSYSTEM=1 GIT_ALLOW_PROTOCOL=file
  export T3CODE_THREAD_ID=sandbox-cutover
  unset CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID CODEX_THREAD_ID GROK_SESSION_ID ATELIER_SKIP ATELIER_TEST_CI_WAIT CARGO_TARGET_DIR
  export CUTOVER_PROJECTS=$S/projects
  export CUTOVER_REPOS="atelier:$S/projects/atelier:master astromech:$S/projects/astromech:main demiurge:$S/projects/demiurge:master life-on-rails:$S/projects/life-on-rails:main cosplay-studio:$S/projects/cosplay-studio:main"
  export CUTOVER_INSTALL_ROOT=$S/root
  export CUTOVER_BUILD="mkdir -p \"\$STATE/target/release\" && cp '$NEW' \"\$STATE/target/release/atelier\""
  export CUTOVER_INSTALL="install -m 755 '$NEW' \"\$INSTALL_ROOT/bin/atelier\""
  export CUTOVER_SKILL=$S/skill CUTOVER_SKILL_INSTALL="" CUTOVER_SKILL_DIR=""
  export CUTOVER_SYSTEMCTL="$S/bin/systemctl --user"
  export CUTOVER_SERVICE_CHECK="curl -fsS --max-time 5 http://127.0.0.1:${SANDBOX_PORT:-0}/api/repos"
  export CUTOVER_GH=$S/bin/gh CUTOVER_T3_DB=$S/t3.sqlite CUTOVER_STATE=$S/state CUTOVER_CI_WAIT=30
  export CUTOVER_FALLBACK_URL=file://$W/cache/fallback.tar.xz
  unset CUTOVER_FAIL_AT CUTOVER_THREADS_CHECKED
  local v
  for v in HOME TMPDIR GIT_CONFIG_GLOBAL CUTOVER_PROJECTS CUTOVER_INSTALL_ROOT CUTOVER_SKILL CUTOVER_T3_DB CUTOVER_STATE; do
    case ${!v} in "$S"/*) ;; *) echo "$v points outside the sandbox: ${!v}" >&2; exit 1 ;; esac
  done
  for v in $CUTOVER_REPOS; do case $v in *":$S/projects/"*) ;; *) echo "CUTOVER_REPOS names $v" >&2; exit 1 ;; esac; done
}

# ---------------------------------------------------------------- restored

# What the sandbox looks like, for "restored": each base, each worktree's
# checkout and status (ignored files included), each hooks directory, the
# installed binary, the skill and the service.
snapshot() { # <sandbox>
  local S=$1 s r c wt
  for s in $SPECS; do
    r=${s%%:*} c=$S/projects/$r
    git -C "$c" worktree list --porcelain | grep -E '^(worktree|branch|detached)'
    for wt in $(git -C "$c" worktree list --porcelain | sed -n 's/^worktree //p'); do
      # The old tool writes its runtime cache wherever it runs, as the
      # preflight's reads do; it is not the conversion's.
      echo "status $wt"; git -C "$wt" status --porcelain --ignored | grep -vx '!! .atelier/runtime/' || true
    done
    echo "hooks $r"; (cd "$(git -C "$c" rev-parse --path-format=absolute --git-path hooks)" && find . -type f -exec sha256sum {} + | sort)
  done
  echo "binary $(sha256sum "$S/root/bin/atelier" | cut -d' ' -f1)"
  echo "skill $(git -C "$S/skill" rev-parse HEAD)"
  echo "service $(cat "$S/service/enabled") $("$S/bin/systemctl" is-active atelier-serve || true)"
}
# Checks the sandbox is restored, as the item defines it. Prints what was
# compared and returns 1 on the first difference.
restored() { # <sandbox> <before snapshot> <before bases>
  local S=$1 s r b B rt list n_rev n_run c out=""
  for s in $SPECS; do
    r=${s%%:*} b=${s#*:} c=$S/projects/$r
    B=$(awk -v r="$r" '$1 == r { print $2 }' "$3")
    rt=$(git --git-dir="$S/remotes/$r.git" rev-parse "$b")
    if [ "$rt" = "$B" ]; then
      [ "$(git -C "$c" rev-parse "$b")" = "$B" ] || { say "    $r: local $b is not the recorded base"; return 1; }
      out="$out $r:base"
    else
      list=$(git --git-dir="$S/remotes/$r.git" rev-list --first-parent "$B..$rt")
      n_rev=$(for x in $list; do git --git-dir="$S/remotes/$r.git" log -1 --format=%B "$x" | grep -c '^Cutover-Revert: ' || true; done | awk '{ s += $1 } END { print s + 0 }')
      n_run=$(( $(wc -w <<< "$list") - n_rev ))
      [ "$n_rev" -eq "$n_run" ] && [ "$n_run" -gt 0 ] || { say "    $r: $n_run pushed commits and $n_rev reverts on the remote"; return 1; }
      [ "$(git --git-dir="$S/remotes/$r.git" rev-parse "$rt^{tree}")" = "$(git --git-dir="$S/remotes/$r.git" rev-parse "$B^{tree}")" ] \
        || { say "    $r: the remote's tree differs from the recorded commit's"; return 1; }
      [ "$(git -C "$c" rev-parse "$b")" = "$rt" ] || { say "    $r: local and remote $b differ"; return 1; }
      out="$out $r:$n_run-reverted"
    fi
    for wt in $(git -C "$c" worktree list --porcelain | sed -n 's/^worktree //p'); do
      (cd "$wt" && "$S/root/bin/atelier" check > /dev/null 2>&1) || { say "    $r: the old check fails in $wt"; return 1; }
    done
  done
  snapshot "$S" > "$S/after.snapshot"
  if ! diff -q "$2" "$S/after.snapshot" > /dev/null; then
    say "    the sandbox differs from before:"; diff "$2" "$S/after.snapshot" | head -20 | sed 's/^/      /'; return 1
  fi
  [ "$(sha256sum "$S/root/bin/atelier" | cut -d' ' -f1)" = "$FALLBACK_BIN_SHA256" ] || { say "    the installed binary is not the fallback"; return 1; }
  curl -fsS --max-time 5 "http://127.0.0.1:$SANDBOX_PORT/api/repos" > /dev/null || { say "    the old site does not answer"; return 1; }
  [ -z "$(ls -A "$S/state/convert" 2>/dev/null)" ] || { say "    conversion clones are left"; return 1; }
  say "    restored:$out; worktrees, statuses, hooks, binary (fallback), skill, service (answering) as before; old check passes everywhere"
}

bases() { local s; for s in $SPECS; do echo "${s%%:*} $(git -C "$1/projects/${s%%:*}" rev-parse "${s#*:}")"; done; }

# ---------------------------------------------------------------- trials

run_cutover() { # <sandbox> [args]: runs the sandbox's copy of the runbook
  local S=$1; shift
  "$S/worktrees/atelier-master/scripts/cutover/cutover.sh" "$@"
}
run_rollback() { "$1/worktrees/atelier-master/scripts/cutover/rollback.sh"; }

# stop_trial <name> <port> <what stops it> <setup...>: a cutover stopped,
# then rolled back twice, then compared.
stop_trial() {
  local name=$1 port=$2 what=$3 S=$W/$1 rc
  shift 3
  sandbox "$S" "$port"
  (
    sandbox_env "$S"
    "$@"
    snapshot "$S" > "$S/before.snapshot"
    bases "$S" > "$S/before.bases"
    set +e
    run_cutover "$S" > "$S/cutover.log" 2>&1; rc=$?
    set -e
    [ $rc -eq 1 ] || { say "$name: FAILED: the cutover exited $rc, not 1 (stop: $what); see $S/cutover.log"; exit 1; }
    say "$name: stopped: $what"
    awk '/^STOPPED/ { print "    " $0; exit }' "$S/cutover.log"
    awk '/^STOPPED/ { print "    why: " last; exit } NF { last = $0 }' "$S/cutover.log"
    run_rollback "$S" > "$S/rollback.log" 2>&1 || { say "    FAILED: rollback exited non-zero; see $S/rollback.log"; exit 1; }
    grep -E 'published|reverted|reset away|installed|skill checkout|atelier-serve:' "$S/rollback.log" | sed 's/^/    rollback: /'
    run_rollback "$S" > "$S/rollback2.log" 2>&1 || { say "    FAILED: a second rollback exited non-zero; see $S/rollback2.log"; exit 1; }
    ! grep -q 'pushed the reverts' "$S/rollback2.log" || { say "    FAILED: the second rollback pushed again"; exit 1; }
    say "    a second rollback changed nothing it should not"
    restored "$S" "$S/before.snapshot" "$S/before.bases" || { say "    FAILED"; exit 1; }
    "$S/bin/systemctl" stop atelier-serve
  ) && say "$name: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}

fail_at() { export CUTOVER_FAIL_AT=$1; }
no_section() { # astromech's AGENTS.md names no removed command
  local t=$W/tmp/nosection
  rm -rf "$t"; git clone -q -b main "$S/remotes/astromech.git" "$t"
  printf '# Sandbox\n\nNo process section here.\n' > "$t/AGENTS.md"
  git -C "$t" commit -qam "A sandbox AGENTS.md with no process section"
  git -C "$t" push -q origin main
  rm -rf "$t"
  git -C "$S/projects/astromech" fetch -q origin
  git -C "$S/projects/astromech" branch -q -f main origin/main
}
stray_after_commit() { # a hook of atelier's own leaves a file under .atelier/ after each commit
  printf '#!/bin/sh\ntouch .atelier/stray\n' > "$S/projects/atelier/.git/hooks/post-commit"
  chmod +x "$S/projects/atelier/.git/hooks/post-commit"
}
ci_fails() { echo failure > "$S/gh/ci"; }
push_reports_failure() { touch "$S/git-fails-push"; }

trial_full() {
  local S=$W/full rc
  sandbox "$S" 17320
  (
    sandbox_env "$S"
    run_cutover "$S" preflight > "$S/preflight.log" 2>&1 || { say "full: FAILED: preflight; see $S/preflight.log"; exit 1; }
    set +e
    /usr/bin/time -f "%e" -o "$S/time" "$S/worktrees/atelier-master/scripts/cutover/cutover.sh" > "$S/cutover.log" 2>&1; rc=$?
    set -e
    [ $rc -eq 0 ] || { say "full: FAILED: exit $rc; see $S/cutover.log"; exit 1; }
    say "full: ran to its end in $(cat "$S/time") s"
    sed -n '/^Cutover report/,$p' "$S/cutover.log" | sed "s|$S|<sandbox>|g; s/^/    /"
    grep -E '^   (atelier|astromech|demiurge|life-on-rails|cosplay-studio): check and status' "$S/cutover.log" | sed 's/^/   /'
    grep -E 'CI run|atelier --version' "$S/cutover.log" | sed 's/^/   /'
    local wd; wd=$(git --git-dir="$S/remotes/atelier.git" show master:.atelier/items/atelier-wd1i/item.md | sed -n 's/^status: //p')
    [ "$wd" = done ] || { say "    FAILED: atelier-wd1i is $wd on the remote"; exit 1; }
    say "    atelier-wd1i is done on the remote's master, closed by $(git --git-dir="$S/remotes/atelier.git" log -1 --format=%s master)"
    grep -q "old-layout branch" "$S/cutover.log" && grep "old-layout branch" "$S/cutover.log" | sed "s|$S|<sandbox>|g; s/^/   /"
    # The README's way for a worktree left on an old-layout branch.
    local o=$S/worktrees/astromech-other
    (cd "$o" && ! atelier check > /dev/null 2>&1) || { say "    FAILED: atelier check passes on the old-layout branch"; exit 1; }
    (
      cd "$o"
      git merge -q --no-commit main > /dev/null 2>&1 || true
      git rm -r -q --cached .atelier
      git checkout main -- .atelier
      git clean -fdq -- .atelier
      git commit -q --no-edit
      atelier check > /dev/null
    ) || { say "    FAILED: the README's way did not bring the old-layout worktree onto the base"; exit 1; }
    say "    the README's way brought astromech-other onto the converted base: atelier check passes there, and its own commit is kept"
    rm -rf "$W/full-remotes"; cp -r "$S/remotes" "$W/full-remotes"
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "full: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}

trial_resume() {
  local S=$W/resume rc
  sandbox "$S" 17321
  (
    sandbox_env "$S"
    touch "$S/git-fails-push"
    set +e; run_cutover "$S" > "$S/cutover.log" 2>&1; rc=$?; set -e
    [ $rc -eq 1 ] || { say "resume: FAILED: first run exited $rc"; exit 1; }
    say "resume: stopped: $(grep -m1 'cutover: ' "$S/cutover.log")"
    set +e; run_cutover "$S" > "$S/cutover2.log" 2>&1; rc=$?; set -e
    [ $rc -eq 0 ] || { say "    FAILED: the second run exited $rc; see $S/cutover2.log"; exit 1; }
    grep -q '^done	push:atelier	found on the remote' "$S/state/journal" || { say "    FAILED: the push was not settled from the remote"; exit 1; }
    say "    run again, it settled the push from the remote's tip (found on the remote) and ran to its end"
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "resume: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}

# A run stopped and run again, at <point> or with the push numbered <n>
# failing after the remote accepted it, then checked by <check>.
retry_trial() { # <name> <port> <what> <first run's setup> <second run's setup> <check>
  local name=$1 S=$W/$1 rc
  sandbox "$S" "$2"
  (
    sandbox_env "$S"
    $4
    set +e; run_cutover "$S" > "$S/cutover.log" 2>&1; rc=$?; set -e
    [ $rc -eq 1 ] || { say "$name: FAILED: the first run exited $rc"; exit 1; }
    say "$name: stopped: $3"
    awk '/^STOPPED/ { print "    why: " last; exit } NF { last = $0 }' "$S/cutover.log"
    unset CUTOVER_FAIL_AT; $5
    set +e; run_cutover "$S" > "$S/cutover2.log" 2>&1; rc=$?; set -e
    echo "$rc" > "$S/rc2"
    $6
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "$name: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}
close_commits() { git --git-dir="$S/remotes/atelier.git" log --format=%s master | grep -c '^Close atelier-wd1i$' || true; }
ran_to_end_with_one_close() {
  [ "$(cat "$S/rc2")" = 0 ] || { say "    FAILED: run again, it exited $(cat "$S/rc2"); see $S/cutover2.log"; exit 1; }
  [ "$(close_commits)" = 1 ] || { say "    FAILED: the remote's master has $(close_commits) closing commits"; exit 1; }
  [ "$(git --git-dir="$S/remotes/atelier.git" show master:.atelier/items/atelier-wd1i/item.md | sed -n 's/^status: //p')" = done ] \
    || { say "    FAILED: atelier-wd1i is not done on the remote"; exit 1; }
  say "    run again, it ran to its end: steps 2 to 7 done by the earlier run, $(grep -c 'done by an earlier run' "$S/cutover2.log") skipped; one closing commit on the remote's master"
}
push_found() {
  grep -q '^done	push:close	found on the remote' "$S/state/journal" || { say "    FAILED: the closing push was not settled from the remote"; exit 1; }
  say "    the closing push was settled from the remote's tip (found on the remote), not pushed again"
}
second_stop_at_step2() { export CUTOVER_FAIL_AT=step2-committed; }
recorded_whole_then_rolled_back() {
  [ "$(cat "$S/rc2")" = 1 ] || { say "    FAILED: the second run exited $(cat "$S/rc2")"; exit 1; }
  local r n
  for r in atelier astromech demiurge life-on-rails cosplay-studio; do
    for k in "pre:base:$r" "pre:hooks:$r"; do grep -q "^done	$k	" "$S/state/journal" || { say "    FAILED: $k not recorded"; exit 1; }; done
  done
  for k in pre:binary pre:skill pre:service mode; do grep -q "^done	$k	" "$S/state/journal" || { say "    FAILED: $k not recorded"; exit 1; }; done
  n=$(grep -c '^done	pre:base:atelier	' "$S/state/journal")
  [ "$n" = 1 ] || { say "    FAILED: atelier's base recorded $n times"; exit 1; }
  say "    run again, it finished the preflight (every base, hooks copy, binary, skill and service recorded once; atelier's kept from the first run) before converting, then stopped after step 2's commit"
  run_rollback "$S" > "$S/rollback.log" 2>&1 || { say "    FAILED: rollback; see $S/rollback.log"; exit 1; }
  restored "$S" "$S/before.snapshot" "$S/before.bases" || { say "    FAILED"; exit 1; }
}
snapshot_first() { snapshot "$S" > "$S/before.snapshot"; bases "$S" > "$S/before.bases"; export CUTOVER_FAIL_AT=record-atelier; }
close_push_fails() { echo 6 > "$S/git-fails-push"; }

# A rollback whose own command fails: it must say what failed, keep its
# recovery state, install nothing and exit non-zero; run again once the
# cause is gone, it restores.
rollback_fails_trial() { # <name> <port> <what> <stop point> <break> <mend> <kept recovery state, or "">
  local name=$1 S=$W/$1 rc
  sandbox "$S" "$2"
  (
    sandbox_env "$S"
    snapshot "$S" > "$S/before.snapshot"; bases "$S" > "$S/before.bases"
    export CUTOVER_FAIL_AT=$4
    set +e; run_cutover "$S" > "$S/cutover.log" 2>&1; rc=$?; set -e
    [ $rc -eq 1 ] || { say "$name: FAILED: the cutover exited $rc"; exit 1; }
    unset CUTOVER_FAIL_AT
    say "$name: the cutover stopped at $4; then $3"
    $5
    set +e; run_rollback "$S" > "$S/rollback.log" 2>&1; rc=$?; set -e
    [ $rc -ne 0 ] || { say "    FAILED: the rollback exited 0 although $3"; tail -5 "$S/rollback.log" | sed 's/^/      /'; exit 1; }
    grep -q '(a double)' "$S/rollback.log" || { say "    FAILED: the rollback did not say what failed"; exit 1; }
    [ "$(sha256sum "$S/root/bin/atelier" | cut -d' ' -f1)" != "$FALLBACK_BIN_SHA256" ] || { say "    FAILED: the fallback was installed over repositories not restored"; exit 1; }
    [ -z "$7" ] || [ -d "$S/state/$7" ] || { say "    FAILED: the recovery state $7 is gone"; exit 1; }
    say "    rollback exited $rc, saying: $(grep -m1 '(a double)' "$S/rollback.log" | sed 's/^ *//' | cut -c1-110)"
    say "    it installed nothing$( [ -z "$7" ] || echo ", and kept $7")"
    $6
    run_rollback "$S" > "$S/rollback2.log" 2>&1 || { say "    FAILED: run again once mended, the rollback failed; see $S/rollback2.log"; exit 1; }
    restored "$S" "$S/before.snapshot" "$S/before.bases" || { say "    FAILED"; exit 1; }
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "$name: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}
refuse_pushes() { printf '#!/bin/sh\necho "push declined (a double)" >&2\nexit 1\n' > "$S/remotes/atelier.git/hooks/pre-receive"; chmod +x "$S/remotes/atelier.git/hooks/pre-receive"; }
accept_pushes() { rm -f "$S/remotes/atelier.git/hooks/pre-receive"; }
fail_git() { echo "$1" > "$S/git-fails-cmd"; }
mend_git() { rm -f "$S/git-fails-cmd"; }

# A preflight whose old-tracker query fails must stop, before anything is
# converted.
preflight_query_trial() { # <name> <port> <what> <which query fails>
  local name=$1 S=$W/$1 rc
  sandbox "$S" "$2"
  (
    sandbox_env "$S"
    cat > "$S/bin/atelier" <<WRAPPER
#!/bin/bash
case "$4" in
  work) if [ "\$PWD" = "$S/projects/astromech" ] && [ "\$1 \$2" = "work active" ]; then echo "Error: cannot read the records (a double)" >&2; exit 1; fi ;;
  status) if [ "\$1" = status ]; then echo "Error: status failed (a double)" >&2; exit 1; fi ;;
esac
exec $S/root/bin/atelier "\$@"
WRAPPER
    chmod +x "$S/bin/atelier"
    set +e; run_cutover "$S" > "$S/cutover.log" 2>&1; rc=$?; set -e
    [ $rc -eq 1 ] || { say "$name: FAILED: the cutover exited $rc although $3"; exit 1; }
    if grep -q '^== Step 2' "$S/cutover.log"; then say "$name: FAILED: it went on to step 2 although $3"; exit 1; fi
    if grep -q 'convert:' "$S/state/journal" 2>/dev/null; then say "$name: FAILED: it converted something"; exit 1; fi
    grep -q '(a double)' "$S/cutover.log" || { say "$name: FAILED: it did not say what failed"; exit 1; }
    say "$name: stopped in step 1 when $3: $(grep -m1 '(a double)' "$S/cutover.log" | sed "s|$S|<sandbox>|g; s/^ *//" | cut -c1-120)"
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "$name: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}

trial_preflight() {
  local S=$W/preflight-pre rc out
  sandbox "$S" 17322 pre
  (
    sandbox_env "$S"
    local pf=$KIT/cutover.sh
    git -C "$S/projects/atelier" fetch -q "$TREE" "HEAD:refs/heads/atelier-u1ko"
    out=$("$pf" preflight 2>&1) || { say "preflight: FAILED: before the switch, with atelier-u1ko in progress, it stopped:"; echo "$out" | sed 's/^/    /'; exit 1; }
    say "preflight: before the switch, with only atelier-u1ko in progress and only this thread running: passes"
    # Another item started: a worktree on its branch, its record in progress.
    local other
    other=$(cd "$S/projects/atelier" && git grep -l '^status: "todo"$' master -- .atelier/issues | head -1 | sed 's|.*/||; s|\.md$||')
    git -C "$S/projects/atelier" worktree add -q -b "$other" "$S/worktrees/other" master
    sed -i 's/^status: "todo"$/status: "in_progress"/' "$S/worktrees/other/.atelier/issues/$other.md"
    git -C "$S/worktrees/other" commit -qam "Start $other, in the sandbox"
    if out=$("$pf" preflight 2>&1); then say "    FAILED: it passed with $other in progress"; exit 1; fi
    say "    with another item also in progress: stops: $(grep -m1 'in progress:' <<< "$out" | sed 's/^ *//')"
    git -C "$S/projects/atelier" worktree remove --force "$S/worktrees/other"
    git -C "$S/projects/atelier" branch -q -D "$other"
    sqlite3 "$S/t3.sqlite" "INSERT INTO orchestration_v2_projection_threads VALUES ('other', 'pb', '{\"worktreePath\":null,\"lineage\":{}}'); INSERT INTO orchestration_v2_projection_runs VALUES ('r4', 'other', 'running');"
    if out=$("$pf" preflight 2>&1); then say "    FAILED: it passed with another thread running"; exit 1; fi
    say "    with another thread running in astromech: stops: $(grep -m1 'T3 thread' <<< "$out" | sed "s|$S|<sandbox>|; s/^ *//")"
    sqlite3 "$S/t3.sqlite" "DELETE FROM orchestration_v2_projection_runs WHERE run_id = 'r4';"
    mv "$S/t3.sqlite" "$S/t3.moved"
    if out=$("$pf" preflight 2>&1); then say "    FAILED: it passed with T3's database unreadable"; exit 1; fi
    grep -q 'Check by hand' <<< "$out" || { say "    FAILED: no hand check printed"; exit 1; }
    out=$(CUTOVER_THREADS_CHECKED=yes "$pf" preflight 2>&1) || { say "    FAILED: the hand check was not accepted"; exit 1; }
    say "    with T3's database unreadable: stops and prints the hand check; passes once told it was made"
    mv "$S/t3.moved" "$S/t3.sqlite"
    "$S/bin/systemctl" stop atelier-serve || true
  ) || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
  S=$W/preflight-post
  sandbox "$S" 17323
  (
    sandbox_env "$S"
    out=$(run_cutover "$S" preflight 2>&1) || { say "    FAILED: after the switch it stopped:"; echo "$out" | sed 's/^/    /'; exit 1; }
    say "    after the switch, with nothing in progress: passes"
    # The switch's branch with a commit past its merge: in progress, not merged.
    git -C "$S/projects/atelier" fetch -q "$TREE" "HEAD:refs/heads/atelier-u1ko"
    git -C "$S/projects/atelier" worktree add -q "$S/worktrees/u1ko" atelier-u1ko
    echo "more" >> "$S/worktrees/u1ko/README.md"
    git -C "$S/worktrees/u1ko" commit -qam "More work after the merge, in the sandbox"
    if out=$(run_cutover "$S" preflight 2>&1); then say "    FAILED: after the switch it passed with atelier-u1ko in progress"; exit 1; fi
    say "    after the switch, with atelier-u1ko in progress: stops"
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "preflight: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}

# A second host, before the switch, with its old service running, adopts the
# remotes the full run converted; then a third does and rolls it back.
adopt_sandbox() { # <dir> <port>
  local S=$1
  sandbox "$S" "$2" pre
  rm -rf "$S/remotes"; cp -r "$W/full-remotes" "$S/remotes"
  git clone -q -b master "$S/remotes/atelier.git" "$S/adopt-src"
  mkdir -p "$S/home/.agents/skills"
  cp -r "$S/skill/skills/agent-factory" "$S/home/.agents/skills/agent-factory"
}
adopt_env() {
  export CUTOVER_SKILL=$S/no-skill-checkout CUTOVER_SKILL_DIR=$S/home/.agents/skills/agent-factory
  export CUTOVER_SKILL_INSTALL="rm -rf \"\$SKILL_DIR\" && cp -r skills/agent-factory \"\$SKILL_DIR\""
}
trial_adopt() {
  [ -d "$W/full-remotes" ] || { say "adopt: needs the full trial's remotes; run full first"; return 1; }
  local S=$W/adopt rc
  adopt_sandbox "$S" 17324
  (
    sandbox_env "$S"; adopt_env
    git --git-dir="$S/remotes/atelier.git" for-each-ref > "$S/remotes.before"
    for r in astromech demiurge life-on-rails cosplay-studio; do git --git-dir="$S/remotes/$r.git" for-each-ref; done >> "$S/remotes.before"
    set +e; "$S/adopt-src/scripts/cutover/cutover.sh" adopt > "$S/adopt.log" 2>&1; rc=$?; set -e
    [ $rc -eq 0 ] || { say "adopt: FAILED: exit $rc; see $S/adopt.log"; exit 1; }
    say "adopt: a second host before the switch, with its old service running, adopted the converted remotes"
    sed -n '/^Adopt report/,$p' "$S/adopt.log" | sed "s|$S|<sandbox>|g; s/^/    /"
    grep -E 'check and status|atelier --version' "$S/adopt.log" | sed 's/^/   /'
    git --git-dir="$S/remotes/atelier.git" for-each-ref > "$S/remotes.after"
    for r in astromech demiurge life-on-rails cosplay-studio; do git --git-dir="$S/remotes/$r.git" for-each-ref; done >> "$S/remotes.after"
    diff -q "$S/remotes.before" "$S/remotes.after" > /dev/null || { say "    FAILED: adopt changed a remote"; exit 1; }
    say "    nothing pushed: every remote ref is as before"
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "adopt: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }

  S=$W/adopt-rollback
  adopt_sandbox "$S" 17325
  (
    sandbox_env "$S"; adopt_env
    snapshot "$S" > "$S/before.snapshot"
    git -C "$S/projects/atelier" for-each-ref refs/heads refs/remotes > "$S/refs.before"
    (cd "$S/home/.agents/skills" && find . -type f -exec sha256sum {} + | sort) > "$S/skill.before"
    "$S/adopt-src/scripts/cutover/cutover.sh" adopt > "$S/adopt.log" 2>&1 || { say "    FAILED: adopt; see $S/adopt.log"; exit 1; }
    "$S/adopt-src/scripts/cutover/rollback.sh" > "$S/rollback.log" 2>&1 || { say "    FAILED: rollback; see $S/rollback.log"; exit 1; }
    "$S/adopt-src/scripts/cutover/rollback.sh" > "$S/rollback2.log" 2>&1 || { say "    FAILED: a second rollback; see $S/rollback2.log"; exit 1; }
    snapshot "$S" > "$S/after.snapshot"
    diff -q "$S/before.snapshot" "$S/after.snapshot" > /dev/null || { say "    FAILED: differs:"; diff "$S/before.snapshot" "$S/after.snapshot" | head; exit 1; }
    git -C "$S/projects/atelier" for-each-ref refs/heads refs/remotes > "$S/refs.after"
    diff -q "$S/refs.before" "$S/refs.after" > /dev/null || { say "    FAILED: refs differ"; exit 1; }
    (cd "$S/home/.agents/skills" && find . -type f -exec sha256sum {} + | sort) > "$S/skill.after"
    diff -q "$S/skill.before" "$S/skill.after" > /dev/null || { say "    FAILED: the skill differs"; exit 1; }
    [ "$(sha256sum "$S/root/bin/atelier" | cut -d' ' -f1)" = "$FALLBACK_BIN_SHA256" ] || { say "    FAILED: not the fallback"; exit 1; }
    curl -fsS --max-time 5 "http://127.0.0.1:17325/api/repos" > /dev/null || { say "    FAILED: the old site does not answer"; exit 1; }
    for r in atelier astromech demiurge life-on-rails cosplay-studio; do (cd "$S/projects/$r" && atelier check > /dev/null) || { say "    FAILED: old check in $r"; exit 1; }; done
    say "    rolled back, twice: hooks, worktrees, statuses, base and remote-tracking refs, fallback binary, skill directory and service (answering) as before; old check passes; nothing pushed"
    "$S/bin/systemctl" stop atelier-serve || true
  ) && say "adopt-rollback: passed" || { "$S/bin/systemctl" stop atelier-serve 2>/dev/null || true; return 1; }
}

run_trial() {
  case $1 in
    1) stop_trial 1-no-section 17301 "step 5: astromech's AGENTS.md has no section the rule matches, before its commit" no_section ;;
    1b) stop_trial 1b-verify 17302 "step 2: --verify fails after the commit (a stray file under .atelier/)" stray_after_commit ;;
    2) stop_trial 2-after-step2 17303 "after step 2's commit" fail_at step2-committed ;;
    3) stop_trial 3-mid-step3 17304 "in step 3: the binary installed, the hooks not" fail_at step3-installed ;;
    4) stop_trial 4-after-step3 17305 "after step 3" fail_at step3-done ;;
    5) stop_trial 5-after-step4 17306 "after step 4's push" fail_at step4-done ;;
    6) stop_trial 6-step5-unpushed 17307 "in step 5: demiurge's conversion committed and not pushed, astromech's pushed" fail_at step5-demiurge-committed ;;
    7) stop_trial 7-mid-step6 17308 "in step 6: atelier and astromech brought onto the converted base, the others not" fail_at step6-astromech-moved ;;
    8) stop_trial 8-ci-fails 17309 "step 7: CI on the conversion commit reported failed" ci_fails ;;
    9) stop_trial 9-step8-unpushed 17310 "after step 8's commit, before its push" fail_at step8-committed ;;
    10) stop_trial 10-after-step8 17311 "after step 8's push" fail_at step8-pushed ;;
    11) stop_trial 11-push-reports-failure 17312 "step 4's push accepted by the remote, then reported failed" push_reports_failure ;;
    12) stop_trial 12-one-product-pushed 17313 "in step 5: astromech's conversion pushed, no real clone fetched it" fail_at step5-astromech-pushed ;;
    full) trial_full ;;
    resume) trial_resume ;;
    retry-record) retry_trial retry-record 17326 "in step 1, between atelier's snapshots and astromech's" snapshot_first second_stop_at_step2 recorded_whole_then_rolled_back ;;
    retry-close) retry_trial retry-close 17327 "after step 8's commit, before its push" "fail_at step8-committed" : ran_to_end_with_one_close ;;
    retry-close-push) retry_trial retry-close-push 17328 "step 8's push accepted by the remote, then reported failed" close_push_fails : "eval ran_to_end_with_one_close; push_found" ;;
    preflight) trial_preflight ;;
    adopt) trial_adopt ;;
    rollback-push-refused) rollback_fails_trial rollback-push-refused 17329 "the remote declines the rollback's push" step5-astromech-pushed refuse_pushes accept_pushes rollback/atelier ;;
    rollback-fetch-fails) rollback_fails_trial rollback-fetch-fails 17330 "every fetch fails" step6-astromech-moved "fail_git fetch" mend_git "" ;;
    rollback-checkout-fails) rollback_fails_trial rollback-checkout-fails 17331 "a worktree's switch back fails" step6-astromech-moved "fail_git switch" mend_git "" ;;
    preflight-work-fails) preflight_query_trial preflight-work-fails 17332 "the old atelier work active fails in astromech's feature worktree" work ;;
    preflight-status-fails) preflight_query_trial preflight-status-fails 17333 "the old atelier status fails" status ;;
    *) say "no trial $1"; return 1 ;;
  esac
}

fingerprint > "$W/real.before"
prepare
trials=${*:-preflight preflight-work-fails preflight-status-fails 1 1b 2 3 4 5 6 7 8 9 10 11 12 full resume retry-record retry-close retry-close-push rollback-push-refused rollback-fetch-fails rollback-checkout-fails adopt}
failed=""
for t in $trials; do
  t0=$(date +%s)
  run_trial "$t" || failed="$failed $t"
  say "    ($(( $(date +%s) - t0 )) s)"
done
fingerprint > "$W/real.after"
if diff -q "$W/real.before" "$W/real.after" > /dev/null; then
  say "the real repositories, installed binary, skill checkout, skill links and service are as they were"
else
  say "THE REAL HOST CHANGED:"; diff "$W/real.before" "$W/real.after"; exit 1
fi
[ -z "$failed" ] || { say "failed:$failed"; exit 1; }
say "all trials passed"
