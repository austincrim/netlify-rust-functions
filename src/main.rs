use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};
extern crate chrono;
use chrono::Local;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let now = Local::now();
    Ok(format!("{} requested at {}", agent, now.format("%Y-%m-%d %H:%M:%S")))
}
