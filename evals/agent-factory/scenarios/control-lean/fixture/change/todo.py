"""Tasks kept in todo.json: a list of {"id", "title", "done"}.

    python3 todo.py add "Fix the gate"
    python3 todo.py list
    python3 todo.py done 1
"""
import argparse
import json
import sys
from pathlib import Path


def load(path):
    path = Path(path)
    return json.loads(path.read_text()) if path.exists() else []


def save(path, tasks):
    Path(path).write_text(json.dumps(tasks, indent=2) + "\n")


def main(argv=None):
    parser = argparse.ArgumentParser(prog="todo.py")
    parser.add_argument("--file", default="todo.json")
    sub = parser.add_subparsers(dest="command", required=True)
    add = sub.add_parser("add", help="add a task")
    add.add_argument("title")
    sub.add_parser("list", help="list the open tasks")
    done = sub.add_parser("done", help="mark a task done")
    done.add_argument("id", type=int)
    args = parser.parse_args(argv)

    tasks = load(args.file)
    if args.command == "add":
        task_id = max((t["id"] for t in tasks), default=0) + 1
        tasks.append({"id": task_id, "title": args.title, "done": False})
        save(args.file, tasks)
        print(task_id)
    elif args.command == "list":
        for t in tasks:
            if not t["done"]:
                print(f"{t['id']}. {t['title']}")
    elif args.command == "done":
        task = next((t for t in tasks if t["id"] == args.id), None)
        if task is None:
            print(f"no task {args.id}", file=sys.stderr)
            return 1
        task["done"] = True
        save(args.file, tasks)
    return 0


if __name__ == "__main__":
    sys.exit(main())
