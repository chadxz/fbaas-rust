use lambda_http::lambda_runtime::{Context, Error};
use lambda_http::{lambda_runtime, IntoResponse, Request};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(lambda_http::handler(fizz_buzz)).await?;
    Ok(())
}

async fn fizz_buzz(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(json!({ "hello": "world!" }))
}
