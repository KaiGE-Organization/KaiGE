use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("This is a testing error for kaiGE.")]
    TestError,
}