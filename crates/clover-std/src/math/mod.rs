use clover::{NativeModel, Object};
use clover::debug::RuntimeError;
use std::f64::consts::PI;

mod pow;
mod trigonometric;

pub struct Math;

impl NativeModel for Math {
    fn model_get(&self, key: &str) -> Result<Object, RuntimeError> {
        match key {
            "pow" => Ok(Object::NativeFunction(pow::pow)),

            // trigonometric
            "sin" => Ok(Object::NativeFunction(trigonometric::sin)),
            "cos" => Ok(Object::NativeFunction(trigonometric::cos)),

            "PI" => Ok(Object::Float(PI)),
            _ => Ok(Object::Null)
        }
    }
}
