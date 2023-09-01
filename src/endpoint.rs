use std::future::Future;

use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Request, Response};

#[async_trait]
pub trait Endpoint: Send + Sync + 'static {
    async fn call(
        &self,
        req: Request<Incoming>,
    ) -> crate::prelude::Result<Response<BoxBody<Bytes, hyper::Error>>>;
}

#[async_trait]
impl<F, Fut, Res> Endpoint for F
where
    F: Send + Sync + 'static + Fn(Request<Incoming>) -> Fut,
    Fut: Future<Output = crate::prelude::Result<Res>> + Send + 'static,
    Res: Into<Bytes>,
{
    async fn call(
        &self,
        req: Request<Incoming>,
    ) -> crate::prelude::Result<Response<BoxBody<Bytes, hyper::Error>>> {
        let fut = (self)(req);
        let res = fut.await?;
        Ok(res.into())
    }
}
