use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

use super::{parse_datetime, validate_link_type, validate_record_kind, Database};
use crate::models::{DomainRecord, RecordLink};
use crate::record_id;
use crate::record_store;

impl Database {
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

    pub fn update_record(
        &self,
        kind: &str,
        id: &str,
        title: Option<&str>,
        status: Option<&str>,
        body: Option<&str>,
        data_json: Option<&str>,
    ) -> Result<bool> {
        record_store::validate_canonical_record_kind(kind)?;
        if let Some(data_json) = data_json {
            let _: serde_json::Value = serde_json::from_str(data_json)?;
        }

        let now = Utc::now().to_rfc3339();
        let mut updates = vec!["updated_at = ?1".to_string()];
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now)];

        if let Some(title) = title {
            updates.push(format!("title = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(title.to_string()));
        }
        if let Some(status) = status {
            updates.push(format!("status = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(status.to_string()));
        }
        if let Some(body) = body {
            updates.push(format!("body = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(body.to_string()));
        }
        if let Some(data_json) = data_json {
            updates.push(format!("data_json = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(data_json.to_string()));
        }

        params_vec.push(Box::new(kind.to_string()));
        params_vec.push(Box::new(id.to_string()));
        let sql = format!(
            "UPDATE records SET {} WHERE kind = ?{} AND id = ?{}",
            updates.join(", "),
            params_vec.len() - 1,
            params_vec.len()
        );
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        Ok(self.conn.execute(&sql, params_refs.as_slice())? > 0)
    }

    pub fn get_record(&self, kind: &str, id: &str) -> Result<Option<DomainRecord>> {
        validate_record_kind(kind)?;
        let mut stmt = self.conn.prepare(
            "SELECT id, kind, title, status, body, data_json, created_at, updated_at
             FROM records WHERE kind = ?1 AND id = ?2",
        )?;
        let record = stmt.query_row(params![kind, id], record_from_row).ok();
        Ok(record)
    }

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

    pub fn require_record(&self, kind: &str, id: &str) -> Result<DomainRecord> {
        self.get_record(kind, id)?
            .ok_or_else(|| anyhow::anyhow!("{} record {} not found", kind, id))
    }

    pub fn list_records(&self, kind: &str, status: Option<&str>) -> Result<Vec<DomainRecord>> {
        validate_record_kind(kind)?;
        let mut records = Vec::new();
        if let Some(status) = status {
            let mut stmt = self.conn.prepare(
                "SELECT id, kind, title, status, body, data_json, created_at, updated_at
                 FROM records WHERE kind = ?1 AND status = ?2 ORDER BY id",
            )?;
            let rows = stmt.query_map(params![kind, status], record_from_row)?;
            for row in rows {
                records.push(row?);
            }
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, kind, title, status, body, data_json, created_at, updated_at
                 FROM records WHERE kind = ?1 ORDER BY id",
            )?;
            let rows = stmt.query_map(params![kind], record_from_row)?;
            for row in rows {
                records.push(row?);
            }
        }
        Ok(records)
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

    pub fn remove_record_link(
        &self,
        source_kind: &str,
        source_id: &str,
        target_kind: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<bool> {
        validate_link_type(relation_type)?;
        let rows = self.conn.execute(
            "DELETE FROM record_links
             WHERE source_kind = ?1 AND source_id = ?2
               AND target_kind = ?3 AND target_id = ?4 AND relation_type = ?5",
            params![
                source_kind,
                source_id,
                target_kind,
                target_id,
                relation_type
            ],
        )?;
        Ok(rows > 0)
    }

    pub fn list_record_links(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        validate_record_ref(self, kind, id)?;
        let mut stmt = self.conn.prepare(
            "SELECT source_kind, source_id, target_kind, target_id, relation_type, created_at
             FROM record_links
             WHERE (source_kind = ?1 AND source_id = ?2) OR (target_kind = ?1 AND target_id = ?2)
             ORDER BY created_at, source_kind, source_id, target_kind, target_id, relation_type",
        )?;
        let rows = stmt.query_map(params![kind, id], link_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn list_all_record_links(&self) -> Result<Vec<RecordLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT source_kind, source_id, target_kind, target_id, relation_type, created_at
             FROM record_links
             ORDER BY source_kind, source_id, target_kind, target_id, relation_type",
        )?;
        let rows = stmt.query_map([], link_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
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

fn record_from_row(row: &rusqlite::Row) -> rusqlite::Result<DomainRecord> {
    Ok(DomainRecord {
        id: row.get(0)?,
        kind: row.get(1)?,
        title: row.get(2)?,
        status: row.get(3)?,
        body: row.get(4)?,
        data_json: row.get(5)?,
        created_at: parse_datetime(row.get::<_, String>(6)?),
        updated_at: parse_datetime(row.get::<_, String>(7)?),
    })
}

fn link_from_row(row: &rusqlite::Row) -> rusqlite::Result<RecordLink> {
    Ok(RecordLink {
        source_kind: row.get(0)?,
        source_id: row.get(1)?,
        target_kind: row.get(2)?,
        target_id: row.get(3)?,
        relation_type: row.get(4)?,
        created_at: parse_datetime(row.get::<_, String>(5)?),
    })
}
