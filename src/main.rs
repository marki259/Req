use std::{error::Error, io::Read};
use std::env;
use std::fs::File;
use std::io::BufReader;

use reqwest;
use tokio;
use serde_json::{json, Value};
use dotenv::dotenv;
use clap::{Command, Arg, ArgAction};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let string_arg = Arg::new("string")
        .help("Takes a string as input")
        .required_unless_present("file")
        .conflicts_with("file")
        .index(1);

    let file_arg = Arg::new("file")
        .short('f')
        .long("file")
        .value_name("FILE")
        .action(ArgAction::Set)
        .help("Specifies the file to use");

    let matches = Command::new("Quick GPT")
        .arg(string_arg)
        .arg(file_arg)
        .get_matches();

    let prompt = if let Some(expression) = matches.get_one::<String>("string") {
        expression.clone()
    }
    else if let Some(input) = matches.get_one::<String>("file") {
        let file = File::open(input)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        buf_reader.read_to_string(&mut content)?;
        content
    }
    else {
        String::from("Hey there")
    };

    // api call
    let openai_apikey = env::var("OPENAI_APIKEY").expect("Expected OPENAI_APIKEY to be declared");

    let client = reqwest::Client::new();
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
              "role": "system",
              "content": "You are a helpful assistant."
            },
            {
              "role": "user",
              "content": format!("{:?}", prompt)
            }
        ]
    });

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(openai_apikey)
        .json(&body)
        .send()
        .await?;

    let response_json: Value = res.json().await?;

    if let Some(choices) = response_json.get("choices").and_then(|c| c.as_array()) {
        for c in choices {
            if let Some(message) = c.get("message") {
                if let Some(content) = message.get("content") {
                    println!("{}", content);
                }
            }
        }
    }

    Ok(())
}
