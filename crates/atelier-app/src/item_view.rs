//! One item as the site's item page shows it (atelier-ahpf): its record's
//! Markdown, where it stands, what it links to, and the review events about
//! it, read from one commit's records.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use ts_rs::TS;

use atelier_core::{Issue, ReviewEvent, ReviewRecord};

use crate::record_index::RecordIndex;
use crate::work_classification::classify_issue;
use crate::workflow_policy::WorkflowPolicy;

/// An item and everything its page shows.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
pub struct ItemView {
    pub issue: Issue,
    pub status_category: Option<String>,
    /// Where the work stands, as the CLI words it, such as `ready`.
    pub state: String,
    pub labels: Vec<String>,
    /// The record's body as committed, in Markdown: its sections, such as a
    /// decision's Question and Options, then its Notes and Report.
    pub body: String,
    pub links: Vec<ItemLink>,
    /// Review events targeting the item, oldest first.
    pub reviews: Vec<ItemReview>,
}

/// Another item this one is linked to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct ItemLink {
    /// How the other item relates to this one: `parent`, `child`,
    /// `blocked_by`, `blocks`, or a typed link such as `advances`, with
    /// `incoming` set when the other item's record declares it.
    pub role: String,
    pub incoming: bool,
    pub id: String,
    /// `None` when the other item is not in the records.
    pub title: Option<String>,
    pub issue_type: Option<String>,
    pub status: Option<String>,
}

/// A review event: a review, approval, finding or trivial exemption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct ItemReview {
    pub id: String,
    /// `review`, `approval`, `finding` or `trivial_exemption`.
    pub event: String,
    pub identity: String,
    pub commit: String,
    pub created_at: DateTime<Utc>,
    /// A finding's recommendation: `fix_now`, `defer`, `no_action` or
    /// `wont_fix`.
    pub recommendation: Option<String>,
    pub disposition: Option<ItemDisposition>,
    /// The event's text, in Markdown.
    pub summary: String,
}

/// What was decided about a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
pub struct ItemDisposition {
    pub decision: String,
    pub identity: String,
    pub recorded_at: DateTime<Utc>,
    pub reason: String,
    pub follow_up: Option<String>,
}

/// The item `id` names, which must be in `index`.
pub fn item_view(index: &RecordIndex, policy: &WorkflowPolicy, id: &str) -> Result<ItemView> {
    let record = index
        .issue_record(id)
        .with_context(|| format!("no item {id}"))?;
    let classified = classify_issue(index, Some(policy), &record.issue)?;
    let text = atelier_records::render_issue_record(record)?;
    let body = atelier_records::split_document(&text)
        .map(|(_, body)| body.trim().to_owned())
        .unwrap_or_default();

    let link = |role: &str, incoming: bool, other: &str| {
        let other_record = index.issue_record(other);
        ItemLink {
            role: role.to_owned(),
            incoming,
            id: other.to_owned(),
            title: other_record.map(|record| record.issue.title.clone()),
            issue_type: other_record.map(|record| record.issue.issue_type.clone()),
            status: other_record.map(|record| record.issue.status.clone()),
        }
    };
    let mut links = Vec::new();
    if let Some(parent) = &record.issue.parent_id {
        links.push(link("parent", false, parent));
    }
    for child in index.child_ids(id) {
        links.push(link("child", false, child));
    }
    for blocker in index.get_blockers(id) {
        links.push(link("blocked_by", false, &blocker));
    }
    for blocked in index.get_blocking(id) {
        links.push(link("blocks", false, &blocked));
    }
    let mut typed = index.get_typed_relations(id);
    typed.sort_by(|left, right| {
        (&left.relation_type, &left.issue_id_1, &left.issue_id_2).cmp(&(
            &right.relation_type,
            &right.issue_id_1,
            &right.issue_id_2,
        ))
    });
    for relation in typed {
        let (other, incoming) = if relation.issue_id_1 == id {
            (relation.issue_id_2, false)
        } else {
            (relation.issue_id_1, true)
        };
        links.push(link(&relation.relation_type, incoming, &other));
    }

    let mut reviews = index
        .reviews()
        .filter(|review| review.target == id)
        .collect::<Vec<_>>();
    reviews.sort_by(|left, right| {
        (left.header.created_at, &left.header.id).cmp(&(right.header.created_at, &right.header.id))
    });
    Ok(ItemView {
        issue: record.issue.clone(),
        status_category: classified.status_category,
        state: classified.classification.label().to_owned(),
        labels: record.labels.clone(),
        body,
        links,
        reviews: reviews.into_iter().map(item_review).collect(),
    })
}

fn item_review(review: &ReviewRecord) -> ItemReview {
    let recommendation = match &review.event {
        ReviewEvent::Finding { recommendation, .. } => Some(recommendation.as_str().to_owned()),
        _ => None,
    };
    ItemReview {
        id: review.header.id.clone(),
        event: review.event.name().to_owned(),
        identity: review.identity.clone(),
        commit: review.commit.clone(),
        created_at: review.header.created_at,
        recommendation,
        disposition: review.disposition().map(|disposition| ItemDisposition {
            decision: disposition.decision.as_str().to_owned(),
            identity: disposition.identity.clone(),
            recorded_at: disposition.recorded_at,
            reason: disposition.reason.clone(),
            follow_up: disposition.follow_up.clone(),
        }),
        summary: review.summary.clone(),
    }
}
