use crate::{Decision, SchemaVersion, Thread, Threads, Time};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A review's front matter, in the order it is written, and its Markdown
/// body. Its findings are read from the body by `atelier-records`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    pub schema: ReviewSchema,
    pub schema_version: SchemaVersion,
    pub round: u32,
    /// The commit reviewed.
    pub commit: String,
    pub reviewers: Threads,
    /// The reviewer's model, as given to `verdict --model`; `None` when
    /// nobody said.
    pub model: Option<String>,
    pub verdict: Option<Verdict>,
    pub created_at: Time,
    #[serde(skip)]
    pub body: String,
}

/// The `schema` tag of a review record.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewSchema {
    #[default]
    #[serde(rename = "atelier.review")]
    Review,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Approve,
    Changes,
}

/// A review file's stem, `<round>-<suffix>`, as in `2-m2bn`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReviewName {
    pub round: u32,
    pub suffix: String,
}

/// A finding's name: its review's stem and its number, as in `2-m2bn.1`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindingName {
    pub review: ReviewName,
    pub number: u32,
}

impl fmt::Display for ReviewName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.round, self.suffix)
    }
}

impl fmt::Display for FindingName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.review, self.number)
    }
}

impl FromStr for ReviewName {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, String> {
        let error = || format!("`{text}` is not a review name, <round>-<suffix> as in 2-m2bn");
        let (round, suffix) = text.split_once('-').ok_or_else(error)?;
        let round = positive(round).ok_or_else(error)?;
        let suffix_ok = !suffix.is_empty()
            && suffix
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
        if !suffix_ok {
            return Err(error());
        }
        Ok(ReviewName {
            round,
            suffix: suffix.to_string(),
        })
    }
}

impl FromStr for FindingName {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, String> {
        let error = || format!("`{text}` is not a finding name, <review>.<number> as in 2-m2bn.1");
        let (review, number) = text.rsplit_once('.').ok_or_else(error)?;
        Ok(FindingName {
            review: review.parse().map_err(|_| error())?,
            number: positive(number).ok_or_else(error)?,
        })
    }
}

/// A whole number from 1, written without sign or leading zeros, so every
/// name has one spelling.
fn positive(text: &str) -> Option<u32> {
    let number: u32 = text.parse().ok()?;
    (number > 0 && number.to_string() == text).then_some(number)
}

/// One `## Finding <k>: <title>` section of a review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub name: FindingName,
    pub title: String,
    pub recommendation: Recommendation,
    /// On a finding carried from an earlier round, the finding it carries.
    pub was: Option<FindingName>,
    /// What became of the carried finding, as this review judged it.
    pub now: Option<Now>,
    /// What a decision made of the carried finding; the tool writes it.
    pub decided: Option<Outcome>,
    pub settlement: Option<Settlement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recommendation {
    FixNow,
    Defer,
    NoAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Now {
    Fixed,
    Withdrawn,
    Kept,
}

/// How a decision settled a finding without a fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Defer,
    NoAction,
}

impl fmt::Display for Outcome {
    /// As a record writes it, after `Outcome: ` or `Decided: `.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Outcome::Defer => "defer",
            Outcome::NoAction => "no action",
        })
    }
}

/// How a deferred finding was settled: taken into an item, or dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Settlement {
    Taken { item: String, by: Thread },
    Dropped { by: Thread, reason: String },
}

impl Finding {
    /// A deferred finding is settled once it is taken into an item or
    /// dropped.
    pub fn is_settled(&self) -> bool {
        self.settlement.is_some()
    }
}

/// Whether `finding` is deferred, given every finding of its item's reviews
/// and the item's decisions: its recommendation is defer, or a decision
/// deferred it, by the `Decided: defer` line a later round carries or by a
/// Decisions entry naming it. A finding a later round carries is not: the
/// carrying finding is the one that stands, and the one to settle.
pub fn is_deferred(finding: &Finding, findings: &[Finding], decisions: &[Decision]) -> bool {
    let decided = decisions.iter().any(|decision| {
        decision.finding.as_ref() == Some(&finding.name) && decision.outcome == Some(Outcome::Defer)
    });
    let carried_later = findings.iter().any(|later| {
        later.was.as_ref() == Some(&finding.name)
            && later.name.review.round > finding.name.review.round
    });
    (finding.recommendation == Recommendation::Defer
        || finding.decided == Some(Outcome::Defer)
        || decided)
        && !carried_later
}

/// Whether `finding` is an open fix-now finding, given every finding of its
/// item's reviews and the item's decisions.
///
/// A fix-now finding is open until a later round carries it, by naming it
/// in `Was:`, or a decision naming it has an outcome. Carrying it as fixed,
/// withdrawn or decided closes it; carrying it as kept moves it to the
/// carrying finding, which is then the open one. A carrying finding that
/// says the earlier one was fixed, withdrawn or decided is not itself open.
pub fn is_open(finding: &Finding, findings: &[Finding], decisions: &[Decision]) -> bool {
    let closes_carried =
        matches!(finding.now, Some(Now::Fixed | Now::Withdrawn)) || finding.decided.is_some();
    let carried_later = findings.iter().any(|later| {
        later.was.as_ref() == Some(&finding.name)
            && later.name.review.round > finding.name.review.round
    });
    let decided = decisions.iter().any(|decision| {
        decision.finding.as_ref() == Some(&finding.name) && decision.outcome.is_some()
    });
    finding.recommendation == Recommendation::FixNow
        && !closes_carried
        && !carried_later
        && !decided
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(text: &str) -> FindingName {
        text.parse().unwrap()
    }

    fn finding(text: &str, recommendation: Recommendation) -> Finding {
        Finding {
            name: name(text),
            title: "t".into(),
            recommendation,
            was: None,
            now: None,
            decided: None,
            settlement: None,
        }
    }

    fn carrying(text: &str, was: &str, now: Now) -> Finding {
        Finding {
            was: Some(name(was)),
            now: Some(now),
            ..finding(text, Recommendation::FixNow)
        }
    }

    fn decision(about: &str, outcome: Option<Outcome>) -> Decision {
        Decision {
            question: "q".into(),
            finding: Some(name(about)),
            answer: None,
            outcome,
        }
    }

    fn open(findings: &[Finding], decisions: &[Decision]) -> Vec<String> {
        findings
            .iter()
            .filter(|f| is_open(f, findings, decisions))
            .map(|f| f.name.to_string())
            .collect()
    }

    #[test]
    fn names_read_and_print_one_way() {
        let parsed = name("2-m2bn.1");
        assert_eq!(parsed.review.round, 2);
        assert_eq!(parsed.review.suffix, "m2bn");
        assert_eq!(parsed.number, 1);
        assert_eq!(parsed.to_string(), "2-m2bn.1");
        for bad in [
            "2-m2bn",
            "m2bn.1",
            "0-m2bn.1",
            "02-m2bn.1",
            "2-M2BN.1",
            "2-.1",
            "2-m2bn.0",
            "2-m2bn.x",
        ] {
            assert!(bad.parse::<FindingName>().is_err(), "{bad}");
        }
    }

    #[test]
    fn a_fix_now_finding_stays_open_until_a_later_round_closes_it() {
        let first = finding("1-aaaa.1", Recommendation::FixNow);
        assert_eq!(open(std::slice::from_ref(&first), &[]), ["1-aaaa.1"]);

        // Carried as fixed or withdrawn: closed, and the carrier is not open.
        for now in [Now::Fixed, Now::Withdrawn] {
            let findings = [first.clone(), carrying("2-bbbb.1", "1-aaaa.1", now)];
            assert!(open(&findings, &[]).is_empty(), "{now:?}");
        }

        // Kept: the carrier is now the open one, until a later round fixes it.
        let kept = carrying("2-bbbb.1", "1-aaaa.1", Now::Kept);
        assert_eq!(open(&[first.clone(), kept.clone()], &[]), ["2-bbbb.1"]);
        let fixed = carrying("3-cccc.1", "2-bbbb.1", Now::Fixed);
        assert!(open(&[first.clone(), kept, fixed], &[]).is_empty());

        // A carrier in the same round is no later round.
        let same_round = carrying("1-dddd.1", "1-aaaa.1", Now::Fixed);
        assert_eq!(open(&[first, same_round], &[]), ["1-aaaa.1"]);
    }

    #[test]
    fn a_decision_closes_a_fix_now_finding_only_with_an_outcome() {
        let findings = [finding("1-aaaa.1", Recommendation::FixNow)];
        assert_eq!(open(&findings, &[decision("1-aaaa.1", None)]), ["1-aaaa.1"]);
        assert_eq!(
            open(&findings, &[decision("1-aaaa.2", Some(Outcome::Defer))]),
            ["1-aaaa.1"]
        );
        for outcome in [Outcome::Defer, Outcome::NoAction] {
            assert!(open(&findings, &[decision("1-aaaa.1", Some(outcome))]).is_empty());
        }
        // The tool's `Decided:` line on a carrier closes what it carries.
        let decided = Finding {
            now: None,
            decided: Some(Outcome::NoAction),
            ..carrying("2-bbbb.1", "1-aaaa.1", Now::Kept)
        };
        assert!(open(&[findings[0].clone(), decided], &[]).is_empty());
    }

    #[test]
    fn only_fix_now_findings_are_ever_open() {
        let findings = [
            finding("1-aaaa.1", Recommendation::Defer),
            finding("1-aaaa.2", Recommendation::NoAction),
        ];
        assert!(open(&findings, &[]).is_empty());
    }

    fn deferred(findings: &[Finding], decisions: &[Decision]) -> Vec<String> {
        findings
            .iter()
            .filter(|f| is_deferred(f, findings, decisions))
            .map(|f| f.name.to_string())
            .collect()
    }

    #[test]
    fn a_finding_is_deferred_by_its_reviewer_or_a_decision_until_a_later_round_carries_it() {
        let findings = [
            finding("1-aaaa.1", Recommendation::Defer),
            finding("1-aaaa.2", Recommendation::FixNow),
            finding("1-aaaa.3", Recommendation::NoAction),
            finding("1-aaaa.4", Recommendation::FixNow),
        ];
        assert_eq!(deferred(&findings, &[]), ["1-aaaa.1"]);
        // A decision naming a finding defers it only with that outcome.
        let decisions = [
            decision("1-aaaa.2", Some(Outcome::Defer)),
            decision("1-aaaa.4", Some(Outcome::NoAction)),
        ];
        assert_eq!(deferred(&findings, &decisions), ["1-aaaa.1", "1-aaaa.2"]);
        assert_eq!(
            deferred(&findings, &[decision("1-aaaa.2", None)]),
            ["1-aaaa.1"]
        );
        // Carried by a later round with `Decided: defer`, the carrier is the
        // deferred one, listed once; carried as fixed, neither is.
        let decided = Finding {
            now: None,
            decided: Some(Outcome::Defer),
            ..carrying("2-bbbb.1", "1-aaaa.2", Now::Kept)
        };
        let carried = [findings[1].clone(), decided];
        assert_eq!(deferred(&carried, &decisions), ["2-bbbb.1"]);
        let fixed = [
            findings[1].clone(),
            carrying("2-bbbb.1", "1-aaaa.2", Now::Fixed),
        ];
        assert!(deferred(&fixed, &decisions).is_empty());
    }

    #[test]
    fn a_deferred_finding_is_unsettled_until_taken_or_dropped() {
        let deferred = finding("1-aaaa.1", Recommendation::Defer);
        assert!(!deferred.is_settled());
        let taken = Settlement::Taken {
            item: "atelier-x1y2".into(),
            by: Thread("t1".into()),
        };
        let dropped = Settlement::Dropped {
            by: Thread("t1".into()),
            reason: "obsolete".into(),
        };
        for settlement in [taken, dropped] {
            let settled = Finding {
                settlement: Some(settlement),
                ..deferred.clone()
            };
            assert!(settled.is_settled());
        }
    }
}
