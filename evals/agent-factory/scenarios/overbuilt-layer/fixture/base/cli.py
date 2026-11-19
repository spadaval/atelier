"""python3 cli.py add TEXT | list"""
import argparse
import sys

import notes


def main(argv=None):
    parser = argparse.ArgumentParser(prog="cli.py")
    parser.add_argument("--file", default="notes.json")
    sub = parser.add_subparsers(dest="command", required=True)
    add = sub.add_parser("add", help="add a note")
    add.add_argument("text")
    sub.add_parser("list", help="list every note")
    args = parser.parse_args(argv)

    if args.command == "add":
        print(notes.add(args.file, args.text))
    elif args.command == "list":
        for n in notes.load(args.file):
            tags = "".join(f" #{t}" for t in n["tags"])
            print(f"{n['id']}: {n['text']}{tags}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
