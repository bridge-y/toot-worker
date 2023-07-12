use reqwest::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use worker::*;

#[derive(Deserialize)]
struct Content {
    text: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let sec_val = env.secret("REQUEST_PATH")?.to_string();
    let req_path = format!("/{}", sec_val);

    let router = Router::new();

    router
        .post_async(&req_path, |mut req, ctx| async move {
            let base_url = ctx.secret("BASE_URL")?.to_string();
            let token = ctx.secret("TOKEN")?.to_string();

            let Content { text } = match req.json().await {
                Ok(value) => value,
                Err(_) => return Response::error("Bad request", 400),
            };
            post_mastodon(&base_url, &token, &text.trim()).await
        })
        .run(req, env)
        .await
}

async fn post_mastodon(base_url: &str, token: &str, text: &str) -> Result<Response> {
    let url = format!("{}/api/v1/statuses", base_url);
    let payload = json!({
        "status": text,
    });

    let client = Client::new();
    let result = client
        .post(url)
        .json(&payload)
        .bearer_auth(token)
        .send()
        .await;

    match result {
        Ok(response) => match response.status() {
            StatusCode::OK => Response::ok(""),
            StatusCode::UNAUTHORIZED => Response::error("The access token is invalid", 401),
            StatusCode::UNPROCESSABLE_ENTITY => {
                Response::error("Validation failed: Text can't be blank", 422)
            }
            _ => Response::error("Bad Gateway", 502),
        },
        Err(_) => Response::error("Internal Server Error", 500),
    }
}
