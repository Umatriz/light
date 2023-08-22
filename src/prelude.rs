pub struct Wrapper<T>(T);

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Message: {message};\nKind: {kind:?}")]
        Light { message: String, kind: LightError },

        #[error(transparent)]
        Io(#[from] std::io::Error),
    }

    #[derive(Debug)]
    pub enum LightError {
        PathParse,
    }

    #[test]
    fn error_test() {
        let err = || -> Result<(), Error> {
            Err(Error::Light {
                message: "test".into(),
                kind: LightError::PathParse,
            })
        };

        err().unwrap()
    }
}
