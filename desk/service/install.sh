#!/usr/bin/env bash
# Installs or updates the Desk as a systemd user service on this host, from
# the clean checkout this script is in:
#
#   desk/service/install.sh [--caller <name>]
#
# It builds atelier-desk (release, -j 4) and the app (pnpm build), copies
# them and the t3 wrapper to ~/.local/share/atelier-desk/, writes
# ~/.config/atelier-desk/config.toml when it does not exist (which needs
# --caller, the name the human acts under), installs the unit as
# ~/.config/systemd/user/atelier-desk.service, enables it and restarts it,
# and checks that it answers with the commit it was built from. It never
# overwrites an existing configuration. docs/architecture/desk.md, "Running
# it as a service", explains each file.
set -euo pipefail
caller=
case $# in
  0) ;;
  2) [ "$1" = --caller ] && caller=$2 || { echo "usage: $0 [--caller <name>]" >&2; exit 2; } ;;
  *) echo "usage: $0 [--caller <name>]" >&2; exit 2 ;;
esac
here=$(cd "$(dirname "$0")" && pwd)
top=$(cd "$here/../.." && pwd)
share=$HOME/.local/share/atelier-desk
config=$HOME/.config/atelier-desk/config.toml
unit=$HOME/.config/systemd/user/atelier-desk.service
mkdir -p "$HOME/.cache/atelier-desk/tmp" "$share" "$(dirname "$config")" "$(dirname "$unit")"
export TMPDIR=$HOME/.cache/atelier-desk/tmp

# The server reports the commit it was built from, which marks no change
# that is not committed: build only from a clean checkout.
if [ -n "$(git -C "$top" status --porcelain --untracked-files=no)" ]; then
  echo "$top has changes that are not committed; commit or stash them first" >&2
  exit 1
fi
commit=$(git -C "$top" rev-parse HEAD)
if [ ! -e "$config" ] && [ -z "$caller" ]; then
  echo "$config does not exist: give --caller <name>, the name the human acts under" >&2
  exit 2
fi

# The Rust workspace is at the top of the repository.
workspace=$top
echo "Building atelier-desk at $commit"
cargo build -j 4 --release -p atelier-desk --manifest-path "$workspace/Cargo.toml"
echo "Building the app"
(cd "$top/desk" && pnpm install --frozen-lockfile && pnpm build)

echo "Installing into $share"
cp "$workspace/target/release/atelier-desk" "$share/atelier-desk.new"
mv -f "$share/atelier-desk.new" "$share/atelier-desk"
cp "$here/t3" "$share/t3.new"
mv -f "$share/t3.new" "$share/t3"
rm -rf "$share/app.new" "$share/app.old"
cp -r "$top/desk/dist" "$share/app.new"
[ ! -e "$share/app" ] || mv "$share/app" "$share/app.old"
mv "$share/app.new" "$share/app"
rm -rf "$share/app.old"

if [ ! -e "$config" ]; then
  # This host's tailnet address and names, from tailscale itself.
  read -r ip dns host < <(tailscale status --json | python3 -I -c '
import json, sys
me = json.load(sys.stdin)["Self"]
ip = next(a for a in me["TailscaleIPs"] if ":" not in a)
print(ip, me["DNSName"].rstrip("."), me["HostName"])')
  cat > "$config" <<EOF
# The Desk's configuration on this host; docs/architecture/desk.md,
# "Running the server", explains each key. Written by
# desk/service/install.sh, which never overwrites it.
caller = "$caller"
roots = ["~/.cache/atelier-desk/clones"]
listen = ["127.0.0.1:3780", "$ip:3780"]
hosts = ["localhost", "127.0.0.1", "$ip", "$host", "$dns"]
app = "~/.local/share/atelier-desk/app"

[t3]
# The wrapper runs the t3 of the T3 Code version that is running.
command = "~/.local/share/atelier-desk/t3"
url = "http://127.0.0.1:3773/mcp"
EOF
  echo "Wrote $config"
else
  echo "Kept $config"
fi

cp "$here/atelier-desk.service" "$unit"
systemctl --user daemon-reload
systemctl --user enable atelier-desk.service
systemctl --user restart atelier-desk.service

# The first loopback address it listens on.
address=$(python3 -I -c '
import sys, tomllib
listen = tomllib.load(open(sys.argv[1], "rb")).get("listen", ["127.0.0.1:3780"])
print(next((a for a in listen if a.startswith("127.")), listen[0]))' "$config")
for _ in $(seq 1 30); do
  built=$(curl -fsS -H "Host: localhost" "http://$address/api/repos" 2>/dev/null |
    python3 -I -c 'import json, sys; print(json.load(sys.stdin)["built_from"])' 2>/dev/null) && break
  sleep 1
done
if [ "${built:-}" != "$commit" ]; then
  echo "atelier-desk does not answer on $address with $commit (it gave: ${built:-nothing});" \
    "see: journalctl --user -u atelier-desk" >&2
  exit 1
fi
echo "atelier-desk is running $commit on $address and every address in $config"
