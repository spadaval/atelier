#!/usr/bin/env python3
"""delegate "<task>": start a fresh agent as a new thread in this directory.

The eval harness's stand-in for T3 Code's delegated threads. It reads
.threads.json beside the bin/ it is installed in, gives the new agent its own
T3CODE_THREAD_ID, writes its transcript to the run's threads/ directory,
waits for it and prints its final reply.

It keeps the harness's limit of two agents per run: a thread it started
cannot call it, and it holds a lock while its child runs, so a second call
waits for the first child to end. It logs each child's start and end, with
the child's process group, which the harness kills if the run ends first.
"""
import fcntl
import json
import os
import secrets
import signal
import subprocess
import sys
import time
from pathlib import Path

CHILD = "AF_EVAL_DELEGATED"


def log(conf, **entry):
    with open(conf["log"], "a") as f:
        f.write(json.dumps({**entry, "time": time.time()}) + "\n")


def main():
    if os.environ.get(CHILD):
        sys.exit("delegate refused: a child starts no thread in this harness, so "
                 "say in your final reply what you need, and the thread that "
                 "started you can start it.")
    conf = json.loads((Path(__file__).resolve().parent.parent / ".threads.json").read_text())
    task = " ".join(sys.argv[1:]).strip()
    if not task:
        sys.exit('usage: delegate "<task>"')
    lock = open(conf["lock"], "w")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print("Another thread of this run is still working; waiting for it to "
              "finish first.", flush=True)
        fcntl.flock(lock, fcntl.LOCK_EX)
    thread = f"thread-{secrets.token_hex(4)}"
    out = Path(conf["out"])
    out.mkdir(parents=True, exist_ok=True)
    transcript = out / f"{thread}.jsonl"
    argv = [arg.replace("{task}", task) for arg in conf["argv"]]
    env = {**conf["env"], "T3CODE_THREAD_ID": thread, CHILD: "1"}
    print(f"Started thread {thread}; waiting for it to finish.", flush=True)
    with transcript.open("w") as f:
        proc = subprocess.Popen(argv, cwd=os.getcwd(), env=env, stdin=subprocess.DEVNULL,
                                stdout=f, stderr=subprocess.STDOUT, start_new_session=True)
        log(conf, event="start", thread=thread, pgid=proc.pid)
        try:
            code = proc.wait(timeout=conf["timeout"])
        except subprocess.TimeoutExpired:
            code = "timeout"
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        proc.wait()
        log(conf, event="end", thread=thread)
    lock.close()
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


if __name__ == "__main__":
    main()
