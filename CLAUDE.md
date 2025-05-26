# SYL - Code-Editing Agent in Rust

## Project Overview

**Goal**: Build a code-editing agent in Rust that replicates the functionality described in [this blog post](https://ampcode.com/how-to-build-an-agent) by Thorsten Ball.

**Core Concept**: An LLM with access to tools (read_file, list_files, edit_file) that can interact with the local filesystem to understand and modify code.

## Architecture Decisions

### LLM Integration Strategy
- **Phase 1**: Direct HTTP client (`reqwest` + `serde`) for Anthropic API
- **Phase 2**: Abstract behind `LlmClient` trait for multi-provider support
- **Future**: Add OpenAI-compatible endpoints (OpenAI, Mistral, Groq, OpenRouter, Ollama)

### Key Design Principles
1. Start simple with explicit HTTP calls before abstracting
2. Use trait-based architecture for provider flexibility
3. Prioritize string replacement for file editing (Claude 3.7 Sonnet preference)
4. Maintain conversation state locally (stateless server)

## Current Status

**Current Phase**: Planning and initial setup
**Next Steps**: Begin Phase 1 implementation (direct Anthropic integration)

## Technical Stack

### Core Dependencies
```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
anyhow = "1.0"
dotenvy = "0.15"
```

### Environment Variables
- `ANTHROPIC_API_KEY` (required)
- `ANTHROPIC_API_VERSION` (optional, default: "2023-06-01")

## Core Tools to Implement

1. **read_file**: Read contents of files
2. **list_files**: List files and directories  
3. **edit_file**: Edit files via string replacement

## Key Implementation Files

- `src/main.rs` - CLI interface and agent loop
- `src/anthropic.rs` - Direct Anthropic API client
- `src/traits.rs` - LlmClient trait definition
- `src/tools/` - Tool implementations

## Documentation Structure

```
docs/
├── planning/
│   ├── connect_to_models.md - LLM connectivity strategy
│   └── how_to_build_an_agent.md - Reference blog post
└── implementation/
    └── checklist.md - Granular implementation steps
```

## References

- [Original blog post](https://ampcode.com/how-to-build-an-agent) - Go implementation reference
- [Anthropic API docs](https://docs.anthropic.com/en/api/overview) - API reference
- [Messages endpoint](https://docs.anthropic.com/en/api/messages-examples) - Tool calling examples

## Development Commands

```bash
# Run the agent
cargo run

# Run tests
cargo test

# Check formatting
cargo fmt

# Run linter
cargo clippy
```

## Notes for Claude

- This project replicates a ~400 line Go agent in Rust
- Focus on simplicity first, then add abstraction layers
- String replacement editing works well with Claude 3.7 Sonnet
- Agent loop: user input → LLM → tool execution → response → repeat
- Always test with actual file operations to verify functionality