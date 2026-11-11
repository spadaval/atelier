//! The app's TypeScript types, committed in `desk/src/api/`, are what
//! `ts-rs` generates from the response types now.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn files(dir: &Path) -> BTreeMap<PathBuf, String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return BTreeMap::new();
    };
    let paths = entries.flatten().map(|entry| entry.path());
    paths
        .map(|path| {
            let text = fs::read_to_string(&path).unwrap();
            (path.strip_prefix(dir).unwrap().to_path_buf(), text)
        })
        .collect()
}

#[test]
fn the_committed_typescript_types_are_current() {
    let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../desk/src/api");
    let fresh = tempfile::tempdir().unwrap();
    atelier_desk::api::export_types(fresh.path()).unwrap();
    let generated = files(fresh.path());
    if std::env::var_os("ATELIER_UPDATE_TYPES").is_some_and(|value| value == "1") {
        let _ = fs::remove_dir_all(&committed);
        fs::create_dir_all(&committed).unwrap();
        for (path, text) in &generated {
            fs::write(committed.join(path), text).unwrap();
        }
    }
    let current = files(&committed);
    let stale: Vec<&PathBuf> = (generated.keys().chain(current.keys()))
        .filter(|path| generated.get(*path) != current.get(*path))
        .collect();
    assert!(
        stale.is_empty(),
        "the TypeScript types in desk/src/api/ differ from the Rust types: {stale:?}. \
         Write them again with: ATELIER_UPDATE_TYPES=1 cargo nextest run -p atelier-desk --test types"
    );
    assert!(generated.contains_key(Path::new("MissionPage.ts")));
}
