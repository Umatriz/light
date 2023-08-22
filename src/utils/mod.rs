use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::prelude::Result;

use crate::prelude::error::*;

pub async fn parse_path(mut stream: TcpStream) -> Result<Vec<String>> {
    let mut buf = vec![0; 1024];
    let m = stream.read(&mut buf).await.unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let res = req.parse(&buf[..m]).unwrap();

    if res.is_partial() {
        match req.path {
            Some(path) => {
                let segments = path.split('/').map(|s| s.to_string()).collect::<Vec<_>>();
                Ok(segments)
            }
            None => Err(Error::Light {
                message: "Error while path parsing".into(),
                kind: LightError::PathParse,
            }),
        }
    } else {
        Err(Error::Light {
            message: "Error while path parsing".into(),
            kind: LightError::PathParse,
        })
    }
}
