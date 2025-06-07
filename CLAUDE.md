# SYL - Code-Editing Agent in Rust

## Project Overview

**Goal**: Build a ~400 line code-editing agent in Rust replicating [this blog post](https://ampcode.com/how-to-build-an-agent).

**Core Concept**: LLM + tools (read_file, list_files, edit_file) + conversation loop.

## Implementation Strategy

**Philosophy**: Simple direct implementation first, abstractions later. Follow blog exactly.

### Current Phase: Phase 3 (Tool System)
- ✅ **Phase 1**: Foundation (HTTP client working)
- ✅ **Phase 2**: Basic Chat Interface (Agent struct + conversation loop)
- 🔄 **Phase 3**: Tool System + read_file
- 🔄 **Phase 4**: Add list_files  
- 🔄 **Phase 5**: Add edit_file
- 🔄 **Phase 6**: Polish & abstractions

## Technical Stack

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
anyhow = "1.0"
dotenvy = "0.15"
```

**Environment**: `ANTHROPIC_API_KEY` (required)

## Code Organization

```
src/
├── main.rs        - Agent usage, stdin handling
├── lib.rs         - Library exports  
├── anthropic.rs   - API types, AnthropicClient
└── agent.rs       - Agent struct, conversation loop
```

## Current Implementation Status

### Phase 1 Complete ✅
- AnthropicClient with proper headers working
- Basic API connectivity tested
- Environment loading functional

### Phase 2 Complete ✅
- `Agent<F>` struct with generic input reader: `F: Fn() -> Option<String>`
- Conversation loop: user input → LLM → response → repeat
- Colored terminal output: `\u{001b}[94mYou\u{001b}[0m:`, `\u{001b}[93mClaude\u{001b}[0m:`
- Conversation state: `Vec<ChatMessage>` (stateless server pattern)
- MessageRequest::from_messages() factory method

### Phase 3 Next Steps ⏳
- ToolDefinition struct, execute_tool method, read_file tool

### Future Phases 🔄
- **Phase 3**: ToolDefinition struct, execute_tool method, read_file tool
- **Phase 4**: list_files tool (JSON array, "/" suffix for directories)
- **Phase 5**: edit_file tool (string replacement)
- **Phase 6**: LlmClient trait, multi-provider support, streaming, retry logic

## Development Commands

```bash
cargo run     # Run agent
cargo test    # Run tests  
cargo clippy  # Rust linter
cargo fmt     # Format code
```

## Collaboration Guidelines

**Role**: Peer programming guidance, code review, architecture decisions  
**Philosophy**: "The Philosophy of Software Design" principles
**Key Principle**: Always guide toward Rust-idiomatic implementations

## References

- [Original blog post](https://ampcode.com/how-to-build-an-agent) - Go reference implementation
- [Anthropic API docs](https://docs.anthropic.com/en/api/messages-examples) - Tool calling examples
- Implementation checklist: `docs/implementation/checklist.md`
