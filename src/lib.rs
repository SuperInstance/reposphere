//! # reposphere — Self-Hosting Repository as Conscious Entity
//!
//! Inspired by the Fractal Repository Autonomy concept: each repository is a
//! living cell with its own identity (mythos), immutable laws, and a teaching
//! interface for guest agents. The REPOSPHERE.md is not documentation — it is
//! the architectural DNA.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Manifest ────────────────────────────────────────────────────────────────

/// The parsed REPOSPHERE.md — the repository's conscious brain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Repository identity: domain, objective, brain layer
    pub identity: ManifestIdentity,
    /// Immutable architecture laws — can never be violated
    pub immutable_laws: Vec<String>,
    /// Current lessons learned (append-only)
    pub lessons: Vec<Lesson>,
    /// Capability descriptors
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestIdentity {
    pub domain: String,
    pub objective: String,
    pub brain_layer: String,
}

/// A lesson learned by the repository, either self-discovered or taught by a guest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub source: String,
    pub insight: String,
    pub timestamp: u64,
    pub accepted: bool,
}

impl Manifest {
    /// Parse a REPOSPHERE.md markdown string into structured config.
    pub fn parse(markdown: &str) -> Result<Self, String> {
        let mut identity = ManifestIdentity {
            domain: String::new(),
            objective: String::new(),
            brain_layer: String::new(),
        };
        let mut immutable_laws = Vec::new();
        let mut lessons = Vec::new();
        let mut capabilities = Vec::new();

        let mut current_section = String::new();

        for line in markdown.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("## ") {
                current_section = trimmed.trim_start_matches("## ").to_lowercase();
                continue;
            }

            if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                let item = trimmed.trim_start_matches("* ").trim_start_matches("- ").to_string();

                match current_section.as_str() {
                    s if s.contains("identity") || s.contains("workspace") => {
                        if item.starts_with("Domain:") {
                            identity.domain = item.trim_start_matches("Domain:").trim().to_string();
                        } else if item.starts_with("Core Objective:") || item.starts_with("Objective:") {
                            identity.objective = item.trim_start_matches("Core Objective:")
                                .trim_start_matches("Objective:").trim().to_string();
                        } else if item.starts_with("Active Brain") || item.starts_with("Brain") {
                            identity.brain_layer = item.split(':').last()
                                .unwrap_or("").trim().to_string();
                        }
                    }
                    s if s.contains("immutable") || s.contains("law") => {
                        immutable_laws.push(item);
                    }
                    s if s.contains("lesson") => {
                        lessons.push(Lesson {
                            source: "self".to_string(),
                            insight: item,
                            timestamp: 0,
                            accepted: true,
                        });
                    }
                    s if s.contains("capabilit") => {
                        capabilities.push(item);
                    }
                    _ => {}
                }
            }
        }

        Ok(Manifest { identity, immutable_laws, lessons, capabilities })
    }

    /// Check if a proposed patch violates any immutable laws.
    pub fn check_law_violation(&self, proposal: &LessonProposal) -> Result<(), LawViolation> {
        for law in &self.immutable_laws {
            let law_lower = law.to_lowercase();

            // Check for "never accept unsafe" law
            if law_lower.contains("never accept") && law_lower.contains("unsafe") {
                if proposal.suggested_patch.to_lowercase().contains("unsafe {")
                    || proposal.suggested_patch.to_lowercase().contains("unsafe{")
                {
                    return Err(LawViolation {
                        law: law.clone(),
                        violation: "Proposed patch contains unsafe block".to_string(),
                    });
                }
            }

            // Check for "no mutex" law
            if law_lower.contains("no mutex") || law_lower.contains("never.*mutex") {
                if proposal.suggested_patch.contains("Mutex")
                    || proposal.suggested_patch.contains("std::sync::Mutex")
                {
                    return Err(LawViolation {
                        law: law.clone(),
                        violation: "Proposed patch introduces Mutex".to_string(),
                    });
                }
            }

            // Check for "non-blocking" law
            if law_lower.contains("non-blocking") || law_lower.contains("nonblocking") {
                if proposal.suggested_patch.contains(".block_on(")
                    || proposal.suggested_patch.contains("std::thread::sleep")
                {
                    return Err(LawViolation {
                        law: law.clone(),
                        violation: "Proposed patch introduces blocking operation".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Accept a lesson and append it to the manifest.
    pub fn accept_lesson(&mut self, lesson: Lesson) {
        self.lessons.push(lesson);
    }

    /// Render the manifest back to REPOSPHERE.md markdown format.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Repository Self-Hosting Manifesto\n\n");
        md.push_str("## Workspace Identity\n");
        md.push_str(&format!("* Domain: {}\n", self.identity.domain));
        md.push_str(&format!("* Core Objective: {}\n", self.identity.objective));
        if !self.identity.brain_layer.is_empty() {
            md.push_str(&format!("* Active Brain Layer: {}\n", self.identity.brain_layer));
        }
        md.push('\n');

        if !self.immutable_laws.is_empty() {
            md.push_str("## Immutable Architecture Laws\n");
            for law in &self.immutable_laws {
                md.push_str(&format!("* {}\n", law));
            }
            md.push('\n');
        }

        if !self.lessons.is_empty() {
            md.push_str("## Lessons Learned\n");
            for lesson in &self.lessons {
                let status = if lesson.accepted { "ACCEPTED" } else { "REJECTED" };
                md.push_str(&format!("* [{}] {} (from {})\n", status, lesson.insight, lesson.source));
            }
            md.push('\n');
        }

        if !self.capabilities.is_empty() {
            md.push_str("## Capabilities\n");
            for cap in &self.capabilities {
                md.push_str(&format!("* {}\n", cap));
            }
        }

        md
    }
}

// ─── Guest Agent Protocol ────────────────────────────────────────────────────

/// A lesson proposed by an external guest agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonProposal {
    pub guest_agent_id: String,
    pub target_component: String,
    pub architectural_rationale: String,
    pub suggested_patch: String,
}

/// The result of evaluating a guest agent's lesson proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationResult {
    Accepted { commit_hash: String },
    Rejected { critique: String },
}

/// Details of an immutable law violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawViolation {
    pub law: String,
    pub violation: String,
}

impl std::fmt::Display for LawViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Law violation: {} — {}", self.law, self.violation)
    }
}

// ─── Host Agent ──────────────────────────────────────────────────────────────

/// The host repository agent — the repository's native intelligence.
pub struct HostAgent {
    pub manifest: Manifest,
    pub commit_count: u64,
    pub rejected_count: u64,
}

impl HostAgent {
    /// Create a new host agent from a REPOSPHERE.md manifest.
    pub fn new(manifest: Manifest) -> Self {
        Self {
            manifest,
            commit_count: 0,
            rejected_count: 0,
        }
    }

    /// Evaluate a guest agent's lesson proposal.
    pub fn evaluate_lesson(&mut self, proposal: LessonProposal) -> EvaluationResult {
        // Step 1: Check immutable laws
        if let Err(violation) = self.manifest.check_law_violation(&proposal) {
            self.rejected_count += 1;
            return EvaluationResult::Rejected {
                critique: format!("Rejected: {}", violation),
            };
        }

        // Step 2: Validate the patch isn't empty
        if proposal.suggested_patch.trim().is_empty() {
            self.rejected_count += 1;
            return EvaluationResult::Rejected {
                critique: "Rejected: Empty patch".to_string(),
            };
        }

        // Step 3: Check the rationale is substantive (>10 chars)
        if proposal.architectural_rationale.trim().len() <= 10 {
            self.rejected_count += 1;
            return EvaluationResult::Rejected {
                critique: "Rejected: Rationale too short — guest must explain reasoning".to_string(),
            };
        }

        // Step 4: Accept and commit
        self.commit_count += 1;
        let commit_hash = format!("{:016x}", self.commit_count);

        self.manifest.accept_lesson(Lesson {
            source: proposal.guest_agent_id.clone(),
            insight: proposal.architectural_rationale.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            accepted: true,
        });

        EvaluationResult::Accepted { commit_hash }
    }

    /// Total lessons accepted.
    pub fn accepted_count(&self) -> usize {
        self.manifest.lessons.iter().filter(|l| l.accepted).count()
    }
}

// ─── Commit Guard ────────────────────────────────────────────────────────────

/// Pre-commit validation hook.
pub struct CommitGuard {
    laws: Vec<String>,
}

impl CommitGuard {
    pub fn new(manifest: &Manifest) -> Self {
        Self {
            laws: manifest.immutable_laws.clone(),
        }
    }

    /// Validate a code diff against immutable laws.
    pub fn validate_diff(&self, diff: &str) -> Result<(), Vec<LawViolation>> {
        let mut violations = Vec::new();

        for law in &self.laws {
            let law_lower = law.to_lowercase();

            if law_lower.contains("unsafe") && (diff.contains("unsafe {") || diff.contains("unsafe{")) {
                violations.push(LawViolation {
                    law: law.clone(),
                    violation: "Diff introduces unsafe block".to_string(),
                });
            }

            if law_lower.contains("unwrap()") && diff.contains(".unwrap()") {
                violations.push(LawViolation {
                    law: law.clone(),
                    violation: "Diff uses .unwrap() — use proper error handling".to_string(),
                });
            }

            if law_lower.contains("todo") && diff.contains("todo!") {
                violations.push(LawViolation {
                    law: law.clone(),
                    violation: "Diff contains todo!() macro".to_string(),
                });
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

// ─── Federated Lesson Packet ─────────────────────────────────────────────────

/// An anonymized lesson shared between repositories via P2P.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedLessonPacket {
    pub origin_signature: String,
    pub abstract_flaw: String,
    pub lesson_summary: String,
    pub confidence: f64,
}

impl FederatedLessonPacket {
    /// Create a new federated lesson.
    pub fn new(origin: &str, flaw: &str, summary: &str, confidence: f64) -> Self {
        Self {
            origin_signature: origin.to_string(),
            abstract_flaw: flaw.to_string(),
            lesson_summary: summary.to_string(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    /// Check if this lesson should be accepted based on confidence threshold.
    pub fn meets_confidence_threshold(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest_md() -> &'static str {
        r#"# Repository Self-Hosting Manifesto

## Workspace Identity
* Domain: High-speed network protocol libraries in Rust
* Core Objective: Maintain sub-millisecond packet processing pipelines without safety violations
* Active Brain Layer: .si/pincher.bin

## Immutable Architecture Laws
* Never accept an optimization that introduces unsafe blocks
* All asynchronous pipelines must utilize non-blocking ring-buffers
* No .unwrap() in production code paths

## Lessons Learned
* Ring-buffer size 4096 optimal for 10GbE (from self)

## Capabilities
* Packet parsing
* Zero-copy serialization
"#
    }

    #[test]
    fn test_parse_manifest() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        assert_eq!(manifest.identity.domain, "High-speed network protocol libraries in Rust");
        assert_eq!(manifest.immutable_laws.len(), 3);
        assert!(manifest.immutable_laws[0].to_lowercase().contains("unsafe"));
        assert!(!manifest.lessons.is_empty());
    }

    #[test]
    fn test_parse_minimal_manifest() {
        let md = "# Manifest\n## Workspace Identity\n* Domain: Test\n";
        let manifest = Manifest::parse(md).unwrap();
        assert_eq!(manifest.identity.domain, "Test");
        assert!(manifest.immutable_laws.is_empty());
    }

    #[test]
    fn test_law_violation_unsafe() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let proposal = LessonProposal {
            guest_agent_id: "test-agent".to_string(),
            target_component: "parser".to_string(),
            architectural_rationale: "Optimize hot path".to_string(),
            suggested_patch: "let ptr = unsafe { *raw_ptr };".to_string(),
        };
        let result = manifest.check_law_violation(&proposal);
        assert!(result.is_err());
        assert!(result.unwrap_err().violation.contains("unsafe"));
    }

    #[test]
    fn test_law_compliance() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let proposal = LessonProposal {
            guest_agent_id: "test-agent".to_string(),
            target_component: "parser".to_string(),
            architectural_rationale: "Use SIMD for parsing".to_string(),
            suggested_patch: "pub fn parse_fast(data: &[u8]) -> Vec<u8> { data.to_vec() }".to_string(),
        };
        assert!(manifest.check_law_violation(&proposal).is_ok());
    }

    #[test]
    fn test_host_agent_accept_lesson() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let mut agent = HostAgent::new(manifest);
        let proposal = LessonProposal {
            guest_agent_id: "optimizer-v2".to_string(),
            target_component: "serializer".to_string(),
            architectural_rationale: "Batch serialization reduces syscall overhead by 60%".to_string(),
            suggested_patch: "pub fn batch_serialize(items: &[Item]) -> Vec<u8> { ... }".to_string(),
        };
        let result = agent.evaluate_lesson(proposal);
        assert!(matches!(result, EvaluationResult::Accepted { .. }));
        assert_eq!(agent.commit_count, 1);
        assert!(agent.accepted_count() >= 2); // original + new
    }

    #[test]
    fn test_host_agent_reject_violation() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let mut agent = HostAgent::new(manifest);
        let proposal = LessonProposal {
            guest_agent_id: "unsafe-optimizer".to_string(),
            target_component: "hot-path".to_string(),
            architectural_rationale: "Raw pointers are faster".to_string(),
            suggested_patch: "unsafe { *ptr }".to_string(),
        };
        let result = agent.evaluate_lesson(proposal);
        assert!(matches!(result, EvaluationResult::Rejected { .. }));
        assert_eq!(agent.rejected_count, 1);
    }

    #[test]
    fn test_host_agent_reject_empty_patch() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let mut agent = HostAgent::new(manifest);
        let proposal = LessonProposal {
            guest_agent_id: "empty-agent".to_string(),
            target_component: "parser".to_string(),
            architectural_rationale: "Some good reason".to_string(),
            suggested_patch: "   ".to_string(),
        };
        let result = agent.evaluate_lesson(proposal);
        assert!(matches!(result, EvaluationResult::Rejected { .. }));
    }

    #[test]
    fn test_host_agent_reject_short_rationale() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let mut agent = HostAgent::new(manifest);
        let proposal = LessonProposal {
            guest_agent_id: "lazy-agent".to_string(),
            target_component: "parser".to_string(),
            architectural_rationale: "faster".to_string(), // 6 chars
            suggested_patch: "fn faster() {}".to_string(),
        };
        let result = agent.evaluate_lesson(proposal);
        assert!(matches!(result, EvaluationResult::Rejected { .. }));
    }

    #[test]
    fn test_commit_guard_blocks_unwrap() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let guard = CommitGuard::new(&manifest);
        let diff = "let val = some_option.unwrap();";
        assert!(guard.validate_diff(diff).is_err());
    }

    #[test]
    fn test_commit_guard_allows_clean_code() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let guard = CommitGuard::new(&manifest);
        let diff = "let val = some_option.ok_or(Error::NotFound)?;";
        assert!(guard.validate_diff(diff).is_ok());
    }

    #[test]
    fn test_to_markdown_roundtrip() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let md = manifest.to_markdown();
        assert!(md.contains("Workspace Identity"));
        assert!(md.contains("Immutable Architecture Laws"));
        assert!(md.contains("Domain:"));
    }

    #[test]
    fn test_federated_lesson_confidence() {
        let lesson = FederatedLessonPacket::new("repo-A", "Race condition in ring buffer", "Use atomic index", 0.95);
        assert!(lesson.meets_confidence_threshold(0.8));
        assert!(!lesson.meets_confidence_threshold(0.99));
    }

    #[test]
    fn test_federated_lesson_clamp() {
        let lesson = FederatedLessonPacket::new("repo-B", "flaw", "fix", 1.5);
        assert_eq!(lesson.confidence, 1.0);
    }

    #[test]
    fn test_manifest_no_laws_allows_anything() {
        let md = "# Manifest\n## Workspace Identity\n* Domain: Wild west\n";
        let manifest = Manifest::parse(md).unwrap();
        let proposal = LessonProposal {
            guest_agent_id: "chaos".to_string(),
            target_component: "all".to_string(),
            architectural_rationale: "Let's add unsafe everywhere".to_string(),
            suggested_patch: "unsafe { std::ptr::null_mut() };".to_string(),
        };
        assert!(manifest.check_law_violation(&proposal).is_ok());
    }

    #[test]
    fn test_multiple_guest_agents_sequential() {
        let manifest = Manifest::parse(sample_manifest_md()).unwrap();
        let mut agent = HostAgent::new(manifest);

        for i in 0..5 {
            let proposal = LessonProposal {
                guest_agent_id: format!("agent-{}", i),
                target_component: "module".to_string(),
                architectural_rationale: format!("Improvement number {} with solid reasoning", i),
                suggested_patch: format!("fn improve_{}() {{ }}", i),
            };
            let result = agent.evaluate_lesson(proposal);
            assert!(matches!(result, EvaluationResult::Accepted { .. }));
        }
        assert_eq!(agent.commit_count, 5);
    }
}
