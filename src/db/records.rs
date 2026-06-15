use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

use super::{validate_link_type, validate_record_kind, Database};
use crate::models::{DomainRecord, RecordLink};
use crate::record_id;
use crate::record_store;

impl Database {
    #[cfg(test)]
    pub fn create_record(
        &self,
        kind: &str,
        title: &str,
        status: &str,
        body: Option<&str>,
        data_json: &str,
    ) -> Result<String> {
        record_store::validate_canonical_record_kind(kind)?;
        let _: serde_json::Value = serde_json::from_str(data_json)?;
        let id = record_id::allocate_issue_id(|candidate| Ok(self.record_exists(candidate)?))?;
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO records (id, kind, title, status, body, data_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
            params![id, kind, title, status, body, data_json, now],
        )?;
        Ok(id)
    }

    pub fn insert_record_rebuild(&self, record: &DomainRecord) -> Result<()> {
        record_store::validate_canonical_record_kind(&record.kind)?;
        let _: serde_json::Value = serde_json::from_str(&record.data_json)?;
        self.conn.execute(
            "INSERT INTO records (id, kind, title, status, body, data_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                record.id,
                record.kind,
                record.title,
                record.status,
                record.body,
                record.data_json,
                record.created_at.to_rfc3339(),
                record.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_record(&self, kind: &str, id: &str) -> Result<Option<DomainRecord>> {
        validate_record_kind(kind)?;
        atelier_sqlite::ProjectionIndex::new(&self.conn).record(kind, id)
    }

    #[cfg(test)]
    pub fn record_exists(&self, id: &str) -> Result<bool> {
        if self.get_issue(id)?.is_some() {
            return Ok(true);
        }
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM records WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn record_kind_for_id(&self, id: &str) -> Result<Option<String>> {
        if record_id::validate_record_id(id).is_err() {
            return Ok(None);
        }
        atelier_sqlite::ProjectionIndex::new(&self.conn).record_kind_for_id(id)
    }

    pub fn require_record(&self, kind: &str, id: &str) -> Result<DomainRecord> {
        self.get_record(kind, id)?
            .ok_or_else(|| anyhow::anyhow!("{} record {} not found", kind, id))
    }

    pub fn list_records(&self, kind: &str, status: Option<&str>) -> Result<Vec<DomainRecord>> {
        validate_record_kind(kind)?;
        atelier_sqlite::ProjectionIndex::new(&self.conn).list_records(kind, status)
    }

    pub fn add_record_link(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        validate_record_ref(self, source_kind, source_id)?;
        validate_record_ref(self, target_kind, target_id)?;
        validate_link_type(relation_type)?;
        if source_kind == target_kind && source_id == target_id {
            anyhow::bail!("Cannot link a record to itself");
        }
        let now = Utc::now().to_rfc3339();
        let rows = self.conn.execute(
            "INSERT OR IGNORE INTO record_links
             (source_kind, source_id, target_kind, target_id, relation_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                source_kind,
                source_id,
                target_kind,
                target_id,
                relation_type,
                now
            ],
        )?;
        Ok(rows > 0)
    }

    pub fn list_record_links(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        validate_record_ref(self, kind, id)?;
        atelier_sqlite::ProjectionIndex::new(&self.conn).record_links(kind, id)
    }

    pub fn list_all_record_links(&self) -> Result<Vec<RecordLink>> {
        atelier_sqlite::ProjectionIndex::new(&self.conn).all_record_links()
    }
}

fn validate_record_ref(db: &Database, kind: &str, id: &str) -> Result<()> {
    validate_record_kind(kind)?;
    record_id::validate_record_id(id)?;
    match kind {
        "issue" => {
            db.require_issue(id)?;
        }
        _ => {
            db.require_record(kind, id)?;
        }
    }
    Ok(())
}
