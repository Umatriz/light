pub struct Wrapper<T>(T);

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Path parsing error")]
    PathParse,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
