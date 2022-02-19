use lambda_http::{handler, lambda_runtime::{self, Context, Error}, IntoResponse, Request, RequestExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(|event: Request, ctx: Context| {
        execute(event, ctx)
    }))
    .await?;
    Ok(())
}

pub async fn execute(request: Request, _ctx: Context) -> Result<impl IntoResponse, Error> {
  println!("{:?}", request);

   Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .get("name")
            .unwrap_or_else(|| "stranger")
    ))
}