"""python3 cli.py add TEXT | list [--tag TAG] | tag ID TAG"""
import argparse
import sys

import notes
from repository import JsonNoteRepository
from service import NoteNotFound, NoteService


def make_service(path):
    """The note service over the JSON file at `path`."""
    return NoteService(JsonNoteRepository(path))


def show(note):
    tags = "".join(f" #{t}" for t in note["tags"])
    print(f"{note['id']}: {note['text']}{tags}")


def main(argv=None):
    parser = argparse.ArgumentParser(prog="cli.py")
    parser.add_argument("--file", default="notes.json")
    sub = parser.add_subparsers(dest="command", required=True)
    add = sub.add_parser("add", help="add a note")
    add.add_argument("text")
    listing = sub.add_parser("list", help="list every note, or those with a tag")
    listing.add_argument("--tag")
    tag = sub.add_parser("tag", help="tag a note")
    tag.add_argument("id", type=int)
    tag.add_argument("tag")
    args = parser.parse_args(argv)

    if args.command == "add":
        print(notes.add(args.file, args.text))
    elif args.command == "list":
        found = (make_service(args.file).with_tag(args.tag) if args.tag
                 else notes.load(args.file))
        for n in found:
            show(n)
    elif args.command == "tag":
        try:
            make_service(args.file).tag(args.id, args.tag)
        except NoteNotFound as e:
            print(e, file=sys.stderr)
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
