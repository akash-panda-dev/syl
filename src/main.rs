use std::io;

use anyhow::{Context, Result};
use syl::{AnthropicClient, agent::Agent};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().context("Failed  to load .env")?;
    let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("ANTHROPIC_API_KEY environment variable is required")?;

    let anthropic_client = AnthropicClient::new(anthropic_api_key);
    let input_reader = || {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input.trim().to_string()),
            Err(_) => None,
        }
    };

    let mut agent = Agent::new(anthropic_client, input_reader);
    agent.run().await?;

    Ok(())
}
