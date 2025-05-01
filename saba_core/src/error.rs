use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    UnexpectedInput(String),
    InvalidResponse(String),
    Other(String),
}
