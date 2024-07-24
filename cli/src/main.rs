use clap::Parser;
use std::io;
use std::io::Write;

// Feel free to change this URL based on the endpoint URL you've set up
const PROMPT_URL: &str = "http://localhost:8000/prompt";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "PROMPT")]
    prompt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let prompt = match cli.prompt {
        Some(prompt) => prompt,
        None => {
            let mut buf = String::new();

            print!("Enter your prompt: ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut buf).unwrap();

            buf
        }
    };

    let json = serde_json::json!({
        "prompt": prompt
    });

    let client = reqwest::Client::new();
    let response = client.post(PROMPT_URL).json(&json).send().await?;

    println!("{}", response.text().await?);

    Ok(())
}
