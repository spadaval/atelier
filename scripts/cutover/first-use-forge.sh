#!/usr/bin/env bash
# The new binary's first questions to real GitHub, read only:
#
#   scripts/cutover/first-use-forge.sh <work directory> <debug build of atelier> <branch>
#
# <branch> is a branch of spadaval/atelier with an open pull request and a
# finished CI run, named for an item that is ready in master's records. A
# clone of this host's atelier master is converted, the item started,
# submitted, reviewed and merged by the new binary, which asks the forge what
# `submit`, `review` and `merge` ask. The clone's fetch URL is GitHub's and
# its pushes go to a local bare repository; Git may use only local files; and
# `gh` is a wrapper that passes `auth status` and GET requests to the real
# `gh` and refuses everything else, so nothing is pushed to or opened on
# GitHub. `merge`'s wait is shortened (ATELIER_TEST_CI_WAIT, debug builds
# only): it stops with exit status 3 and says what the last answer said.
set -euo pipefail
[ $# -eq 3 ] || { echo "usage: $0 <work directory> <debug atelier> <branch>" >&2; exit 2; }
mkdir -p "$1"; W=$(cd "$1" && pwd); BIN=$2 BRANCH=$3
[ -z "$(ls -A "$W")" ] || { echo "$W is not empty" >&2; exit 2; }
KIT=$(cd "$(dirname "$0")" && pwd)
REAL_GH=$(command -v gh)
mkdir -p "$W/bin" "$W/tmp"
cp "$BIN" "$W/bin/atelier"
cat > "$W/bin/gh" <<EOF
#!/bin/sh
echo "\$*" >> $W/gh.calls
case "\$*" in
  "auth status "*) exec $REAL_GH "\$@" ;;
  "api --hostname github.com -X GET "*) n=\$(wc -l < $W/gh.calls); $REAL_GH "\$@" | tee $W/gh.answer.\$n ;;
  *) echo "refused by the read-only gh wrapper: \$*" | tee -a $W/gh.refused >&2; exit 1 ;;
esac
EOF
chmod +x "$W/bin/gh"
export PATH="$W/bin:$PATH" TMPDIR=$W/tmp GIT_ALLOW_PROTOCOL=file GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=$W/gitconfig
unset T3CODE_THREAD_ID CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID CODEX_THREAD_ID ATELIER_SKIP
git config --global user.name first-use; git config --global user.email first-use@example.invalid
git config --global advice.detachedHead false

C=$W/atelier
git clone -q -b master /home/supadava/projects/atelier "$C"
git init -q --bare "$W/push.git"
git -C "$C" remote set-url origin https://github.com/spadaval/atelier.git
git -C "$C" remote set-url --push origin "$W/push.git"
cd "$C"
python3 "$KIT/convert_records.py" > "$W/convert.log"
git add -A && git commit -qm "Convert, for the forge's first use"
f=.atelier/items/$BRANCH/item.md
[ -f "$f" ] || { echo "$BRANCH is not an item in master's converted records" >&2; exit 1; }
T3CODE_THREAD_ID=first-use-implementer atelier start "$BRANCH" > "$W/start.log"
sed -i 's/^- \[ \] /- [x] /' "$f"
grep -q '^## Report' "$f" || printf '\n## Report\n\nThe forge'"'"'s first use: nothing here is real work.\n' >> "$f"
git commit -qam "Tick the lines, for the forge's first use"
echo "== submit"; T3CODE_THREAD_ID=first-use-implementer atelier submit 2>&1 | tee "$W/submit.log"
echo "== review"; T3CODE_THREAD_ID=first-use-reviewer atelier review "$BRANCH" > "$W/review.log" 2>&1
grep -i -E 'pull request|github.com' "$W/review.log" | head -3
review=$(ls -t .atelier/items/"$BRANCH"/reviews/*.md | head -1)
sed -i '/^## Summary$/a The forge'"'"'s first use.' "$review"
T3CODE_THREAD_ID=first-use-reviewer atelier verdict approve > "$W/verdict.log" 2>&1
echo "== merge"
set +e
T3CODE_THREAD_ID=first-use-implementer ATELIER_TEST_CI_WAIT=20000,5000 atelier merge 2>&1 | tee "$W/merge.log"
rc=${PIPESTATUS[0]}
set -e
echo "merge exit status: $rc"
echo "== gh calls"; sed 's/^/  /' "$W/gh.calls"
echo "== what the forge answered"
for a in "$W"/gh.answer.*; do
  jq -r 'if type == "array" then "pull requests: \(length); \(map("#\(.number) \(.head.ref) \(.state)") | join(", "))"
         else "CI runs: \(.total_count); latest: \(.workflow_runs[0] | "\(.head_sha[0:8]) \(.status) \(.conclusion)")" end' "$a" | sed 's/^/  /'
done | sort | uniq -c
[ ! -s "$W/gh.refused" ] && echo "refused: none" || { echo "refused:"; cat "$W/gh.refused"; }
echo "pushed to the local bare repository: $(git --git-dir="$W/push.git" for-each-ref --format='%(refname:short)' | tr '\n' ' ')"
