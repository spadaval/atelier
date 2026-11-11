#!/usr/bin/env bash
# Rehearses the cutover on fresh scratch clones of the five repositories,
# with the converter and the new binary of the tree this script is in:
#
#   scripts/cutover/rehearse.sh <a new or empty work directory>
#
# Build next/target/debug/atelier first. Each clone's pushes go to a local
# bare repository, Git may use only local files, and `gh` is a double, so
# nothing reaches the network and no real repository is changed. Each clone
# is converted, verified, checked and given Atelier's Git hooks; one item is
# taken from ready to done in atelier and life-on-rails, and 30 items are
# started in astromech to time `status` and `list` with that many branches.
# It prints counts, timings, file names and pass or fail, never a record's
# text; each repository's command output goes to <work>/<repo>.log. It stops
# at the first failure. The clones are left for inspection.
set -euo pipefail
[ $# -eq 1 ] || { echo "usage: $0 <work directory>" >&2; exit 2; }
[ ! -e "$1" ] || [ -z "$(ls -A "$1")" ] || { echo "$1 is not empty: give a new directory" >&2; exit 2; }
here=$(cd "$(dirname "$0")" && pwd)
new=$here/../../next/target/debug/atelier
[ -x "$new" ] || { echo "Build $new first: cargo build -j 4 --manifest-path next/Cargo.toml" >&2; exit 2; }
mkdir -p "$1/bin" "$1/new" "$1/gh" "$1/tmp"
W=$(cd "$1" && pwd)
cp "$new" "$W/new/atelier"
: > "$W/sh.calls"

# `atelier` on PATH is the copy, through a line that logs each call, so a
# hook that ran another `atelier` shows as a missing call.
printf '#!/bin/sh\necho "$*" >> %s/atelier.calls\nexec %s/new/atelier "$@"\n' "$W" "$W" > "$W/bin/atelier"
# `merge` runs each merge check with `sh -c`, found on PATH: this one logs it.
printf '#!/bin/sh\n[ "$1" != -c ] || echo "$2" >> %s/sh.calls\nexec /bin/sh "$@"\n' "$W" > "$W/bin/sh"
# The gh double. A pull request is a file; CI has passed on the commit the
# branch's push URL holds.
cat > "$W/bin/gh" <<EOF
#!/bin/sh
d=$W/gh
echo "\$*" >> \$d/calls
for a; do case \$a in head=*:*) b=\${a#head=*:} ;; head=*|branch=*) b=\${a#*=} ;; esac; done
pull='{"number":1,"html_url":"https://github.com/o/r/pull/1"}'
case "\$*" in
  "auth status"*) ;;
  *"-X POST "*/pulls*) touch "\$d/pr-\$b"; echo "\$pull" ;;
  */pulls*) if [ -e "\$d/pr-\$b" ]; then echo "[\$pull]"; else echo '[]'; fi ;;
  */runs*) sha=\$(git ls-remote "\$(git remote get-url --push origin)" "refs/heads/\$b" | cut -f1)
    echo "{\"total_count\":1,\"workflow_runs\":[{\"id\":1,\"head_sha\":\"\$sha\",\"status\":\"completed\",\"conclusion\":\"success\",\"html_url\":\"https://github.com/o/r/actions/runs/1\",\"created_at\":\"2026-11-10T00:00:00Z\"}]}" ;;
  *) echo "gh double: no answer for: \$*" >&2; exit 1 ;;
esac
EOF
chmod +x "$W/bin/atelier" "$W/bin/gh" "$W/bin/sh"
export PATH="$W/bin:$PATH" TMPDIR="$W/tmp" GIT_ALLOW_PROTOCOL=file CARGO_NET_OFFLINE=true \
  GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL="$W/gitconfig" T3CODE_THREAD_ID=rehearsal-planner
# No harness's thread, no skip and no shortened wait reach the clones.
unset CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID ATELIER_SKIP ATELIER_TEST_CI_WAIT
git config --global user.name rehearsal
git config --global user.email rehearsal@example.invalid
git config --global advice.detachedHead false
[ "$(command -v atelier)" = "$W/bin/atelier" ] && [ "$(command -v gh)" = "$W/bin/gh" ] \
  || { echo "PATH does not put the new atelier and the gh double first" >&2; exit 1; }
echo "new binary: $(du -h "$W/new/atelier" | cut -f1), first on PATH with the gh double; Git limited to local files"

P=rehearsal-planner I=rehearsal-implementer R=rehearsal-reviewer
ms() { echo $(( ($(date +%s%N) - $1) / 1000000 )); }
calls() { grep -c -- "$1" "$W/atelier.calls" || true; }

# step <name> <function> [args]: runs it in the clone, its output to the log,
# and prints one line, or the log's name and stops.
step() {
  local name=$1 t0 rc
  shift
  t0=$(date +%s%N)
  set +e
  (set -e; cd "$C"; "$@" >> "$log" 2>&1)
  rc=$?
  set -e
  if [ $rc -ne 0 ]; then
    printf '%-15s %-22s FAILED (exit %d); see %s\n' "$repo" "$name" $rc "$log"
    exit 1
  fi
  printf '%-15s %-22s ok %6d ms  %s\n' "$repo" "$name" "$(ms "$t0")" "$(cat "$W/note" 2>/dev/null)"
  rm -f "$W/note"
}
note() { echo "$*" > "$W/note"; }

isolate() {
  url=$(git -C "$src" config remote.origin.url)
  git remote set-url origin "$url"
  git init -q --bare -b "$base" "$W/$repo.git"
  git config "url.$W/$repo.git.pushInsteadOf" "$url"
  git config protocol.allow never
  git config protocol.file.allow always
  # Pushes go to the bare repository; the fetch URL keeps the GitHub form
  # the forge code reads, and Git refuses its transport.
  [ "$(git remote)" = origin ]
  [ "$(git remote get-url --push origin)" = "$W/$repo.git" ]
  [ "$(git config --get-regexp '^url\.')" = "url.$W/$repo.git.pushinsteadof $url" ]
  ! git ls-remote origin > "$W/tmp/ls" 2>&1
  grep -q "not allowed" "$W/tmp/ls"
  git push -q origin "$base"
  note "pushes go to $repo.git; fetch transport refused"
}

names_atelier() {
  local f n out=""
  for f in AGENTS.md .github/workflows/*; do
    [ -f "$f" ] || continue
    n=$(grep -c -i -E 'atelier' "$f" || true)
    [ "$n" -eq 0 ] || out="$out $f:$n"
  done
  note "lines naming Atelier:${out:- none}"
}

convert() {
  pre=$(git rev-parse HEAD)
  python3 "$here/convert_records.py"
  git add -A
  git commit -qm "Convert Atelier records to the new layout"
  python3 "$here/convert_records.py" --verify "$pre"
  note "items: $(ls .atelier/items | wc -l), missions: $(ls .atelier/missions 2>/dev/null | wc -l)"
}

# The product clone gets the forge its real repository does not name, so the
# forge path runs there too.
name_forge() {
  printf '\n[forge]\nprovider = "github"\nremote = "origin"\n' >> .atelier/config.toml
  atelier check
  git commit -qam "Name the forge, for the rehearsal"
}

check() { atelier check | tee "$W/tmp/check"; note "$(head -1 "$W/tmp/check")"; }

push() { git push -q origin "$base"; }

hooks() {
  atelier setup
  local d h
  d=$(git rev-parse --path-format=absolute --git-path hooks)
  for h in pre-commit pre-push post-checkout; do [ -x "$d/$h" ]; done
  # A deliberately broken record: the pre-commit hook must refuse it.
  local f head before
  f=$(ls .atelier/items/*/item.md | head -1)
  sed -i 's/^status: .*/status: finished/' "$f"
  head=$(git rev-parse HEAD)
  before=$(calls "check --hook pre-commit")
  ! git commit -qam "Break a record"
  [ "$(git rev-parse HEAD)" = "$head" ]
  [ "$(calls "check --hook pre-commit")" -gt "$before" ]
  git checkout -q -- "$f"
  note "$(cd "$d" && ls | grep -v sample | tr '\n' ' ')installed; broken $(basename "$(dirname "$f")") refused"
}

timed() { # status and list on the base
  local t0 a b
  t0=$(date +%s%N); atelier status > "$W/tmp/status"; a=$(ms "$t0")
  ! grep -q "atelier setup" "$W/tmp/status"
  t0=$(date +%s%N); atelier list > /dev/null; b=$(ms "$t0")
  note "status $a ms, list $b ms, $(git branch | wc -l) local branches"
}

full_item() {
  local id f
  id=$(T3CODE_THREAD_ID=$P atelier new "Say in the README that this clone rehearsed the cutover" \
    | sed -n 's/^Created draft item \([^ ,]*\),.*/\1/p')
  f=.atelier/items/$id/item.md
  sed -i -e '/^## Goal$/a Say in the README that this clone rehearsed the cutover.' \
    -e '/^## Done when$/a - [ ] README.md ends with a line saying so.' \
    -e '/^## Scope$/a README.md.' "$f"
  T3CODE_THREAD_ID=$P atelier ready "$id" --by sumeet
  git add "$f"
  git commit -qm "Plan $id"
  git push -q origin "$base"
  T3CODE_THREAD_ID=$I atelier start "$id"
  echo "This clone rehearsed the Atelier cutover." >> README.md
  git commit -qam "Say that this clone rehearsed the cutover"
  sed -i 's/^- \[ \] README.md/- [x] README.md/' "$f"
  printf '\n## Report\n\nREADME.md ends with the line.\n' >> "$f"
  T3CODE_THREAD_ID=$I atelier submit
  T3CODE_THREAD_ID=$R atelier review "$id"
  local review
  review=$(ls -t .atelier/items/"$id"/reviews/*.md | head -1)
  sed -i '/^## Summary$/a The README gains one line, as the Done when line asks.' "$review"
  T3CODE_THREAD_ID=$R atelier verdict approve
  local checks ran
  checks=$(grep -c '^\[\[merge_check\]\]' .atelier/config.toml || true)
  ran=$(wc -l < "$W/sh.calls")
  T3CODE_THREAD_ID=$I atelier merge
  ran=$(( $(wc -l < "$W/sh.calls") - ran ))
  [ "$ran" -eq "$checks" ]
  git switch -q "$base"
  grep -q '^status: done$' "$f"
  [ "$(git rev-parse "$base")" = "$(git --git-dir="$W/$repo.git" rev-parse "$base")" ]
  note "$id done; $ran merge checks ran; merge commit $(git rev-parse --short "$base") pushed; $(ls "$W/gh" | grep -c '^pr-') pull requests opened in all"
}

start_30() {
  local id n=0
  for id in $(atelier list --ready | sed -n 's/^ *\([a-z0-9-]*-[a-z0-9]\{4\}\) .*/\1/p'); do
    T3CODE_THREAD_ID=$I atelier start "$id"
    git switch -q "$base"
    n=$((n + 1))
    [ $n -lt 30 ] || break
  done
  [ $n -eq 30 ]
  note "30 items started"
}

S=/home/supadava/projects
t_all=$(date +%s%N)
for spec in atelier:master astromech:main demiurge:master life-on-rails:main cosplay-studio:main; do
  repo=${spec%%:*} base=${spec#*:} src=$S/$repo C=$W/$repo log=$W/$repo.log
  git clone -q -b "$base" "$src" "$C"
  step clone isolate
  step "names Atelier" names_atelier
  step convert convert
  step check check
  [ "$repo" != life-on-rails ] || step "forge, for rehearsal" name_forge
  step "push to bare" push
  step "setup and pre-commit" hooks
  step "status and list" timed
  case $repo in
    atelier | life-on-rails) step "item ready to done" full_item ;;
    astromech) step "start 30 items" start_30; step "status and list" timed ;;
  esac
done
echo "all steps passed in $(( $(ms "$t_all") / 1000 )) s; $(wc -l < "$W/gh/calls") gh calls, all to the double"
