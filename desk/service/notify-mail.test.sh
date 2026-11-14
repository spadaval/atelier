#!/usr/bin/env bash
# Checks notify-mail against a gog stand-in: it looks the address up by
# alias, and sends one mail with the Desk's title, text and link. Nothing
# is sent.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d "${XDG_CACHE_HOME:-$HOME/.cache}/notify-mail-test.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
cat > "$scratch/gog" <<'STUB'
#!/usr/bin/env bash
case "$1 $2 $3" in
  "auth alias list") printf 'ALIAS\tEMAIL\nhome\tme@example.test\n' ;;
  "gmail send --no-input") printf '%s\n' "$@" > "$GOG_LOG" ;;
  *) echo "unexpected: $*" >&2; exit 1 ;;
esac
STUB
chmod +x "$scratch/gog"
export PATH="$scratch:$PATH" GOG_LOG="$scratch/log" GOG_KEYRING_PASSWORD=x
"$here/notify-mail" home "Plan proposed: X" "Mission demo-aaaa in demo is proposed." "http://desk/r/demo/mission/demo-aaaa"
expected='gmail
send
--no-input
--account
home
--to
me@example.test
--subject
Desk: Plan proposed: X
--body
Mission demo-aaaa in demo is proposed.

http://desk/r/demo/mission/demo-aaaa'
[ "$(cat "$scratch/log")" = "$expected" ] || { echo "notify-mail sent:"; cat "$scratch/log"; exit 1; } >&2
if "$here/notify-mail" nobody t x l 2>/dev/null; then echo "an unknown alias must fail" >&2; exit 1; fi
echo "notify-mail: ok"
