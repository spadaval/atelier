# Settings and the journal, shared by cutover.sh and rollback.sh. Sourced,
# not run.
#
# Everything outside the scripts that they touch comes from this block or
# the environment, with this host's values as defaults. The trials
# (trials.sh) run the same scripts with every one pointed at a sandbox.

# Each repository as <name>:<main clone>:<base branch>, atelier first.
P=${CUTOVER_PROJECTS:-$HOME/projects}
REPOS=${CUTOVER_REPOS:-"atelier:$P/atelier:master astromech:$P/astromech:main demiurge:$P/demiurge:master life-on-rails:$P/life-on-rails:main cosplay-studio:$P/cosplay-studio:main"}
REMOTE=${CUTOVER_REMOTE:-origin}
# The most AGENTS.md sections naming a removed command that each product
# repository holds, counted for this item; more stops the runbook.
SECTIONS=${CUTOVER_SECTIONS:-"astromech:1 demiurge:1 life-on-rails:1 cosplay-studio:2"}
# Workflows that run Atelier, replaced by the reviewed one: <repo>:<path>.
WORKFLOWS=${CUTOVER_WORKFLOWS:-"astromech:.github/workflows/atelier.yml"}
# The tree's binary, built in step 2 for its `check`, and the install of
# step 3, which reuses that build. Both run in atelier's base worktree.
INSTALL_ROOT=${CUTOVER_INSTALL_ROOT:-$HOME/.cargo}
BUILD=${CUTOVER_BUILD:-'cargo build --release --locked -j 4 -p atelier-cli'}
INSTALL=${CUTOVER_INSTALL:-'cargo install --path crates/atelier-cli --locked -j 4 --root "$INSTALL_ROOT"'}
# The skill checkout every session loads Agent Factory from, and, for
# `adopt` on a host that installs the skill another way, the command that
# installs it and the directory it installs into.
SKILL=${CUTOVER_SKILL:-$P/atelier-skill}
SKILL_INSTALL=${CUTOVER_SKILL_INSTALL:-}
SKILL_DIR=${CUTOVER_SKILL_DIR:-}
SYSTEMCTL=${CUTOVER_SYSTEMCTL:-systemctl --user}
SERVICE=${CUTOVER_SERVICE:-atelier-serve}
SERVICE_CHECK=${CUTOVER_SERVICE_CHECK:-'curl -fsS --max-time 5 http://127.0.0.1:7341/api/repos'}
GH=${CUTOVER_GH:-gh}
GH_REPO=${CUTOVER_GH_REPO:-spadaval/atelier}
CI_WORKFLOW=${CUTOVER_CI_WORKFLOW:-ci.yml}
CI_WAIT=${CUTOVER_CI_WAIT:-900}
T3_DB=${CUTOVER_T3_DB:-$HOME/.t3/userdata/statev2.sqlite}
STATE=${CUTOVER_STATE:-$HOME/.cache/atelier-cutover}
# The fallback release (atelier-mlwx): v0.4.0's archive for this host.
FALLBACK_URL=${CUTOVER_FALLBACK_URL:-https://github.com/spadaval/atelier/releases/download/v0.4.0/atelier-cli-x86_64-unknown-linux-gnu.tar.xz}
FALLBACK_SHA256=${CUTOVER_FALLBACK_SHA256:-d13f5e4df5c3382715a1247f1ede175d5e4361309e63a5a36e96ab55b1ce20d7}
# The item the runbook closes in step 8, and the one switch item that may be
# in progress before the switch merges.
CUTOVER_ITEM=${CUTOVER_ITEM:-atelier-wd1i}
SWITCH_ITEM=${CUTOVER_SWITCH_ITEM:-atelier-u1ko}
# The paths a conversion writes new files into, which a rollback cleans;
# a reset gives back what it changed or removed.
CONVERTED_PATHS=".atelier AGENTS.md skills .claude .codex .github/workflows"
SKIP_REASON="cutover conversion"

export ATELIER_NO_UPDATE_CHECK=1
JOURNAL=$STATE/journal
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

say() { printf '%s\n' "$*"; }
die() { printf 'cutover: %s\n' "$*" >&2; exit 1; }

# The repositories' fields: name, main clone, base.
repo_names() { local s; for s in $REPOS; do echo "${s%%:*}"; done; }
repo_field() { # <name> <2|3>
  local s
  for s in $REPOS; do [ "${s%%:*}" != "$1" ] || { echo "$s" | cut -d: -f"$2"; return; }; done
  die "no repository named $1 in the settings"
}
clone_of() { repo_field "$1" 2; }
base_of() { repo_field "$1" 3; }
setting_for() { # <list> <name>: the value for <name> in a "<name>:<value> ..." list
  local s
  for s in $1; do [ "${s%%:*}" != "$2" ] || { echo "${s#*:}"; return; }; done
}

# The commit the remote's base points at, read from the remote itself.
remote_tip() { # <name>
  git -C "$(clone_of "$1")" ls-remote "$REMOTE" "refs/heads/$(base_of "$1")" | cut -f1
}
# Each worktree of a clone, as "<path> <branch or detached:<commit>>".
worktrees() { # <clone>
  git -C "$1" worktree list --porcelain | awk '
    /^worktree /{ p = substr($0, 10) }
    /^HEAD /{ h = $2 }
    /^branch /{ b = substr($2, 12) }
    /^detached/{ b = "detached:" h }
    /^$/{ if (p != "") print p " " b; p = ""; b = "" }
    END{ if (p != "") print p " " b }'
}
# The worktree that has the base checked out, or nothing.
base_worktree() { # <name>
  worktrees "$(clone_of "$1")" | awk -v b="$(base_of "$1")" '$2 == b { print $1; exit }'
}
hooks_dir() { # <clone>
  git -C "$1" rev-parse --path-format=absolute --git-path hooks
}

# The journal: one line per event, tab-separated. `start <key> <fields>`
# is written before the act it describes and `done <key> <result>` after.
# Keys are unique; a key started and not done is settled by looking at the
# state itself.
jstart() { # <key> <fields>...
  [ "$(jstate "$1")" = none ] || return 0
  local k=$1; shift
  printf 'start\t%s\t%s\n' "$k" "$*" >> "$JOURNAL"
}
jdone() { printf 'done\t%s\t%s\n' "$1" "${2:-}" >> "$JOURNAL"; }
jstate() {
  [ -f "$JOURNAL" ] || { echo none; return; }
  awk -F'\t' -v k="$1" '$2 == k { s = $1 } END { print (s == "" ? "none" : s == "done" ? "done" : "started") }' "$JOURNAL"
}
jfields() { awk -F'\t' -v k="$1" '$1 == "start" && $2 == k { print $3; exit }' "$JOURNAL"; }
jresult() { awk -F'\t' -v k="$1" '$1 == "done" && $2 == k { print $3; exit }' "$JOURNAL"; }
# Keys started with a prefix, oldest first.
jkeys() { [ -f "$JOURNAL" ] && awk -F'\t' -v p="$1" '$1 == "start" && index($2, p) == 1 { print $2 }' "$JOURNAL" || true; }
