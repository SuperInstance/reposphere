//! # Reposphere
//!
//! Self-hosting repository modeled as a conscious entity with immutable architecture laws.

use serde::{Deserialize, Serialize};

// ── manifest ────────────────────────────────────────────────────────────────

/// Parsed representation of a REPOSPHERE.md manifest file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub laws: Vec<String>,
    pub allowed_guests: Vec<String>,
}

/// Parse a REPOSPHERE.md-style text blob into a structured `Manifest`.
///
/// Expected format (simplified key-value with `---` law separator):
///
/// ```text
/// name: my-repo
/// version: 0.1.0
/// guests: agent-a,agent-b
/// ---
/// no unsafe code
/// all public fns must be documented
/// ```
pub fn parse_manifest(text: &str) -> Result<Manifest, String> {
    let mut name = String::new();
    let mut version = String::new();
    let mut allowed_guests = Vec::new();
    let mut laws = Vec::new();
    let mut in_laws = false;

    for line in text.lines() {
        let line = line.trim();
        if line == "---" {
            in_laws = true;
            continue;
        }
        if in_laws {
            if !line.is_empty() {
                laws.push(line.to_string());
            }
        } else if let Some(val) = line.strip_prefix("name:") {
            name = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("version:") {
            version = val.trim().to_string();
        } else if let Some(val) = line.strip_prefix("guests:") {
            allowed_guests = val
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }

    if name.is_empty() {
        return Err("missing name field".into());
    }
    if version.is_empty() {
        return Err("missing version field".into());
    }

    Ok(Manifest {
        name,
        version,
        laws,
        allowed_guests,
    })
}

// ── mythos ──────────────────────────────────────────────────────────────────

/// Immutable architecture laws enforced on the repository.
#[derive(Debug, Clone)]
pub struct Mythos {
    laws: Vec<String>,
}

impl Mythos {
    pub fn new(laws: Vec<String>) -> Self {
        Self { laws }
    }

    /// Check whether `content` violates any law. Returns the first violated law, if any.
    pub fn check_violation(&self, content: &str) -> Option<&str> {
        let lower = content.to_lowercase();
        for law in &self.laws {
            let law_lower = law.to_lowercase();
            // Simplified: if the law says "no X" and content contains X, it's a violation.
            if let Some(forbidden) = law_lower.strip_prefix("no ") {
                if lower.contains(forbidden.trim()) {
                    return Some(law.as_str());
                }
            }
            if let Some(forbidden) = law_lower.strip_prefix("never ") {
                if lower.contains(forbidden.trim()) {
                    return Some(law.as_str());
                }
            }
        }
        None
    }

    /// Return the number of laws.
    pub fn law_count(&self) -> usize {
        self.laws.len()
    }

    /// Return a reference to the laws slice.
    pub fn laws(&self) -> &[String] {
        &self.laws
    }
}

// ── guest_agent ─────────────────────────────────────────────────────────────

/// A proposed lesson / code change from a guest agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LessonProposal {
    pub author: String,
    pub content: String,
    pub description: String,
}

/// Evaluation result for a lesson proposal.
#[derive(Debug, Clone, PartialEq)]
pub enum Evaluation {
    Accepted(String),
    Rejected(String),
}

/// Evaluate a lesson proposal against the mythos and guest allowlist.
pub fn evaluate_proposal(
    proposal: &LessonProposal,
    mythos: &Mythos,
    allowed_guests: &[String],
) -> Evaluation {
    if !allowed_guests.contains(&proposal.author) {
        return Evaluation::Rejected(format!("guest '{}' not in allowlist", proposal.author));
    }
    if let Some(violation) = mythos.check_violation(&proposal.content) {
        return Evaluation::Rejected(format!("violates law: {}", violation));
    }
    Evaluation::Accepted(format!("lesson '{}' accepted", proposal.description))
}

// ── commit_guard ────────────────────────────────────────────────────────────

/// Result of a pre-commit check.
#[derive(Debug, Clone, PartialEq)]
pub struct GuardResult {
    pub allowed: bool,
    pub reason: Option<String>,
}

/// Run a pre-commit validation of `content` against the mythos.
pub fn commit_guard(content: &str, mythos: &Mythos) -> GuardResult {
    match mythos.check_violation(content) {
        Some(law) => GuardResult {
            allowed: false,
            reason: Some(format!("blocked: violates law '{}'", law)),
        },
        None => GuardResult {
            allowed: true,
            reason: None,
        },
    }
}

// ── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest_text() -> &'static str {
        "name: test-repo\nversion: 1.0.0\nguests: alice,bob\n---\nno unsafe\nno unwrap"
    }

    fn sample_mythos() -> Mythos {
        Mythos::new(vec![
            "no unsafe".into(),
            "no unwrap".into(),
            "never panic".into(),
        ])
    }

    #[test]
    fn test_parse_manifest_basic() {
        let m = parse_manifest(sample_manifest_text()).unwrap();
        assert_eq!(m.name, "test-repo");
        assert_eq!(m.version, "1.0.0");
    }

    #[test]
    fn test_parse_manifest_guests() {
        let m = parse_manifest(sample_manifest_text()).unwrap();
        assert_eq!(m.allowed_guests, vec!["alice", "bob"]);
    }

    #[test]
    fn test_parse_manifest_laws() {
        let m = parse_manifest(sample_manifest_text()).unwrap();
        assert_eq!(m.laws, vec!["no unsafe", "no unwrap"]);
    }

    #[test]
    fn test_parse_manifest_missing_name() {
        let text = "version: 1.0.0";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn test_parse_manifest_missing_version() {
        let text = "name: foo";
        assert!(parse_manifest(text).is_err());
    }

    #[test]
    fn test_mythos_violation_unsafe() {
        let mythos = sample_mythos();
        assert_eq!(
            mythos.check_violation("let x = unsafe { *ptr };"),
            Some("no unsafe")
        );
    }

    #[test]
    fn test_mythos_violation_unwrap() {
        let mythos = sample_mythos();
        assert_eq!(
            mythos.check_violation("val.unwrap()"),
            Some("no unwrap")
        );
    }

    #[test]
    fn test_mythos_no_violation() {
        let mythos = sample_mythos();
        assert_eq!(mythos.check_violation("val.ok()?"), None);
    }

    #[test]
    fn test_evaluate_accepted() {
        let mythos = sample_mythos();
        let proposal = LessonProposal {
            author: "alice".into(),
            content: "let x = val.ok()?;".into(),
            description: "safe access pattern".into(),
        };
        let result = evaluate_proposal(&proposal, &mythos, &["alice".into(), "bob".into()]);
        assert!(matches!(result, Evaluation::Accepted(_)));
    }

    #[test]
    fn test_evaluate_rejected_guest() {
        let mythos = sample_mythos();
        let proposal = LessonProposal {
            author: "eve".into(),
            content: "safe code".into(),
            description: "nice try".into(),
        };
        let result = evaluate_proposal(&proposal, &mythos, &["alice".into()]);
        assert!(matches!(result, Evaluation::Rejected(_)));
    }

    #[test]
    fn test_evaluate_rejected_violation() {
        let mythos = sample_mythos();
        let proposal = LessonProposal {
            author: "alice".into(),
            content: "unsafe { std::ptr::read(p) }".into(),
            description: "raw pointer".into(),
        };
        let result = evaluate_proposal(&proposal, &mythos, &["alice".into()]);
        assert!(matches!(result, Evaluation::Rejected(_)));
    }

    #[test]
    fn test_commit_guard_allows_clean() {
        let mythos = sample_mythos();
        let result = commit_guard("fn foo() -> Result<u8, Error> { Ok(42) }", &mythos);
        assert!(result.allowed);
        assert!(result.reason.is_none());
    }

    #[test]
    fn test_commit_guard_blocks_unsafe() {
        let mythos = sample_mythos();
        let result = commit_guard("unsafe { *ptr }", &mythos);
        assert!(!result.allowed);
        assert!(result.reason.is_some());
    }

    #[test]
    fn test_mythos_law_count() {
        let mythos = sample_mythos();
        assert_eq!(mythos.law_count(), 3);
    }

    #[test]
    fn test_mythos_never_violation() {
        let mythos = sample_mythos();
        assert_eq!(mythos.check_violation("panic!(\"boom\")"), Some("never panic"));
    }
}
