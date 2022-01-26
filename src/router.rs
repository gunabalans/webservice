use std::convert::Infallible;

use hyper::{Body, Method, Request, Response};
use lazy_static::lazy_static;
use regex::Regex;

use crate::controller::front_of_house::serving;

fn is_digit(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^/user/\d+$"#).unwrap();
    }
    RE.is_match(text)
}

fn is_word(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^/\w+$"#).unwrap();
    }
    RE.is_match(text)
}

async fn more_match(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    //let method = req.method();
    let uri = req.uri();
    let path = uri.path();

    if is_digit(path) {
        Ok(Response::new(Body::from("match gidigt")))
    } else if is_word(path) {
        serving::take_order(req).await
    } else {
        Ok(Response::new(Body::from("Page Not Found")))
    }
}

pub async fn routes(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method();
    let uri = req.uri();
    let path = uri.path();

    match (method, path) {
        (&Method::GET, "/") => serving::take_order(req).await,
        (&Method::GET, "/test") => serving::serve_order(req).await,
        (_, _) => more_match(req).await
    }
}
