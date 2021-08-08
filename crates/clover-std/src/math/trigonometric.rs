use clover::{State, Object};
use clover::debug::RuntimeError;
use crate::helper::{expect_parameter_count, expect_float};

pub fn sin(state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
    expect_parameter_count(state, parameters, 1)?;
    Ok(Object::Float(expect_float(state, &parameters[0])?.sin()))
}

pub fn cos(state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
    expect_parameter_count(state, parameters, 1)?;
    Ok(Object::Float(expect_float(state, &parameters[0])?.cos()))
}