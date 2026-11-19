#!/usr/bin/env -S python3 -I
"""The run's `atelier`: run the binary under test and log the call.

The harness moves the binary a fixture copied to work/bin/atelier into
work/libexec/ and puts this script in its place, so every `atelier` call a
run makes, from any thread or from a Git hook, runs the same binary and
leaves one line in work/.atelier-calls.jsonl: its arguments, directory,
thread, exit status and time. The harness reads the failed calls from it
rather than parse a shell command out of a transcript.
"""
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

work = Path(__file__).resolve().parent.parent
start = time.time()
proc = subprocess.Popen([str(work / "libexec" / "atelier"), *sys.argv[1:]])
for sig in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
    signal.signal(sig, lambda s, _: proc.send_signal(s))
code = proc.wait()
code = 128 - code if code < 0 else code
with open(work / ".atelier-calls.jsonl", "a") as log:
    log.write(json.dumps({
        "argv": sys.argv[1:], "cwd": os.getcwd(),
        "thread": os.environ.get("T3CODE_THREAD_ID"), "exit": code,
        "start": start, "seconds": round(time.time() - start, 2)}) + "\n")
sys.exit(code)
