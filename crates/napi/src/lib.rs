extern crate napi_sys;

mod result;
mod env;

pub use result::{NapiError, NapiResult};
pub use env::NapiEnv;

pub mod sys {
    pub use napi_sys::*;
}
