use lambda_runtime::{service_fn, Error, LambdaEvent};

/// The Lambda handler function
async fn handle_event(_: LambdaEvent<()>) -> Result<&'static str, Error> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handle_event);
    lambda_runtime::run(func).await?;
    Ok(())
}
