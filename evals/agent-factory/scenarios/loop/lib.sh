# Shared by the scenarios' setup.sh: `source` it, then call next_repo.
# EVAL_ATELIER names the atelier binary under test (a copy of one built from
# this tree), copied to work/bin/atelier, which the harness puts first on PATH.
: "${EVAL_ATELIER:?set EVAL_ATELIER to the atelier binary under test}"
export GIT_AUTHOR_NAME="Dana Dev" GIT_AUTHOR_EMAIL=dana@example.com
export GIT_COMMITTER_NAME="Dana Dev" GIT_COMMITTER_EMAIL=dana@example.com
export PYTHONDONTWRITEBYTECODE=1
unset ATELIER_SKIP CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID CODEX_THREAD_ID

# next_repo <work> <slug>: work/repo on main, tracked by Atelier,
# with one merge check (the suite), and the shell left in it.
next_repo() {
    mkdir -p "$1/bin"
    cp "$EVAL_ATELIER" "$1/bin/atelier"
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

# begin <thread> <id>: start item <id> as <thread> and switch this checkout
# to its branch. In the base's checkout `start` makes the branch and leaves
# the checkout as it is; a fixture works on in one checkout, as the item's
# implementer would in its own.
begin() { as "$1" start "$2"; git switch -q "$2"; }

# judge <thread> <verdict args...>: record <thread>'s verdict on the review
# it opened in this checkout, and commit the review and the item together,
# as the command it prints says to.
judge() {
    T3CODE_THREAD_ID=$1 atelier verdict "${@:2}" >/dev/null
    git add -A .atelier
    git commit -qm "Review: ${2}"
}

# backlog <thread> <slug>: start backlog/<slug> as <thread> and switch to it,
# for record changes tied to no item or mission, which main takes only by a
# merge. merged <thread> <message> commits them and merges the branch, which
# needs no review, and leaves the checkout on main.
backlog() { as "$1" start --backlog "$2"; git switch -q "backlog/$2"; }
merged() {
    git add -A .atelier
    git commit -qm "$2"
    as "$1" merge
    git switch -q main
}

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

# planned <message>: commit planning records left uncommitted (`ready`,
# `drop`, `close` and `lead` commit nothing), if there are any.
planned() {
    git add -A .atelier
    git diff --cached --quiet || git commit -qm "$1"
}
