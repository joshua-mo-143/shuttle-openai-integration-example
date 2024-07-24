use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use shuttle_openai::async_openai::config::OpenAIConfig;
use shuttle_openai::async_openai::types::CreateCompletionRequestArgs;
use shuttle_openai::async_openai::Client;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_openai::OpenAI] cfg: Client<OpenAIConfig>) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .layer(Extension(cfg));

    Ok(router.into())
}

#[derive(Deserialize, Serialize)]
struct Prompt {
    inner: String,
}

async fn prompt(
    Extension(openai): Extension<Client<OpenAIConfig>>,
    Json(prompt): Json<Prompt>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let request = match CreateCompletionRequestArgs::default()
        .model("gpt-4o")
        .prompt("Tell me a joke about the universe")
        .max_tokens(40_u32)
        .build()
    {
        Ok(req) => req,
        Err(e) => return Err(e.to_string()),
    };

    let response = match openai.completions().create(request).await {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };

    Ok(response.choices.into_iter().next().unwrap().text)
}
