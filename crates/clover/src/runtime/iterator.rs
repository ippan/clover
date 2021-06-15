use crate::runtime::state::State;
use crate::runtime::program::RuntimeError;
use crate::runtime::object::Object;

impl State {
    pub fn for_next(&mut self, enumerable_index: usize) -> Result<(), RuntimeError> {
        let iterator_index = enumerable_index + 1;

        let enumerable = self.current_frame().locals[enumerable_index].clone();

        let iterator = if let Object::Integer(iterator) = self.current_frame().locals[iterator_index].clone() {
            iterator
        } else {
            0
        };

        let jump = match enumerable {
            Object::Integer(value) => {
                if iterator < value {
                    self.push(Object::Integer(iterator));
                    false
                } else {
                    // iterator greater than enumerable object, finish loop
                    true
                }
            },
            Object::Array(array) => {
                let index = iterator as usize;

                if index < array.borrow().len() {
                    self.push(array.borrow()[index].clone());
                    false
                } else {
                    // iterator greater than enumerable array len, finish loop
                    true
                }
            },
            Object::Instance(instance) => {
                let model_index = instance.borrow().model_index;
                let model = &self.get_program().models[model_index];
                let index = iterator as usize;

                if index < model.property_names.len() {
                    let object = Object::String(model.property_names[index].clone());
                    self.push(object);
                    false
                } else {
                    true
                }
            }
            _ => true
        };

        self.push(Object::Boolean(jump));

        Ok(())
    }

    pub fn iterate(&mut self, iterator_index: usize) {
        if let Object::Integer(iterator) = self.current_frame().locals[iterator_index].clone() {
            self.current_frame_as_mut().locals[iterator_index] = Object::Integer(iterator + 1);
        };
    }
}