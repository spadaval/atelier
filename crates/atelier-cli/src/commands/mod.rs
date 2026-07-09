pub mod activity_log;
pub mod bundle;
pub mod comment;
pub mod create;
pub mod deps;
pub mod evidence;
pub mod forgejo;
pub mod history;
pub mod import;
pub mod init;
pub mod issue;
pub mod issue_status;
pub mod issue_workflow;
pub mod man;
pub mod mission;
pub(crate) mod objective_status;
pub mod pr;
pub mod prune;
pub mod relate;
pub mod status;
pub mod work;
pub(crate) mod work_order;
pub mod workflow;
pub(crate) mod workflow_actions;
pub(crate) mod workflow_planning;

#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::atomic::{AtomicU64, Ordering};

    use atelier_core::Issue;
    use atelier_sqlite::{
        Database, IssueBlockCacheRow, IssueCacheQuery, IssueCacheRow, RecordSourceCacheRow,
    };
    use chrono::Utc;

    static NEXT_FIXTURE_ID: AtomicU64 = AtomicU64::new(1);

    pub trait DomainCacheFixture {
        fn cache_fixture_issue(
            &self,
            title: &str,
            description: Option<&str>,
            priority: &str,
        ) -> anyhow::Result<String>;

        fn cache_fixture_subissue(
            &self,
            parent_id: &str,
            title: &str,
            description: Option<&str>,
            priority: &str,
        ) -> anyhow::Result<String>;

        fn cache_fixture_insert(&self, issue: &Issue) -> anyhow::Result<()>;

        fn cache_fixture_close(&self, id: &str) -> anyhow::Result<()>;
    }

    impl DomainCacheFixture for Database {
        fn cache_fixture_issue(
            &self,
            title: &str,
            description: Option<&str>,
            priority: &str,
        ) -> anyhow::Result<String> {
            self.cache_fixture_subissue_with_parent(None, title, description, priority)
        }

        fn cache_fixture_subissue(
            &self,
            parent_id: &str,
            title: &str,
            description: Option<&str>,
            priority: &str,
        ) -> anyhow::Result<String> {
            self.cache_fixture_subissue_with_parent(Some(parent_id), title, description, priority)
        }

        fn cache_fixture_insert(&self, issue: &Issue) -> anyhow::Result<()> {
            let labels = self.get_labels(&issue.id)?;
            let relations = self.issue_cache_relations(&issue.id)?;
            let mut blocks = Vec::new();
            for candidate in self.query_issue_cache(&IssueCacheQuery::default())? {
                if self
                    .issue_cache_blockers(&candidate.id)?
                    .iter()
                    .any(|blocker| blocker == &issue.id)
                {
                    blocks.push(IssueBlockCacheRow {
                        blocker_id: issue.id.clone(),
                        blocked_id: candidate.id,
                    });
                }
            }
            self.index_issue(
                &IssueCacheRow {
                    id: issue.id.clone(),
                    title: issue.title.clone(),
                    status: issue.status.clone(),
                    issue_type: issue.issue_type.clone(),
                    priority: issue.priority.clone(),
                    fields: issue.fields.clone(),
                    parent_id: issue.parent_id.clone(),
                    created_at: issue.created_at,
                    updated_at: issue.updated_at,
                    closed_at: issue.closed_at,
                },
                &labels,
                &blocks,
                &relations,
                &RecordSourceCacheRow {
                    path: format!("issues/{}.md", issue.id),
                    record_kind: "issue".to_string(),
                    record_id: issue.id.clone(),
                    size_bytes: 0,
                    modified_micros: None,
                    content_hash: None,
                    indexed_at: Utc::now(),
                },
            )
        }

        fn cache_fixture_close(&self, id: &str) -> anyhow::Result<()> {
            let mut issue = self.require_issue(id)?;
            let now = Utc::now();
            issue.status = "done".to_string();
            issue.updated_at = now;
            issue.closed_at = Some(now);
            self.cache_fixture_insert(&issue)
        }
    }

    trait DomainCacheFixturePrivate {
        fn cache_fixture_subissue_with_parent(
            &self,
            parent_id: Option<&str>,
            title: &str,
            description: Option<&str>,
            priority: &str,
        ) -> anyhow::Result<String>;
    }

    impl DomainCacheFixturePrivate for Database {
        fn cache_fixture_subissue_with_parent(
            &self,
            parent_id: Option<&str>,
            title: &str,
            description: Option<&str>,
            priority: &str,
        ) -> anyhow::Result<String> {
            let id = format!(
                "atelier-t{:08}",
                NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
            );
            let now = Utc::now();
            self.cache_fixture_insert(&Issue {
                id: id.clone(),
                title: title.to_string(),
                description: description.map(str::to_string),
                status: "todo".to_string(),
                issue_type: "task".to_string(),
                priority: priority.to_string(),
                fields: Default::default(),
                parent_id: parent_id.map(str::to_string),
                created_at: now,
                updated_at: now,
                closed_at: None,
            })?;
            Ok(id)
        }
    }
}
