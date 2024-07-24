use axum::{response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use shuttle_openai::async_openai::config::OpenAIConfig;
use shuttle_openai::async_openai::types::{CreateChatCompletionRequestArgs, ChatCompletionRequestUserMessageArgs};
use shuttle_openai::async_openai::Client;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_openai::OpenAI(
    api_key="{secrets.OPENAI_API_KEY}"
)] cfg: Client<OpenAIConfig>) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/prompt", post(prompt))
        .layer(Extension(cfg))
        .nest_service("/", ServeDir::new("static"));

    Ok(router.into())
}

#[derive(Deserialize, Serialize)]
struct Prompt {
    prompt: String,
}

async fn prompt(
    Extension(openai): Extension<Client<OpenAIConfig>>,
    Json(prompt): Json<Prompt>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o")
        .messages([
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt.prompt)
                .build().unwrap()
                .into()
        ])
        .build().unwrap();


    let response = match openai.chat().create(request).await {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };

    Ok(response.choices.into_iter().next().unwrap().message.content.unwrap())
}
