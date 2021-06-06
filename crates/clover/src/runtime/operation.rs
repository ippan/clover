use crate::runtime::object::Object;
use crate::runtime::program::RuntimeError;
use crate::runtime::state::State;

const META_METHODS: &[ &str ] = &[ "_add", "_sub", "_mul", "_div", "_mod", "_eq", "_gt", "_lt" ];



fn integer_add(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left + value)),

        _ => Err(RuntimeError::new("can not add integer with object", state.last_position()))
    }
}

fn integer_sub(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left - value)),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_mul(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left * value)),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_div(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left / value)),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_mod(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Integer(left % value)),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_eq(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left == *value)),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_gt(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left > *value)),

        _ => Err(RuntimeError::new("can not sub integer with object", state.last_position()))
    }
}

fn integer_lt(state: &State, left: i64, right: &Object) -> Result<Object, RuntimeError> {
    match right {
        Object::Integer(value) => Ok(Object::Boolean(left < *value)),

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
        _ => Err(RuntimeError::new("unknown operation", state.last_position()))
    }
}


pub fn binary_operation(state: &State, left: &Object, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
    match left {
        Object::Integer(value) => integer_operation(state, *value, right, operand),

        _ => Err(RuntimeError::new("unknown object", state.last_position()))
    }
}