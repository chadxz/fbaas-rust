use lambda_http::lambda_runtime::Context;
use lambda_http::handler;
use lambda_http::{lambda_runtime, Request, IntoResponse};
use lambda_http::lambda_runtime::Error;
use serde_json::{ json };

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(fizz_buzz)).await?;
    Ok(())
}

async fn fizz_buzz(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(json!({ "hello": "world!" }))
}