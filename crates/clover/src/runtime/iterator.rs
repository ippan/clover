use crate::runtime::state::State;
use crate::runtime::program::RuntimeError;
use crate::runtime::object::Object;

pub fn for_next(state: &mut State, enumerable_index: usize) -> Result<(), RuntimeError> {
    let iterator_index = enumerable_index + 1;

    let enumerable = state.current_frame().locals[enumerable_index].clone();

    let iterator = if let Object::Integer(iterator) = state.current_frame().locals[iterator_index].clone() {
        iterator
    } else {
        0
    };

    let jump = match enumerable {
        Object::Integer(value) => {
            if iterator < value {
                state.push(Object::Integer(iterator));
                false
            } else {
                // iterator greater than enumerable object, finish loop
                true
            }
        },
        Object::Array(array) => {
            let index = iterator as usize;

            if index < array.borrow().len() {
                state.push(array.borrow()[index].clone());
                false
            } else {
                // iterator greater than enumerable array len, finish loop
                true
            }
        }
        _ => true
    };

    state.push(Object::Boolean(jump));

    Ok(())
}

pub fn iterate(state: &mut State, iterator_index: usize) {
    if let Object::Integer(iterator) = state.current_frame().locals[iterator_index].clone() {
        state.current_frame_as_mut().locals[iterator_index] = Object::Integer(iterator + 1);
    };
}