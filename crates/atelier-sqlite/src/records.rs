use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

use super::{parse_datetime, validate_link_type, validate_record_kind, Database};
use crate::record_id;
use atelier_core::{Record, RecordLink};
use atelier_records as record_store;

#[derive(Debug, Clone, PartialEq)]
pub struct RecordSummary {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub source_path: String,
}

impl Database {
    pub fn create_record(&self, kind: &str, title: &str, status: &str) -> Result<String> {
        record_store::validate_canonical_record_kind(kind)?;
        let id = record_id::allocate_issue_id(|candidate| Ok(self.record_exists(candidate)?))?;
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO records (kind, id, title, status, created_at, updated_at, source_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?5, '')",
            params![kind, id, title, status, now],
        )?;
        Ok(id)
    }

    pub fn insert_record_rebuild(&self, record: &Record) -> Result<()> {
        self.insert_record_rebuild_from_source(record, "")
    }

    pub fn insert_record_rebuild_from_source(
        &self,
        record: &Record,
        source_path: &str,
    ) -> Result<()> {
        let header = record.header();
        record_store::validate_canonical_record_kind(&header.kind)?;
        self.conn.execute(
            "INSERT INTO records (kind, id, title, status, created_at, updated_at, source_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                header.kind,
                header.id,
                header.title,
                header.status,
                header.created_at.to_rfc3339(),
                header.updated_at.to_rfc3339(),
                source_path,
            ],
        )?;
        self.insert_typed_record_projection(record)?;
        Ok(())
    }

    pub fn replace_record(&self, record: &Record, source_path: &str) -> Result<()> {
        self.transaction(|| {
            self.remove_record(&record.header().kind, &record.header().id)?;
            self.insert_record_rebuild_from_source(record, source_path)
        })
    }

    pub fn remove_record(&self, kind: &str, id: &str) -> Result<()> {
        validate_record_kind(kind)?;
        record_id::validate_record_id(id)?;
        self.conn.execute(
            "DELETE FROM record_links
             WHERE (source_kind = ?1 AND source_id = ?2)
                OR (target_kind = ?1 AND target_id = ?2)",
            params![kind, id],
        )?;
        self.conn.execute(
            "DELETE FROM record_labels WHERE kind = ?1 AND id = ?2",
            params![kind, id],
        )?;
        match kind {
            "evidence" => {
                self.conn
                    .execute("DELETE FROM evidence WHERE id = ?1", params![id])?;
            }
            "plan" => {
                self.conn
                    .execute("DELETE FROM plans WHERE id = ?1", params![id])?;
            }
            "milestone" => {
                self.conn
                    .execute("DELETE FROM milestones WHERE id = ?1", params![id])?;
            }
            _ => {}
        }
        self.conn.execute(
            "DELETE FROM records WHERE kind = ?1 AND id = ?2",
            params![kind, id],
        )?;
        Ok(())
    }

    pub fn remove_projected_record(&self, kind: &str, id: &str) -> Result<()> {
        validate_record_kind(kind)?;
        record_id::validate_record_id(id)?;
        if kind == "issue" {
            self.conn
                .execute("DELETE FROM labels WHERE issue_id = ?1", params![id])?;
            self.conn.execute(
                "DELETE FROM dependencies WHERE blocker_id = ?1 OR blocked_id = ?1",
                params![id],
            )?;
            self.conn.execute(
                "DELETE FROM relations WHERE issue_id_1 = ?1 OR issue_id_2 = ?1",
                params![id],
            )?;
            self.conn.execute(
                "DELETE FROM record_links
                 WHERE (source_kind = 'issue' AND source_id = ?1)
                    OR (target_kind = 'issue' AND target_id = ?1)",
                params![id],
            )?;
            self.conn
                .execute("DELETE FROM issues WHERE id = ?1", params![id])?;
        } else {
            self.remove_record(kind, id)?;
        }
        Ok(())
    }

    pub fn replace_record_labels(&self, kind: &str, id: &str, labels: &[String]) -> Result<()> {
        validate_record_kind(kind)?;
        record_id::validate_record_id(id)?;
        self.conn.execute(
            "DELETE FROM record_labels WHERE kind = ?1 AND id = ?2",
            params![kind, id],
        )?;
        for label in labels {
            self.conn.execute(
                "INSERT OR IGNORE INTO record_labels (kind, id, label)
                 VALUES (?1, ?2, ?3)",
                params![kind, id, label],
            )?;
        }
        Ok(())
    }

    pub fn replace_outgoing_links(&self, kind: &str, id: &str, links: &[RecordLink]) -> Result<()> {
        validate_record_kind(kind)?;
        record_id::validate_record_id(id)?;
        self.conn.execute(
            "DELETE FROM record_links WHERE source_kind = ?1 AND source_id = ?2",
            params![kind, id],
        )?;
        for link in links {
            self.conn.execute(
                "INSERT OR IGNORE INTO record_links
                 (source_kind, source_id, target_kind, target_id, relation_type, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    link.source_kind,
                    link.source_id,
                    link.target_kind,
                    link.target_id,
                    link.relation_type,
                    link.created_at.to_rfc3339(),
                ],
            )?;
        }
        Ok(())
    }

    pub fn records_linking_to(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        validate_record_kind(kind)?;
        record_id::validate_record_id(id)?;
        let mut stmt = self.conn.prepare(
            "SELECT source_kind, source_id, target_kind, target_id, relation_type, created_at
             FROM record_links
             WHERE target_kind = ?1 AND target_id = ?2
             ORDER BY source_kind, source_id, relation_type",
        )?;
        let rows = stmt.query_map(params![kind, id], link_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn get_record(&self, kind: &str, id: &str) -> Result<Option<RecordSummary>> {
        validate_record_kind(kind)?;
        let mut stmt = self.conn.prepare(
            "SELECT kind, id, title, status, created_at, updated_at, source_path
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

    pub fn record_kind_for_id(&self, id: &str) -> Result<Option<String>> {
        if record_id::validate_record_id(id).is_err() {
            return Ok(None);
        }
        if self.get_issue(id)?.is_some() {
            return Ok(Some("issue".to_string()));
        }

        let mut stmt = self
            .conn
            .prepare("SELECT kind FROM records WHERE id = ?1 ORDER BY kind LIMIT 1")?;
        let kind = stmt
            .query_row(params![id], |row| row.get::<_, String>(0))
            .ok();
        Ok(kind)
    }

    pub fn require_record(&self, kind: &str, id: &str) -> Result<RecordSummary> {
        self.get_record(kind, id)?
            .ok_or_else(|| anyhow::anyhow!("{} record {} not found", kind, id))
    }

    pub fn list_records(&self, kind: &str, status: Option<&str>) -> Result<Vec<RecordSummary>> {
        validate_record_kind(kind)?;
        let mut records = Vec::new();
        if let Some(status) = status {
            let mut stmt = self.conn.prepare(
                "SELECT kind, id, title, status, created_at, updated_at, source_path
                 FROM records WHERE kind = ?1 AND status = ?2 ORDER BY id",
            )?;
            let rows = stmt.query_map(params![kind, status], record_from_row)?;
            for row in rows {
                records.push(row?);
            }
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT kind, id, title, status, created_at, updated_at, source_path
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

impl Database {
    fn insert_typed_record_projection(&self, record: &Record) -> Result<()> {
        match record {
            Record::Evidence(record) => {
                let data = &record.data;
                self.conn.execute(
                    "INSERT INTO evidence
                     (id, evidence_type, captured_at, command, path, uri, proof_scope,
                      agent_identity, independence_level, exit_code, exit_status, success, spawn_error)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                    params![
                        record.header.id,
                        data.evidence_type,
                        data.captured_at.to_rfc3339(),
                        data.command,
                        data.path,
                        data.uri,
                        data.proof_scope,
                        data.agent_identity,
                        data.independence_level,
                        data.exit_code,
                        data.exit_status,
                        data.success.map(|value| if value { 1_i64 } else { 0_i64 }),
                        data.spawn_error,
                    ],
                )?;
            }
            Record::Plan(record) => {
                let data = &record.data;
                self.conn.execute(
                    "INSERT INTO plans (id, revision, owner) VALUES (?1, ?2, ?3)",
                    params![record.header.id, data.revision, data.owner],
                )?;
            }
            Record::Milestone(record) => {
                let data = &record.data;
                self.conn.execute(
                    "INSERT INTO milestones (id, desired_state) VALUES (?1, ?2)",
                    params![record.header.id, data.desired_state],
                )?;
            }
            _ => {}
        }
        Ok(())
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

fn record_from_row(row: &rusqlite::Row) -> rusqlite::Result<RecordSummary> {
    Ok(RecordSummary {
        kind: row.get(0)?,
        id: row.get(1)?,
        title: row.get(2)?,
        status: row.get(3)?,
        created_at: parse_datetime(row.get::<_, String>(4)?),
        updated_at: parse_datetime(row.get::<_, String>(5)?),
        source_path: row.get(6)?,
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
