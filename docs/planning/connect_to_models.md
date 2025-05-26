## Rust LLM Connectivity Plan (Markdown Doc)


---

### 0 · Purpose & High-Level Shape

1. **Start with a minimal, explicit HTTP client** (`reqwest + serde`) for Anthropic.
2. **Wrap that client behind a trait** so we can swap in other providers later without touching agent logic.
3. **Iteratively add provider modules** that reuse the same internal message format, mapping to:

   * Anthropic native REST
   * OpenAI-spec endpoints (OpenAI, Mistral, Groq, OpenRouter, local Ollama, etc.)
   * Future bespoke APIs

---

### 1 · Phase 1 — Direct `reqwest` → Anthropic

| Step                       | Detail                                                                                                                                                                                                                                                                                                           | Rationale                                                                                          |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| **1.1 Add crates**         | `toml<br/>reqwest = { version = "0.12", features = ["json", "stream"] }<br/>serde = { version = "1", features = ["derive"] }<br/>tokio = { version = "1", features = ["rt-multi-thread", "macros"] } `                                                                                                           | Async HTTP, JSON, multithread runtime.                                                             |
| **1.2 Config via env**     | `ANTHROPIC_API_KEY`, optional `ANTHROPIC_API_VERSION` (default `2023-06-01`).                                                                                                                                                                                                                                    | Keeps secrets out of source; version pin avoids silent breaking changes.                           |
| **1.3 Request structs**    | `rust<br/>#[derive(Serialize)]<br/>struct MsgRequest<'a> {<br/>    model: &'a str,<br/>    max_tokens: u16,<br/>    messages: Vec<ChatMsg<'a>>,<br/>    stream: Option<bool>,<br/>    tools: Option<Vec<ToolSchema>>,<br/>}<br/>#[derive(Serialize)]<br/>struct ChatMsg<'a> { role: &'a str, content: &'a str }` | Mirrors the **Messages** schema in docs; add optional `stream/tools` now to avoid refactors later. |
| **1.4 Headers**            | `text<br/>x-api-key: $ANTHROPIC_API_KEY<br/>anthropic-version: 2023-06-01<br/>content-type: application/json` ([Anthropic][1])                                                                                                                                                                                   | Required by Anthropic; version header is mandatory.                                                |
| **1.5 Blocking call**      | `POST https://api.anthropic.com/v1/messages` → deserialize JSON.                                                                                                                                                                                                                                                 | Baseline synchronous path (easier to debug).                                                       |
| **1.6 Streaming path**     | If `stream=true`, parse Server-Sent Events: `event: message_start`, `content_block_delta`, … until `message_stop`.                                                                                                                                                                                               | Enables token-level display & early tool invocation.                                               |
| **1.7 Retries & back-off** | 429 → exponential back-off (min 200 ms, jitter); 5xx → 3 retries max.                                                                                                                                                                                                                                            | Matches cookbook guidance.                                                                         |
| **1.8 Error type**         | `enum AnthropicError { Api(ApiError), Http(reqwest::Error), Parse(serde_json::Error) }`                                                                                                                                                                                                                          | Keeps provider-specific detail encapsulated.                                                       |

> **Result:** one file (`anthropic.rs`) implementing `async fn send(&self, req: MsgRequest) -> Result<MsgResponse, AnthropicError>`.

---

### 2 · Phase 2 — Abstraction Layer

```rust
#[async_trait::async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, msgs: &[ChatMsg<'_>], opts: ChatOpts)
        -> Result<ChatResponse, LlmError>;

    /// Optional: called when Claude/Ollama asks for a tool.
    async fn call_tool(&self, call: ToolInvocation)
        -> Result<ToolResult, LlmError> { /* default no-op */ }
}
```

*Implementation modules*

| Module        | Uses                                               | Notes                                                    |
| ------------- | -------------------------------------------------- | -------------------------------------------------------- |
| `anthropic`   | direct REST (Phase 1)                              | Full feature set.                                        |
| `openai_like` | `async-openai` or raw `reqwest`; base URL injected | Covers OpenAI, Mistral, Groq, OpenRouter, local servers. |
| `mock`        | in-memory echo                                     | Enables deterministic unit tests.                        |

The agent logic depends **only** on `dyn LlmClient`.

---

### 3 · Phase 3 — Adding More Providers

1. **OpenAI cloud**

   * `api_base = https://api.openai.com/v1`
   * Same message schema; headers `Authorization: Bearer $OPENAI_API_KEY`.
2. **OpenRouter aggregator**

   * `api_base = https://openrouter.ai/api/v1`
   * Supply `Authorization: Bearer $OPENROUTER_API_KEY`; choose model by name.
3. **Local Ollama / LM Studio**

   * `api_base = http://127.0.0.1:11434/v1`
   * No auth; models pulled on demand.
4. **Provider-specifics**

   * Wrap edge-cases (Anthropic “thinking”, Claude file-upload) behind trait *extensions* so core remains portable.

---

### 4 · Phase 4 — Observability & Ops

| Concern               | Implementation sketch                                                                                           |
| --------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Tracing**           | `tracing::info_span!("anthropic_request", req_id)`; include `request-id` from response headers for correlation. |
| **Metrics**           | `histogram!` for latency, `counter!` for tokens, grouped by provider / model.                                   |
| **Config hot-reload** | Watch `config.toml`; rebuild `Arc<ClientCfg>` to change throttles or switch providers at runtime.               |
| **Secrets**           | Load once with `dotenvy` or Secrets Manager; never log.                                                         |

---

### 5 · Phase 5 — Future-Proofing Checklist

* **Version header bump**: Track Anthropic changelog; expose `anthropic-version` as config so ops can pin or roll forward without deploy.
* **Beta features**: Add `anthropic-beta: tools-2025-03` when experimenting.
* **JSON Schema evolution**: Use `serde(deny_unknown_fields)` only in tests; allow forward-compat in prod.
* **Fallback routing**: If provider-A ≥ 3 × HTTP 429, automatically switch to backup provider module.

---

### 6 · Appendix — Minimal Code Skeleton

<details>
<summary>Click to view</summary>

```rust
// lib.rs
pub mod anthropic;
pub mod openai_like;
pub mod traits;

// main.rs
use agent::{traits::LlmClient, anthropic::Anthropic};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client: Box<dyn LlmClient> = match std::env::var("LLM_PROVIDER")
        .unwrap_or_else(|_| "anthropic".into())
        .as_str()
    {
        "openai" => Box::new(openai_like::OpenAI::from_env()?),
        _        => Box::new(Anthropic::from_env()?),
    };

    let reply = client
        .chat(&[("user", "Hello, world!")], Default::default())
        .await?;

    println!("{}", reply.text());
    Ok(())
}
```

</details>

---

### 7 · Key Doc Links

* Anthropic API overview (headers, versioning) ([Anthropic][1])
* Messages endpoint reference & examples (tool calling, streaming) ([Anthropic][2])

---

**End of plan**

[1]: https://docs.anthropic.com/en/api/overview?utm_source=chatgpt.com "Overview - Anthropic API"
[2]: https://docs.anthropic.com/en/api/messages-examples?utm_source=chatgpt.com "Messages examples - Anthropic API"

