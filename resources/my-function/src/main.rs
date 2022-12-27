use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let method = event.method().to_string();
    let path = event.raw_http_path();

    let message = format!("Hello from AWS Lambda, you did an HTTP {} request for {}\n", method, path);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    // Just out of curiosity, never worked with rust before and it's interesting that we don't use `await` below as we would in js.(I mean we only use async at the top, or is it a bug?) 
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
