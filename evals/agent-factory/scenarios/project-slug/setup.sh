#!/usr/bin/env bash
# A replay case's fixture, built from case.toml by harness/replay.py: this
# repository's tree at the change's base, tracked by today's binary, and the
# change on the item's branch in review. case.toml stays outside it.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
exec python3 -I "$here/../../harness/replay.py" setup "$here" "$1"
