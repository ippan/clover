use clover::{State, Object};
use clover::debug::RuntimeError;

pub fn expect_parameter_count(state: &State, parameters: &[ Object ], count: usize) -> Result<(), RuntimeError> {
    if parameters.len() != count {
        return Err(RuntimeError::new(format!("except {} parameters, got {}", count, parameters.len()).as_str(), state.last_position()));
    };

    Ok(())
}

pub fn expect_float(state: &State, object: &Object) -> Result<f64, RuntimeError> {
    match object {
        Object::Float(value) => Ok(*value),
        _ => Err(RuntimeError::new("can accept Float only", state.last_position()))
    }
}