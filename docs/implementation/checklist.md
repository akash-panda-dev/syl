# Implementation Checklist

**Status**: Ready to begin Phase 1
**Goal**: Build a code-editing agent in Rust using the concepts from [blog post](https://ampcode.com/how-to-build-an-agent)

---

## Phase 1: Foundation (Direct Anthropic Integration)

### 1.1 Dependencies & Setup
- [x] Add `reqwest = { version = "0.12", features = ["json", "stream"] }` to Cargo.toml
- [x] Add `serde = { version = "1.0", features = ["derive"] }` to Cargo.toml  
- [x] Add `serde_json = "1.0"` to Cargo.toml
- [x] Add `tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }` to Cargo.toml
- [x] Add `anyhow = "1.0"` to Cargo.toml
- [x] Add `dotenvy = "0.15"` to Cargo.toml
- [x] Test `cargo build` succeeds

### 1.2 Environment & Configuration
- [x] Create environment variable loading for `ANTHROPIC_API_KEY`
- [x] Add optional `ANTHROPIC_API_VERSION` (default: "2023-06-01")
- [x] Add error handling for missing API key
- [x] Test environment loading with `.env` file

### 1.3 Core Data Structures ✅
- [x] Create `ChatMessage` struct in `src/anthropic.rs`
- [x] Create `MessageRequest` struct (model, max_tokens, messages, tools?, stream?)
- [x] Create `MessageResponse` struct for API responses
- [x] Add `#[derive(Serialize, Deserialize)]` to structs
- [x] Test JSON serialization/deserialization
- [x] Organize code into modules (`src/lib.rs`, `src/anthropic.rs`)

### 1.4 HTTP Client Setup ⏳
- [ ] Create `reqwest::Client` with required headers:
  - [ ] `x-api-key: $ANTHROPIC_API_KEY`
  - [ ] `anthropic-version: 2023-06-01`
  - [ ] `content-type: application/json`
- [ ] Create basic client configuration
- [ ] Test header setup

### 1.5 Basic API Integration ⏳
- [ ] Implement POST to `https://api.anthropic.com/v1/messages`
- [ ] Add JSON request body serialization
- [ ] Add JSON response deserialization
- [ ] Test basic "Hello, world!" message
- [ ] Verify conversation continuity

### 1.6 Streaming Support 🔄
- [ ] Add `stream: true` parameter support
- [ ] Implement Server-Sent Events parsing
- [ ] Handle events: `message_start`, `content_block_delta`, `message_stop`
- [ ] Test streaming responses

### 1.7 Retry Logic & Resilience 🔄
- [ ] Implement exponential backoff for 429 errors (min 200ms, jitter)
- [ ] Add 3 retries max for 5xx errors
- [ ] Add timeout handling
- [ ] Test retry behavior

### 1.8 Error Handling 🔄
- [ ] Create `enum AnthropicError { Api(ApiError), Http(reqwest::Error), Parse(serde_json::Error) }`
- [ ] Implement `From` traits for error conversion
- [ ] Add proper error messages
- [ ] Test error scenarios

---

## Phase 2: Agent Core

### 2.1 Agent Structure ⏳
- [ ] Create `Agent` struct with:
  - [ ] `client: AnthropicClient`
  - [ ] `conversation: Vec<ChatMessage>`
  - [ ] `tools: Vec<ToolDefinition>`
- [ ] Implement `Agent::new()` constructor
- [ ] Add conversation state management

### 2.2 Basic Conversation Loop ⏳
- [ ] Implement `Agent::run()` method (like blog's `Run`)
- [ ] Add stdin user input handling
- [ ] Add colored output: `\u001b[94mYou\u001b[0m:` and `\u001b[93mClaude\u001b[0m:`
- [ ] Implement conversation continuation
- [ ] Add Ctrl-C graceful shutdown
- [ ] Test basic chat functionality

### 2.3 Tool System Foundation ⏳
- [ ] Create `ToolDefinition` struct:
  - [ ] `name: String`
  - [ ] `description: String`
  - [ ] `input_schema: serde_json::Value`
  - [ ] `function: fn(serde_json::Value) -> Result<String, Error>`
- [ ] Implement tool execution dispatcher
- [ ] Add tool result handling
- [ ] Test tool registration

---

## Phase 3: Essential Tools (Replicating Blog)

### 3.1 read_file Tool ⏳
- [ ] Define tool schema for file path input
- [ ] Implement `read_file(path: String) -> Result<String, Error>`
- [ ] Add error handling for missing files
- [ ] Add file permission error handling
- [ ] Test with various file types
- [ ] Add tool description: "Read the contents of a given relative file path. Use this when you want to see what's inside a file. Do not use this with directory names."

### 3.2 list_files Tool ⏳
- [ ] Create `ListFilesInput` struct with optional path
- [ ] Implement `list_files(path: Option<String>) -> Result<String, Error>`
- [ ] Return JSON array of files/directories
- [ ] Add "/" suffix for directories (like blog)
- [ ] Handle recursive directory traversal
- [ ] Test with various directory structures
- [ ] Add tool description: "List files and directories at a given path. If no path is provided, lists files in the current directory."

### 3.3 edit_file Tool ⏳
- [ ] Create `EditFileInput` struct with path, old_str, new_str
- [ ] Implement `edit_file(input: EditFileInput) -> Result<String, Error>`
- [ ] Add string replacement logic (`str.replace(old_str, new_str)`)
- [ ] Add file creation for new files (when old_str is empty)
- [ ] Add validation: `old_str != new_str`
- [ ] Add validation: exactly one match required
- [ ] Handle file permissions
- [ ] Test file editing scenarios
- [ ] Add tool description: "Make edits to a text file. Replaces 'old_str' with 'new_str' in the given file. 'old_str' and 'new_str' MUST be different from each other. If the file specified with path doesn't exist, it will be created."

---

## Phase 4: Integration & Polish

### 4.1 Tool Integration ⏳
- [ ] Wire all tools into agent's tool registry
- [ ] Implement tool calling flow:
  - [ ] Detect `content.type == "tool_use"`
  - [ ] Extract tool name and input
  - [ ] Execute tool function
  - [ ] Return result to conversation
- [ ] Add conversation state management for tool results
- [ ] Test tool chaining (list → read → edit)

### 4.2 CLI Interface ⏳
- [ ] Create main function with user interaction loop
- [ ] Add colored output formatting
- [ ] Implement graceful shutdown (Ctrl-C)
- [ ] Add startup message: "Chat with Claude (use 'ctrl-c' to quit)"
- [ ] Add tool execution display: `tool: read_file({"path":"main.rs"})`
- [ ] Test CLI user experience

### 4.3 Error Handling & Polish ⏳
- [ ] Create comprehensive error types for different failure modes
- [ ] Add retry logic for API calls
- [ ] Handle tool execution failures gracefully
- [ ] Add input validation
- [ ] Add helpful error messages
- [ ] Test error scenarios

---

## Phase 5: Testing & Validation

### 5.1 Basic Functionality Tests 🔄
- [ ] Test file reading with various file types
- [ ] Test directory listing with nested structures
- [ ] Test file editing with string replacement
- [ ] Verify conversation continuity across multiple turns
- [ ] Test tool chaining scenarios

### 5.2 Error Scenarios 🔄
- [ ] Test API failures and retries
- [ ] Test file permission errors
- [ ] Test missing file errors
- [ ] Test invalid tool inputs
- [ ] Verify graceful degradation

### 5.3 Integration Tests 🔄
- [ ] Test complete agent workflow
- [ ] Test multi-turn conversations with file operations
- [ ] Test conversation state persistence

---

**Legend**: ⏳ = High Priority, 🔄 = Medium Priority
