use atelier_records::{is_record, load};
use std::path::Path;

fn item(id: &str, blocked_by: &str) -> Vec<u8> {
    format!(
        "---\nschema: atelier.item\nschema_version: 3\nid: {id}\ntitle: Say hello\n\
         status: ready\nmission: null\nlabels: []\npriority: high\nblocked_by: {blocked_by}\n\
         implementers: []\nready: null\ntrivial: null\ncaused_by: null\n\
         created_at: 2026-11-10T09:00:00Z\n---\n\n## Goal\n\nHello.\n\n## Done when\n\n- [ ] It says hello.\n"
    )
    .into_bytes()
}

fn file(path: &str, content: impl Into<Vec<u8>>) -> (String, Vec<u8>) {
    (path.to_string(), content.into())
}

#[test]
fn only_records_are_read() {
    for path in [
        ".atelier/config.toml",
        ".atelier/items/demo-ab12/item.md",
        ".atelier/items/demo-ab12/reviews/1-x7qd.md",
        ".atelier/missions/demo-mm11/mission.md",
        ".atelier/missions/demo-mm11/reviews/1-x7qd.md",
    ] {
        assert!(is_record(path), "{path}");
    }
    for path in [
        ".atelier/missions/demo-mm11/notes.md",
        ".atelier/items/demo-ab12/artifacts/shot.png",
        ".atelier/items/demo-ab12/reviews/notes.txt",
        "items/demo-ab12/item.md",
        "README.md",
    ] {
        assert!(!is_record(path), "{path}");
    }
}

#[test]
fn a_file_that_does_not_read_leaves_the_others_read_and_validated() {
    let loaded = load([
        file(
            ".atelier/items/demo-ab12/item.md",
            item("demo-ab12", "[demo-zz99]"),
        ),
        file(".atelier/items/demo-ab12/reviews/1-x7qd.md", [0xff, 0xfe]),
        file(".atelier/items/demo-cd34/item.md", "no front matter\n"),
        file(
            ".atelier/items/demo-ef56/reviews/1-x7qd.md",
            "---\nround: 1\n---\n",
        ),
        file(".atelier/missions/demo-mm11/notes.md", "Just notes.\n"),
        file(".atelier/items/demo-ab12/artifacts/shot.png", [0x89, 0xff]),
    ]);
    let shown = |problems: &[atelier_records::Problem]| -> Vec<String> {
        problems.iter().map(|p| p.to_string()).collect()
    };
    assert_eq!(
        shown(&loaded.unread),
        [
            ".atelier/items/demo-ab12/reviews/1-x7qd.md: the file is not UTF-8",
            ".atelier/items/demo-cd34/item.md: front matter: the file does not start with front matter between `---` lines",
            ".atelier/items/demo-ef56/reviews/1-x7qd.md: front matter: missing field `schema`",
            ".atelier/config.toml: the file is missing",
            ".atelier/items/demo-ef56/item.md: its directory has reviews and no item.md",
        ]
    );
    assert_eq!(loaded.items.len(), 1);
    assert_eq!(
        loaded.items[0].path,
        Path::new(".atelier/items/demo-ab12/item.md")
    );
    assert_eq!(
        shown(&loaded.invalid),
        [".atelier/items/demo-ab12/item.md: blocked_by: `demo-zz99` names no item"]
    );
}
