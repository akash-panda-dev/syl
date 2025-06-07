# Implementation Checklist

**Goal**: Build a ~400 line code-editing agent in Rust following [the blog post](https://ampcode.com/how-to-build-an-agent) exactly.

**Philosophy**: Simple direct implementation first, abstractions later.

---

## Phase 1: Foundation ✅

### 1.1 Dependencies & Setup ✅
- [x] Add `reqwest`, `serde`, `tokio`, `anyhow`, `dotenvy` to Cargo.toml
- [x] Test `cargo build` succeeds

### 1.2 Environment & Configuration ✅
- [x] Create environment variable loading for `ANTHROPIC_API_KEY`
- [x] Test environment loading with `.env` file

### 1.3 Core Data Structures ✅
- [x] Create `ChatMessage`, `MessageRequest`, `MessageResponse` structs in `src/anthropic.rs`
- [x] Add `#[derive(Serialize, Deserialize)]` to structs
- [x] Test JSON serialization/deserialization

### 1.4 HTTP Client Setup ✅
- [x] Create `AnthropicClient` struct with `reqwest::Client`
- [x] Implement headers: `x-api-key`, `anthropic-version`, `content-type`
- [x] Implement `send_message` method
- [x] Test basic API connectivity

---

## Phase 2: Basic Chat Interface ✅

### 2.1 Agent Structure ✅
- [x] Create `Agent<F>` struct with generic input reader:
  - [x] `client: AnthropicClient`
  - [x] `input_reader: F where F: Fn() -> Option<String>`
- [x] Implement `Agent::new()` constructor
- [x] Create stdin input handling function in main.rs

### 2.2 Basic Conversation Loop ✅
- [x] Implement `Agent::run()` method (like blog's `Run`)
- [x] Add colored output: `\u{001b}[94mYou\u{001b}[0m:` and `\u{001b}[93mClaude\u{001b}[0m:`
- [x] Implement conversation continuation with `Vec<ChatMessage>`
- [x] Add graceful shutdown on EOF/Ctrl-C
- [x] Print startup message: "Chat with Syl (use 'ctrl-c' to quit)"
- [x] Test basic chat functionality without tools
- [x] Refactor main.rs to use Agent instead of direct client calls
- [x] Add `MessageRequest::from_messages()` factory method

---

## Phase 3: Tool System + read_file ⏳

### 3.1 Tool Infrastructure ⏳
- [ ] Create `ToolDefinition` struct:
  - [ ] `name: String`
  - [ ] `description: String`
  - [ ] `input_schema: serde_json::Value`
  - [ ] `function: fn(serde_json::Value) -> Result<String, Error>`
- [ ] Add `tools: Vec<ToolDefinition>` field to Agent
- [ ] Update `send_message` to include tools in request
- [ ] Add tool execution logic to Agent::run() loop
- [ ] Implement `execute_tool()` method with tool lookup by name

### 3.2 read_file Tool ⏳
- [ ] Create `ReadFileDefinition` with schema
- [ ] Implement `read_file(path: String) -> Result<String, Error>`
- [ ] Add description: "Read the contents of a given relative file path. Use this when you want to see what's inside a file. Do not use this with directory names."
- [ ] Add tool to Agent's tool registry
- [ ] Test tool execution: ask Claude to read a file

---

## Phase 4: list_files Tool ⏳

### 4.1 list_files Implementation ⏳
- [ ] Create `ListFilesDefinition` with optional path parameter
- [ ] Implement `list_files(path: Option<String>) -> Result<String, Error>`
- [ ] Return JSON array of files/directories
- [ ] Add "/" suffix for directories (like blog)
- [ ] Add description: "List files and directories at a given path. If no path is provided, lists files in the current directory."
- [ ] Add tool to Agent's tool registry
- [ ] Test tool chaining: list files → read specific files

---

## Phase 5: edit_file Tool ⏳

### 5.1 edit_file Implementation ⏳
- [ ] Create `EditFileDefinition` with path, old_str, new_str parameters
- [ ] Implement `edit_file(input: EditFileInput) -> Result<String, Error>`
- [ ] Add string replacement logic (`str.replace(old_str, new_str)`)
- [ ] Add file creation for new files (when old_str is empty)
- [ ] Add validation: `old_str != new_str` and exactly one match required
- [ ] Add description: "Make edits to a text file. Replaces 'old_str' with 'new_str' in the given file. 'old_str' and 'new_str' MUST be different from each other. If the file specified with path doesn't exist, it will be created."
- [ ] Add tool to Agent's tool registry
- [ ] Test file creation and editing scenarios

---

## Phase 6: Polish & Abstractions 🔄

### 6.1 Working Agent Validation ⏳
- [ ] Test complete workflow: ask Claude to create/edit files
- [ ] Verify tool chaining works (list → read → edit)
- [ ] Test conversation continuity across multiple turns
- [ ] Ensure ~400 lines total (as per blog)

### 6.2 Optional Enhancements 🔄
- [ ] Add `LlmClient` trait for multi-provider support  
- [ ] Add streaming support
- [ ] Add retry logic & resilience
- [ ] Add comprehensive error handling
- [ ] Add support for other LLM providers
- [ ] Add better CLI interface

---

**Legend**: ⏳ = High Priority, 🔄 = Future/Optional

**Key Insight**: Follow the blog exactly - simple direct implementation first, no abstractions until it works.