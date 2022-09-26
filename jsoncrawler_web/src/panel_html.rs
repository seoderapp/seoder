use hyper::{Body, Request, Response};
use std::convert::Infallible;

/// generate the web panel
pub async fn panel_handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Crawler panel interface setup".into()))
}
