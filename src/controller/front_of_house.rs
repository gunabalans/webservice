mod hosting {
    fn add_to_waitlist() {}

    fn seat_at_table() {}
}


pub mod serving {
    use std::convert::Infallible;

    use hyper::{Request, Body, Response};

    pub async fn take_order(_req: Request<Body>)-> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("take_order page")))
    }
    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use me::controller::front_of_house::serving::serve_order;
    ///
    /// assert_eq!(serve_order(_req), );
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn serve_order(_req: Request<Body>)-> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("serve_order page")))
    }


    pub async fn take_payment(_req: Request<Body>)-> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("take_payment page")))
    }
}