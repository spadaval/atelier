use std::collections::BTreeMap;
use std::path::Path;

use atelier_sqlite::{Database, IssueCacheRow, RecordSourceCacheRow};
use chrono::Utc;

pub fn open_cache(path: &Path) -> Option<Database> {
    Database::open(path).ok()
}

pub fn index_issue(db: &Database, id: &str, title: &str, status: &str, priority: &str) -> bool {
    let now = Utc::now();
    db.index_issue(
        &IssueCacheRow {
            id: id.to_string(),
            title: title.to_string(),
            status: status.to_string(),
            issue_type: "task".to_string(),
            priority: priority.to_string(),
            fields: BTreeMap::new(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        &[],
        &[],
        &[],
        &RecordSourceCacheRow {
            path: format!("issues/{id}.md"),
            record_kind: "issue".to_string(),
            record_id: id.to_string(),
            size_bytes: 0,
            modified_micros: None,
            content_hash: None,
            indexed_at: now,
        },
    )
    .is_ok()
}
