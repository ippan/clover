use crate::runtime::object::Object;
use crate::runtime::program::RuntimeError;
use crate::runtime::state::State;

const META_METHODS: &[ &str ] = &[ "_add", "_sub", "_mul", "_div", "_mod", "_eq", "_gt", "_lt", "_gte", "_lte" ];



fn integer_add(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left + value)),
        Object::Float(_) => float_add(state, left as f64, right),
        Object::String(value) => Ok(Object::String(left.to_string() + value)),

        _ => Err(RuntimeError::new("can not add integer with object", state.last_position()))
    }
}

fn integer_sub(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left - value)),
        Object::Float(_) => float_sub(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_mul(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left * value)),
        Object::Float(_) => float_mul(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_div(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left / value)),
        Object::Float(_) => float_div(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_mod(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left % value)),
        Object::Float(_) => float_mod(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_eq(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left == *value)),
        Object::Float(_) => float_eq(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_gt(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left > *value)),
        Object::Float(_) => float_gt(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_lt(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left < *value)),
        Object::Float(_) => float_lt(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_gte(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left >= *value)),
        Object::Float(_) => float_gte(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_lte(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left <= *value)),
        Object::Float(_) => float_lte(state, left as f64, right),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_operation(state: &State, left: i64, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
    match operand {
        0 => integer_add(state, left, right),
        1 => integer_sub(state, left, right),
        2 => integer_mul(state, left, right),
        3 => integer_div(state, left, right),
        4 => integer_mod(state, left, right),
        5 => integer_eq(state, left, right),
        6 => integer_gt(state, left, right),
        7 => integer_lt(state, left, right),
        8 => integer_gte(state, left, right),
        9 => integer_lte(state, left, right),

        _ => Err(RuntimeError::new("unknown operation", state.last_position()))
    }
}

fn float_add(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Float(left + value)),
        Object::Integer(value) => Ok(Object::Float(left + *value as f64)),
        Object::String(value) => Ok(Object::String(left.to_string() + value)),

        _ => Err(RuntimeError::new("can not add float with object", state.last_position()))
    }
}

fn float_sub(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Float(left - value)),
        Object::Integer(value) => Ok(Object::Float(left - *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_mul(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Float(left * value)),
        Object::Integer(value) => Ok(Object::Float(left * *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_div(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Float(left / value)),
        Object::Integer(value) => Ok(Object::Float(left / *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_mod(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Float(left % value)),
        Object::Integer(value) => Ok(Object::Float(left % *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_eq(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Boolean(left == *value)),
        Object::Integer(value) => Ok(Object::Boolean(left == *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_gt(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Boolean(left > *value)),
        Object::Integer(value) => Ok(Object::Boolean(left > *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_lt(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Boolean(left < *value)),
        Object::Integer(value) => Ok(Object::Boolean(left < *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_gte(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Boolean(left >= *value)),
        Object::Integer(value) => Ok(Object::Boolean(left >= *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_lte(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Boolean(left <= *value)),
        Object::Integer(value) => Ok(Object::Boolean(left <= *value as f64)),

        _ => Err(RuntimeError::new("can not sub float with object", state.last_position()))
    }
}

fn float_operation(state: &State, left: f64, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
    match operand {
        0 => float_add(state, left, right),
        1 => float_sub(state, left, right),
        2 => float_mul(state, left, right),
        3 => float_div(state, left, right),
        4 => float_mod(state, left, right),
        5 => float_eq(state, left, right),
        6 => float_gt(state, left, right),
        7 => float_lt(state, left, right),
        8 => float_gte(state, left, right),
        9 => float_lte(state, left, right),

        _ => Err(RuntimeError::new("unknown operation", state.last_position()))
    }
}

fn string_operation(state: &State, left: &str, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
    match operand {
        0 => {
            match right {
                Object::String(_) | Object::Integer(_) | Object::Float(_) | Object::Boolean(_) | Object::Null => Ok(Object::String(left.to_string() + &right.to_string())),
                _ => Err(RuntimeError::new("can not add string with object", state.last_position()))
            }
        },

        _ => Err(RuntimeError::new("unknown operation", state.last_position()))
    }
}

pub fn binary_operation(state: &State, left: &Object, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
    match left {
        Object::Integer(value) => integer_operation(state, *value, right, operand),
        Object::Float(value) => float_operation(state, *value, right, operand),
        Object::String(value) => string_operation(state, value, right, operand),

        _ => Err(RuntimeError::new("unknown object", state.last_position()))
    }
}

pub fn negative_operation(state: &State, target: &Object) -> Result<Object, RuntimeError> {
    match target {
        Object::Integer(value) => Ok(Object::Integer(-*value)),
        Object::Float(value) => Ok(Object::Float(-*value)),

        _ => Err(RuntimeError::new("object can not do minus operation", state.last_position()))
    }
}