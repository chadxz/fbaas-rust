use lambda_runtime::{ handler_fn, Context, Error };
use serde_json::{ json, Value };

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(fizz_buzz);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn fizz_buzz(_: Value, _: Context) -> Result<Value, Error> {
    Ok(json!({ "statusCode": 200, "body": "{\"hello\": \"world\"}" }))
}