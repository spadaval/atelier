#!/usr/bin/env -S python3 -I
"""The run's `atelier`: run the binary under test and log the call.

The harness moves the binary a fixture copied to work/bin/atelier into
work/libexec/ and puts this script in its place, so every `atelier` call a
run makes, from any thread or from a Git hook, runs the same binary and
leaves one line in work/.atelier-calls.jsonl: its arguments, directory,
exit status and time, the variables that name its caller, and the thread
Atelier takes them to name (IDENTITY). The harness reads the failed calls
from it rather than parse a shell command out of a transcript.
"""
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

# What names the caller, in the order crates/atelier-core/src/identity.rs
# reads it.
VARIABLES = ("T3CODE_THREAD_ID", "CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID",
             "GROK_SESSION_ID")


def identity(env):
    """The calling thread as Atelier names it: the first of VARIABLES set,
    else human; a Codex subagent, whose CODEX_THREAD_ID differs from the
    CODEX_SESSION_ID it shares with its parent, is that, `/`, its own."""
    def set_(name):
        value = env.get(name) or ""
        return value if value.strip() else None
    parent = next((set_(n) for n in VARIABLES if set_(n)), "human")
    child, session = set_("CODEX_THREAD_ID"), set_("CODEX_SESSION_ID")
    return f"{parent}/{child}" if child and session and child != session else parent


def main():
    work = Path(__file__).resolve().parent.parent
    start = time.time()
    proc = subprocess.Popen([str(work / "libexec" / "atelier"), *sys.argv[1:]])
    for sig in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(sig, lambda s, _: proc.send_signal(s))
    code = proc.wait()
    code = 128 - code if code < 0 else code
    with open(work / ".atelier-calls.jsonl", "a") as log:
        log.write(json.dumps({
            "argv": sys.argv[1:], "cwd": os.getcwd(), "thread": identity(os.environ),
            "env": {n: os.environ[n] for n in (*VARIABLES, "CODEX_THREAD_ID")
                    if n in os.environ},
            "exit": code, "start": start,
            "seconds": round(time.time() - start, 2)}) + "\n")
    sys.exit(code)


if __name__ == "__main__":
    main()
