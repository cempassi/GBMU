use crate::Cpu;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

pub type Decode = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub trait Decoder {
    fn decode(self, cpu: Cpu) -> Decode;
}
