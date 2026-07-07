#!/usr/bin/env bash
set -euo pipefail

# Compatibility entry point for the original quality-only guard. The active
# command-guidance guard now scans every in-scope document reachable from
# docs/index.md, including the complete quality index.
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
exec "$repo_root/scripts/check_active_command_guidance.sh" "$@"
