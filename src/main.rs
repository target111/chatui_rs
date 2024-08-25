use chatui_rs::{
    client::{OpenAIClient, OpenAIConfigBuilder},
    types::ChatModel,
};

use std::io::{self, Write};

fn main() {
    let config = OpenAIConfigBuilder::default()
        .api_key("sk-")
        .model(ChatModel::GPT4o)
        .build()
        .unwrap();

    let mut client = OpenAIClient::new(config);

    loop {
        let mut input = String::new();
        print!("Input: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match client.chat(&input) {
            Ok(response) => println!("Response: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
