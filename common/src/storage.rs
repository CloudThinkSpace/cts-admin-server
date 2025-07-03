use anyhow::Result;
use axum::{body::Bytes, BoxError};
use futures_util::stream::Stream;

pub mod file;
pub mod oss;

pub trait Storage<T> {
    fn read(&mut self) -> Result<T>;
    fn write<S, E>(&mut self) -> Result<Vec<u8>>
    where
        S: Stream<Item = Result<Bytes, E>>,
        E: Into<BoxError>;
}
