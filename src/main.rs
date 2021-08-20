use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    Ok(format!("🦀 Hello, Netlify 🦀\n{}", agent))
}
