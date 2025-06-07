use std::io::{self, Write};

use crate::{AnthropicClient, ChatMessage, ContentBlockType, MessageRequest, Role};
use anyhow::Result;

pub struct Agent<F>
where
    F: Fn() -> Option<String>,
{
    client: AnthropicClient,
    input_reader: F,
}

impl<F> Agent<F>
where
    F: Fn() -> Option<String>,
{
    pub fn new(client: AnthropicClient, input_reader: F) -> Self {
        Self {
            client,
            input_reader,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut conversation: Vec<ChatMessage> = Vec::new();

        println!("Chat with Syl (use 'ctrl-c' to quit)");

        loop {
            print!("\u{001b}[94mYou\u{001b}[0m: ");
            io::stdout().flush().ok();

            let user_input = match (self.input_reader)() {
                Some(input) => input,
                None => break,
            };

            conversation.push(ChatMessage {
                role: Role::User,
                content: user_input,
            });

            let request = MessageRequest::from_messages(conversation.clone());
            let response = self.client.send_message(request).await?;

            for content_block in response.content_blocks {
                conversation.push(ChatMessage {
                    role: Role::Assistant,
                    content: content_block.text.clone(),
                });

                match content_block.block_type {
                    ContentBlockType::Text => {
                        println!("\u{001b}[93mClaude\u{001b}[0m: {}", content_block.text);
                    }
                }
            }
        }

        Ok(())
    }
}
