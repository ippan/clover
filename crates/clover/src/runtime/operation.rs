use crate::runtime::object::Object;
use crate::runtime::program::RuntimeError;
use crate::runtime::state::State;

const META_METHODS: &[ &str ] = &[ "_add", "_sub", "_mul", "_div", "_mod", "_eq", "_gt", "_lt" ];



fn integer_add(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left + value)),
        Object::Float(_) => float_add(state, left as f64, right),

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

        // 256 | 6
        0x106 => Ok(Object::Boolean(integer_gt(state, left, right)?.to_bool() | integer_eq(state, left, right)?.to_bool())),
        // 256 | 7
        0x107 => Ok(Object::Boolean(integer_lt(state, left, right)?.to_bool() | integer_eq(state, left, right)?.to_bool())),
        _ => Err(RuntimeError::new("unknown operation", state.last_position()))
    }
}

fn float_add(state: &State, left: f64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Float(value) => Ok(Object::Float(left + value)),
        Object::Integer(value) => Ok(Object::Float(left + *value as f64)),

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

        // 256 | 6
        0x106 => Ok(Object::Boolean(float_gt(state, left, right)?.to_bool() | float_eq(state, left, right)?.to_bool())),
        // 256 | 7
        0x107 => Ok(Object::Boolean(float_lt(state, left, right)?.to_bool() | float_eq(state, left, right)?.to_bool())),
        _ => Err(RuntimeError::new("unknown operation", state.last_position()))
    }
}

pub fn binary_operation(state: &State, left: &Object, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
    match left {
        Object::Integer(value) => integer_operation(state, *value, right, operand),
        Object::Float(value) => float_operation(state, *value, right, operand),

        _ => Err(RuntimeError::new("unknown object", state.last_position()))
    }
}