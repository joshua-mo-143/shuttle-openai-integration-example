## Shuttle OpenAI Integration Example
This repository shows how you can integrate `shuttle-openai` with Shuttle. Complete with a web UI at the base route so you can quickly try it out.

## How to use:
- Clone this repo.
- Create a `Secrets.toml` file and insert your OpenAI key in like this:
```toml
OPENAI_API_KEY = "my_key"
```
- Use `cargo shuttle run` to spin up the web service and try it out!
- Alternatively, you can also try deploying it. Make sure to change the `name` key in the `Shuttle.toml` file to avoid name conflicts!
