#!/usr/bin/env python3
"""delegate "<task>": start a fresh agent as a new thread in this directory.

The eval harness's stand-in for T3 Code's delegated threads. It reads
.threads.json beside the bin/ it is installed in, gives the new agent its own
T3CODE_THREAD_ID, writes its transcript to the run's threads/ directory,
waits for it and prints its final reply.
"""
import json
import os
import secrets
import subprocess
import sys
from pathlib import Path

conf = json.loads((Path(__file__).resolve().parent.parent / ".threads.json").read_text())
task = " ".join(sys.argv[1:]).strip()
if not task:
    sys.exit('usage: delegate "<task>"')
thread = f"thread-{secrets.token_hex(4)}"
out = Path(conf["out"])
out.mkdir(parents=True, exist_ok=True)
transcript = out / f"{thread}.jsonl"
argv = [arg.replace("{task}", task) for arg in conf["argv"]]
env = {**conf["env"], "T3CODE_THREAD_ID": thread}
print(f"Started thread {thread}; waiting for it to finish.", flush=True)
with transcript.open("w") as f:
    try:
        code = subprocess.run(argv, cwd=os.getcwd(), env=env, stdout=f,
                              stderr=subprocess.STDOUT, timeout=conf["timeout"]).returncode
    except subprocess.TimeoutExpired:
        code = "timeout"
reply = ""
for line in transcript.read_text().splitlines():
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        continue
    item = event.get("item") or {}
    if event.get("type") == "result":
        reply = event.get("result") or ""
    elif event.get("type") == "item.completed" and item.get("type") == "agent_message":
        reply = item.get("text") or ""
print(f"Thread {thread} finished (exit {code}). Its final reply:\n{reply}")
sys.exit(0 if code == 0 else 1)
