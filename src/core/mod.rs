use std::borrow::Cow;

use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Method, Request, Response};

mod endpoint;

pub struct Core<F>
where
    F: Fn(Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>,
{
    method: Method,
    path: Cow<'static, str>,
    func: F,
}
