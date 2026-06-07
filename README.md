# Reposphere

[![crates.io](https://img.shields.io/crates/v/reposphere.svg)](https://crates.io/crates/reposphere)
[![docs.rs](https://docs.rs/reposphere/badge.svg)](https://docs.rs/reposphere)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Self-hosting repositories modeled as conscious entities with immutable architecture laws.**
>
> A repository isn't a folder of files. It's a living system with its own laws, its own immune system, and its own capacity to accept or reject visitors.

---

## The Problem

Modern repositories are passive containers — they accept any commit, any dependency, any pattern. There's no concept of a repository having *identity* or *boundaries*. When multiple agents (CI bots, automated PRs, AI coding assistants) interact with a repo, there's no framework for defining what the repo will and won't accept.

## Why This Exists

Reposphere treats a repository as a **conscious entity** with:
- **Mythos**: Immutable architecture laws that cannot be violated (like "no unsafe code" or "never panic")
- **Guest Agents**: Explicit allowlists of which agents may contribute
- **Lesson Proposals**: Structured change requests that are evaluated against the mythos
- **Commit Guards**: Pre-commit validation that enforces the laws automatically

This creates a self-defending repository — one that can participate in federated learning while maintaining its own identity.

## Architecture

```
  ┌─────────────────────────────────────────┐
  │              REPOSPHERE.md              │
  │  ┌─────────┐ ┌────────┐ ┌───────────┐  │
  │  │  Name   │ │Version │ │  Guests   │  │
  │  └─────────┘ └────────┘ └───────────┘  │
  │  ┌───────────────────────────────────┐  │
  │  │           LAWS (Mythos)           │  │
  │  │  • no unsafe code                 │  │
  │  │  • all public fns documented      │  │
  │  │  • never panic                    │  │
  │  └───────────────────────────────────┘  │
  └─────────────────┬───────────────────────┘
                    │
         ┌──────────▼──────────┐
         │    Commit Guard     │
         │  (pre-commit hook)  │
         └──────────┬──────────┘
                    │
    ┌───────────────▼───────────────┐
    │      Lesson Evaluator         │
    │  ┌────────┐    ┌──────────┐   │
    │  │Guest ✅│ or │Guest ❌  │   │
    │  └────────┘    └──────────┘   │
    │  ┌────────┐    ┌──────────┐   │
    │  │Law ✅  │ or │Law ❌    │   │
    │  └────────┘    └──────────┘   │
    └───────────────────────────────┘
```

## Installation

```toml
[dependencies]
reposphere = "0.1"
```

## API Reference

### `Manifest`

Parsed representation of a `REPOSPHERE.md` configuration file:

```rust
use reposphere::parse_manifest;

let text = "
name: my-repo
version: 0.1.0
guests: agent-a,agent-b
---
no unsafe code
all public fns must be documented
";

let manifest = parse_manifest(text).unwrap();
assert_eq!(manifest.name, "my-repo");
assert_eq!(manifest.laws, vec!["no unsafe code", "all public fns must be documented"]);
assert_eq!(manifest.allowed_guests, vec!["agent-a", "agent-b"]);
```

### `Mythos`

Immutable architecture laws enforced on the repository:

```rust
use reposphere::Mythos;

let mythos = Mythos::new(vec![
    "no unsafe".into(),
    "no unwrap".into(),
    "never panic".into(),
]);

// Check for violations
assert_eq!(mythos.check_violation("let x = unsafe { *ptr };"), Some("no unsafe"));
assert_eq!(mythos.check_violation("val.ok()?"), None); // clean
assert_eq!(mythos.law_count(), 3);
```

### `LessonProposal`

A proposed code change from a guest agent:

```rust
use reposphere::{LessonProposal, Evaluation, evaluate_proposal, Mythos};

let proposal = LessonProposal {
    author: "alice".into(),
    content: "let x = val.ok()?;".into(),
    description: "safe access pattern".into(),
};

let mythos = Mythos::new(vec!["no unsafe".into()]);
let result = evaluate_proposal(&proposal, &mythos, &["alice".into(), "bob".into()]);
assert!(matches!(result, Evaluation::Accepted(_)));
```

### `commit_guard`

Pre-commit validation against the mythos:

```rust
use reposphere::{commit_guard, Mythos};

let mythos = Mythos::new(vec!["no unsafe".into()]);

let clean = commit_guard("fn foo() -> Result<u8, Error> { Ok(42) }", &mythos);
assert!(clean.allowed);

let blocked = commit_guard("unsafe { *ptr }", &mythos);
assert!(!blocked.allowed);
```

## Usage Examples

### Example 1: Federated Learning with Guest Agents

```rust
use reposphere::*;

let manifest = parse_manifest("
name: cognitive-core
version: 2.0.0
guests: learning-agent,audit-bot
---
no unsafe
no unwrap
never panic
").unwrap();

let mythos = Mythos::new(manifest.laws);

// A learning agent proposes a change
let proposal = LessonProposal {
    author: "learning-agent".into(),
    content: "result.ok()?".into(),
    description: "error propagation improvement".into(),
};

match evaluate_proposal(&proposal, &mythos, &manifest.allowed_guests) {
    Evaluation::Accepted(msg) => println!("✅ {}", msg),
    Evaluation::Rejected(msg) => println!("❌ {}", msg),
}
```

### Example 2: Pre-Commit Hook Integration

```rust
use reposphere::{commit_guard, Mythos};

let mythos = Mythos::new(vec![
    "no unsafe".into(),
    "no unwrap".into(),
    "never panic".into(),
]);

// In a git pre-commit hook:
let staged_content = "fn handle(input: &str) -> Option<&str> { Some(input) }";
let result = commit_guard(staged_content, &mythos);

if !result.allowed {
    eprintln!("Commit blocked: {:?}", result.reason);
    std::process::exit(1);
}
```

### Example 3: Rejecting Unknown Agents

```rust
use reposphere::*;

let mythos = Mythos::new(vec!["no unsafe".into()]);
let proposal = LessonProposal {
    author: "eve".into(), // not in the allowlist!
    content: "safe code".into(),
    description: "nice try".into(),
};

let result = evaluate_proposal(&proposal, &mythos, &["alice".into()]);
assert!(matches!(result, Evaluation::Rejected(_)));
```

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `parse_manifest` | O(n) | n = manifest text length |
| `check_violation` | O(L × n) | L laws × n content length |
| `evaluate_proposal` | O(G + L×n) | G guests + violation check |
| `commit_guard` | O(L × n) | Law scan |

## Comparison with Alternatives

| Feature | reposphere | git hooks | CODEOWNERS |
|---------|-----------|-----------|------------|
| Law-based validation | ✅ Semantic | ❌ Script-based | ❌ Owner-based |
| Guest agent management | ✅ Native | ❌ Manual | ❌ |
| Self-hosting manifest | ✅ REPOSPHERE.md | ❌ | ❌ |
| Federated learning support | ✅ Native | ❌ | ❌ |
| Serde serialization | ✅ | ❌ | ❌ |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes
4. Commit with conventional commits (`feat:`, `fix:`, `docs:`)
5. Push and open a Pull Request

All contributions must pass `cargo test` and `cargo clippy`.
