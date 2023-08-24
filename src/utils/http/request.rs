use httparse::Request;

use crate::prelude::Result;

use crate::prelude::error::*;

pub async fn parse_path<'a>(
    buf: &'a [u8],
    // 1.5 days spent to understand that i can just pass `headers` as an argument
    headers: &'a mut [httparse::Header<'a>; 16],
) -> Result<(Vec<String>, Request<'a, 'a>)> {
    let mut req = httparse::Request::new(headers);

    let res = req.parse(buf).unwrap();

    if res.is_partial() {
        match req.path {
            Some(path) => {
                let segments = path.split('/').map(|s| s.to_string()).collect::<Vec<_>>();
                Ok((segments, req))
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
