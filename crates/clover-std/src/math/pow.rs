use clover::runtime::object::Object;
use clover::runtime::program::RuntimeError;
use clover::runtime::state::State;

pub fn pow(state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
    if parameters.len() != 2 {
        return Err(RuntimeError::new(format!("except 2 parameters, got {}", parameters.len()).as_str(), state.last_position()));
    };

    let base_object = parameters[0].clone();
    let exponent_object = parameters[1].clone();

    let type_error = Err(RuntimeError::new("Math.pow can accept Integer or Float only", state.last_position()));

    Ok(match base_object {
        Object::Integer(base) => {
            match exponent_object {
                Object::Integer(exponent) => {
                    if exponent >= 0 {
                        Object::Integer(base.pow(exponent as u32))
                    } else {
                        return Err(RuntimeError::new("Integer can have exponent greater or equal zero", state.last_position()));
                    }
                },
                Object::Float(exponent) => {
                    Object::Float((base as f64).powf(exponent))
                }
                _ => { return type_error; }
            }

        },
        Object::Float(base) => {
            match exponent_object {
                Object::Integer(exponent) => {
                    Object::Float(base.powf(exponent as f64))
                },
                Object::Float(exponent) => {
                    Object::Float(base.powf(exponent))
                }
                _ => { return type_error; }
            }
        }
        _ => { return type_error; }
    })
}
