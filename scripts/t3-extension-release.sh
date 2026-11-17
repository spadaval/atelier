#!/usr/bin/env bash
# Steps of the T3 Code extension's release (.github/workflows/t3-extension-release.yml).
#
# Usage:
#   scripts/t3-extension-release.sh check <commit> [<master>]
#   scripts/t3-extension-release.sh notes <tag>
#   scripts/t3-extension-release.sh bundle <version> <out-dir>
#   scripts/t3-extension-release.sh verify <zip> <version>
#
# check refuses a commit that is not on <master>'s first-parent history
# (default origin/master), then finds the tested commit: the newest commit on
# that history, at or before <commit>, for which scripts/ci-jobs.sh runs the
# `extension` job when given the commit and its first parent, as CI does on
# master. It accepts only when that commit's CI run on master concluded
# success with the `extension` job run in it and not skipped, and prints the
# tested commit. The path rule is read from ci-jobs.sh, never copied here.
# It asks GitHub with `gh api`, for $GITHUB_REPOSITORY or the current
# repository.
#
# notes prints the release notes for <tag>: the merges on master's
# first-parent history that changed t3-extension/, since the previous
# t3-extension-v* tag, or for the first release since the import, the oldest
# commit that changed t3-extension/, which is left out.
#
# bundle builds t3-extension/ at HEAD (pnpm install --frozen-lockfile, then
# pnpm build) and writes t3-extension-<version>.zip, holding under
# t3-extension-<version>/ the files of t3-extension/ at HEAD but the build's
# inputs (src/ and the files that configure the build), the built
# dist/custom.js and a VERSION file, and its .sha256 beside it. It needs Node
# and pnpm.
#
# verify unpacks a bundle, compares its files with what bundle puts in from
# t3-extension/ at HEAD, checks it holds dist/custom.js and nothing else under
# dist/, and installs it into an empty home with install.py, naming with --app
# a stand-in for T3 Code's app whose executable runs Node, as a Linux runner
# has no T3 Code. The install and the installed `t3-custom check`, which must
# report the bundle's version, run with nothing on PATH, so they need no Node
# but T3's executable, as on a user's machine.
set -euo pipefail

scripts=$(cd "$(dirname "$0")" && pwd)

refuse() {
  echo "t3-extension-release: $*" >&2
  exit 1
}

# What the build reads, which a bundle leaves out: it holds the build instead.
build_inputs=(src package.json pnpm-lock.yaml tsconfig.json vite.config.ts)

# extension_files <dir>: t3-extension/ at HEAD in <dir>, but the build's inputs.
extension_files() {
  git archive HEAD:t3-extension | tar -x -C "$1"
  (cd "$1" && rm -rf "${build_inputs[@]}")
}

check() {
  local commit master tested=
  commit=$(git rev-parse --verify "${1:?a commit}^{commit}")
  master=${2:-origin/master}
  # grep reads to the end: with -q it would exit early and fail the pipe.
  git rev-list --first-parent "$master" | grep -x "$commit" >/dev/null ||
    refuse "$commit is not a merge on $master's first-parent history; tag a commit of master."

  local candidate
  while IFS= read -r candidate; do
    local base
    base=$(git rev-parse -q --verify "$candidate^1" || true)
    local jobs
    jobs=$("$scripts/ci-jobs.sh" "$base" "$candidate" 2>/dev/null) ||
      refuse "scripts/ci-jobs.sh failed for $candidate."
    case $(printf '%s\n' "$jobs" | sed -n 's/^extension=//p') in
      true)
        tested=$candidate
        break
        ;;
      false) ;;
      *) refuse "scripts/ci-jobs.sh printed no extension= line for $candidate." ;;
    esac
  done < <(git rev-list --first-parent "$commit")
  [ -n "$tested" ] || refuse "no commit at or before $commit runs the extension's job."
  echo "t3-extension-release: tested commit is $tested ($(git log -1 --format=%s "$tested"))." >&2

  local repo runs
  repo=${GITHUB_REPOSITORY:-$(gh repo view --json nameWithOwner --jq .nameWithOwner)}
  # The CI workflow's push runs for the tested commit, newest first.
  runs=$(gh api "repos/$repo/actions/runs?head_sha=$tested&event=push&per_page=100" \
    --jq '.workflow_runs[] | select(.path == ".github/workflows/ci.yml") | "\(.id) \(.status) \(.conclusion)"')
  [ -n "$runs" ] || refuse "no CI run on master for the tested commit $tested."
  local id status conclusion job
  while read -r id status conclusion; do
    if [ "$status" != completed ]; then
      echo "t3-extension-release: CI run $id for $tested has not finished ($status)." >&2
      continue
    fi
    if [ "$conclusion" != success ]; then
      echo "t3-extension-release: CI run $id for $tested concluded $conclusion." >&2
      continue
    fi
    job=$(gh api "repos/$repo/actions/runs/$id/jobs" --jq '.jobs[] | select(.name == "extension") | .conclusion')
    if [ "$job" = success ]; then
      echo "t3-extension-release: CI run $id passed with the extension's job." >&2
      echo "$tested"
      return 0
    fi
    echo "t3-extension-release: CI run $id for $tested concluded success, but its extension job was ${job:-absent}." >&2
  done <<< "$runs"
  refuse "CI did not pass with the extension's job on the tested commit $tested; re-run this workflow once it has."
}

notes() {
  local tag previous range
  tag=${1:?a tag}
  previous=$(git describe --tags --abbrev=0 --first-parent --match 't3-extension-v*' "$tag^" 2>/dev/null || true)
  local merges
  if [ -n "$previous" ]; then
    echo "Items merged since $previous that changed the extension:"
    merges=$(git log --first-parent --format=%s "$previous..$tag" -- t3-extension/)
  else
    # The oldest commit that changed t3-extension/ is the import itself.
    echo "Items merged since the extension was imported into this repository:"
    merges=$(git log --first-parent --format=%s "$tag" -- t3-extension/ | sed '$d')
  fi
  echo
  printf '%s\n' "$merges" | sed -nE 's/^Merge ([a-z]+-[a-z0-9]+): (.*)$/- \1: \2/p'
}

bundle() {
  local version out name stage
  version=${1:?a version}
  out=${2:?an output directory}
  name=t3-extension-$version
  mkdir -p "$out"
  out=$(cd "$out" && pwd)
  stage=$(mktemp -d)
  trap 'rm -rf "$stage"' RETURN
  mkdir "$stage/build" "$stage/$name" "$stage/$name/dist"
  git archive HEAD:t3-extension | tar -x -C "$stage/build"
  (cd "$stage/build" && pnpm install --frozen-lockfile && pnpm build) >&2 ||
    refuse "building t3-extension/ at $(git rev-parse HEAD) failed."
  extension_files "$stage/$name"
  cp "$stage/build/dist/custom.js" "$stage/$name/dist/custom.js"
  printf '%s\n' "$version" > "$stage/$name/VERSION"
  rm -f "$out/$name.zip"
  (cd "$stage" && python3 -m zipfile -c "$out/$name.zip" "$name")
  (cd "$out" && sha256sum "$name.zip" > "$name.zip.sha256")
  echo "$out/$name.zip"
}

verify() {
  local zip version work name
  zip=$(cd "$(dirname "${1:?a bundle}")" && pwd)/$(basename "$1")
  version=${2:?a version}
  name=t3-extension-$version
  work=$(mktemp -d)
  trap 'rm -rf "$work"' RETURN
  mkdir "$work/bundle" "$work/tree" "$work/home"
  (cd "$(dirname "$zip")" && sha256sum -c --quiet "$(basename "$zip").sha256") ||
    refuse "the bundle does not match its checksum."
  python3 -m zipfile -e "$zip" "$work/bundle"
  extension_files "$work/tree"
  printf '%s\n' "$version" > "$work/tree/VERSION"
  diff -r -x dist "$work/tree" "$work/bundle/$name" ||
    refuse "the bundle's files differ from t3-extension/ at HEAD without ${build_inputs[*]}."
  [ "$(ls -A "$work/bundle/$name/dist" 2>/dev/null)" = custom.js ] ||
    refuse "the bundle's dist/ holds $(ls -A "$work/bundle/$name/dist" 2>/dev/null | paste -sd' ' || true), not the built custom.js alone."
  echo "t3-extension-release: the bundle holds t3-extension/ at $(git rev-parse HEAD) without its build's inputs, dist/custom.js and VERSION." >&2

  # The stand-in names Node by its path, as T3's own executable carries it.
  local app="$work/T3 Code.app" node python
  node=$(command -v node) || refuse "verify needs Node for the stand-in for T3 Code's executable."
  python=$(command -v python3)
  mkdir -p "$app/Contents/MacOS" "$work/empty"
  "$python" -c 'import plistlib, sys; plistlib.dump({"CFBundleExecutable": "T3 Code"}, open(sys.argv[1], "wb"))' \
    "$app/Contents/Info.plist"
  printf '#!/bin/sh\nexec %q "$@"\n' "$node" > "$app/Contents/MacOS/T3 Code"
  chmod +x "$app/Contents/MacOS/T3 Code"
  (cd "$work/bundle/$name" &&
    HOME=$work/home PATH=$work/empty "$python" install.py --home "$work/home/T3 Custom" --app "$app") >&2
  local report
  report=$(PATH=$work/empty "$work/home/T3 Custom/t3-custom" check)
  echo "t3-extension-release: installed t3-custom check: $report" >&2
  [ "$(printf '%s' "$report" | jq -r '.valid and .release == $v' --arg v "$version")" = true ] ||
    refuse "the installed extension did not report release $version."
}

command=${1:-}
shift || true
case $command in
  check | notes | bundle | verify) "$command" "$@" ;;
  *) refuse "usage: $0 check <commit> [<master>] | notes <tag> | bundle <version> <out-dir> | verify <zip> <version>" ;;
esac
