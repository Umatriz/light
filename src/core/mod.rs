use std::fmt::Debug;

use http_types::{Request, Response};

use crate::prelude::Result;

mod endpoint;

#[async_trait::async_trait]
pub trait Core
where
    Self: Debug,
{
    async fn send(&self, request: Request) -> Response {
        match request.method() {
            http_types::Method::Connect => todo!(),
            http_types::Method::Delete => todo!(),
            http_types::Method::Get => todo!(),
            http_types::Method::Head => todo!(),
            http_types::Method::Options => todo!(),
            http_types::Method::Post => todo!(),
            http_types::Method::Put => todo!(),
            _ => not_found(),
        }
    }

    fn path(&self) -> String;
}

fn not_found() -> Response {
    Response::new(http_types::StatusCode::BadRequest)
}

// #[cfg(test)]
// mod test {
//     use std::any::Any;

//     use super::*;

//     #[derive(Debug)]
//     pub struct AuthCore;

//     #[async_trait::async_trait]
//     impl Core for AuthCore {
//         type Response = ();

//         async fn send(&self) -> crate::prelude::Result<Response<Self::Response>> {
//             let resp = Response::new(());

//             Ok(resp)
//         }

//         fn path(&self) -> String {
//             String::from("test")
//         }
//     }

//     pub struct AuthCluster {
//         cores: Vec<Box<dyn Core<Response = dyn Any>>>,
//     }
// }
