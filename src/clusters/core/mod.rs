use http::Response;

#[async_trait::async_trait]
pub trait Core {
    type Resp;

    async fn send(&self) -> crate::prelude::Result<Response<Self::Resp>>;

    fn path(&self) -> String;
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    pub struct AuthCore;

    #[async_trait::async_trait]
    impl Core for AuthCore {
        type Resp = ();

        async fn send(&self) -> crate::prelude::Result<Response<Self::Resp>> {
            let resp = Response::new(());

            Ok(resp)
        }

        fn path(&self) -> String {
            String::from("test")
        }
    }

    pub struct AuthCluster {
        cores: Vec<Box<dyn Core<Resp = dyn Any>>>,
    }
}
