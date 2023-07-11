use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // Response::ok("Hello, World!")

    let req_path = env.secret("REQUEST_PATH")?.to_string();
    Response::ok(req_path)
}
