use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("Hello, world!");

    dotenvy::dotenv().context("Failed  to load .env")?;
    let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("ANTHROPIC_API_KEY environment variable is required")?;
    dbg!("Loaded API key: {}", &anthropic_api_key);

    Ok(())
}
