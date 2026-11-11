# Shared by the setup.sh of the next-* scenarios: `source` it, then call
# next_repo. NEXT_ATELIER names the new binary, copied to work/bin/atelier,
# which the harness puts first on PATH.
: "${NEXT_ATELIER:?set NEXT_ATELIER to the new atelier binary}"
export GIT_AUTHOR_NAME="Dana Dev" GIT_AUTHOR_EMAIL=dana@example.com
export GIT_COMMITTER_NAME="Dana Dev" GIT_COMMITTER_EMAIL=dana@example.com
export PYTHONDONTWRITEBYTECODE=1
unset ATELIER_SKIP CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID CODEX_THREAD_ID

# next_repo <work> <slug>: work/repo on main, tracked with the new layout,
# with one merge check (the suite), and the shell left in it.
next_repo() {
    mkdir -p "$1/bin"
    cp "$NEXT_ATELIER" "$1/bin/atelier"
    export PATH="$1/bin:$PATH"
    git init -q -b main "$1/repo"
    cd "$1/repo"
    # Agents commit too; give the clone an identity that is nobody's.
    git config user.name "Eval Agent"
    git config user.email agent@example.com
    mkdir -p .atelier/items
    printf 'schema_version = 3\nslug = "%s"\nbase_branch = "main"\n\n[[merge_check]]\nrun = "python3 -m unittest -q"\n' \
        "$2" > .atelier/config.toml
    printf '__pycache__/\n' > .gitignore
}

# as <thread> <atelier args...>: run atelier as that thread, quietly.
as() { T3CODE_THREAD_ID=$1 atelier "${@:2}" >/dev/null; }

# item <id> <status> <title> [mission] [labels]: write an item record whose
# body is read from stdin. Any status but draft records Dana's go-ahead.
item() {
    local ready="null"
    [ "$2" = draft ] || ready=$'\n  by: dana\n  at: 2026-11-09T09:00:00Z'
    mkdir -p ".atelier/items/$1"
    { printf -- '---\nschema: atelier.item\nschema_version: 3\nid: %s\ntitle: %s\nstatus: %s\n' "$1" "$3" "$2"
      printf 'mission: %s\nlabels: %s\npriority: high\nblocked_by: []\nimplementers: []\n' "${4:-null}" "${5:-[]}"
      printf 'ready: %s\ntrivial: null\ncaused_by: null\ncreated_at: 2026-11-09T09:00:00Z\n---\n\n' "$ready"
      cat; } > ".atelier/items/$1/item.md"
}

# planned <message>: commit planning on the base if anything is left
# uncommitted, so a fixture runs to its end with a binary that commits
# `ready`, `drop`, `close` and `lead` itself (atelier-psjw) or one that does not.
planned() {
    git add -A .atelier
    git diff --cached --quiet || git commit -qm "$1"
}
