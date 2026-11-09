#!/usr/bin/env bash
# Serves the fixture repositories the Rust API tests use (alpha and beta)
# with a built `atelier`, for the Playwright suite.
#
# ATELIER_BIN names the binary (default: the workspace's target/debug/atelier,
# which embeds web/dist when it was built after `pnpm build`). PORT is the
# port to serve on.
set -euo pipefail
web=$(cd "$(dirname "$0")/.." && pwd)
workspace=$(cd "$web/../../.." && pwd)
bin=${ATELIER_BIN:-${CARGO_TARGET_DIR:-$workspace/target}/debug/atelier}
port=${PORT:-7342}

# Fixture repositories are Git repositories, so they live outside every
# checkout: under the cache, never under the workspace.
fleet=$(mktemp -d "${XDG_CACHE_HOME:-$HOME/.cache}/atelier-e2e-fleet.XXXXXX")
trap 'rm -rf "$fleet"' EXIT
(cd "$workspace" && ATELIER_FIXTURE_FLEET=$fleet cargo test -q -p atelier-cli --test serve -- \
  --ignored --exact serve::api::write_fixture_fleet) >&2
"$bin" serve --bind "127.0.0.1:$port" --root "$fleet"
