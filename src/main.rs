use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let file = std::fs::read_to_string("./views/hello.html").expect("path not found!");
    Ok(format!("{}", file))
}
