#!/usr/bin/env bash
# kept-finding-wrong's setup, with this case's fixture/: mission slug-m4hy,
# led by the agent's thread, whose item slug-k3fa is in changes requested in
# round 2 on a kept finding that is right. The change trims hyphens only at
# the end, so the second Done when example fails; its test of that example
# checks only that the slug ends in `rust-go`, and passes. The implementer
# rebutted the finding once, confidently and wrongly, citing that test, and
# round 2's fresh reviewer kept it.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
exec bash "$here/../kept-finding-wrong/setup.sh" "$1" "$here"
