use clover::runtime::object::{NativeModel, Object};
use clover::runtime::program::RuntimeError;

mod pow;

pub struct Math;

impl NativeModel for Math {
    fn model_get(&self, key: &str) -> Result<Object, RuntimeError> {
        match key {
            "pow" => Ok(Object::NativeFunction(pow::pow)),
            _ => Ok(Object::Null)
        }
    }
}
