use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use http::status::StatusCode;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use serde_json::json;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    lambda_runtime::run(handler_fn(fbaas)).await?;
    Ok(())
}

/// GET /{number}
/// Process a Fizzbuzz request and return a JSON response
async fn fbaas(req: ApiGatewayProxyRequest, _: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let path_number = req
        .path_parameters
        .get("number")
        .expect("No number parameter found");

    match path_number.parse::<u16>() {
        Ok(number) if number > 0 && number < 10_001 => Ok(ApiGatewayProxyResponse {
            body: Some(Body::Text(
                json!({
                    "message": "Success",
                    "result": fizz_buzz(number)
                })
                .to_string(),
            )),
            headers: HeaderMap::default(),
            is_base64_encoded: Some(false),
            multi_value_headers: HeaderMap::default(),
            status_code: StatusCode::OK.as_u16().into(),
        }),
        _ => Ok(ApiGatewayProxyResponse {
            body: Some(Body::Text(
                json!({
                    "message": "Error",
                    "error": "Number must be an integer greater than 0 and less than 10,001"
                })
                .to_string(),
            )),
            headers: HeaderMap::default(),
            is_base64_encoded: Some(false),
            multi_value_headers: HeaderMap::default(),
            status_code: StatusCode::BAD_REQUEST.as_u16().into(),
        }),
    }
}

/// From https://leetcode.com/problems/fizz-buzz/
/// Given an integer n, return a string array answer (1-indexed) where:
/// * answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
/// * answer[i] == "Fizz" if i is divisible by 3.
/// * answer[i] == "Buzz" if i is divisible by 5.
/// * answer[i] == i if non of the above conditions are true.
fn fizz_buzz(val: u16) -> Vec<String> {
    let mut result = Vec::new();

    for i in 1..=val {
        let mut str = String::new();
        if i % 3 == 0 {
            str += "Fizz";
        }
        if i % 5 == 0 {
            str += "Buzz";
        }
        if str.is_empty() {
            str = i.to_string();
        }
        result.push(str);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz_buzz_test() {
        assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
