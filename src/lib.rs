use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // Response::ok("Hello, World!")

    let sec_val = env.secret("REQUEST_PATH")?.to_string();
    let req_path = format!("/{}", sec_val);

    let router = Router::new();

    router.post_async(&req_path, |mut req, ctx| async move {
        // Content-Type: plain/text is expected
        let content = req.text().await?;
        Response::from_bytes(content.into())
    }).run(req, env).await
}
